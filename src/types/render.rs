use std::io;

use crossterm::style::Color;

pub struct RenderState {
    pub camera: super::Camera,
    pub use_c_frame_provider: bool,
    pub c_frame_provider: super::CFrameProvider,
}

pub struct DrawFramebufferParams<'a> {
    pub rgb: &'a [u8],
    pub source_width: usize,
    pub source_height: usize,
}

pub(crate) struct TruncateToWidthParams<'a> {
    pub(crate) text: &'a str,
    pub(crate) max_chars: usize,
}

pub struct FramebufferValidationParams {
    pub rgb_len: usize,
    pub required_len: usize,
    pub target_width: usize,
    pub target_height: usize,
    pub source_width: usize,
    pub source_height: usize,
}

#[derive(Clone, Copy)]
pub(crate) struct ToneMappedColor {
    pub(crate) red: u8,
    pub(crate) green: u8,
    pub(crate) blue: u8,
}

impl ToneMappedColor {
    pub(crate) fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub(crate) fn blend(self, other: ToneMappedColor) -> ToneMappedColor {
        ToneMappedColor::new(
            ((self.red as u16 + other.red as u16) / 2) as u8,
            ((self.green as u16 + other.green as u16) / 2) as u8,
            ((self.blue as u16 + other.blue as u16) / 2) as u8,
        )
    }
}

pub(crate) struct FramebufferCellStyle {
    pub(crate) foreground: Color,
    pub(crate) background: Color,
    pub(crate) glyph: &'static str,
}

pub(crate) struct FramebufferCellSpan {
    pub(crate) start_x: usize,
    pub(crate) next_x: usize,
    pub(crate) y: usize,
    pub(crate) foreground: Color,
    pub(crate) background: Color,
    pub(crate) text: String,
}

#[derive(Clone, Copy)]
pub(crate) struct FramebufferViewport {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) offset_x: usize,
    pub(crate) offset_y: usize,
}

pub(crate) struct FramebufferViewportParams {
    pub(crate) available_width: usize,
    pub(crate) available_height: usize,
    pub(crate) source_width: usize,
    pub(crate) source_height: usize,
}

#[derive(Clone, Copy)]
pub(crate) struct FramebufferRegionSamplingParams<'a> {
    pub(crate) rgb: &'a [u8],
    pub(crate) source_width: usize,
    pub(crate) source_height: usize,
    pub(crate) source_column_start: f32,
    pub(crate) source_column_end: f32,
    pub(crate) source_row_start: f32,
    pub(crate) source_row_end: f32,
}

#[derive(Clone, Copy)]
pub(crate) struct AveragedColorInRegionParams<'a> {
    pub(crate) sampling: FramebufferRegionSamplingParams<'a>,
    pub(crate) sample_grid: usize,
}

#[derive(Clone, Copy)]
pub(crate) struct CoarseFramebufferCellSignatureParams<'a> {
    pub(crate) rgb: &'a [u8],
    pub(crate) source_width: usize,
    pub(crate) source_height: usize,
    pub(crate) source_column_start: f32,
    pub(crate) source_column_end: f32,
    pub(crate) upper_source_row_start: f32,
    pub(crate) upper_source_row_end: f32,
    pub(crate) lower_source_row_start: f32,
    pub(crate) lower_source_row_end: f32,
}

pub(crate) struct SamplePositionParams {
    pub(crate) region_start: f32,
    pub(crate) region_end: f32,
    pub(crate) sample_index: usize,
    pub(crate) sample_count: usize,
    pub(crate) source_limit: usize,
}

pub(crate) struct ApproximateLumaParams {
    pub(crate) red: u8,
    pub(crate) green: u8,
    pub(crate) blue: u8,
}

pub(crate) struct FramebufferCellStyleParams {
    pub(crate) upper_color: ToneMappedColor,
    pub(crate) lower_color: ToneMappedColor,
    pub(crate) previous_glyph: Option<&'static str>,
}

pub(crate) struct ShouldKeepPreviousGlyphParams {
    pub(crate) previous_glyph: Option<&'static str>,
    pub(crate) luma_delta: u16,
    pub(crate) color_delta: u16,
}

pub(crate) struct GlyphStyleParams {
    pub(crate) glyph: &'static str,
    pub(crate) upper_color: ToneMappedColor,
    pub(crate) lower_color: ToneMappedColor,
}

pub(crate) struct DensityCellParams {
    pub(crate) upper_color: ToneMappedColor,
    pub(crate) lower_color: ToneMappedColor,
    pub(crate) upper_luminance: u16,
    pub(crate) lower_luminance: u16,
}

pub(crate) struct ColorPairParams {
    pub(crate) upper_color: ToneMappedColor,
    pub(crate) lower_color: ToneMappedColor,
}

pub(crate) struct SaturateToneMappedColorParams {
    pub(crate) color: ToneMappedColor,
}

pub(crate) struct FramebufferCellSourceWindowParams {
    pub(crate) target_column: usize,
    pub(crate) target_row: usize,
    pub(crate) scale_x: f32,
    pub(crate) scale_y: f32,
}

pub(crate) struct FramebufferCellSourceWindow {
    pub(crate) source_column_start: f32,
    pub(crate) source_column_end: f32,
    pub(crate) upper_source_row_start: f32,
    pub(crate) upper_source_row_end: f32,
    pub(crate) lower_source_row_start: f32,
    pub(crate) lower_source_row_end: f32,
}

pub(crate) struct ApplyUnsharpMaskParams<'a> {
    pub(crate) original: &'a ToneMappedColor,
    pub(crate) blurred: &'a ToneMappedColor,
    pub(crate) strength: f32,
}

pub(crate) struct RegionUnsharpStrengthParams {
    pub(crate) source_row_start: f32,
    pub(crate) source_row_end: f32,
    pub(crate) source_height: usize,
}

pub(crate) struct PushFramebufferSpanParams<'a> {
    pub(crate) stdout: &'a mut io::Stdout,
    pub(crate) queued_span: &'a mut Option<FramebufferCellSpan>,
    pub(crate) screen_x: usize,
    pub(crate) screen_y: usize,
    pub(crate) cell: FramebufferCellStyle,
}

pub(crate) struct FlushFramebufferSpanParams<'a> {
    pub(crate) stdout: &'a mut io::Stdout,
    pub(crate) queued_span: &'a mut Option<FramebufferCellSpan>,
}

pub(crate) struct PrepareFramebufferCacheParams {
    pub(crate) viewport: FramebufferViewport,
}

pub(crate) struct RenderFramebufferLoopParams<'a> {
    pub(crate) render_params: DrawFramebufferParams<'a>,
    pub(crate) viewport: FramebufferViewport,
}

pub(crate) struct ProcessFramebufferRowParams<'a> {
    pub(crate) rgb: &'a [u8],
    pub(crate) source_width: usize,
    pub(crate) source_height: usize,
    pub(crate) target_row: usize,
    pub(crate) scale_x: f32,
    pub(crate) scale_y: f32,
    pub(crate) viewport: FramebufferViewport,
}

pub(crate) struct RenderFramebufferCellParams<'a> {
    pub(crate) rgb: &'a [u8],
    pub(crate) source_width: usize,
    pub(crate) source_height: usize,
    pub(crate) target_row: usize,
    pub(crate) target_column: usize,
    pub(crate) scale_x: f32,
    pub(crate) scale_y: f32,
    pub(crate) viewport: FramebufferViewport,
    pub(crate) queued_span: &'a mut Option<FramebufferCellSpan>,
}

pub(crate) struct AccumulateGridSamplesParams<'a> {
    pub(crate) rgb: &'a [u8],
    pub(crate) source_width: usize,
    pub(crate) sample_grid: usize,
    pub(crate) source_row_start: f32,
    pub(crate) source_row_end: f32,
    pub(crate) source_height: usize,
    pub(crate) source_column_start: f32,
    pub(crate) source_column_end: f32,
}

pub(crate) struct SampleLumaGridParams<'a> {
    pub(crate) rgb: &'a [u8],
    pub(crate) source_width: usize,
    pub(crate) source_height: usize,
    pub(crate) grid: usize,
    pub(crate) source_column_start: f32,
    pub(crate) source_column_end: f32,
    pub(crate) source_row_start: f32,
    pub(crate) source_row_end: f32,
}

pub(crate) struct AccumulateSampleColorParams<'a> {
    pub(crate) rgb: &'a [u8],
    pub(crate) source_width: usize,
    pub(crate) sample_grid: usize,
    pub(crate) sample_row: usize,
    pub(crate) source_column_start: f32,
    pub(crate) source_column_end: f32,
}

pub struct TerminalRenderer {
    pub stdout: io::Stdout,
    pub cols: u16,
    pub rows: u16,
    pub framebuffer_cache: Vec<Option<FramebufferCellCacheEntry>>,
    pub framebuffer_cache_cols: u16,
    pub framebuffer_cache_rows: u16,
    pub framebuffer_viewport_signature: Option<FramebufferViewportSignature>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FramebufferCellCacheEntry {
    pub foreground: Color,
    pub background: Color,
    pub glyph: &'static str,
    pub source_signature: u64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FramebufferViewportSignature {
    pub width: usize,
    pub height: usize,
    pub offset_x: usize,
    pub offset_y: usize,
}
