use super::*;
use crate::constants::{
    ERR_INVALID_LUMP_DATA, ERR_INVALID_WAD_HEADER, ERR_INVALID_WAD_SIGNATURE,
    ERR_VERTEX_LUMP_MISSING, VERTEXES_LUMP_NAME, WAD_EMPTY_TEXTURE_NAME, WAD_FLAT_FLOOR0_1,
    WAD_MISSING_TEXTURE_NAME, WAD_SIGNATURE_IWAD,
};
use crate::tests::fixtures;
use crate::types::{Sector, TextureDef, TexturePatchRef};

use fixtures::wad_files::{cleanup_temp_test_file, write_temp_test_file};

#[test]
fn blit_patch_supports_tall_posts() {
    let mut patch_data = Vec::new();
    patch_data.extend_from_slice(&1u16.to_le_bytes());
    patch_data.extend_from_slice(&260u16.to_le_bytes());
    patch_data.extend_from_slice(&0u16.to_le_bytes());
    patch_data.extend_from_slice(&0u16.to_le_bytes());
    patch_data.extend_from_slice(&12u32.to_le_bytes());
    patch_data.extend_from_slice(&[250, 2, 0, 10, 11, 0]);
    patch_data.extend_from_slice(&[1, 2, 0, 12, 13, 0]);
    patch_data.push(PATCH_POST_END);

    let mut target = vec![0u8; 260];
    blit_patch(BlitPatchParams {
        patch_data: &patch_data,
        origin_x: 0,
        origin_y: 0,
        target_width: 1,
        target_height: 260,
        target_pixels: &mut target,
    });

    assert_eq!(&target[245..255], &[0, 0, 0, 0, 0, 10, 12, 13, 0, 0]);
}

#[test]
fn load_flats_skips_absent_or_placeholder_names() {
    let lumps = [LumpInfo {
        name: WAD_FLAT_FLOOR0_1.to_string(),
        file_pos: 0,
        size: FLAT_LUMP_SIZE as i32,
    }];
    let sectors = [
        Sector {
            floor_texture: WAD_FLAT_FLOOR0_1.to_string(),
            ceiling_texture: "".to_string(),
            light_level: 160,
        },
        Sector {
            floor_texture: WAD_EMPTY_TEXTURE_NAME.to_string(),
            ceiling_texture: WAD_MISSING_TEXTURE_NAME.to_string(),
            light_level: 160,
        },
    ];

    let names: Vec<String> = sectors
        .iter()
        .flat_map(|sector| [sector.floor_texture.clone(), sector.ceiling_texture.clone()])
        .filter(|name| is_valid_flat_name(name))
        .collect();

    assert!(names.contains(&WAD_FLAT_FLOOR0_1.to_string()));
    assert!(names.contains(&WAD_MISSING_TEXTURE_NAME.to_string()));
    assert!(!names.contains(&WAD_EMPTY_TEXTURE_NAME.to_string()));
    assert!(!names.contains(&"".to_string()));
    assert_eq!(lumps.len(), 1);
}

#[test]
fn find_required_map_lump_accepts_matching_lump_name() {
    let lumps = vec![
        LumpInfo {
            name: "E1M1".to_string(),
            file_pos: 0,
            size: 0,
        },
        LumpInfo {
            name: VERTEXES_LUMP_NAME.to_string(),
            file_pos: 0,
            size: 0,
        },
    ];

    let lump = required_map_lump(RequiredMapLumpParams {
        lumps: &lumps,
        map_index: 0,
        offset: 1,
        expected_name: VERTEXES_LUMP_NAME,
        error_message: ERR_VERTEX_LUMP_MISSING,
    })
    .expect("expected matching lump to be accepted");

    assert_eq!(lump.name, VERTEXES_LUMP_NAME);
}

#[test]
fn find_required_map_lump_rejects_unexpected_lump_name() {
    let lumps = vec![
        LumpInfo {
            name: "E1M1".to_string(),
            file_pos: 0,
            size: 0,
        },
        LumpInfo {
            name: "THINGS".to_string(),
            file_pos: 0,
            size: 0,
        },
    ];

    let err = match required_map_lump(RequiredMapLumpParams {
        lumps: &lumps,
        map_index: 0,
        offset: 1,
        expected_name: VERTEXES_LUMP_NAME,
        error_message: ERR_VERTEX_LUMP_MISSING,
    }) {
        Ok(_) => panic!("expected wrong lump order to be rejected"),
        Err(err) => err,
    };

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    assert_eq!(err.to_string(), ERR_VERTEX_LUMP_MISSING);
}

#[test]
fn validate_wad_rejects_invalid_signature() {
    let path = write_temp_test_file(b"BADS\x00\x00\x00\x00\x0c\x00\x00\x00");

    let err = match validate_wad(path.to_str().expect("temp path should be valid utf-8")) {
        Ok(_) => panic!("expected invalid signature to be rejected"),
        Err(err) => err,
    };

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    assert_eq!(err.to_string(), ERR_INVALID_WAD_SIGNATURE);

    cleanup_temp_test_file(&path);
}

#[test]
fn validate_wad_rejects_truncated_header() {
    let path = write_temp_test_file(b"IWAD");

    let err = match validate_wad(path.to_str().expect("temp path should be valid utf-8")) {
        Ok(_) => panic!("expected truncated header to be rejected"),
        Err(err) => err,
    };

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    assert_eq!(err.to_string(), ERR_INVALID_WAD_HEADER);

    cleanup_temp_test_file(&path);
}

#[test]
fn read_directory_rejects_truncated_directory_entries() {
    let mut data = Vec::new();
    data.extend_from_slice(WAD_SIGNATURE_IWAD.as_bytes());
    data.extend_from_slice(&1i32.to_le_bytes());
    data.extend_from_slice(&(WAD_HEADER_SIZE as i32).to_le_bytes());
    data.extend_from_slice(&[0u8; 8]);
    let path = write_temp_test_file(&data);

    let err = match read_directory(ReadDirectoryParams {
        path: path.to_str().expect("temp path should be valid utf-8"),
        header: &WadHeader {
            num_lumps: 1,
            directory_offset: WAD_HEADER_SIZE as i32,
        },
    }) {
        Ok(_) => panic!("expected truncated directory entry to be rejected"),
        Err(err) => err,
    };

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    assert_eq!(err.to_string(), ERR_INVALID_WAD_HEADER);

    cleanup_temp_test_file(&path);
}

#[test]
fn read_lump_data_rejects_truncated_lump_payload() {
    let path = write_temp_test_file(&[1u8, 2u8]);
    let lump = LumpInfo {
        name: "TEST".to_string(),
        file_pos: 0,
        size: 4,
    };

    let err = read_lump_data(ReadLumpDataParams {
        path: path.to_str().expect("temp path should be valid utf-8"),
        lump: &lump,
    })
    .expect_err("expected truncated lump payload to be rejected");

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    assert_eq!(err.to_string(), ERR_INVALID_LUMP_DATA);

    cleanup_temp_test_file(&path);
}

#[test]
fn compose_textures_skips_missing_patch_lumps_with_safe_fallback() {
    let texture_defs = vec![TextureDef {
        name: "BROKEN".to_string(),
        width: 4,
        height: 4,
        patches: vec![TexturePatchRef {
            origin_x: 0,
            origin_y: 0,
            patch_name: "MISSINGP".to_string(),
        }],
    }];
    let lumps = vec![LumpInfo {
        name: "UNUSED".to_string(),
        file_pos: 0,
        size: 0,
    }];

    let textures = compose_textures(ComposeTexturesParams {
        path: "/tmp/unused.wad",
        lumps: &lumps,
        texture_defs: &texture_defs,
    })
    .expect("missing optional patch lump should not fail texture composition");

    assert_eq!(textures.len(), 1);
    assert_eq!(textures[0].name, "BROKEN");
    assert_eq!(textures[0].width, 4);
    assert_eq!(textures[0].height, 4);
    assert!(textures[0].pixels.iter().all(|pixel| *pixel == 0));
}
