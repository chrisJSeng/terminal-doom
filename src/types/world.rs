#[derive(Clone, Copy)]
pub struct Vertex {
    pub x: i16,
    pub y: i16,
}

pub struct LineDef {
    pub start_vertex: u16,
    pub end_vertex: u16,
    pub right_sidedef: u16,
    pub left_sidedef: u16,
}

#[allow(dead_code)]
pub struct SideDef {
    pub sector: u16,
    pub middle_texture: String,
}

pub struct Sector {
    pub floor_texture: String,
    pub ceiling_texture: String,
    pub light_level: i16,
}

#[allow(dead_code)]
pub struct PaletteColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct TexturePatchRef {
    pub origin_x: i16,
    pub origin_y: i16,
    pub patch_name: String,
}

pub struct TextureDef {
    pub name: String,
    pub width: u16,
    pub height: u16,
    pub patches: Vec<TexturePatchRef>,
}

#[allow(dead_code)]
pub struct MapTexture {
    pub name: String,
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<u8>,
}

#[allow(dead_code)]
pub struct FlatTexture {
    pub name: String,
    pub pixels: Vec<u8>,
}

pub struct Thing {
    pub x: i16,
    pub y: i16,
    pub thing_type: u16,
}

pub struct Seg {
    pub linedef: u16,
}

#[allow(dead_code)]
pub struct MapData {
    pub vertexes: Vec<Vertex>,
    pub lines: Vec<LineDef>,
    pub sidedefs: Vec<SideDef>,
    pub sectors: Vec<Sector>,
    pub palette: Vec<PaletteColor>,
    pub textures: Vec<MapTexture>,
    pub flats: Vec<FlatTexture>,
    pub things: Vec<Thing>,
    pub bsp_line_indices: Vec<u16>,
}

#[derive(Clone, Copy)]
pub struct MapBounds {
    pub min_x: f32,
    pub min_y: f32,
    pub width: f32,
    pub height: f32,
}

#[allow(dead_code)]
pub struct Camera {
    pub offset_x: f32,
    pub offset_y: f32,
    pub zoom: f32,
}

pub struct WadHeader {
    pub num_lumps: i32,
    pub directory_offset: i32,
}

pub struct LumpInfo {
    pub name: String,
    pub file_pos: i32,
    pub size: i32,
}

pub struct MapTextureWithPixelsParams {
    pub name: String,
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<u8>,
}

pub struct FlatTextureWithPixelsParams {
    pub name: String,
    pub pixels: Vec<u8>,
}

pub struct LumpInfoWithRawParams {
    pub name: String,
    pub file_pos: i32,
    pub size: i32,
}

impl MapTexture {
    pub fn with_pixels(params: MapTextureWithPixelsParams) -> Self {
        let MapTextureWithPixelsParams {
            name,
            width,
            height,
            pixels,
        } = params;

        Self {
            name,
            width: width.max(1),
            height: height.max(1),
            pixels,
        }
    }
}

impl FlatTexture {
    pub fn with_pixels(params: FlatTextureWithPixelsParams) -> Self {
        let FlatTextureWithPixelsParams { name, pixels } = params;
        Self { name, pixels }
    }
}

impl LumpInfo {
    pub fn with_raw(params: LumpInfoWithRawParams) -> Self {
        let LumpInfoWithRawParams {
            name,
            file_pos,
            size,
        } = params;
        Self {
            name,
            file_pos,
            size,
        }
    }
}

pub struct LoadMapParams<'a> {
    pub path: &'a str,
    pub map_name: &'a str,
}

#[cfg(test)]
pub struct SegmentDistanceParams {
    pub px: f32,
    pub py: f32,
    pub ax: f32,
    pub ay: f32,
    pub bx: f32,
    pub by: f32,
}

pub struct ComposeTexturesParams<'a> {
    pub path: &'a str,
    pub lumps: &'a [LumpInfo],
    pub texture_defs: &'a [TextureDef],
}

pub struct BlitPatchParams<'a> {
    pub patch_data: &'a [u8],
    pub origin_x: i16,
    pub origin_y: i16,
    pub target_width: u16,
    pub target_height: u16,
    pub target_pixels: &'a mut [u8],
}

pub struct LoadFlatsParams<'a> {
    pub path: &'a str,
    pub lumps: &'a [LumpInfo],
    pub sectors: &'a [Sector],
}

pub(crate) struct ApplyTexturePatchesParams<'a> {
    pub(crate) path: &'a str,
    pub(crate) lumps: &'a [LumpInfo],
    pub(crate) texture_definition: &'a TextureDef,
    pub(crate) target_pixels: &'a mut [u8],
}

pub(crate) struct BlitPatchColumnParams<'a> {
    pub(crate) patch_data: &'a [u8],
    pub(crate) column: usize,
    pub(crate) origin_x: i16,
    pub(crate) origin_y: i16,
    pub(crate) target_width: u16,
    pub(crate) target_height: u16,
    pub(crate) target_pixels: &'a mut [u8],
}

pub(crate) struct BlitPatchPostParams<'a> {
    pub(crate) patch_data: &'a [u8],
    pub(crate) data_start: usize,
    pub(crate) length: usize,
    pub(crate) origin_x: i16,
    pub(crate) origin_y: i16,
    pub(crate) column: usize,
    pub(crate) previous_post_top: i16,
    pub(crate) target_width: u16,
    pub(crate) target_height: u16,
    pub(crate) target_pixels: &'a mut [u8],
}

pub struct ReadNameParams<'a> {
    pub chunk: &'a [u8],
    pub offset: usize,
    pub len: usize,
}

pub(crate) struct CollectBspLineIndicesParams<'a> {
    pub(crate) segments: &'a [Seg],
    pub(crate) line_count: usize,
}

pub(crate) struct ReadDirectoryParams<'a> {
    pub(crate) path: &'a str,
    pub(crate) header: &'a WadHeader,
}

pub(crate) struct ReadLumpDataParams<'a> {
    pub(crate) path: &'a str,
    pub(crate) lump: &'a LumpInfo,
}

pub(crate) struct NonNegativeI32PairParams {
    pub(crate) first: i32,
    pub(crate) second: i32,
}

pub(crate) struct RequiredMapLumpParams<'a> {
    pub(crate) lumps: &'a [LumpInfo],
    pub(crate) map_index: usize,
    pub(crate) offset: usize,
    pub(crate) expected_name: &'a str,
    pub(crate) error_message: &'a str,
}

pub(crate) struct ReadAndParseLumpParams<'a, F> {
    pub(crate) path: &'a str,
    pub(crate) lump: &'a LumpInfo,
    pub(crate) parse_fn: F,
}
