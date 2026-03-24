use crate::constants::{
    UI_ALIVE_NO, UI_ALIVE_YES, UI_BACKEND_LABEL_FRAMEBUFFER, UI_FLAG_OFF, UI_FLAG_ON, UI_PICKUP_NO,
    UI_PICKUP_YES, UI_SOURCE_LABEL_C_FFI, UI_SOURCE_LABEL_TEST, UI_STATUS_ALIVE_PREFIX,
    UI_STATUS_ANGLE_PREFIX, UI_STATUS_ARMOR_PREFIX, UI_STATUS_BULLETS_PREFIX,
    UI_STATUS_COLLISION_PREFIX, UI_STATUS_HEALTH_PREFIX, UI_STATUS_PICKUP_PREFIX,
    UI_STATUS_POS_PREFIX, UI_STATUS_SEPARATOR, UI_STATUS_SHELLS_PREFIX, UI_STATUS_TEXT,
    UI_STATUS_ZOOM_PREFIX,
};
use crate::types::{BoolLabelParams, RuntimeStatusTextParams, StatusTextParams};

pub fn bool_label(params: BoolLabelParams) -> &'static str {
    let BoolLabelParams {
        flag,
        false_label,
        true_label,
    } = params;

    [false_label, true_label][flag as usize]
}

pub fn build_status_text(params: StatusTextParams<'_>) -> String {
    let StatusTextParams {
        backend_label,
        source_label,
        collision_flag,
        pickup_flag,
        health,
        armor,
        bullets,
        shells,
        player_alive,
        world_x,
        world_y,
        angle_degrees,
        zoom,
        max_width,
    } = params;

    let always_visible = vec![
        format!("{}:{}", backend_label, source_label),
        format!("{}:{}", UI_STATUS_HEALTH_PREFIX, health),
        format!("{}:{}", UI_STATUS_ARMOR_PREFIX, armor),
        format!("{}:{}", UI_STATUS_BULLETS_PREFIX, bullets),
        format!("{}:{}", UI_STATUS_SHELLS_PREFIX, shells),
        format!("{}:{:.0},{:.0}", UI_STATUS_POS_PREFIX, world_x, world_y),
        format!("{}:{}", UI_STATUS_ANGLE_PREFIX, angle_degrees),
    ];
    let context_segments = vec![
        format!("{}:{}", UI_STATUS_COLLISION_PREFIX, collision_flag),
        format!("{}:{}", UI_STATUS_PICKUP_PREFIX, pickup_flag),
        format!("{}:{}", UI_STATUS_ALIVE_PREFIX, player_alive),
        format!("{}:{:.2}", UI_STATUS_ZOOM_PREFIX, zoom),
    ];
    let hint_segments = vec![UI_STATUS_TEXT.to_string()];

    let full_text = join_segments(
        &[
            hint_segments,
            always_visible.clone(),
            context_segments.clone(),
        ]
        .concat(),
    );

    if full_text.chars().count() <= max_width {
        return full_text;
    }

    let context_text = join_segments(&[always_visible.clone(), context_segments].concat());

    if context_text.chars().count() <= max_width {
        return context_text;
    }

    join_segments(&always_visible)
}

pub fn build_runtime_status_text(params: RuntimeStatusTextParams<'_>) -> String {
    let RuntimeStatusTextParams {
        game_state,
        last_collision,
        last_pickup,
        render_state,
        max_width,
    } = params;

    let collision_flag = bool_label(BoolLabelParams {
        flag: last_collision,
        false_label: UI_FLAG_OFF,
        true_label: UI_FLAG_ON,
    });
    let backend = UI_BACKEND_LABEL_FRAMEBUFFER;
    let source_label = bool_label(BoolLabelParams {
        flag: render_state.use_c_frame_provider,
        false_label: UI_SOURCE_LABEL_TEST,
        true_label: UI_SOURCE_LABEL_C_FFI,
    });
    let player_alive = bool_label(BoolLabelParams {
        flag: game_state.player.is_alive(),
        false_label: UI_ALIVE_NO,
        true_label: UI_ALIVE_YES,
    });
    let pickup_flag = bool_label(BoolLabelParams {
        flag: last_pickup,
        false_label: UI_PICKUP_NO,
        true_label: UI_PICKUP_YES,
    });

    let provider_state = render_state.c_frame_provider.engine_player_state;
    let use_engine_status =
        render_state.use_c_frame_provider && render_state.c_frame_provider.engine_initialized;

    let engine_snapshot = use_engine_status.then_some((
        provider_state.health,
        provider_state.armor,
        provider_state.bullets,
        provider_state.shells,
        provider_state.angle,
    ));
    let (health, armor, bullets, shells, angle) =
        engine_snapshot.unwrap_or((game_state.player.health, 0, 0, 0, game_state.player.angle));

    let world_x = game_state.player.x;
    let world_y = game_state.player.y;
    let angle_degrees = normalize_angle_degrees(angle);
    let zoom = render_state.camera.zoom;

    let status_params = StatusTextParams {
        backend_label: backend,
        source_label,
        collision_flag,
        pickup_flag,
        health,
        armor,
        bullets,
        shells,
        player_alive,
        world_x,
        world_y,
        angle_degrees,
        zoom,
        max_width,
    };

    build_status_text(status_params)
}

fn join_segments(segments: &[String]) -> String {
    segments.join(UI_STATUS_SEPARATOR)
}

fn normalize_angle_degrees(angle_radians: f32) -> u16 {
    angle_radians.to_degrees().rem_euclid(360.0).round() as u16
}

#[cfg(test)]
#[path = "../tests/app/status_tests.rs"]
mod tests;
