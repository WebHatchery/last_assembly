//! The gameplay screen's view layer, split by surface: [`panels`] draws the
//! docked sidebars, [`placement`] the world-space build ghost, [`overlays`] the
//! modals and banners, and [`actions`] holds the resulting state mutations.
//!
//! This diverges from `docs/CODE_STANDARDS.md` §7.2's `UiAction`/dispatcher
//! pattern: panels call `self.*` mutation methods (defined in `actions`)
//! directly from inside their draw functions, rather than returning an intent
//! enum for a separate dispatcher to apply. Accepted here rather than fixed —
//! `GameplayState` is the sole owner of both the view and the state being
//! mutated, so a dispatcher would add a layer of indirection without buying
//! real decoupling (there's no second consumer of the intents, and no
//! alternate view that would ever replay them). See the code review dated
//! 2026-07-17 (Severity 3) for the fuller argument and the sites involved.

mod actions;
mod overlays;
mod panels;
mod placement;

use macroquad::prelude::*;

use super::GameplayState;

pub(crate) const EDGE_RAIL_W: f32 = 62.0;
pub(crate) const EDGE_RAIL_MARGIN: f32 = 12.0;
pub(crate) const EDGE_BUTTON_H: f32 = 52.0;
pub(crate) const EDGE_BUTTON_GAP: f32 = 8.0;

#[derive(Clone, Copy)]
pub(crate) enum EdgeSide {
    Left,
    Right,
}

impl GameplayState {
    pub(crate) fn edge_button_rect(&self, side: EdgeSide, index: usize) -> Rect {
        let button_h = if screen_height() < 500.0 {
            44.0
        } else {
            EDGE_BUTTON_H
        };
        let gap = if screen_height() < 500.0 {
            4.0
        } else {
            EDGE_BUTTON_GAP
        };
        let x = match side {
            EdgeSide::Left => EDGE_RAIL_MARGIN,
            EdgeSide::Right => screen_width() - EDGE_RAIL_MARGIN - EDGE_RAIL_W,
        };
        Rect::new(
            x,
            self.constants.ui.hud_height + 14.0 + index as f32 * (button_h + gap),
            EDGE_RAIL_W,
            button_h,
        )
    }

    pub(crate) fn camera_control_rect(&self, index: usize) -> Rect {
        self.edge_button_rect(EdgeSide::Right, index + 2)
    }

    pub(crate) fn build_panel_rect(&self) -> Rect {
        let y = self.constants.ui.hud_height + 12.0;
        let w = (screen_width() * 0.25).clamp(270.0, 320.0);
        Rect::new(
            EDGE_RAIL_MARGIN + EDGE_RAIL_W + 10.0,
            y,
            w,
            (screen_height() - y - 12.0).min(650.0),
        )
    }

    pub(crate) fn sector_panel_rect(&self) -> Rect {
        let y = self.constants.ui.hud_height + 12.0;
        let w = (screen_width() * 0.27).clamp(270.0, 340.0);
        let reveal_x = self.map_state.max_visible_x() + 120.0;
        let row_count = self
            .map_state
            .buildings
            .iter()
            .filter(|building| {
                self.map_state.is_building_visible(building) && building.position.x <= reveal_x
            })
            .count();
        let content_h =
            56.0 + row_count as f32 * 94.0 + row_count.saturating_sub(1) as f32 * 9.0 + 12.0;
        Rect::new(
            screen_width() - EDGE_RAIL_MARGIN - EDGE_RAIL_W - 10.0 - w,
            y,
            w,
            (screen_height() - y - 12.0).min(content_h.max(180.0)),
        )
    }

    pub(crate) fn beacon_panel_rect(&self) -> Rect {
        let y = self.constants.ui.hud_height + 12.0;
        let w = (screen_width() * 0.24).clamp(270.0, 320.0);
        Rect::new(
            screen_width() - EDGE_RAIL_MARGIN - EDGE_RAIL_W - 10.0 - w,
            y,
            w,
            (screen_height() - y - 12.0).min(224.0),
        )
    }

    pub(crate) fn context_panel_rect(&self) -> Rect {
        let x = EDGE_RAIL_MARGIN + EDGE_RAIL_W + 4.0;
        Rect::new(
            x,
            screen_height() - self.constants.ui.bottom_context_h - 12.0,
            (screen_width() - x * 2.0).max(280.0),
            self.constants.ui.bottom_context_h,
        )
    }

    pub(crate) fn any_dock_panel_open(&self) -> bool {
        self.show_build_panel || self.show_sector_panel || self.show_beacon_panel
    }

    pub(crate) fn has_selection(&self) -> bool {
        self.selected_core
            || self.selected_tower.is_some()
            || self.selected_slot.is_some()
            || self.selected_building.is_some()
    }

    pub(crate) fn coach_rect(&self) -> Rect {
        let bx = EDGE_RAIL_MARGIN + EDGE_RAIL_W + 18.0;
        let bw = (screen_width() - (EDGE_RAIL_MARGIN + EDGE_RAIL_W + 18.0) * 2.0).max(320.0);
        Rect::new(bx, self.constants.ui.hud_height + 10.0, bw, 96.0)
    }

    pub(crate) fn camera_input_is_captured(&self, point: Vec2) -> bool {
        if point.y < self.constants.ui.hud_height
            || point.x <= EDGE_RAIL_MARGIN + EDGE_RAIL_W
            || point.x >= screen_width() - EDGE_RAIL_MARGIN - EDGE_RAIL_W
        {
            return true;
        }
        if self.show_intro
            || self.show_workforce
            || self.show_settings
            || self.salvage_report.is_some()
            || self.pending_depth_directive.is_some()
            || self.paused
            || self.hud_detail.is_some()
        {
            return true;
        }
        let inside = |rect: Rect| rect.contains(point);
        if (self.show_build_panel && inside(self.build_panel_rect()))
            || (self.show_sector_panel && inside(self.sector_panel_rect()))
            || (self.show_beacon_panel && inside(self.beacon_panel_rect()))
            || (self.has_selection() && inside(self.context_panel_rect()))
            || (self.coach.active && inside(self.coach_rect()))
        {
            return true;
        }
        false
    }

    pub(crate) fn close_dock_panels(&mut self) {
        self.show_build_panel = false;
        self.show_sector_panel = false;
        self.show_beacon_panel = false;
    }

    pub(crate) fn open_only_build_panel(&mut self) {
        let open = !self.show_build_panel;
        self.close_dock_panels();
        self.show_build_panel = open;
    }

    pub(crate) fn open_only_sector_panel(&mut self) {
        let open = !self.show_sector_panel;
        self.close_dock_panels();
        self.show_sector_panel = open;
    }

    pub(crate) fn open_only_beacon_panel(&mut self) {
        let open = !self.show_beacon_panel;
        self.close_dock_panels();
        self.show_beacon_panel = open;
    }
}

/// Exactly one thing is selected at a time, and each selection clears the rest.
/// These keep that invariant in one place instead of repeating five assignments
/// at every call site.
impl GameplayState {
    fn select_slot_only(&mut self, idx: usize) {
        self.selected_slot = Some(idx);
        self.selected_building = None;
        self.selected_tower = self.map_state.slots[idx].tower_index;
        self.selected_core = false;
        self.selected_upgrade = None;
    }

    fn select_building_only(&mut self, idx: usize) {
        self.selected_building = Some(idx);
        self.selected_slot = None;
        self.selected_tower = None;
        self.selected_core = false;
        self.selected_upgrade = None;
    }

    /// Selecting the core opens the factory console, so pre-arm it with the next
    /// unpurchased upgrade rather than opening on an empty detail pane.
    fn select_core_only(&mut self) {
        self.selected_core = true;
        self.selected_slot = None;
        self.selected_building = None;
        self.selected_tower = None;
        if self.selected_upgrade.is_none() {
            if let Some(upg) = self
                .available_upgrades()
                .into_iter()
                .find(|u| !self.factory.has_upgrade(&u.id))
            {
                self.selected_upgrade = Some(upg.id.clone());
            }
        }
    }

    pub(crate) fn clear_selection(&mut self) {
        self.selected_slot = None;
        self.selected_building = None;
        self.selected_tower = None;
        self.selected_core = false;
        self.selected_upgrade = None;
    }
}
