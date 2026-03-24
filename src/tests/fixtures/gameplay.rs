use crate::constants::{
    THING_TYPE_PLAYER_1_START, WAD_FLAT_CEIL1_1, WAD_FLAT_FLOOR0_1, WAD_INVALID_SIDEDEF_INDEX,
    WAD_TEXTURE_STARTAN2,
};
use crate::types::{
    FlatTexture, GameState, GameStateInit, LineDef, MapData, MapTexture, PaletteColor, Player,
    Sector, SideDef, Thing, Vertex,
};

#[allow(dead_code)]
pub fn sample_game_state() -> GameState {
    let map = MapData {
        vertexes: vec![
            Vertex { x: 0, y: 0 },
            Vertex { x: 100, y: 0 },
            Vertex { x: 100, y: 100 },
            Vertex { x: 0, y: 100 },
        ],
        lines: vec![
            LineDef {
                start_vertex: 0,
                end_vertex: 1,
                right_sidedef: 0,
                left_sidedef: WAD_INVALID_SIDEDEF_INDEX,
            },
            LineDef {
                start_vertex: 1,
                end_vertex: 2,
                right_sidedef: 0,
                left_sidedef: WAD_INVALID_SIDEDEF_INDEX,
            },
            LineDef {
                start_vertex: 2,
                end_vertex: 3,
                right_sidedef: 0,
                left_sidedef: WAD_INVALID_SIDEDEF_INDEX,
            },
            LineDef {
                start_vertex: 3,
                end_vertex: 0,
                right_sidedef: 0,
                left_sidedef: WAD_INVALID_SIDEDEF_INDEX,
            },
        ],
        sidedefs: vec![SideDef {
            sector: 0,
            middle_texture: WAD_TEXTURE_STARTAN2.to_string(),
        }],
        sectors: vec![Sector {
            floor_texture: WAD_FLAT_FLOOR0_1.to_string(),
            ceiling_texture: WAD_FLAT_CEIL1_1.to_string(),
            light_level: 160,
        }],
        palette: vec![
            PaletteColor { r: 0, g: 0, b: 0 },
            PaletteColor {
                r: 200,
                g: 200,
                b: 200,
            },
        ],
        textures: vec![MapTexture {
            name: WAD_TEXTURE_STARTAN2.to_string(),
            width: 64,
            height: 128,
            pixels: vec![1; 64 * 128],
        }],
        flats: vec![
            FlatTexture {
                name: WAD_FLAT_FLOOR0_1.to_string(),
                pixels: vec![1; 64 * 64],
            },
            FlatTexture {
                name: WAD_FLAT_CEIL1_1.to_string(),
                pixels: vec![1; 64 * 64],
            },
        ],
        things: vec![Thing {
            x: 60,
            y: 60,
            thing_type: THING_TYPE_PLAYER_1_START,
        }],
        bsp_line_indices: vec![0, 1, 2, 3],
    };

    let player = Player {
        x: 50.0,
        y: 50.0,
        angle: 0.0,
        health: 100,
    };

    GameState::new(GameStateInit { map, player })
}

#[allow(dead_code)]
pub fn sample_pickup_game_state(thing_type: u16) -> GameState {
    GameState::new(GameStateInit {
        map: MapData {
            vertexes: vec![Vertex { x: 0, y: 0 }, Vertex { x: 64, y: 64 }],
            lines: vec![],
            sidedefs: vec![SideDef {
                sector: 0,
                middle_texture: String::new(),
            }],
            sectors: vec![Sector {
                floor_texture: String::new(),
                ceiling_texture: String::new(),
                light_level: 160,
            }],
            palette: vec![PaletteColor { r: 0, g: 0, b: 0 }],
            textures: vec![],
            flats: vec![],
            things: vec![Thing {
                x: 10,
                y: 10,
                thing_type,
            }],
            bsp_line_indices: vec![],
        },
        player: Player {
            x: 10.0,
            y: 10.0,
            angle: 0.0,
            health: 50,
        },
    })
}
