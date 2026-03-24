use crate::constants::{
    LINEDEF_BYTE_SIZE, LINEDEF_END_VERTEX_OFFSET, LINEDEF_LEFT_SIDEDEF_OFFSET,
    LINEDEF_RIGHT_SIDEDEF_OFFSET, LINEDEF_START_VERTEX_OFFSET, PLAYPAL_CHANNELS,
    PLAYPAL_COLOR_COUNT, PNAMES_COUNT_OFFSET, PNAMES_NAMES_OFFSET, SECTOR_BYTE_SIZE,
    SECTOR_CEILING_TEXTURE_OFFSET, SECTOR_FLOOR_TEXTURE_OFFSET, SECTOR_LIGHT_LEVEL_OFFSET,
    SEG_BYTE_SIZE, SEG_LINEDEF_OFFSET, SIDEDEF_BYTE_SIZE, SIDEDEF_MIDDLE_TEXTURE_OFFSET,
    SIDEDEF_SECTOR_OFFSET, SIDEDEF_TEXTURE_NAME_LEN, TEXTURE_COUNT_OFFSET,
    TEXTURE_DEF_HEIGHT_OFFSET, TEXTURE_DEF_NAME_OFFSET, TEXTURE_DEF_PATCHES_OFFSET,
    TEXTURE_DEF_PATCH_COUNT_OFFSET, TEXTURE_DEF_WIDTH_OFFSET, TEXTURE_NAME_LEN,
    TEXTURE_OFFSET_TABLE_START, TEXTURE_PATCH_BYTE_SIZE, TEXTURE_PATCH_ORIGIN_X_OFFSET,
    TEXTURE_PATCH_ORIGIN_Y_OFFSET, TEXTURE_PATCH_PATCH_INDEX_OFFSET, THING_BYTE_SIZE,
    THING_TYPE_OFFSET, THING_X_OFFSET, THING_Y_OFFSET, VERTEX_BYTE_SIZE, VERTEX_X_OFFSET,
    VERTEX_Y_OFFSET,
};
use crate::types::{
    LineDef, PaletteColor, ReadNameParams, Sector, Seg, SideDef, TextureDef, TexturePatchRef,
    Thing, Vertex,
};

pub(super) fn parse_vertexes(data: &[u8]) -> Vec<Vertex> {
    data.chunks_exact(VERTEX_BYTE_SIZE)
        .map(|chunk| Vertex {
            x: i16::from_le_bytes([chunk[VERTEX_X_OFFSET], chunk[VERTEX_X_OFFSET + 1]]),
            y: i16::from_le_bytes([chunk[VERTEX_Y_OFFSET], chunk[VERTEX_Y_OFFSET + 1]]),
        })
        .collect()
}

pub(super) fn parse_linedefs(data: &[u8]) -> Vec<LineDef> {
    data.chunks_exact(LINEDEF_BYTE_SIZE)
        .map(|chunk| LineDef {
            start_vertex: u16::from_le_bytes([
                chunk[LINEDEF_START_VERTEX_OFFSET],
                chunk[LINEDEF_START_VERTEX_OFFSET + 1],
            ]),
            end_vertex: u16::from_le_bytes([
                chunk[LINEDEF_END_VERTEX_OFFSET],
                chunk[LINEDEF_END_VERTEX_OFFSET + 1],
            ]),
            right_sidedef: u16::from_le_bytes([
                chunk[LINEDEF_RIGHT_SIDEDEF_OFFSET],
                chunk[LINEDEF_RIGHT_SIDEDEF_OFFSET + 1],
            ]),
            left_sidedef: u16::from_le_bytes([
                chunk[LINEDEF_LEFT_SIDEDEF_OFFSET],
                chunk[LINEDEF_LEFT_SIDEDEF_OFFSET + 1],
            ]),
        })
        .collect()
}

pub(super) fn parse_sidedefs(data: &[u8]) -> Vec<SideDef> {
    data.chunks_exact(SIDEDEF_BYTE_SIZE)
        .map(|chunk| SideDef {
            sector: u16::from_le_bytes([
                chunk[SIDEDEF_SECTOR_OFFSET],
                chunk[SIDEDEF_SECTOR_OFFSET + 1],
            ]),
            middle_texture: read_name(ReadNameParams {
                chunk,
                offset: SIDEDEF_MIDDLE_TEXTURE_OFFSET,
                len: SIDEDEF_TEXTURE_NAME_LEN,
            }),
        })
        .collect()
}

pub(super) fn parse_sectors(data: &[u8]) -> Vec<Sector> {
    data.chunks_exact(SECTOR_BYTE_SIZE)
        .map(|chunk| Sector {
            floor_texture: read_name(ReadNameParams {
                chunk,
                offset: SECTOR_FLOOR_TEXTURE_OFFSET,
                len: TEXTURE_NAME_LEN,
            }),
            ceiling_texture: read_name(ReadNameParams {
                chunk,
                offset: SECTOR_CEILING_TEXTURE_OFFSET,
                len: TEXTURE_NAME_LEN,
            }),
            light_level: i16::from_le_bytes([
                chunk[SECTOR_LIGHT_LEVEL_OFFSET],
                chunk[SECTOR_LIGHT_LEVEL_OFFSET + 1],
            ]),
        })
        .collect()
}

pub(super) fn parse_things(data: &[u8]) -> Vec<Thing> {
    data.chunks_exact(THING_BYTE_SIZE)
        .map(|chunk| Thing {
            x: i16::from_le_bytes([chunk[THING_X_OFFSET], chunk[THING_X_OFFSET + 1]]),
            y: i16::from_le_bytes([chunk[THING_Y_OFFSET], chunk[THING_Y_OFFSET + 1]]),
            thing_type: u16::from_le_bytes([
                chunk[THING_TYPE_OFFSET],
                chunk[THING_TYPE_OFFSET + 1],
            ]),
        })
        .collect()
}

pub(super) fn parse_segs(data: &[u8]) -> Vec<Seg> {
    data.chunks_exact(SEG_BYTE_SIZE)
        .map(|chunk| Seg {
            linedef: u16::from_le_bytes([chunk[SEG_LINEDEF_OFFSET], chunk[SEG_LINEDEF_OFFSET + 1]]),
        })
        .collect()
}

pub(super) fn parse_pnames(data: &[u8]) -> Vec<String> {
    let has_minimum_header = data.len() >= PNAMES_NAMES_OFFSET;

    if !has_minimum_header {
        return Vec::new();
    }

    let patch_name_count = i32::from_le_bytes([
        data[PNAMES_COUNT_OFFSET],
        data[PNAMES_COUNT_OFFSET + 1],
        data[PNAMES_COUNT_OFFSET + 2],
        data[PNAMES_COUNT_OFFSET + 3],
    ])
    .max(0) as usize;

    let mut patch_names = Vec::with_capacity(patch_name_count);
    let mut offset = PNAMES_NAMES_OFFSET;

    for _ in 0..patch_name_count {
        let does_name_overflow_buffer = offset + TEXTURE_NAME_LEN > data.len();

        if does_name_overflow_buffer {
            break;
        }

        patch_names.push(read_name(ReadNameParams {
            chunk: data,
            offset,
            len: TEXTURE_NAME_LEN,
        }));

        offset += TEXTURE_NAME_LEN;
    }

    patch_names
}

pub(super) fn parse_texture_defs(data: &[u8], patch_names: &[String]) -> Vec<TextureDef> {
    let has_texture_table_header = data.len() >= TEXTURE_OFFSET_TABLE_START;

    if !has_texture_table_header {
        return Vec::new();
    }

    let texture_count = i32::from_le_bytes([
        data[TEXTURE_COUNT_OFFSET],
        data[TEXTURE_COUNT_OFFSET + 1],
        data[TEXTURE_COUNT_OFFSET + 2],
        data[TEXTURE_COUNT_OFFSET + 3],
    ])
    .max(0) as usize;

    let mut texture_definitions = Vec::with_capacity(texture_count);

    for index in 0..texture_count {
        let table_offset = TEXTURE_OFFSET_TABLE_START + (index * 4);
        let does_table_entry_overflow = table_offset + 4 > data.len();

        if does_table_entry_overflow {
            break;
        }

        let def_offset = i32::from_le_bytes([
            data[table_offset],
            data[table_offset + 1],
            data[table_offset + 2],
            data[table_offset + 3],
        ])
        .max(0) as usize;

        let does_definition_header_overflow = def_offset + TEXTURE_DEF_PATCHES_OFFSET > data.len();

        if does_definition_header_overflow {
            continue;
        }

        let name = read_name(ReadNameParams {
            chunk: data,
            offset: def_offset + TEXTURE_DEF_NAME_OFFSET,
            len: TEXTURE_NAME_LEN,
        });

        let width = u16::from_le_bytes([
            data[def_offset + TEXTURE_DEF_WIDTH_OFFSET],
            data[def_offset + TEXTURE_DEF_WIDTH_OFFSET + 1],
        ]);

        let height = u16::from_le_bytes([
            data[def_offset + TEXTURE_DEF_HEIGHT_OFFSET],
            data[def_offset + TEXTURE_DEF_HEIGHT_OFFSET + 1],
        ]);

        let patch_count = u16::from_le_bytes([
            data[def_offset + TEXTURE_DEF_PATCH_COUNT_OFFSET],
            data[def_offset + TEXTURE_DEF_PATCH_COUNT_OFFSET + 1],
        ]) as usize;

        let mut patches = Vec::with_capacity(patch_count);

        for patch_idx in 0..patch_count {
            let patch_offset =
                def_offset + TEXTURE_DEF_PATCHES_OFFSET + (patch_idx * TEXTURE_PATCH_BYTE_SIZE);
            let does_patch_entry_overflow = patch_offset + TEXTURE_PATCH_BYTE_SIZE > data.len();

            if does_patch_entry_overflow {
                break;
            }

            let origin_x = i16::from_le_bytes([
                data[patch_offset + TEXTURE_PATCH_ORIGIN_X_OFFSET],
                data[patch_offset + TEXTURE_PATCH_ORIGIN_X_OFFSET + 1],
            ]);

            let origin_y = i16::from_le_bytes([
                data[patch_offset + TEXTURE_PATCH_ORIGIN_Y_OFFSET],
                data[patch_offset + TEXTURE_PATCH_ORIGIN_Y_OFFSET + 1],
            ]);

            let patch_index = u16::from_le_bytes([
                data[patch_offset + TEXTURE_PATCH_PATCH_INDEX_OFFSET],
                data[patch_offset + TEXTURE_PATCH_PATCH_INDEX_OFFSET + 1],
            ]) as usize;

            let patch_name = patch_names.get(patch_index).cloned().unwrap_or_default();

            patches.push(TexturePatchRef {
                origin_x,
                origin_y,
                patch_name,
            });
        }

        texture_definitions.push(TextureDef {
            name,
            width,
            height,
            patches,
        });
    }

    texture_definitions
}

pub(super) fn parse_playpal(data: &[u8]) -> Vec<PaletteColor> {
    let required_len = PLAYPAL_COLOR_COUNT * PLAYPAL_CHANNELS;

    if data.len() < required_len {
        return Vec::new();
    }

    let mut colors = Vec::with_capacity(PLAYPAL_COLOR_COUNT);

    for index in 0..PLAYPAL_COLOR_COUNT {
        let offset = index * PLAYPAL_CHANNELS;

        colors.push(PaletteColor {
            r: data[offset],
            g: data[offset + 1],
            b: data[offset + 2],
        });
    }

    colors
}

fn read_name(params: ReadNameParams<'_>) -> String {
    let ReadNameParams { chunk, offset, len } = params;

    String::from_utf8_lossy(&chunk[offset..offset + len])
        .trim_matches(char::from(0))
        .to_string()
}
