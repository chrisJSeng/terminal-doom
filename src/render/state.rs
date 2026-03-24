use crate::constants::{CAMERA_DEFAULT_OFFSET_X, CAMERA_DEFAULT_OFFSET_Y, CAMERA_DEFAULT_ZOOM};
use crate::types::{Camera, RenderState};

pub fn default_render_state() -> RenderState {
    RenderState {
        camera: Camera {
            offset_x: CAMERA_DEFAULT_OFFSET_X,
            offset_y: CAMERA_DEFAULT_OFFSET_Y,
            zoom: CAMERA_DEFAULT_ZOOM,
        },
        use_c_frame_provider: true,
        c_frame_provider: crate::types::CFrameProvider::new(),
    }
}
