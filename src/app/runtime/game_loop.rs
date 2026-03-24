use std::io;
use std::time::Instant;

use crate::types::{
    AppBootstrap, ApplyExternalEngineInputParams, ApplyExternalFrameInputParams,
    DrawFramebufferParams, InputAction, InputStateUpdateParams, RuntimeStatusTextParams,
};

use super::super::input::{poll_input_actions, InputState};
use super::super::status::build_runtime_status_text;
use crate::framebuffer::FrameProvider;

pub fn run_game_loop(app: AppBootstrap) -> io::Result<()> {
    let AppBootstrap {
        mut game_state,
        mut renderer,
        mut render_state,
    } = app;

    let mut input_state = InputState::new();

    loop {
        renderer.refresh_size()?;

        let (pressed_actions, _) = poll_input_actions()?;
        let now = Instant::now();

        input_state.update(InputStateUpdateParams {
            pressed: &pressed_actions,
            now,
        });

        let should_quit = apply_external_frame_input(ApplyExternalFrameInputParams {
            input_state: &input_state,
            pressed_actions: &pressed_actions,
            render_state: &mut render_state,
        });

        if should_quit {
            break;
        }

        renderer.begin_framebuffer_frame()?;
        let frame = render_state.c_frame_provider.next_frame();

        renderer.draw_framebuffer_rgb(DrawFramebufferParams {
            rgb: &frame.rgb,
            source_width: frame.width,
            source_height: frame.height,
        })?;

        if render_state.c_frame_provider.engine_initialized {
            render_state
                .c_frame_provider
                .sync_player_from_engine(&mut game_state.player);
        }

        let status_text = build_runtime_status_text(RuntimeStatusTextParams {
            game_state: &game_state,
            last_collision: false,
            last_pickup: false,
            render_state: &render_state,
            max_width: renderer.cols as usize,
        });

        renderer.show_status(&status_text)?;
    }

    drop(renderer);
    std::thread::sleep(std::time::Duration::from_millis(200));

    Ok(())
}

fn apply_external_frame_input(params: ApplyExternalFrameInputParams<'_>) -> bool {
    let ApplyExternalFrameInputParams {
        input_state,
        pressed_actions,
        render_state,
    } = params;

    for action in input_state.ordered_held_actions() {
        if apply_external_engine_input(ApplyExternalEngineInputParams {
            action,
            render_state,
        }) {
            return true;
        }
    }

    for action in input_state.discrete_pressed_actions(pressed_actions) {
        if apply_external_engine_input(ApplyExternalEngineInputParams {
            action,
            render_state,
        }) {
            return true;
        }
    }

    false
}

fn apply_external_engine_input(params: ApplyExternalEngineInputParams<'_>) -> bool {
    let ApplyExternalEngineInputParams {
        action,
        render_state,
    } = params;

    match action {
        InputAction::Quit => true,
        InputAction::MoveForward
        | InputAction::MoveBackward
        | InputAction::StrafeLeft
        | InputAction::StrafeRight
        | InputAction::RotateLeft
        | InputAction::RotateRight
        | InputAction::Fire
        | InputAction::Use
        | InputAction::ToggleAutomap
        | InputAction::OpenMenu
        | InputAction::ConfirmMenu
        | InputAction::BackMenu
        | InputAction::WeaponPrev
        | InputAction::WeaponNext => {
            render_state.c_frame_provider.queue_input_action(action);
            false
        }
    }
}

#[cfg(test)]
#[path = "../../tests/app/game_loop_tests.rs"]
mod game_loop_tests;
