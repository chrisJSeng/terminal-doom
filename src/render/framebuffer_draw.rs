use std::io::{self, Write};

use crossterm::{
    cursor, execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

use crate::constants::{
    ERR_FRAMEBUFFER_INVALID_DIMENSIONS, ERR_FRAMEBUFFER_OVERFLOW, ERR_FRAMEBUFFER_SIZE,
    FRAMEBUFFER_ADAPTIVE_HIGH_CONTRAST_DELTA, FRAMEBUFFER_ADAPTIVE_LOW_CONTRAST_DELTA,
    FRAMEBUFFER_COLOR_MAX_F32, FRAMEBUFFER_CONTRAST, FRAMEBUFFER_DENSITY_LUMA_MAX,
    FRAMEBUFFER_EDGE_DETECTION_THRESHOLD, FRAMEBUFFER_FLAT_CELL_THRESHOLD, FRAMEBUFFER_GAMMA,
    FRAMEBUFFER_GLYPH_HYSTERESIS_CHROMA, FRAMEBUFFER_GLYPH_HYSTERESIS_LUMA,
    FRAMEBUFFER_HALF_BLOCK_LUMA_THRESHOLD, FRAMEBUFFER_HEAVY_DENSITY_CHROMA_THRESHOLD,
    FRAMEBUFFER_LUMA_BLUE_WEIGHT, FRAMEBUFFER_LUMA_GREEN_WEIGHT, FRAMEBUFFER_LUMA_RED_WEIGHT,
    FRAMEBUFFER_LUMA_WEIGHT_DIVISOR, FRAMEBUFFER_MIDPOINT, FRAMEBUFFER_SAMPLE_GRID_BASE,
    FRAMEBUFFER_SAMPLE_GRID_EDGE_DETECTION, FRAMEBUFFER_SAMPLE_GRID_MAX,
    FRAMEBUFFER_SAMPLE_GRID_MIN, FRAMEBUFFER_SAMPLE_GRID_PROBE_MIN,
    FRAMEBUFFER_SAMPLE_POSITION_CENTER_OFFSET, FRAMEBUFFER_SATURATION_BOOST,
    FRAMEBUFFER_SHADOW_LIFT, FRAMEBUFFER_SIGNATURE_ROTATE_STEP_BITS,
    FRAMEBUFFER_SIGNATURE_SAMPLE_SPLITS, FRAMEBUFFER_SOFT_BLEND_CELL_THRESHOLD,
    FRAMEBUFFER_SOFT_CLIP, FRAMEBUFFER_SOFT_CLIP_STRENGTH, FRAMEBUFFER_STATUS_RESERVED_ROWS,
    FRAMEBUFFER_UI_SHARPEN_BONUS, FRAMEBUFFER_UI_SHARPEN_BOTTOM_EXTRA_BONUS,
    FRAMEBUFFER_UI_SHARPEN_BOTTOM_START_RATIO, FRAMEBUFFER_UI_SHARPEN_TOP_END_RATIO,
    FRAMEBUFFER_UNIT_INTERVAL_MAX, FRAMEBUFFER_UNIT_INTERVAL_MIN, FRAMEBUFFER_UNSHARP_STRENGTH,
    FRAMEBUFFER_VIEWPORT_MIN_CELL_WIDTH, FRAMEBUFFER_VIEWPORT_MIN_PIXEL_HEIGHT,
    FRAMEBUFFER_VIEWPORT_MIN_SCALE_NUMERATOR, FRAMEBUFFER_VIEWPORT_TOP_OFFSET,
    RENDER_CELL_DENSITY_HEAVY, RENDER_CELL_SOLID, RENDER_CELL_UPPER_HALF, RGB_CHANNELS,
};
use crate::types::{
    AccumulateGridSamplesParams, AccumulateSampleColorParams, ApplyUnsharpMaskParams,
    ApproximateLumaParams, AveragedColorInRegionParams, CoarseFramebufferCellSignatureParams,
    ColorPairParams, DensityCellParams, DrawFramebufferParams, FlushFramebufferSpanParams,
    FramebufferCellCacheEntry, FramebufferCellSourceWindow, FramebufferCellSourceWindowParams,
    FramebufferCellSpan, FramebufferCellStyle, FramebufferCellStyleParams,
    FramebufferRegionSamplingParams, FramebufferValidationParams, FramebufferViewport,
    FramebufferViewportParams, FramebufferViewportSignature, GlyphStyleParams,
    PrepareFramebufferCacheParams, ProcessFramebufferRowParams, PushFramebufferSpanParams,
    RegionUnsharpStrengthParams, RenderFramebufferCellParams, RenderFramebufferLoopParams,
    SampleLumaGridParams, SamplePositionParams, SaturateToneMappedColorParams,
    ShouldKeepPreviousGlyphParams, TerminalRenderer, ToneMappedColor,
};

impl TerminalRenderer {
    pub fn draw_framebuffer_rgb(&mut self, params: DrawFramebufferParams) -> io::Result<()> {
        let DrawFramebufferParams {
            rgb,
            source_width,
            source_height,
        } = params;

        let required_len = source_width
            .checked_mul(source_height)
            .and_then(|px| px.checked_mul(RGB_CHANNELS))
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, ERR_FRAMEBUFFER_OVERFLOW))?;

        let available_width = self.cols as usize;
        let available_height =
            self.rows
                .saturating_sub(FRAMEBUFFER_STATUS_RESERVED_ROWS as u16) as usize;

        let viewport_params = FramebufferViewportParams {
            available_width,
            available_height,
            source_width,
            source_height,
        };
        let viewport = framebuffer_viewport(viewport_params);

        let prepare_cache_params = PrepareFramebufferCacheParams { viewport };

        self.prepare_framebuffer_cache(prepare_cache_params)?;

        let validation_params = FramebufferValidationParams {
            rgb_len: rgb.len(),
            required_len,
            target_width: viewport.width,
            target_height: viewport.height,
            source_width,
            source_height,
        };

        self.validate_framebuffer(validation_params)?;

        let draw_params = DrawFramebufferParams {
            rgb,
            source_width,
            source_height,
        };
        let loop_params = RenderFramebufferLoopParams {
            render_params: draw_params,
            viewport,
        };

        self.render_framebuffer_loop(loop_params)?;

        execute!(self.stdout, ResetColor)?;
        self.stdout.flush()
    }

    fn validate_framebuffer(&mut self, params: FramebufferValidationParams) -> io::Result<()> {
        let FramebufferValidationParams {
            rgb_len,
            required_len,
            target_width,
            target_height,
            source_width,
            source_height,
        } = params;

        if rgb_len < required_len {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                ERR_FRAMEBUFFER_SIZE,
            ));
        }

        let target_dims_valid = target_width > 0 && target_height > 0;
        let source_dims_valid = source_width > 0 && source_height > 0;
        let all_dimensions_valid = target_dims_valid && source_dims_valid;

        if !all_dimensions_valid {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                ERR_FRAMEBUFFER_INVALID_DIMENSIONS,
            ));
        }

        Ok(())
    }

    fn render_framebuffer_loop(&mut self, params: RenderFramebufferLoopParams) -> io::Result<()> {
        let RenderFramebufferLoopParams {
            render_params,
            viewport,
        } = params;

        let DrawFramebufferParams {
            rgb,
            source_width,
            source_height,
        } = render_params;

        let scale_x = source_width as f32 / viewport.width.max(1) as f32;
        let scale_y = source_height as f32 / (viewport.height * 2).max(1) as f32;

        for target_row in 0..viewport.height {
            let row_params = ProcessFramebufferRowParams {
                rgb,
                source_width,
                source_height,
                target_row,
                scale_x,
                scale_y,
                viewport,
            };
            self.process_framebuffer_row(row_params)?;
        }

        Ok(())
    }

    fn process_framebuffer_row(&mut self, params: ProcessFramebufferRowParams) -> io::Result<()> {
        let ProcessFramebufferRowParams {
            rgb,
            source_width,
            source_height,
            target_row,
            scale_x,
            scale_y,
            viewport,
        } = params;

        let mut queued_span: Option<FramebufferCellSpan> = None;

        for target_column in 0..viewport.width {
            let render_cell_params = RenderFramebufferCellParams {
                rgb,
                source_width,
                source_height,
                target_row,
                target_column,
                scale_x,
                scale_y,
                viewport,
                queued_span: &mut queued_span,
            };
            self.render_framebuffer_cell(render_cell_params)?;
        }

        let flush_span_params = FlushFramebufferSpanParams {
            stdout: &mut self.stdout,
            queued_span: &mut queued_span,
        };
        Self::flush_framebuffer_span(flush_span_params)?;

        Ok(())
    }

    fn render_framebuffer_cell(&mut self, params: RenderFramebufferCellParams) -> io::Result<()> {
        let RenderFramebufferCellParams {
            rgb,
            source_width,
            source_height,
            target_row,
            target_column,
            scale_x,
            scale_y,
            viewport,
            queued_span,
        } = params;

        let source_window_params = FramebufferCellSourceWindowParams {
            target_column,
            target_row,
            scale_x,
            scale_y,
        };

        let source_window = framebuffer_cell_source_window(source_window_params);
        let screen_x = target_column + viewport.offset_x;
        let screen_y = target_row + viewport.offset_y + FRAMEBUFFER_STATUS_RESERVED_ROWS;
        let cache_index = self.framebuffer_cache_index(screen_x, screen_y);
        let previous_entry = self.framebuffer_cache[cache_index];

        let coarse_signature_params = CoarseFramebufferCellSignatureParams {
            rgb,
            source_width,
            source_height,
            source_column_start: source_window.source_column_start,
            source_column_end: source_window.source_column_end,
            upper_source_row_start: source_window.upper_source_row_start,
            upper_source_row_end: source_window.upper_source_row_end,
            lower_source_row_start: source_window.lower_source_row_start,
            lower_source_row_end: source_window.lower_source_row_end,
        };

        let coarse_signature = coarse_framebuffer_cell_signature(coarse_signature_params);
        let has_matching_signature =
            previous_entry.is_some_and(|cached| cached.source_signature == coarse_signature);

        if has_matching_signature {
            let flush_cache_hit_params = FlushFramebufferSpanParams {
                stdout: &mut self.stdout,
                queued_span,
            };

            return Self::flush_framebuffer_span(flush_cache_hit_params);
        }

        let upper_sampling_params = FramebufferRegionSamplingParams {
            rgb,
            source_width,
            source_height,
            source_column_start: source_window.source_column_start,
            source_column_end: source_window.source_column_end,
            source_row_start: source_window.upper_source_row_start,
            source_row_end: source_window.upper_source_row_end,
        };
        let upper_color = sampled_region_color(upper_sampling_params);
        let lower_sampling_params = FramebufferRegionSamplingParams {
            rgb,
            source_width,
            source_height,
            source_column_start: source_window.source_column_start,
            source_column_end: source_window.source_column_end,
            source_row_start: source_window.lower_source_row_start,
            source_row_end: source_window.lower_source_row_end,
        };

        let lower_color = sampled_region_color(lower_sampling_params);
        let cell_style_params = FramebufferCellStyleParams {
            upper_color,
            lower_color,
            previous_glyph: previous_entry.map(|entry| entry.glyph),
        };
        let cell = framebuffer_cell_style(cell_style_params);
        let has_matching_style = previous_entry.is_some_and(|cached| {
            cached.foreground == cell.foreground
                && cached.background == cell.background
                && cached.glyph == cell.glyph
        });

        self.framebuffer_cache[cache_index] = Some(cell.cache_entry(coarse_signature));

        if has_matching_style {
            let flush_style_params = FlushFramebufferSpanParams {
                stdout: &mut self.stdout,
                queued_span,
            };

            return Self::flush_framebuffer_span(flush_style_params);
        }

        let push_span_params = PushFramebufferSpanParams {
            stdout: &mut self.stdout,
            queued_span,
            screen_x,
            screen_y,
            cell,
        };

        Self::push_framebuffer_span(push_span_params)
    }

    fn push_framebuffer_span(params: PushFramebufferSpanParams) -> io::Result<()> {
        let PushFramebufferSpanParams {
            stdout,
            queued_span,
            screen_x,
            screen_y,
            cell,
        } = params;

        if let Some(span) = queued_span.as_mut() {
            if span.can_append(screen_x, screen_y, &cell) {
                span.push(cell.glyph);

                return Ok(());
            }
        }

        let flush_span_params = FlushFramebufferSpanParams {
            stdout,
            queued_span,
        };

        Self::flush_framebuffer_span(flush_span_params)?;

        *queued_span = Some(FramebufferCellSpan::new(screen_x, screen_y, cell));

        Ok(())
    }

    fn flush_framebuffer_span(params: FlushFramebufferSpanParams) -> io::Result<()> {
        let FlushFramebufferSpanParams {
            stdout,
            queued_span,
        } = params;
        let Some(span) = queued_span.take() else {
            return Ok(());
        };

        queue!(
            stdout,
            cursor::MoveTo(span.start_x as u16, span.y as u16),
            SetForegroundColor(span.foreground),
            SetBackgroundColor(span.background),
            Print(span.text)
        )
    }

    fn prepare_framebuffer_cache(
        &mut self,
        params: PrepareFramebufferCacheParams,
    ) -> io::Result<()> {
        let PrepareFramebufferCacheParams { viewport } = params;
        let viewport_signature = FramebufferViewportSignature {
            width: viewport.width,
            height: viewport.height,
            offset_x: viewport.offset_x,
            offset_y: viewport.offset_y,
        };
        let cache_size = self.cols as usize * self.rows as usize;
        let cache_shape_changed =
            self.framebuffer_cache_cols != self.cols || self.framebuffer_cache_rows != self.rows;
        let viewport_changed = self.framebuffer_viewport_signature != Some(viewport_signature);
        let cache_length_mismatch = self.framebuffer_cache.len() != cache_size;
        let should_rebuild_cache = cache_shape_changed || viewport_changed || cache_length_mismatch;

        if should_rebuild_cache {
            self.framebuffer_cache = vec![None; cache_size];
            self.framebuffer_cache_cols = self.cols;
            self.framebuffer_cache_rows = self.rows;
            self.framebuffer_viewport_signature = Some(viewport_signature);
            execute!(
                self.stdout,
                cursor::MoveTo(0, 0),
                crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
            )?;
        }

        Ok(())
    }

    fn framebuffer_cache_index(&self, x: usize, y: usize) -> usize {
        y * self.cols as usize + x
    }
}

fn framebuffer_cell_source_window(
    params: FramebufferCellSourceWindowParams,
) -> FramebufferCellSourceWindow {
    let FramebufferCellSourceWindowParams {
        target_column,
        target_row,
        scale_x,
        scale_y,
    } = params;

    FramebufferCellSourceWindow {
        source_column_start: target_column as f32 * scale_x,
        source_column_end: (target_column + 1) as f32 * scale_x,
        upper_source_row_start: (target_row * 2) as f32 * scale_y,
        upper_source_row_end: ((target_row * 2) + 1) as f32 * scale_y,
        lower_source_row_start: ((target_row * 2) + 1) as f32 * scale_y,
        lower_source_row_end: ((target_row * 2) + 2) as f32 * scale_y,
    }
}

fn sampled_region_color(params: FramebufferRegionSamplingParams) -> ToneMappedColor {
    let sample_grid = adaptive_sample_count(params);
    let averaged_color_params = AveragedColorInRegionParams {
        sampling: params,
        sample_grid,
    };

    averaged_color_in_region(averaged_color_params)
}

impl ToneMappedColor {
    fn to_terminal_color(self) -> Color {
        Color::Rgb {
            r: self.red,
            g: self.green,
            b: self.blue,
        }
    }
}

impl FramebufferCellStyle {
    fn cache_entry(&self, source_signature: u64) -> FramebufferCellCacheEntry {
        FramebufferCellCacheEntry {
            foreground: self.foreground,
            background: self.background,
            glyph: self.glyph,
            source_signature,
        }
    }
}

impl FramebufferCellSpan {
    fn new(screen_x: usize, screen_y: usize, cell: FramebufferCellStyle) -> Self {
        Self {
            start_x: screen_x,
            next_x: screen_x + 1,
            y: screen_y,
            foreground: cell.foreground,
            background: cell.background,
            text: String::from(cell.glyph),
        }
    }

    fn can_append(&self, screen_x: usize, screen_y: usize, cell: &FramebufferCellStyle) -> bool {
        self.y == screen_y
            && self.next_x == screen_x
            && self.foreground == cell.foreground
            && self.background == cell.background
    }

    fn push(&mut self, glyph: &'static str) {
        self.text.push_str(glyph);
        self.next_x += 1;
    }
}

fn framebuffer_viewport(params: FramebufferViewportParams) -> FramebufferViewport {
    let FramebufferViewportParams {
        available_width,
        available_height,
        source_width,
        source_height,
    } = params;

    if available_width == 0 || available_height == 0 {
        return FramebufferViewport {
            width: 0,
            height: 0,
            offset_x: 0,
            offset_y: 0,
        };
    }

    let available_pixel_width = available_width as f32;
    let available_pixel_height = (available_height * 2) as f32;
    let width_scale = available_pixel_width / source_width.max(1) as f32;
    let height_scale = available_pixel_height / source_height.max(1) as f32;

    let scale = width_scale
        .min(height_scale)
        .max(FRAMEBUFFER_VIEWPORT_MIN_SCALE_NUMERATOR / source_width.max(source_height) as f32);

    let viewport_width = (source_width as f32 * scale)
        .floor()
        .max(FRAMEBUFFER_VIEWPORT_MIN_CELL_WIDTH) as usize;

    let viewport_pixel_height = (source_height as f32 * scale)
        .floor()
        .max(FRAMEBUFFER_VIEWPORT_MIN_PIXEL_HEIGHT) as usize;

    let viewport_height = (viewport_pixel_height / 2).max(1).min(available_height);
    let clamped_width = viewport_width.min(available_width);
    let offset_x = available_width.saturating_sub(clamped_width) / 2;
    let offset_y = FRAMEBUFFER_VIEWPORT_TOP_OFFSET;

    FramebufferViewport {
        width: clamped_width,
        height: viewport_height,
        offset_x,
        offset_y,
    }
}

fn averaged_color_in_region(params: AveragedColorInRegionParams) -> ToneMappedColor {
    let AveragedColorInRegionParams {
        sampling,
        sample_grid,
    } = params;

    let FramebufferRegionSamplingParams {
        rgb,
        source_width,
        source_height,
        source_column_start,
        source_column_end,
        source_row_start,
        source_row_end,
    } = sampling;

    let blurred = {
        let accumulate_params = AccumulateGridSamplesParams {
            rgb,
            source_width,
            sample_grid,
            source_row_start,
            source_row_end,
            source_height,
            source_column_start,
            source_column_end,
        };
        let (red_total, green_total, blue_total, sample_count) =
            accumulate_grid_samples(accumulate_params);

        let blurred_color = ToneMappedColor::new(
            tone_map_channel((red_total / sample_count.max(1)) as u8),
            tone_map_channel((green_total / sample_count.max(1)) as u8),
            tone_map_channel((blue_total / sample_count.max(1)) as u8),
        );
        let blurred_saturate_params = SaturateToneMappedColorParams {
            color: blurred_color,
        };

        saturate_tone_mapped_color(blurred_saturate_params)
    };

    if sample_grid <= 1 {
        return blurred;
    }

    let sharp = {
        let sharp_sample_row_params = SamplePositionParams {
            region_start: source_row_start,
            region_end: source_row_end,
            sample_index: 0,
            sample_count: 1,
            source_limit: source_height,
        };
        let sample_row = sample_position(sharp_sample_row_params);
        let sharp_sample_col_params = SamplePositionParams {
            region_start: source_column_start,
            region_end: source_column_end,
            sample_index: 0,
            sample_count: 1,
            source_limit: source_width,
        };

        let sample_column = sample_position(sharp_sample_col_params);
        let pixel_offset = (sample_row * source_width + sample_column) * RGB_CHANNELS;
        let sharp_color = ToneMappedColor::new(
            tone_map_channel(rgb[pixel_offset]),
            tone_map_channel(rgb[pixel_offset + 1]),
            tone_map_channel(rgb[pixel_offset + 2]),
        );
        let sharp_saturate_params = SaturateToneMappedColorParams { color: sharp_color };

        saturate_tone_mapped_color(sharp_saturate_params)
    };

    let unsharp_strength = region_unsharp_strength(RegionUnsharpStrengthParams {
        source_row_start,
        source_row_end,
        source_height,
    });

    let unsharp_color = apply_unsharp_mask(ApplyUnsharpMaskParams {
        original: &sharp,
        blurred: &blurred,
        strength: unsharp_strength,
    });
    let final_saturate_params = SaturateToneMappedColorParams {
        color: unsharp_color,
    };

    saturate_tone_mapped_color(final_saturate_params)
}

fn framebuffer_cell_style(params: FramebufferCellStyleParams) -> FramebufferCellStyle {
    let FramebufferCellStyleParams {
        upper_color,
        lower_color,
        previous_glyph,
    } = params;

    let upper_luminance = perceived_luminance(upper_color);
    let lower_luminance = perceived_luminance(lower_color);
    let luma_delta = upper_luminance.abs_diff(lower_luminance);
    let color_delta = upper_color.red.abs_diff(lower_color.red) as u16
        + upper_color.green.abs_diff(lower_color.green) as u16
        + upper_color.blue.abs_diff(lower_color.blue) as u16;

    let should_keep_params = ShouldKeepPreviousGlyphParams {
        previous_glyph,
        luma_delta,
        color_delta,
    };

    if should_keep_previous_glyph(should_keep_params) {
        let glyph_style_params = GlyphStyleParams {
            glyph: previous_glyph.unwrap_or(RENDER_CELL_SOLID),
            upper_color,
            lower_color,
        };

        return glyph_style(glyph_style_params);
    }

    let is_flat_color_transition = color_delta <= FRAMEBUFFER_FLAT_CELL_THRESHOLD as u16;

    if is_flat_color_transition {
        let flat_cell_params = ColorPairParams {
            upper_color,
            lower_color,
        };

        return solid_blended_cell(flat_cell_params);
    }

    let is_soft_blend_color_range = color_delta <= FRAMEBUFFER_SOFT_BLEND_CELL_THRESHOLD as u16;
    let is_low_luma_delta = luma_delta < FRAMEBUFFER_HALF_BLOCK_LUMA_THRESHOLD as u16;
    let should_use_soft_blend = is_soft_blend_color_range && is_low_luma_delta;

    if should_use_soft_blend {
        let soft_blend_params = ColorPairParams {
            upper_color,
            lower_color,
        };

        return solid_blended_cell(soft_blend_params);
    }

    let should_use_upper_half_cell = luma_delta >= FRAMEBUFFER_HALF_BLOCK_LUMA_THRESHOLD as u16;

    if should_use_upper_half_cell {
        let upper_half_params = ColorPairParams {
            upper_color,
            lower_color,
        };

        return upper_half_cell(upper_half_params);
    }

    let is_density_luma_range = luma_delta <= FRAMEBUFFER_DENSITY_LUMA_MAX as u16;
    let is_heavy_density_chroma = color_delta >= FRAMEBUFFER_HEAVY_DENSITY_CHROMA_THRESHOLD as u16;
    let should_use_density_cell = is_density_luma_range && is_heavy_density_chroma;

    if should_use_density_cell {
        let density_params = DensityCellParams {
            upper_color,
            lower_color,
            upper_luminance,
            lower_luminance,
        };

        return density_cell(density_params);
    }

    let final_solid_params = ColorPairParams {
        upper_color,
        lower_color,
    };

    solid_blended_cell(final_solid_params)
}

fn should_keep_previous_glyph(params: ShouldKeepPreviousGlyphParams) -> bool {
    let ShouldKeepPreviousGlyphParams {
        previous_glyph,
        luma_delta,
        color_delta,
    } = params;

    match previous_glyph {
        Some(RENDER_CELL_UPPER_HALF) => {
            luma_delta + FRAMEBUFFER_GLYPH_HYSTERESIS_LUMA as u16
                >= FRAMEBUFFER_HALF_BLOCK_LUMA_THRESHOLD as u16
        }
        Some(RENDER_CELL_DENSITY_HEAVY) => {
            luma_delta
                <= FRAMEBUFFER_DENSITY_LUMA_MAX as u16 + FRAMEBUFFER_GLYPH_HYSTERESIS_LUMA as u16
                && color_delta + FRAMEBUFFER_GLYPH_HYSTERESIS_CHROMA as u16
                    >= FRAMEBUFFER_HEAVY_DENSITY_CHROMA_THRESHOLD as u16
        }
        Some(RENDER_CELL_SOLID) => {
            color_delta
                <= FRAMEBUFFER_SOFT_BLEND_CELL_THRESHOLD as u16
                    + FRAMEBUFFER_GLYPH_HYSTERESIS_CHROMA as u16
                && luma_delta
                    < FRAMEBUFFER_HALF_BLOCK_LUMA_THRESHOLD as u16
                        + FRAMEBUFFER_GLYPH_HYSTERESIS_LUMA as u16
        }
        _ => false,
    }
}

fn glyph_style(params: GlyphStyleParams) -> FramebufferCellStyle {
    let GlyphStyleParams {
        glyph,
        upper_color,
        lower_color,
    } = params;

    match glyph {
        RENDER_CELL_UPPER_HALF => {
            let upper_half_params = ColorPairParams {
                upper_color,
                lower_color,
            };
            upper_half_cell(upper_half_params)
        }
        RENDER_CELL_DENSITY_HEAVY => {
            let upper_lum = perceived_luminance(upper_color);
            let lower_lum = perceived_luminance(lower_color);
            let density_params = DensityCellParams {
                upper_color,
                lower_color,
                upper_luminance: upper_lum,
                lower_luminance: lower_lum,
            };
            density_cell(density_params)
        }
        _ => {
            let solid_params = ColorPairParams {
                upper_color,
                lower_color,
            };
            solid_blended_cell(solid_params)
        }
    }
}

fn upper_half_cell(params: ColorPairParams) -> FramebufferCellStyle {
    let ColorPairParams {
        upper_color,
        lower_color,
    } = params;

    FramebufferCellStyle {
        foreground: upper_color.to_terminal_color(),
        background: lower_color.to_terminal_color(),
        glyph: RENDER_CELL_UPPER_HALF,
    }
}

fn density_cell(params: DensityCellParams) -> FramebufferCellStyle {
    let DensityCellParams {
        upper_color,
        lower_color,
        upper_luminance,
        lower_luminance,
    } = params;

    let (bright_color, dark_color) = match upper_luminance >= lower_luminance {
        true => (upper_color, lower_color),
        false => (lower_color, upper_color),
    };

    FramebufferCellStyle {
        foreground: bright_color.to_terminal_color(),
        background: dark_color.to_terminal_color(),
        glyph: RENDER_CELL_DENSITY_HEAVY,
    }
}

fn solid_blended_cell(params: ColorPairParams) -> FramebufferCellStyle {
    let ColorPairParams {
        upper_color,
        lower_color,
    } = params;

    let blended = upper_color.blend(lower_color);

    FramebufferCellStyle {
        foreground: blended.to_terminal_color(),
        background: blended.to_terminal_color(),
        glyph: RENDER_CELL_SOLID,
    }
}

fn accumulate_grid_samples(params: AccumulateGridSamplesParams) -> (u32, u32, u32, u32) {
    let AccumulateGridSamplesParams {
        rgb,
        source_width,
        sample_grid,
        source_row_start,
        source_row_end,
        source_height,
        source_column_start,
        source_column_end,
    } = params;

    let mut red_total = 0u32;
    let mut green_total = 0u32;
    let mut blue_total = 0u32;
    let mut sample_count = 0u32;

    for sample_y in 0..sample_grid {
        let sample_row_params = SamplePositionParams {
            region_start: source_row_start,
            region_end: source_row_end,
            sample_index: sample_y,
            sample_count: sample_grid,
            source_limit: source_height,
        };
        let sample_row = sample_position(sample_row_params);

        let accumulate_params = AccumulateSampleColorParams {
            rgb,
            source_width,
            sample_grid,
            sample_row,
            source_column_start,
            source_column_end,
        };
        let (row_red, row_green, row_blue, row_count) =
            accumulate_sample_colors_in_row(accumulate_params);

        red_total += row_red;
        green_total += row_green;
        blue_total += row_blue;
        sample_count += row_count;
    }

    (red_total, green_total, blue_total, sample_count)
}

fn accumulate_sample_colors_in_row(params: AccumulateSampleColorParams) -> (u32, u32, u32, u32) {
    let AccumulateSampleColorParams {
        rgb,
        source_width,
        sample_grid,
        sample_row,
        source_column_start,
        source_column_end,
    } = params;

    let mut row_red = 0u32;
    let mut row_green = 0u32;
    let mut row_blue = 0u32;
    let mut row_count = 0u32;

    for sample_x in 0..sample_grid {
        let sample_col_params = SamplePositionParams {
            region_start: source_column_start,
            region_end: source_column_end,
            sample_index: sample_x,
            sample_count: sample_grid,
            source_limit: source_width,
        };

        let sample_column = sample_position(sample_col_params);
        let accumulate_params = AccumulateSampleColorParams {
            rgb,
            source_width,
            sample_grid,
            sample_row,
            source_column_start,
            source_column_end,
        };
        let (sample_r, sample_g, sample_b, _) =
            accumulate_sample_color(sample_row, sample_column, accumulate_params);

        row_red += sample_r;
        row_green += sample_g;
        row_blue += sample_b;
        row_count += 1;
    }

    (row_red, row_green, row_blue, row_count)
}

fn accumulate_sample_color(
    sample_row: usize,
    sample_column: usize,
    params: AccumulateSampleColorParams,
) -> (u32, u32, u32, u32) {
    let AccumulateSampleColorParams {
        rgb, source_width, ..
    } = params;

    let pixel_offset = (sample_row * source_width + sample_column) * RGB_CHANNELS;
    let r = rgb[pixel_offset] as u32;
    let g = rgb[pixel_offset + 1] as u32;
    let b = rgb[pixel_offset + 2] as u32;

    (r, g, b, 1)
}

fn luma_from_rgb(r: u8, g: u8, b: u8) -> u16 {
    (r as u16 * FRAMEBUFFER_LUMA_RED_WEIGHT
        + g as u16 * FRAMEBUFFER_LUMA_GREEN_WEIGHT
        + b as u16 * FRAMEBUFFER_LUMA_BLUE_WEIGHT)
        / FRAMEBUFFER_LUMA_WEIGHT_DIVISOR
}

fn perceived_luminance(color: ToneMappedColor) -> u16 {
    luma_from_rgb(color.red, color.green, color.blue)
}

fn sample_position(params: SamplePositionParams) -> usize {
    let SamplePositionParams {
        region_start,
        region_end,
        sample_index,
        sample_count,
        source_limit,
    } = params;

    let region_size = (region_end - region_start).max(1.0);
    let sample_offset = (sample_index as f32 + FRAMEBUFFER_SAMPLE_POSITION_CENTER_OFFSET)
        / sample_count.max(1) as f32;
    let sample_position = region_start + region_size * sample_offset;

    (sample_position.floor() as usize).min(source_limit.saturating_sub(1))
}

fn tone_map_channel(channel: u8) -> u8 {
    let normalized = channel as f32 / FRAMEBUFFER_COLOR_MAX_F32;
    let gamma_corrected = normalized.powf(FRAMEBUFFER_GAMMA);
    let lifted = FRAMEBUFFER_SHADOW_LIFT
        + gamma_corrected * (FRAMEBUFFER_UNIT_INTERVAL_MAX - FRAMEBUFFER_SHADOW_LIFT);
    let contrasted = (lifted - FRAMEBUFFER_MIDPOINT) * FRAMEBUFFER_CONTRAST + FRAMEBUFFER_MIDPOINT;
    let soft_clipped = match contrasted > FRAMEBUFFER_SOFT_CLIP {
        true => {
            FRAMEBUFFER_SOFT_CLIP
                + (contrasted - FRAMEBUFFER_SOFT_CLIP) * FRAMEBUFFER_SOFT_CLIP_STRENGTH
        }
        false => contrasted,
    };

    (soft_clipped.clamp(FRAMEBUFFER_UNIT_INTERVAL_MIN, FRAMEBUFFER_UNIT_INTERVAL_MAX)
        * FRAMEBUFFER_COLOR_MAX_F32) as u8
}

fn saturate_tone_mapped_color(params: SaturateToneMappedColorParams) -> ToneMappedColor {
    let SaturateToneMappedColorParams { color } = params;
    let luminance = perceived_luminance(color) as f32;

    let saturate_channel = |channel: u8| -> u8 {
        let boosted = luminance + (channel as f32 - luminance) * FRAMEBUFFER_SATURATION_BOOST;
        boosted.clamp(0.0, FRAMEBUFFER_COLOR_MAX_F32) as u8
    };

    ToneMappedColor::new(
        saturate_channel(color.red),
        saturate_channel(color.green),
        saturate_channel(color.blue),
    )
}

fn detect_edges_in_region(params: FramebufferRegionSamplingParams) -> bool {
    let FramebufferRegionSamplingParams {
        rgb,
        source_width,
        source_height,
        source_column_start,
        source_column_end,
        source_row_start,
        source_row_end,
    } = params;

    let grid = FRAMEBUFFER_SAMPLE_GRID_EDGE_DETECTION;

    let sample_luma_params = SampleLumaGridParams {
        rgb,
        source_width,
        source_height,
        grid,
        source_column_start,
        source_column_end,
        source_row_start,
        source_row_end,
    };

    let luma_samples = sample_luma_grid(sample_luma_params);
    let max_local_diff = calculate_max_local_diff(&luma_samples);

    max_local_diff > FRAMEBUFFER_EDGE_DETECTION_THRESHOLD
}

fn sample_luma_grid(params: SampleLumaGridParams) -> Vec<u8> {
    let SampleLumaGridParams {
        rgb,
        source_width,
        source_height,
        grid,
        source_column_start,
        source_column_end,
        source_row_start,
        source_row_end,
    } = params;

    let mut luma_samples = Vec::with_capacity(grid * grid);

    for sample_y in 0..grid {
        let sample_row_params = SamplePositionParams {
            region_start: source_row_start,
            region_end: source_row_end,
            sample_index: sample_y,
            sample_count: grid,
            source_limit: source_height,
        };
        let sample_row = sample_position(sample_row_params);
        for sample_x in 0..grid {
            let sample_column_params = SamplePositionParams {
                region_start: source_column_start,
                region_end: source_column_end,
                sample_index: sample_x,
                sample_count: grid,
                source_limit: source_width,
            };
            let sample_column = sample_position(sample_column_params);

            let pixel_offset = (sample_row * source_width + sample_column) * RGB_CHANNELS;
            let luma_params = ApproximateLumaParams {
                red: rgb[pixel_offset],
                green: rgb[pixel_offset + 1],
                blue: rgb[pixel_offset + 2],
            };
            let luma = approximate_luma(luma_params);
            luma_samples.push(luma);
        }
    }

    luma_samples
}

fn calculate_max_local_diff(luma_samples: &[u8]) -> u8 {
    let mut max_local_diff = 0u8;

    for sample_index in 0..luma_samples.len() {
        let neighbor_indices = [
            sample_index.wrapping_sub(1),
            sample_index + 1,
            sample_index.wrapping_sub(3),
            sample_index + 3,
        ];
        let max_neighbor_diff =
            compare_with_neighbors(luma_samples, sample_index, &neighbor_indices);
        max_local_diff = max_local_diff.max(max_neighbor_diff);
    }

    max_local_diff
}

fn compare_with_neighbors(
    luma_samples: &[u8],
    sample_index: usize,
    neighbor_indices: &[usize],
) -> u8 {
    let mut max_diff = 0u8;
    let in_bounds_neighbors = neighbor_indices.iter().filter(|&&n| n < luma_samples.len());

    for &neighbor_index in in_bounds_neighbors {
        let diff = luma_samples[sample_index].abs_diff(luma_samples[neighbor_index]);
        max_diff = max_diff.max(diff);
    }

    max_diff
}

fn apply_unsharp_mask(params: ApplyUnsharpMaskParams<'_>) -> ToneMappedColor {
    let ApplyUnsharpMaskParams {
        original,
        blurred,
        strength,
    } = params;

    let sharpen_channel = |orig: u8, blur: u8| -> u8 {
        let diff = orig.abs_diff(blur) as f32;
        let sharpened = orig as f32 + diff * strength;
        sharpened.clamp(0.0, FRAMEBUFFER_COLOR_MAX_F32) as u8
    };

    ToneMappedColor::new(
        sharpen_channel(original.red, blurred.red),
        sharpen_channel(original.green, blurred.green),
        sharpen_channel(original.blue, blurred.blue),
    )
}

fn region_unsharp_strength(params: RegionUnsharpStrengthParams) -> f32 {
    let RegionUnsharpStrengthParams {
        source_row_start,
        source_row_end,
        source_height,
    } = params;

    let source_height_f32 = source_height.max(1) as f32;
    let row_center = (source_row_start + source_row_end) * 0.5;
    let row_ratio = (row_center / source_height_f32).clamp(0.0, 1.0);

    let is_top_band = row_ratio <= FRAMEBUFFER_UI_SHARPEN_TOP_END_RATIO;
    let is_bottom_band = row_ratio >= FRAMEBUFFER_UI_SHARPEN_BOTTOM_START_RATIO;
    let is_ui_band = is_top_band || is_bottom_band;

    if is_ui_band {
        let bottom_extra = match is_bottom_band {
            true => FRAMEBUFFER_UI_SHARPEN_BOTTOM_EXTRA_BONUS,
            false => FRAMEBUFFER_VIEWPORT_TOP_OFFSET as f32,
        };

        return FRAMEBUFFER_UNSHARP_STRENGTH + FRAMEBUFFER_UI_SHARPEN_BONUS + bottom_extra;
    }

    FRAMEBUFFER_UNSHARP_STRENGTH
}

fn adaptive_sample_count(params: FramebufferRegionSamplingParams) -> usize {
    let FramebufferRegionSamplingParams {
        rgb,
        source_width,
        source_height,
        source_column_start,
        source_column_end,
        source_row_start,
        source_row_end,
    } = params;

    let probe_grid = FRAMEBUFFER_SAMPLE_GRID_BASE.max(FRAMEBUFFER_SAMPLE_GRID_PROBE_MIN);
    let col_lo_params = SamplePositionParams {
        region_start: source_column_start,
        region_end: source_column_end,
        sample_index: 0,
        sample_count: probe_grid,
        source_limit: source_width,
    };
    let col_lo = sample_position(col_lo_params);
    let col_hi_params = SamplePositionParams {
        region_start: source_column_start,
        region_end: source_column_end,
        sample_index: probe_grid - 1,
        sample_count: probe_grid,
        source_limit: source_width,
    };
    let col_hi = sample_position(col_hi_params);
    let row_lo_params = SamplePositionParams {
        region_start: source_row_start,
        region_end: source_row_end,
        sample_index: 0,
        sample_count: probe_grid,
        source_limit: source_height,
    };
    let row_lo = sample_position(row_lo_params);
    let row_hi_params = SamplePositionParams {
        region_start: source_row_start,
        region_end: source_row_end,
        sample_index: probe_grid - 1,
        sample_count: probe_grid,
        source_limit: source_height,
    };
    let row_hi = sample_position(row_hi_params);
    let probe_points = [
        (col_lo, row_lo),
        (col_hi, row_lo),
        (col_lo, row_hi),
        (col_hi, row_hi),
    ];
    let mut min_luma = u8::MAX;
    let mut max_luma = u8::MIN;

    for (sample_column, sample_row) in probe_points {
        let pixel_offset = (sample_row * source_width + sample_column) * RGB_CHANNELS;
        let approx_luma_params = ApproximateLumaParams {
            red: rgb[pixel_offset],
            green: rgb[pixel_offset + 1],
            blue: rgb[pixel_offset + 2],
        };

        let luma = approximate_luma(approx_luma_params);

        min_luma = min_luma.min(luma);
        max_luma = max_luma.max(luma);
    }

    let contrast_delta = max_luma.saturating_sub(min_luma);

    let region_sampling_params = FramebufferRegionSamplingParams {
        rgb,
        source_width,
        source_height,
        source_column_start,
        source_column_end,
        source_row_start,
        source_row_end,
    };

    let has_edge_in_region = detect_edges_in_region(region_sampling_params);
    let is_low_contrast_region = contrast_delta <= FRAMEBUFFER_ADAPTIVE_LOW_CONTRAST_DELTA;
    let is_high_contrast_region = contrast_delta >= FRAMEBUFFER_ADAPTIVE_HIGH_CONTRAST_DELTA;

    match (
        has_edge_in_region,
        is_low_contrast_region,
        is_high_contrast_region,
    ) {
        (true, _, _) => FRAMEBUFFER_SAMPLE_GRID_MAX,
        (false, true, _) => FRAMEBUFFER_SAMPLE_GRID_MIN,
        (false, false, true) => FRAMEBUFFER_SAMPLE_GRID_MAX,
        _ => FRAMEBUFFER_SAMPLE_GRID_BASE,
    }
}

fn approximate_luma(params: ApproximateLumaParams) -> u8 {
    let ApproximateLumaParams { red, green, blue } = params;
    luma_from_rgb(red, green, blue) as u8
}

fn coarse_framebuffer_cell_signature(params: CoarseFramebufferCellSignatureParams) -> u64 {
    let CoarseFramebufferCellSignatureParams {
        rgb,
        source_width,
        source_height,
        source_column_start,
        source_column_end,
        upper_source_row_start,
        upper_source_row_end,
        lower_source_row_start,
        lower_source_row_end,
    } = params;

    let col_lo_params = SamplePositionParams {
        region_start: source_column_start,
        region_end: source_column_end,
        sample_index: 0,
        sample_count: 2,
        source_limit: source_width,
    };
    let col_lo = sample_position(col_lo_params);
    let col_hi_params = SamplePositionParams {
        region_start: source_column_start,
        region_end: source_column_end,
        sample_index: FRAMEBUFFER_SIGNATURE_SAMPLE_SPLITS - 1,
        sample_count: FRAMEBUFFER_SIGNATURE_SAMPLE_SPLITS,
        source_limit: source_width,
    };
    let col_hi = sample_position(col_hi_params);
    let upper_row_params = SamplePositionParams {
        region_start: upper_source_row_start,
        region_end: upper_source_row_end,
        sample_index: 0,
        sample_count: 1,
        source_limit: source_height,
    };
    let upper_row = sample_position(upper_row_params);
    let lower_row_params = SamplePositionParams {
        region_start: lower_source_row_start,
        region_end: lower_source_row_end,
        sample_index: 0,
        sample_count: 1,
        source_limit: source_height,
    };
    let lower_row = sample_position(lower_row_params);
    let sample_points = [
        (col_lo, upper_row),
        (col_hi, upper_row),
        (col_lo, lower_row),
        (col_hi, lower_row),
    ];

    let mut signature = 0u64;

    for (index, (sample_column, sample_row)) in sample_points.into_iter().enumerate() {
        let pixel_offset = (sample_row * source_width + sample_column) * RGB_CHANNELS;
        let packed = ((rgb[pixel_offset] as u64) << 16)
            | ((rgb[pixel_offset + 1] as u64) << 8)
            | rgb[pixel_offset + 2] as u64;
        signature ^= packed.rotate_left((index as u32) * FRAMEBUFFER_SIGNATURE_ROTATE_STEP_BITS);
    }

    signature
}

#[cfg(test)]
#[path = "../tests/render/framebuffer_tests.rs"]
mod tests;
