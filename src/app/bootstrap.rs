use std::io;

use crate::constants::{
    MAP_CENTER_DIVISOR, THING_TYPE_PLAYER_1_START, THING_TYPE_PLAYER_4_START,
    UI_BOOTSTRAP_CWD_UNAVAILABLE, UI_BOOTSTRAP_INIT_RENDERER, UI_BOOTSTRAP_LOAD_MAP_PREFIX,
    UI_BOOTSTRAP_PREFIX, UI_BOOTSTRAP_SEARCH_WAD, UI_BOOTSTRAP_START,
    UI_BOOTSTRAP_WAD_CANDIDATES_LABEL, UI_BOOTSTRAP_WAD_CANDIDATES_SEPARATOR,
    UI_BOOTSTRAP_WAD_CURRENT_DIR_LABEL, UI_BOOTSTRAP_WAD_FOUND_PREFIX, WAD_CANDIDATES,
    WAD_DEFAULT_MAP_NAME, WAD_NOT_FOUND_MESSAGE,
};
use crate::render::default_render_state;
use crate::types::{
    AppBootstrap, GameState, GameStateInit, LoadMapParams, MapPlayerSpawnParams, Player,
    TerminalRenderer,
};
use crate::wad::load_map_data;

pub fn bootstrap_app() -> io::Result<AppBootstrap> {
    log_bootstrap_step(UI_BOOTSTRAP_START);
    log_bootstrap_step(UI_BOOTSTRAP_SEARCH_WAD);

    let wad_path = find_wad_path().ok_or_else(wad_not_found_error)?;

    log_bootstrap_step(&format!("{UI_BOOTSTRAP_WAD_FOUND_PREFIX} {wad_path}"));
    log_bootstrap_step(&format!(
        "{UI_BOOTSTRAP_LOAD_MAP_PREFIX} {WAD_DEFAULT_MAP_NAME}"
    ));

    let mut map_data = load_map_data(LoadMapParams {
        path: wad_path,
        map_name: WAD_DEFAULT_MAP_NAME,
    })?;
    let bounds = map_data.bounds();

    log_bootstrap_step(UI_BOOTSTRAP_INIT_RENDERER);

    let renderer = TerminalRenderer::new()?;

    let (spawn_x, spawn_y) = map_player_spawn(MapPlayerSpawnParams {
        map_data: &map_data,
        bounds,
    });

    map_data
        .things
        .retain(|thing| !is_player_start_thing_type(thing.thing_type));

    let player = Player::new(spawn_x, spawn_y);

    let game_state = GameState::new(GameStateInit {
        map: map_data,
        player,
    });
    let mut render_state = default_render_state();

    render_state
        .c_frame_provider
        .configure_from_map(&game_state.map, &game_state.player);

    Ok(AppBootstrap {
        game_state,
        render_state,
        renderer,
    })
}

fn find_wad_path() -> Option<&'static str> {
    WAD_CANDIDATES
        .iter()
        .copied()
        .find(|path| std::path::Path::new(path).exists())
}

fn wad_not_found_error() -> io::Error {
    let cwd = std::env::current_dir()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| UI_BOOTSTRAP_CWD_UNAVAILABLE.to_string());
    let candidates = WAD_CANDIDATES.join(UI_BOOTSTRAP_WAD_CANDIDATES_SEPARATOR);

    let message = format!(
        "{WAD_NOT_FOUND_MESSAGE}\n{UI_BOOTSTRAP_WAD_CURRENT_DIR_LABEL} {cwd}\n{UI_BOOTSTRAP_WAD_CANDIDATES_LABEL} {candidates}"
    );

    io::Error::new(io::ErrorKind::NotFound, message)
}

fn log_bootstrap_step(message: &str) {
    eprintln!("{UI_BOOTSTRAP_PREFIX} {message}");
}

fn map_player_spawn(params: MapPlayerSpawnParams<'_>) -> (f32, f32) {
    let MapPlayerSpawnParams { map_data, bounds } = params;

    let player_start = map_data
        .things
        .iter()
        .find(|thing| is_player_start_thing_type(thing.thing_type));

    if let Some(spawn) = player_start {
        return (spawn.x as f32, spawn.y as f32);
    }

    (
        bounds.min_x + (bounds.width / MAP_CENTER_DIVISOR),
        bounds.min_y + (bounds.height / MAP_CENTER_DIVISOR),
    )
}

fn is_player_start_thing_type(thing_type: u16) -> bool {
    (THING_TYPE_PLAYER_1_START..=THING_TYPE_PLAYER_4_START).contains(&thing_type)
}

#[cfg(test)]
#[path = "../tests/app/bootstrap_tests.rs"]
mod tests;
