use super::*;

#[test]
fn framebuffer_cell_style_uses_solid_for_flat_cells() {
    let style = framebuffer_cell_style(FramebufferCellStyleParams {
        upper_color: ToneMappedColor {
            red: 100,
            green: 110,
            blue: 120,
        },
        lower_color: ToneMappedColor {
            red: 102,
            green: 112,
            blue: 121,
        },
        previous_glyph: None,
    });

    assert_eq!(style.glyph, RENDER_CELL_SOLID);
}

#[test]
fn framebuffer_cell_style_uses_solid_for_soft_mid_contrast_cells() {
    let style = framebuffer_cell_style(FramebufferCellStyleParams {
        upper_color: ToneMappedColor {
            red: 120,
            green: 120,
            blue: 120,
        },
        lower_color: ToneMappedColor {
            red: 132,
            green: 136,
            blue: 140,
        },
        previous_glyph: None,
    });

    assert_eq!(style.glyph, RENDER_CELL_SOLID);
}

#[test]
fn framebuffer_cell_style_uses_density_glyph_for_stronger_mid_contrast_cells() {
    let style = framebuffer_cell_style(FramebufferCellStyleParams {
        upper_color: ToneMappedColor {
            red: 160,
            green: 100,
            blue: 100,
        },
        lower_color: ToneMappedColor {
            red: 80,
            green: 115,
            blue: 145,
        },
        previous_glyph: None,
    });

    assert_eq!(style.glyph, RENDER_CELL_DENSITY_HEAVY);
}

#[test]
fn framebuffer_cell_style_prefers_solid_over_density_for_general_mid_contrast_cells() {
    let style = framebuffer_cell_style(FramebufferCellStyleParams {
        upper_color: ToneMappedColor {
            red: 110,
            green: 110,
            blue: 110,
        },
        lower_color: ToneMappedColor {
            red: 128,
            green: 134,
            blue: 138,
        },
        previous_glyph: None,
    });

    assert_eq!(style.glyph, RENDER_CELL_SOLID);
}

#[test]
fn framebuffer_cell_style_keeps_upper_half_for_high_contrast_cells() {
    let style = framebuffer_cell_style(FramebufferCellStyleParams {
        upper_color: ToneMappedColor {
            red: 20,
            green: 24,
            blue: 28,
        },
        lower_color: ToneMappedColor {
            red: 220,
            green: 224,
            blue: 228,
        },
        previous_glyph: None,
    });

    assert_eq!(style.glyph, RENDER_CELL_UPPER_HALF);
}

#[test]
fn framebuffer_cell_style_prefers_upper_half_for_luminance_split_cells() {
    let style = framebuffer_cell_style(FramebufferCellStyleParams {
        upper_color: ToneMappedColor {
            red: 62,
            green: 66,
            blue: 70,
        },
        lower_color: ToneMappedColor {
            red: 86,
            green: 90,
            blue: 94,
        },
        previous_glyph: None,
    });

    assert_eq!(style.glyph, RENDER_CELL_UPPER_HALF);
}

#[test]
fn framebuffer_cell_style_keeps_previous_upper_half_near_threshold() {
    let style = framebuffer_cell_style(FramebufferCellStyleParams {
        upper_color: ToneMappedColor {
            red: 92,
            green: 96,
            blue: 100,
        },
        lower_color: ToneMappedColor {
            red: 112,
            green: 116,
            blue: 120,
        },
        previous_glyph: Some(RENDER_CELL_UPPER_HALF),
    });

    assert_eq!(style.glyph, RENDER_CELL_UPPER_HALF);
}

#[test]
fn framebuffer_cell_style_keeps_previous_solid_near_threshold() {
    let style = framebuffer_cell_style(FramebufferCellStyleParams {
        upper_color: ToneMappedColor {
            red: 110,
            green: 110,
            blue: 110,
        },
        lower_color: ToneMappedColor {
            red: 126,
            green: 130,
            blue: 134,
        },
        previous_glyph: Some(RENDER_CELL_SOLID),
    });

    assert_eq!(style.glyph, RENDER_CELL_SOLID);
}

#[test]
fn coarse_signature_changes_when_sampled_pixels_change() {
    let mut rgb = vec![0u8; 4 * 4 * RGB_CHANNELS];
    let base_signature = coarse_framebuffer_cell_signature(CoarseFramebufferCellSignatureParams {
        rgb: &rgb,
        source_width: 4,
        source_height: 4,
        source_column_start: 0.0,
        source_column_end: 2.0,
        upper_source_row_start: 0.0,
        upper_source_row_end: 1.0,
        lower_source_row_start: 1.0,
        lower_source_row_end: 2.0,
    });
    rgb[0] = 255;
    let changed_signature =
        coarse_framebuffer_cell_signature(CoarseFramebufferCellSignatureParams {
            rgb: &rgb,
            source_width: 4,
            source_height: 4,
            source_column_start: 0.0,
            source_column_end: 2.0,
            upper_source_row_start: 0.0,
            upper_source_row_end: 1.0,
            lower_source_row_start: 1.0,
            lower_source_row_end: 2.0,
        });

    assert_ne!(base_signature, changed_signature);
}

#[test]
fn adaptive_sample_count_returns_min_for_low_contrast_region() {
    let rgb = vec![64u8; 4 * 4 * RGB_CHANNELS];
    let sample_count = adaptive_sample_count(FramebufferRegionSamplingParams {
        rgb: &rgb,
        source_width: 4,
        source_height: 4,
        source_column_start: 0.0,
        source_column_end: 2.0,
        source_row_start: 0.0,
        source_row_end: 2.0,
    });

    assert_eq!(sample_count, FRAMEBUFFER_SAMPLE_GRID_MIN);
}

#[test]
fn adaptive_sample_count_returns_max_for_high_contrast_region() {
    let mut rgb = vec![0u8; 4 * 4 * RGB_CHANNELS];
    rgb[0] = 0;
    rgb[1] = 0;
    rgb[2] = 0;
    let high = (3 * 4 + 3) * RGB_CHANNELS;
    rgb[high] = 255;
    rgb[high + 1] = 255;
    rgb[high + 2] = 255;

    let sample_count = adaptive_sample_count(FramebufferRegionSamplingParams {
        rgb: &rgb,
        source_width: 4,
        source_height: 4,
        source_column_start: 0.0,
        source_column_end: 4.0,
        source_row_start: 0.0,
        source_row_end: 4.0,
    });
    assert_eq!(sample_count, FRAMEBUFFER_SAMPLE_GRID_MAX);
}

#[test]
fn framebuffer_viewport_is_top_aligned() {
    let viewport = framebuffer_viewport(FramebufferViewportParams {
        available_width: 120,
        available_height: 40,
        source_width: 320,
        source_height: 200,
    });
    assert_eq!(viewport.offset_y, 0);
}

#[test]
fn region_unsharp_strength_boosts_top_and_bottom_bands() {
    let source_height = 200;
    let top_strength = region_unsharp_strength(RegionUnsharpStrengthParams {
        source_row_start: 0.0,
        source_row_end: 6.0,
        source_height,
    });
    let middle_strength = region_unsharp_strength(RegionUnsharpStrengthParams {
        source_row_start: 90.0,
        source_row_end: 96.0,
        source_height,
    });
    let bottom_strength = region_unsharp_strength(RegionUnsharpStrengthParams {
        source_row_start: 194.0,
        source_row_end: 200.0,
        source_height,
    });

    assert!(top_strength > middle_strength);
    assert!(bottom_strength > middle_strength);
}

#[test]
fn region_unsharp_strength_keeps_base_strength_in_middle() {
    let source_height = 200;
    let middle_strength = region_unsharp_strength(RegionUnsharpStrengthParams {
        source_row_start: 92.0,
        source_row_end: 98.0,
        source_height,
    });

    assert!((middle_strength - FRAMEBUFFER_UNSHARP_STRENGTH).abs() < f32::EPSILON);
}

#[test]
fn region_unsharp_strength_boosts_bottom_more_than_top() {
    let source_height = 200;
    let top_strength = region_unsharp_strength(RegionUnsharpStrengthParams {
        source_row_start: 0.0,
        source_row_end: 6.0,
        source_height,
    });
    let bottom_strength = region_unsharp_strength(RegionUnsharpStrengthParams {
        source_row_start: 194.0,
        source_row_end: 200.0,
        source_height,
    });

    assert!(bottom_strength > top_strength);
}
