use crate::render::default_render_state;
use crate::types::{ApplyExternalEngineInputParams, InputAction};

use super::apply_external_engine_input;

#[test]
fn apply_external_engine_input_queues_weapon_action() {
    let mut render_state = default_render_state();

    assert!(!apply_external_engine_input(
        ApplyExternalEngineInputParams {
            action: InputAction::WeaponPrev,
            render_state: &mut render_state,
        }
    ));
    assert_ne!(render_state.c_frame_provider.input_mask, 0);
}
