use crate::types::{LineDef, MapData, PaletteColor, Player, Sector, SideDef, Vertex};

#[allow(dead_code)]
pub fn sample_non_solid_collision_case() -> (MapData, Player) {
    let map = MapData {
        vertexes: vec![
            Vertex { x: 0, y: 0 },
            Vertex { x: 120, y: 0 },
            Vertex { x: 120, y: 120 },
            Vertex { x: 0, y: 120 },
        ],
        lines: vec![LineDef {
            start_vertex: 0,
            end_vertex: 1,
            right_sidedef: 0,
            left_sidedef: 1,
        }],
        sidedefs: vec![
            SideDef {
                sector: 0,
                middle_texture: String::new(),
            },
            SideDef {
                sector: 0,
                middle_texture: String::new(),
            },
        ],
        sectors: vec![Sector {
            floor_texture: String::new(),
            ceiling_texture: String::new(),
            light_level: 160,
        }],
        palette: vec![PaletteColor { r: 0, g: 0, b: 0 }],
        textures: vec![],
        flats: vec![],
        things: vec![],
        bsp_line_indices: vec![],
    };

    let player = Player {
        x: 60.0,
        y: 20.0,
        angle: 0.0,
        health: 100,
    };

    (map, player)
}
