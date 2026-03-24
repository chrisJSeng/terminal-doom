pub const WAD_DEFAULT_MAP_NAME: &str = "E1M1";
pub const WAD_CANDIDATES: [&str; 2] = ["doom1.wad", "DOOM1.WAD"];

pub const WAD_NOT_FOUND_MESSAGE: &str =
    "WAD nao encontrado. Coloque doom1.wad ou DOOM1.WAD na pasta do projeto.";

pub const WAD_SIGNATURE_IWAD: &str = "IWAD";
pub const WAD_SIGNATURE_PWAD: &str = "PWAD";
pub const WAD_INVALID_SIDEDEF_INDEX: u16 = u16::MAX;

#[cfg(test)]
pub const WAD_TEXTURE_STARTAN2: &str = "STARTAN2";
#[cfg(test)]
pub const WAD_FLAT_FLOOR0_1: &str = "FLOOR0_1";
#[cfg(test)]
pub const WAD_FLAT_CEIL1_1: &str = "CEIL1_1";
#[cfg(test)]
pub const WAD_MISSING_TEXTURE_NAME: &str = "MISSING";

pub const WAD_HEADER_SIZE: usize = 12;
pub const WAD_LUMP_ENTRY_SIZE: usize = 16;
pub const WAD_SIGNATURE_OFFSET: usize = 0;
pub const WAD_SIGNATURE_LEN: usize = 4;
pub const WAD_NUM_LUMPS_OFFSET: usize = 4;
pub const WAD_DIR_OFFSET_OFFSET: usize = 8;

pub const PNAMES_LUMP_NAME: &str = "PNAMES";
pub const TEXTURE1_LUMP_NAME: &str = "TEXTURE1";
pub const PLAYPAL_LUMP_NAME: &str = "PLAYPAL";
pub const VERTEXES_LUMP_NAME: &str = "VERTEXES";
pub const THINGS_LUMP_NAME: &str = "THINGS";
pub const LINEDEFS_LUMP_NAME: &str = "LINEDEFS";
pub const SIDEDEFS_LUMP_NAME: &str = "SIDEDEFS";
pub const SEGS_LUMP_NAME: &str = "SEGS";
pub const SECTORS_LUMP_NAME: &str = "SECTORS";
pub const TEXTURE_NAME_LEN: usize = 8;
pub const TEXTURE_COUNT_OFFSET: usize = 0;
pub const TEXTURE_OFFSET_TABLE_START: usize = 4;
pub const TEXTURE_DEF_NAME_OFFSET: usize = 0;
pub const TEXTURE_DEF_WIDTH_OFFSET: usize = 12;
pub const TEXTURE_DEF_HEIGHT_OFFSET: usize = 14;
pub const TEXTURE_DEF_PATCH_COUNT_OFFSET: usize = 20;
pub const TEXTURE_DEF_PATCHES_OFFSET: usize = 22;
pub const TEXTURE_PATCH_BYTE_SIZE: usize = 10;
pub const TEXTURE_PATCH_ORIGIN_X_OFFSET: usize = 0;
pub const TEXTURE_PATCH_ORIGIN_Y_OFFSET: usize = 2;
pub const TEXTURE_PATCH_PATCH_INDEX_OFFSET: usize = 4;
pub const PNAMES_COUNT_OFFSET: usize = 0;
pub const PNAMES_NAMES_OFFSET: usize = 4;
pub const PLAYPAL_COLOR_COUNT: usize = 256;
pub const PLAYPAL_CHANNELS: usize = 3;
pub const WAD_EMPTY_TEXTURE_NAME: &str = "-";

pub const PATCH_HEADER_WIDTH_OFFSET: usize = 0;
pub const PATCH_HEADER_HEIGHT_OFFSET: usize = 2;
pub const PATCH_HEADER_COLUMN_OFFSETS_START: usize = 8;
pub const PATCH_POST_END: u8 = 255;
pub const PATCH_POST_TRAILER_SIZE: usize = 1;

pub const FLAT_LUMP_SIZE: usize = 64 * 64;

pub const LINEDEF_BYTE_SIZE: usize = 14;
pub const LINEDEF_START_VERTEX_OFFSET: usize = 0;
pub const LINEDEF_END_VERTEX_OFFSET: usize = 2;
pub const LINEDEF_RIGHT_SIDEDEF_OFFSET: usize = 10;
pub const LINEDEF_LEFT_SIDEDEF_OFFSET: usize = 12;
pub const LINEDEF_LUMP_OFFSET: usize = 2;

pub const SIDEDEF_BYTE_SIZE: usize = 30;
pub const SIDEDEF_MIDDLE_TEXTURE_OFFSET: usize = 20;
pub const SIDEDEF_TEXTURE_NAME_LEN: usize = 8;
pub const SIDEDEF_SECTOR_OFFSET: usize = 28;
pub const SIDEDEF_LUMP_OFFSET: usize = 3;

pub const THING_BYTE_SIZE: usize = 10;
pub const THING_X_OFFSET: usize = 0;
pub const THING_Y_OFFSET: usize = 2;
pub const THING_TYPE_OFFSET: usize = 6;
pub const THING_LUMP_OFFSET: usize = 1;

pub const THING_TYPE_PLAYER_1_START: u16 = 1;
pub const THING_TYPE_PLAYER_4_START: u16 = 4;

pub const SEG_BYTE_SIZE: usize = 12;
pub const SEG_LINEDEF_OFFSET: usize = 6;
pub const SEG_LUMP_OFFSET: usize = 5;

pub const VERTEX_BYTE_SIZE: usize = 4;
pub const VERTEX_X_OFFSET: usize = 0;
pub const VERTEX_Y_OFFSET: usize = 2;
pub const VERTEX_LUMP_OFFSET: usize = 4;

pub const SECTOR_BYTE_SIZE: usize = 26;
pub const SECTOR_FLOOR_TEXTURE_OFFSET: usize = 4;
pub const SECTOR_CEILING_TEXTURE_OFFSET: usize = 12;
pub const SECTOR_LIGHT_LEVEL_OFFSET: usize = 20;
pub const SECTOR_LUMP_OFFSET: usize = 8;

pub const ERR_INVALID_WAD_SIGNATURE: &str = "Assinatura WAD invalida.";
pub const ERR_INVALID_WAD_HEADER: &str = "Cabecalho WAD com valores invalidos.";
pub const ERR_MAP_NOT_FOUND: &str = "Mapa nao encontrado no WAD.";
pub const ERR_TEXTURE1_LUMP_MISSING: &str = "Lump TEXTURE1 ausente.";
pub const ERR_PNAMES_LUMP_MISSING: &str = "Lump PNAMES ausente.";
pub const ERR_PLAYPAL_LUMP_MISSING: &str = "Lump PLAYPAL ausente.";
pub const ERR_VERTEX_LUMP_MISSING: &str = "Lump de vertexes ausente.";
pub const ERR_LINEDEF_LUMP_MISSING: &str = "Lump de linedefs ausente.";
pub const ERR_SIDEDEF_LUMP_MISSING: &str = "Lump de sidedefs ausente.";
pub const ERR_THING_LUMP_MISSING: &str = "Lump de things ausente.";
pub const ERR_SEG_LUMP_MISSING: &str = "Lump de segs ausente.";
pub const ERR_SECTOR_LUMP_MISSING: &str = "Lump de sectors ausente.";
pub const ERR_INVALID_LUMP_DATA: &str = "Lump com tamanho ou posicao invalida.";
