use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

use crate::constants::{
    ERR_INVALID_LUMP_DATA, ERR_INVALID_WAD_HEADER, ERR_INVALID_WAD_SIGNATURE,
    ERR_LINEDEF_LUMP_MISSING, ERR_MAP_NOT_FOUND, ERR_PLAYPAL_LUMP_MISSING, ERR_PNAMES_LUMP_MISSING,
    ERR_SECTOR_LUMP_MISSING, ERR_SEG_LUMP_MISSING, ERR_SIDEDEF_LUMP_MISSING,
    ERR_TEXTURE1_LUMP_MISSING, ERR_THING_LUMP_MISSING, ERR_VERTEX_LUMP_MISSING, FLAT_LUMP_SIZE,
    LINEDEFS_LUMP_NAME, LINEDEF_LUMP_OFFSET, PATCH_HEADER_COLUMN_OFFSETS_START,
    PATCH_HEADER_HEIGHT_OFFSET, PATCH_HEADER_WIDTH_OFFSET, PATCH_POST_END, PATCH_POST_TRAILER_SIZE,
    PLAYPAL_LUMP_NAME, PNAMES_LUMP_NAME, SECTORS_LUMP_NAME, SECTOR_LUMP_OFFSET, SEGS_LUMP_NAME,
    SEG_LUMP_OFFSET, SIDEDEFS_LUMP_NAME, SIDEDEF_LUMP_OFFSET, TEXTURE1_LUMP_NAME, THINGS_LUMP_NAME,
    THING_LUMP_OFFSET, VERTEXES_LUMP_NAME, VERTEX_LUMP_OFFSET, WAD_DIR_OFFSET_OFFSET,
    WAD_EMPTY_TEXTURE_NAME, WAD_HEADER_SIZE, WAD_LUMP_ENTRY_SIZE, WAD_NUM_LUMPS_OFFSET,
    WAD_SIGNATURE_IWAD, WAD_SIGNATURE_LEN, WAD_SIGNATURE_OFFSET, WAD_SIGNATURE_PWAD,
};
use crate::types::{
    ApplyTexturePatchesParams, BlitPatchColumnParams, BlitPatchParams, BlitPatchPostParams,
    CollectBspLineIndicesParams, ComposeTexturesParams, FlatTexture, FlatTextureWithPixelsParams,
    LoadFlatsParams, LoadMapParams, LumpInfo, LumpInfoWithRawParams, MapData, MapTexture,
    MapTextureWithPixelsParams, NonNegativeI32PairParams, ReadAndParseLumpParams,
    ReadDirectoryParams, ReadLumpDataParams, RequiredMapLumpParams, WadHeader,
};

use super::parser::{
    parse_linedefs, parse_playpal, parse_pnames, parse_sectors, parse_segs, parse_sidedefs,
    parse_texture_defs, parse_things, parse_vertexes,
};

pub fn load_map_data(params: LoadMapParams) -> io::Result<MapData> {
    let LoadMapParams { path, map_name } = params;
    let header = validate_wad(path)?;
    let read_directory_params = ReadDirectoryParams {
        path,
        header: &header,
    };
    let lumps = read_directory(read_directory_params)?;

    let map_index = lumps
        .iter()
        .position(|l| l.name == map_name)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, ERR_MAP_NOT_FOUND))?;

    let pnames_lump = lumps
        .iter()
        .find(|l| l.name == PNAMES_LUMP_NAME)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, ERR_PNAMES_LUMP_MISSING))?;

    let texture1_lump = lumps
        .iter()
        .find(|l| l.name == TEXTURE1_LUMP_NAME)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, ERR_TEXTURE1_LUMP_MISSING))?;

    let playpal_lump = lumps
        .iter()
        .find(|l| l.name == PLAYPAL_LUMP_NAME)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, ERR_PLAYPAL_LUMP_MISSING))?;

    let vertex_lump_params = RequiredMapLumpParams {
        lumps: &lumps,
        map_index,
        offset: VERTEX_LUMP_OFFSET,
        expected_name: VERTEXES_LUMP_NAME,
        error_message: ERR_VERTEX_LUMP_MISSING,
    };
    let vertex_lump = required_map_lump(vertex_lump_params)?;

    let thing_lump_params = RequiredMapLumpParams {
        lumps: &lumps,
        map_index,
        offset: THING_LUMP_OFFSET,
        expected_name: THINGS_LUMP_NAME,
        error_message: ERR_THING_LUMP_MISSING,
    };
    let thing_lump = required_map_lump(thing_lump_params)?;

    let linedef_lump_params = RequiredMapLumpParams {
        lumps: &lumps,
        map_index,
        offset: LINEDEF_LUMP_OFFSET,
        expected_name: LINEDEFS_LUMP_NAME,
        error_message: ERR_LINEDEF_LUMP_MISSING,
    };
    let linedef_lump = required_map_lump(linedef_lump_params)?;

    let sidedef_lump_params = RequiredMapLumpParams {
        lumps: &lumps,
        map_index,
        offset: SIDEDEF_LUMP_OFFSET,
        expected_name: SIDEDEFS_LUMP_NAME,
        error_message: ERR_SIDEDEF_LUMP_MISSING,
    };
    let sidedef_lump = required_map_lump(sidedef_lump_params)?;

    let seg_lump_params = RequiredMapLumpParams {
        lumps: &lumps,
        map_index,
        offset: SEG_LUMP_OFFSET,
        expected_name: SEGS_LUMP_NAME,
        error_message: ERR_SEG_LUMP_MISSING,
    };
    let seg_lump = required_map_lump(seg_lump_params)?;

    let sector_lump_params = RequiredMapLumpParams {
        lumps: &lumps,
        map_index,
        offset: SECTOR_LUMP_OFFSET,
        expected_name: SECTORS_LUMP_NAME,
        error_message: ERR_SECTOR_LUMP_MISSING,
    };
    let sector_lump = required_map_lump(sector_lump_params)?;

    let vertexes_params = ReadAndParseLumpParams {
        path,
        lump: vertex_lump,
        parse_fn: parse_vertexes,
    };
    let vertexes = read_and_parse_lump(vertexes_params)?;

    let things_params = ReadAndParseLumpParams {
        path,
        lump: thing_lump,
        parse_fn: parse_things,
    };
    let things = read_and_parse_lump(things_params)?;

    let lines_params = ReadAndParseLumpParams {
        path,
        lump: linedef_lump,
        parse_fn: parse_linedefs,
    };
    let lines = read_and_parse_lump(lines_params)?;

    let sidedefs_params = ReadAndParseLumpParams {
        path,
        lump: sidedef_lump,
        parse_fn: parse_sidedefs,
    };
    let sidedefs = read_and_parse_lump(sidedefs_params)?;

    let segments_params = ReadAndParseLumpParams {
        path,
        lump: seg_lump,
        parse_fn: parse_segs,
    };
    let segments = read_and_parse_lump(segments_params)?;

    let sectors_params = ReadAndParseLumpParams {
        path,
        lump: sector_lump,
        parse_fn: parse_sectors,
    };
    let sectors = read_and_parse_lump(sectors_params)?;

    let palette_params = ReadAndParseLumpParams {
        path,
        lump: playpal_lump,
        parse_fn: parse_playpal,
    };
    let palette = read_and_parse_lump(palette_params)?;

    let patch_names_params = ReadAndParseLumpParams {
        path,
        lump: pnames_lump,
        parse_fn: parse_pnames,
    };
    let patch_names = read_and_parse_lump(patch_names_params)?;

    let texture1_data_params = ReadLumpDataParams {
        path,
        lump: texture1_lump,
    };
    let texture1_data = read_lump_data(texture1_data_params)?;

    let texture_defs = parse_texture_defs(&texture1_data, &patch_names);
    let compose_textures_params = ComposeTexturesParams {
        path,
        lumps: &lumps,
        texture_defs: &texture_defs,
    };
    let textures = compose_textures(compose_textures_params)?;

    let load_flats_params = LoadFlatsParams {
        path,
        lumps: &lumps,
        sectors: &sectors,
    };
    let flats = load_flats(load_flats_params)?;

    let bsp_line_indices_params = CollectBspLineIndicesParams {
        segments: &segments,
        line_count: lines.len(),
    };
    let bsp_line_indices = collect_bsp_line_indices(bsp_line_indices_params);

    Ok(MapData {
        vertexes,
        lines,
        sidedefs,
        sectors,
        palette,
        textures,
        flats,
        things,
        bsp_line_indices,
    })
}

fn compose_textures(params: ComposeTexturesParams<'_>) -> io::Result<Vec<MapTexture>> {
    let ComposeTexturesParams {
        path,
        lumps,
        texture_defs,
    } = params;

    let mut textures = Vec::with_capacity(texture_defs.len());

    for texture_definition in texture_defs {
        let width = texture_definition.width.max(1) as usize;
        let height = texture_definition.height.max(1) as usize;
        let mut pixels = vec![0u8; width * height];

        let apply_texture_patches_params = ApplyTexturePatchesParams {
            path,
            lumps,
            texture_definition,
            target_pixels: &mut pixels,
        };

        apply_texture_patches(apply_texture_patches_params)?;

        let map_texture_with_pixels_params = MapTextureWithPixelsParams {
            name: texture_definition.name.clone(),
            width: texture_definition.width,
            height: texture_definition.height,
            pixels,
        };

        textures.push(MapTexture::with_pixels(map_texture_with_pixels_params));
    }

    Ok(textures)
}

fn apply_texture_patches(params: ApplyTexturePatchesParams<'_>) -> io::Result<()> {
    let ApplyTexturePatchesParams {
        path,
        lumps,
        texture_definition,
        target_pixels,
    } = params;

    texture_definition
        .patches
        .iter()
        .try_for_each(|patch| -> io::Result<()> {
            let patch_lump = match lumps.iter().find(|lump| lump.name == patch.patch_name) {
                Some(patch_lump) => patch_lump,
                None => return Ok(()),
            };

            let patch_data_params = ReadLumpDataParams {
                path,
                lump: patch_lump,
            };
            let patch_data = read_lump_data(patch_data_params)?;

            let blit_patch_params = BlitPatchParams {
                patch_data: &patch_data,
                origin_x: patch.origin_x,
                origin_y: patch.origin_y,
                target_width: texture_definition.width,
                target_height: texture_definition.height,
                target_pixels,
            };

            blit_patch(blit_patch_params);

            Ok(())
        })
}

fn required_map_lump<'a>(params: RequiredMapLumpParams<'a>) -> io::Result<&'a LumpInfo> {
    let RequiredMapLumpParams {
        lumps,
        map_index,
        offset,
        expected_name,
        error_message,
    } = params;

    let Some(lump) = lumps.get(map_index + offset) else {
        return Err(io::Error::new(io::ErrorKind::InvalidData, error_message));
    };

    if lump.name != expected_name {
        return Err(io::Error::new(io::ErrorKind::InvalidData, error_message));
    }

    Ok(lump)
}

fn read_and_parse_lump<T, F>(params: ReadAndParseLumpParams<'_, F>) -> io::Result<T>
where
    F: FnOnce(&[u8]) -> T,
{
    let ReadAndParseLumpParams {
        path,
        lump,
        parse_fn,
    } = params;

    let read_lump_data_params = ReadLumpDataParams { path, lump };
    let lump_data = read_lump_data(read_lump_data_params)?;

    Ok(parse_fn(&lump_data))
}

fn blit_patch(params: BlitPatchParams<'_>) {
    let BlitPatchParams {
        patch_data,
        origin_x,
        origin_y,
        target_width,
        target_height,
        target_pixels,
    } = params;

    if patch_data.len() < PATCH_HEADER_COLUMN_OFFSETS_START {
        return;
    }

    let patch_width = u16::from_le_bytes([
        patch_data[PATCH_HEADER_WIDTH_OFFSET],
        patch_data[PATCH_HEADER_WIDTH_OFFSET + 1],
    ]) as usize;
    let _patch_height = u16::from_le_bytes([
        patch_data[PATCH_HEADER_HEIGHT_OFFSET],
        patch_data[PATCH_HEADER_HEIGHT_OFFSET + 1],
    ]);

    for column in 0..patch_width {
        let blit_patch_column_params = BlitPatchColumnParams {
            patch_data,
            column,
            origin_x,
            origin_y,
            target_width,
            target_height,
            target_pixels,
        };
        let continue_reading_columns = blit_patch_column(blit_patch_column_params);

        if !continue_reading_columns {
            break;
        }
    }
}

fn blit_patch_column(params: BlitPatchColumnParams<'_>) -> bool {
    let BlitPatchColumnParams {
        patch_data,
        column,
        origin_x,
        origin_y,
        target_width,
        target_height,
        target_pixels,
    } = params;

    let offset_pos = PATCH_HEADER_COLUMN_OFFSETS_START + (column * 4);

    if offset_pos + 4 > patch_data.len() {
        return false;
    }

    let mut post_offset = u32::from_le_bytes([
        patch_data[offset_pos],
        patch_data[offset_pos + 1],
        patch_data[offset_pos + 2],
        patch_data[offset_pos + 3],
    ]) as usize;
    let mut previous_post_top = -1i16;

    while post_offset < patch_data.len() {
        let top_delta = patch_data[post_offset];
        let has_reached_post_end = top_delta == PATCH_POST_END;
        let has_invalid_post_header = post_offset + 3 > patch_data.len();
        let should_stop_column_read = has_reached_post_end || has_invalid_post_header;

        if should_stop_column_read {
            break;
        }

        let length = patch_data[post_offset + 1] as usize;
        let data_start = post_offset + 3;
        let data_end = data_start + length;

        if data_end > patch_data.len() {
            break;
        }

        let continues_from_previous_post = top_delta as i16 <= previous_post_top;

        previous_post_top = match continues_from_previous_post {
            true => previous_post_top + top_delta as i16,
            false => top_delta as i16,
        };

        let blit_patch_post_params = BlitPatchPostParams {
            patch_data,
            data_start,
            length,
            origin_x,
            origin_y,
            column,
            previous_post_top,
            target_width,
            target_height,
            target_pixels,
        };

        blit_patch_post(blit_patch_post_params);

        post_offset = data_end + PATCH_POST_TRAILER_SIZE;
    }

    true
}

fn blit_patch_post(params: BlitPatchPostParams<'_>) {
    let BlitPatchPostParams {
        patch_data,
        data_start,
        length,
        origin_x,
        origin_y,
        column,
        previous_post_top,
        target_width,
        target_height,
        target_pixels,
    } = params;

    for row in 0..length {
        let dst_x = origin_x + column as i16;
        let dst_y = origin_y + previous_post_top + row as i16;
        let target_pixel_out_of_bounds =
            dst_x < 0 || dst_y < 0 || dst_x >= target_width as i16 || dst_y >= target_height as i16;

        if target_pixel_out_of_bounds {
            continue;
        }

        let dst_index = dst_y as usize * target_width as usize + dst_x as usize;

        target_pixels[dst_index] = patch_data[data_start + row];
    }
}

fn load_flats(params: LoadFlatsParams<'_>) -> io::Result<Vec<FlatTexture>> {
    let LoadFlatsParams {
        path,
        lumps,
        sectors,
    } = params;

    let mut names: Vec<String> = sectors
        .iter()
        .flat_map(|sector| [sector.floor_texture.clone(), sector.ceiling_texture.clone()])
        .filter(|name| is_valid_flat_name(name))
        .collect();

    names.sort();
    names.dedup();

    let mut flats = Vec::new();

    for name in names {
        let lump = match lumps.iter().find(|lump| lump.name == name) {
            Some(lump) => lump,
            None => continue,
        };
        let read_lump_data_params = ReadLumpDataParams { path, lump };
        let data = read_lump_data(read_lump_data_params)?;

        if data.len() < FLAT_LUMP_SIZE {
            continue;
        }

        let flat_texture_with_pixels_params = FlatTextureWithPixelsParams {
            name,
            pixels: data[..FLAT_LUMP_SIZE].to_vec(),
        };

        flats.push(FlatTexture::with_pixels(flat_texture_with_pixels_params));
    }

    Ok(flats)
}

fn is_valid_flat_name(name: &str) -> bool {
    !name.is_empty() && name != WAD_EMPTY_TEXTURE_NAME
}

fn collect_bsp_line_indices(params: CollectBspLineIndicesParams<'_>) -> Vec<u16> {
    let CollectBspLineIndicesParams {
        segments,
        line_count,
    } = params;

    let mut seen_linedefs = vec![false; line_count];
    let mut ordered_linedefs = Vec::new();

    for segment in segments {
        let linedef_index = segment.linedef as usize;
        let is_index_out_of_bounds = linedef_index >= line_count;
        let was_linedef_already_seen = !is_index_out_of_bounds && seen_linedefs[linedef_index];
        let should_skip_linedef = is_index_out_of_bounds || was_linedef_already_seen;

        if should_skip_linedef {
            continue;
        }

        seen_linedefs[linedef_index] = true;
        ordered_linedefs.push(segment.linedef);
    }

    ordered_linedefs
}

fn validate_wad(path: &str) -> io::Result<WadHeader> {
    let mut file = File::open(path)?;
    let mut buffer = [0; WAD_HEADER_SIZE];

    file.read_exact(&mut buffer)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, ERR_INVALID_WAD_HEADER))?;

    let signature = String::from_utf8_lossy(
        &buffer[WAD_SIGNATURE_OFFSET..WAD_SIGNATURE_OFFSET + WAD_SIGNATURE_LEN],
    )
    .to_string();

    let is_iwad = signature == WAD_SIGNATURE_IWAD;
    let is_pwad = signature == WAD_SIGNATURE_PWAD;
    let is_valid_signature = is_iwad || is_pwad;

    if !is_valid_signature {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            ERR_INVALID_WAD_SIGNATURE,
        ));
    }

    let num_lumps = i32::from_le_bytes([
        buffer[WAD_NUM_LUMPS_OFFSET],
        buffer[WAD_NUM_LUMPS_OFFSET + 1],
        buffer[WAD_NUM_LUMPS_OFFSET + 2],
        buffer[WAD_NUM_LUMPS_OFFSET + 3],
    ]);
    let directory_offset = i32::from_le_bytes([
        buffer[WAD_DIR_OFFSET_OFFSET],
        buffer[WAD_DIR_OFFSET_OFFSET + 1],
        buffer[WAD_DIR_OFFSET_OFFSET + 2],
        buffer[WAD_DIR_OFFSET_OFFSET + 3],
    ]);

    let non_negative_pair_params = NonNegativeI32PairParams {
        first: num_lumps,
        second: directory_offset,
    };
    let has_valid_header = has_non_negative_i32_pair(non_negative_pair_params);

    if !has_valid_header {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            ERR_INVALID_WAD_HEADER,
        ));
    }

    Ok(WadHeader {
        num_lumps,
        directory_offset,
    })
}

fn read_directory(params: ReadDirectoryParams<'_>) -> io::Result<Vec<LumpInfo>> {
    let ReadDirectoryParams { path, header } = params;

    let non_negative_pair_params = NonNegativeI32PairParams {
        first: header.num_lumps,
        second: header.directory_offset,
    };
    let has_valid_header = has_non_negative_i32_pair(non_negative_pair_params);

    if !has_valid_header {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            ERR_INVALID_WAD_HEADER,
        ));
    }

    let mut file = File::open(path)?;
    let mut lumps = Vec::with_capacity(header.num_lumps as usize);

    file.seek(SeekFrom::Start(header.directory_offset as u64))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, ERR_INVALID_WAD_HEADER))?;

    for _ in 0..header.num_lumps {
        let mut buffer = [0; WAD_LUMP_ENTRY_SIZE];

        file.read_exact(&mut buffer)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, ERR_INVALID_WAD_HEADER))?;

        let file_pos = i32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        let size = i32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
        let name = String::from_utf8_lossy(&buffer[8..WAD_LUMP_ENTRY_SIZE])
            .trim_matches(char::from(0))
            .to_string();

        let lump_info_with_raw_params = LumpInfoWithRawParams {
            name,
            file_pos,
            size,
        };

        lumps.push(LumpInfo::with_raw(lump_info_with_raw_params));
    }

    Ok(lumps)
}

fn read_lump_data(params: ReadLumpDataParams<'_>) -> io::Result<Vec<u8>> {
    let ReadLumpDataParams { path, lump } = params;

    let non_negative_pair_params = NonNegativeI32PairParams {
        first: lump.file_pos,
        second: lump.size,
    };
    let has_valid_lump = has_non_negative_i32_pair(non_negative_pair_params);

    if !has_valid_lump {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            ERR_INVALID_LUMP_DATA,
        ));
    }

    let mut file = File::open(path)?;
    let mut buffer = vec![0u8; lump.size as usize];

    file.seek(SeekFrom::Start(lump.file_pos as u64))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, ERR_INVALID_LUMP_DATA))?;
    file.read_exact(&mut buffer)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, ERR_INVALID_LUMP_DATA))?;

    Ok(buffer)
}

fn has_non_negative_i32_pair(params: NonNegativeI32PairParams) -> bool {
    let NonNegativeI32PairParams { first, second } = params;

    first >= 0 && second >= 0
}

#[cfg(test)]
#[path = "../tests/wad/reader_tests.rs"]
mod tests;
