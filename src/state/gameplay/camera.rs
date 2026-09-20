//! Camera bounds derived from the currently revealed factory sections.

use macroquad::prelude::*;
use macroquad_toolkit::camera::CameraBounds;

use super::GameplayState;

impl GameplayState {
    pub(crate) fn reset_camera_view(&mut self) {
        let (vmin, vmax) = self.map_state.visible_bounds();
        let pad = 120.0;
        let viewport_w = screen_width().max(640.0);
        let viewport_h = (screen_height() - self.constants.ui.hud_height).max(240.0);
        let content_w = (vmax.x - vmin.x).max(1.0) + pad * 2.0;
        let content_h = (vmax.y - vmin.y).max(1.0) + pad * 2.0;
        self.camera.target = (vmin + vmax) * 0.5;
        self.camera.zoom = (viewport_w / content_w)
            .min(viewport_h / content_h)
            .clamp(self.camera.config.min_zoom, self.camera.config.max_zoom);
        self.sync_camera_bounds();
    }

    /// Grow the camera's pannable area to match the currently-revealed sections
    /// so newly-powered wings of the factory become reachable, while keeping the
    /// unrevealed expanse out of reach.
    pub(crate) fn sync_camera_bounds(&mut self) {
        let (vmin, vmax) = self.map_state.visible_bounds();
        let pad = 120.0;
        let bmin = vec2((vmin.x - pad).max(0.0), (vmin.y - pad).max(0.0));
        let bmax = vec2(
            (vmax.x + pad).min(self.map_state.map_size.x),
            (vmax.y + pad).min(self.map_state.map_size.y),
        );
        self.camera.set_bounds(Some(CameraBounds::new(bmin, bmax)));
    }
}
