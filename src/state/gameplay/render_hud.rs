//! Compact telemetry strip, edge controls, and short-lived screen feedback.
//!
//! The factory floor is the primary surface. Larger consoles are opened from
//! the edge controls only when the player needs them.

mod beacon_panel;

use crate::data::strings::{fill, text};
use crate::data::GameData;
use crate::ui::{self, ConsoleButtonState, ConsoleIcon};
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::notifications::{
    draw_notification, Notification, NotificationRenderConfig, NotificationType,
};
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

use super::helpers::{beacon_color, threat_color};
use super::ui::{EdgeSide, EDGE_RAIL_MARGIN, EDGE_RAIL_W};
use super::ui_advice::{AdviceTarget, AlertSeverity, PowerGridSnapshot, UiAdvice};
use super::{GameplayState, HudDetail};

impl GameplayState {
    pub(super) fn draw_hud(&mut self, data: &GameData) {
        let advice = self.build_ui_advice(data);
        let hud_h = self.constants.ui.hud_height;
        let sw = screen_width();
        draw_rectangle(0.0, 0.0, sw, hud_h, Color::new(0.018, 0.026, 0.031, 0.94));
        draw_rectangle(
            0.0,
            hud_h - 3.0,
            sw,
            3.0,
            Color::new(0.16, 0.42, 0.42, 0.82),
        );

        let gap = 12.0;
        let margin = 14.0;
        let status_w = (sw * 0.15).clamp(140.0, 184.0);
        let threat_w = (sw * 0.18).clamp(156.0, 214.0);
        let header_right = sw - EDGE_RAIL_MARGIN - EDGE_RAIL_W - 10.0;
        let metrics_w = (header_right - margin - status_w - threat_w - gap * 2.0).max(220.0);
        let status_rect = Rect::new(margin, 9.0, status_w, hud_h - 18.0);
        let metrics_rect = Rect::new(status_rect.right() + gap, 9.0, metrics_w, hud_h - 18.0);
        let threat_rect = Rect::new(metrics_rect.right() + gap, 9.0, threat_w, hud_h - 18.0);

        draw_line(
            status_rect.right() + gap * 0.5,
            12.0,
            status_rect.right() + gap * 0.5,
            hud_h - 12.0,
            1.0,
            Color::new(0.24, 0.42, 0.44, 0.5),
        );
        draw_line(
            metrics_rect.right() + gap * 0.5,
            12.0,
            metrics_rect.right() + gap * 0.5,
            hud_h - 12.0,
            1.0,
            Color::new(0.24, 0.42, 0.44, 0.5),
        );

        self.draw_compact_status(status_rect);
        self.draw_compact_metrics(metrics_rect, &advice.power);
        self.draw_compact_threat(threat_rect, &advice);

        if !self.has_selection()
            && !self.coach.active
            && self.salvage_report.is_none()
            && self.pending_depth_directive.is_none()
        {
            self.draw_compact_next(data, &advice);
        }
        self.draw_hud_detail(&advice);
    }

    fn draw_compact_status(&mut self, rect: Rect) {
        let t = &text().hud;
        draw_ui_text(
            &t.survival,
            rect.x + 10.0,
            rect.y + 17.0,
            11.0,
            dark::POSITIVE,
        );
        ui::draw_bounded_text(
            &self.depth_readout(),
            rect.x + 10.0,
            rect.y + 34.0,
            rect.w - 68.0,
            10.0,
            dark::TEXT_DIM,
        );
        ui::draw_bounded_text(
            &self.beacon_phase.label().to_uppercase(),
            rect.x + 10.0,
            rect.y + 52.0,
            rect.w - 68.0,
            11.0,
            beacon_color(&self.beacon_phase),
        );
        let phase_rect = Rect::new(rect.x, rect.y + 35.0, rect.w - 58.0, 24.0);
        if phase_rect.contains(vec2(mouse_position().0, mouse_position().1))
            && is_mouse_button_released(MouseButton::Left)
        {
            self.hud_detail = Some(HudDetail::Beacon);
        }
        let speed_label = if self.time_scale > 1.0 {
            &t.speed_fast
        } else {
            &t.speed_normal
        };
        if ui::draw_console_button(
            rect.x + rect.w - 54.0,
            rect.y + 9.0,
            48.0,
            44.0,
            speed_label,
            ConsoleButtonState::Affordable,
        ) {
            self.time_scale = if self.time_scale > 1.0 {
                1.0
            } else {
                self.constants.gameplay.speed_multiplier
            };
        }
    }

    fn draw_compact_metrics(&mut self, rect: Rect, power: &PowerGridSnapshot) {
        let t = &text().hud;
        let entries = [
            (
                ConsoleIcon::People,
                t.pop.as_str(),
                format!("{}/{}", self.population.count, self.shelter_capacity()),
                dark::TEXT_BRIGHT,
            ),
            (
                ConsoleIcon::Scrap,
                t.scrap.as_str(),
                format!("{:.0}", self.resources.scrap),
                dark::TEXT_BRIGHT,
            ),
            (
                ConsoleIcon::Power,
                t.power.as_str(),
                format!("{:.0} {:+.0}", power.battery, power.net),
                dark::POSITIVE,
            ),
            (
                ConsoleIcon::Food,
                t.food.as_str(),
                format!("{:.0}", self.population.food_supply),
                dark::TEXT_BRIGHT,
            ),
            (
                ConsoleIcon::Water,
                t.water.as_str(),
                format!("{:.0}", self.resources.water),
                dark::TEXT_BRIGHT,
            ),
            (
                ConsoleIcon::Morale,
                t.condition.as_str(),
                format!(
                    "{:.0}/{:.0}",
                    self.population.morale, self.population.health
                ),
                if self.population.morale < 25.0 {
                    dark::NEGATIVE
                } else {
                    dark::POSITIVE
                },
            ),
        ];
        let compact = rect.w < 420.0;
        let columns = if compact { 3 } else { entries.len() };
        let rows = if compact { 2 } else { 1 };
        let col_w = rect.w / columns as f32;
        let row_h = rect.h / rows as f32;
        for (idx, (icon, label, value, color)) in entries.iter().enumerate() {
            let column = if compact { idx % columns } else { idx };
            let row = if compact { idx / columns } else { 0 };
            let x = rect.x + column as f32 * col_w;
            let y = rect.y + row as f32 * row_h;
            ui::draw_icon(
                *icon,
                x + 4.0,
                y + if compact { 5.0 } else { 9.0 },
                if compact { 14.0 } else { 18.0 },
                Color::new(color.r, color.g, color.b, 0.78),
            );
            let text_x = x + if compact { 22.0 } else { 29.0 };
            draw_ui_text(
                label,
                text_x,
                y + if compact { 15.0 } else { 20.0 },
                8.0,
                dark::TEXT_DIM,
            );
            draw_ui_text(
                value,
                text_x,
                y + if compact { 32.0 } else { 42.0 },
                if compact { 12.0 } else { 14.0 },
                *color,
            );
        }
        let power_cell = Rect::new(
            rect.x + col_w * 2.0,
            rect.y,
            col_w,
            if compact { row_h } else { rect.h },
        );
        if power_cell.contains(vec2(mouse_position().0, mouse_position().1))
            && is_mouse_button_released(MouseButton::Left)
        {
            self.hud_detail = Some(HudDetail::Power);
        }
    }

    fn draw_compact_threat(&mut self, rect: Rect, advice: &UiAdvice) {
        let t = &text().hud;
        let color = threat_color(&self.threat, &self.constants.threat);
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(color.r, color.g, color.b, 0.12),
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.5,
            Color::new(color.r, color.g, color.b, 0.55),
        );
        draw_ui_text(&t.threat, rect.x + 10.0, rect.y + 17.0, 11.0, color);
        draw_ui_text(
            &format!(
                "{:.0}% · {}",
                self.threat.awareness_level(),
                self.threat.reaction_tier(&self.constants.threat).label()
            ),
            rect.x + 10.0,
            rect.y + 34.0,
            10.0,
            dark::TEXT_BRIGHT,
        );
        let wave = if !self.beacon_active && !self.shutdown_triggered {
            t.beacon_offline.clone()
        } else if self.between_waves {
            fill(
                &t.wave_incoming,
                &[
                    ("wave", &(self.current_wave + 1).to_string()),
                    ("secs", &format!("{:.0}", self.wave_timer.max(0.0))),
                ],
            )
        } else {
            fill(
                &t.wave_active,
                &[
                    ("wave", &self.current_wave.to_string()),
                    ("alive", &self.wave_manager.alive_count().to_string()),
                ],
            )
        };
        ui::draw_bounded_text(
            &wave,
            rect.x + 10.0,
            rect.y + 51.0,
            rect.w - 20.0,
            9.0,
            dark::TEXT_DIM,
        );
        if let Some(alert) = advice.alerts.first() {
            let alert_color = if alert.severity == AlertSeverity::Critical {
                dark::NEGATIVE
            } else {
                dark::WARNING
            };
            draw_rectangle(rect.x + rect.w - 15.0, rect.y + 7.0, 8.0, 8.0, alert_color);
        } else {
            let composition = super::ui_advice::format_enemy_counts(&advice.wave_preview.counts);
            ui::draw_bounded_text(
                &composition,
                rect.x + 10.0,
                rect.y + 64.0,
                rect.w - 20.0,
                9.0,
                dark::TEXT_DIM,
            );
        }
        if rect.contains(vec2(mouse_position().0, mouse_position().1))
            && is_mouse_button_released(MouseButton::Left)
        {
            self.hud_detail = Some(HudDetail::Threat);
        }
    }

    fn draw_compact_next(&mut self, data: &GameData, advice: &UiAdvice) {
        let t = &text().hud;
        if matches!(advice.suggested_action.target, AdviceTarget::None) {
            return;
        }
        let side_clearance = EDGE_RAIL_MARGIN + EDGE_RAIL_W + 8.0;
        let rect_w = 420.0_f32.min(screen_width() - side_clearance * 2.0);
        let rect = Rect::new(
            ((screen_width() - rect_w) * 0.5).max(side_clearance),
            self.constants.ui.hud_height + 12.0,
            rect_w,
            90.0,
        );
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.035, 0.07, 0.10, 0.96),
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.5,
            Color::new(0.20, 0.55, 0.75, 0.8),
        );
        draw_ui_text(
            &t.next_step,
            rect.x + 10.0,
            rect.y + 16.0,
            10.0,
            dark::ACCENT,
        );
        ui::draw_bounded_text(
            &advice.suggested_action.label,
            rect.x + 10.0,
            rect.y + 34.0,
            rect.w - 86.0,
            13.0,
            dark::TEXT_BRIGHT,
        );
        ui::draw_bounded_text(
            &advice.suggested_action.cost,
            rect.x + 10.0,
            rect.y + 50.0,
            rect.w - 86.0,
            9.0,
            dark::TEXT_DIM,
        );
        let interactive = !self.show_intro
            && !self.show_workforce
            && self.salvage_report.is_none()
            && self.pending_depth_directive.is_none()
            && !self.paused;
        let state = if !interactive || matches!(advice.suggested_action.target, AdviceTarget::None)
        {
            ConsoleButtonState::Disabled
        } else {
            ConsoleButtonState::Recommended
        };
        if interactive
            && ui::draw_console_button(
                rect.x + rect.w - 74.0,
                rect.y + 7.0,
                64.0,
                44.0,
                &t.focus,
                state,
            )
        {
            let target = advice.suggested_action.target.clone();
            self.apply_advice_focus(&target, data);
        }
        ui::draw_bounded_text(
            &advice.suggested_action.detail,
            rect.x + 10.0,
            rect.y + 81.0,
            rect.w - 86.0,
            8.0,
            dark::TEXT_DIM,
        );
        ui::draw_bounded_text(
            &advice.risk,
            rect.x + 10.0,
            rect.y + 65.0,
            rect.w - 86.0,
            9.0,
            dark::WARNING,
        );
    }

    fn draw_hud_detail(&mut self, advice: &UiAdvice) {
        let Some(detail) = self.hud_detail else {
            return;
        };
        let t = &text().hud;
        let panel_w = 320.0_f32.min(screen_width() - 32.0);
        let panel_h = 150.0_f32.min(screen_height() - self.constants.ui.hud_height - 24.0);
        let panel_x = (screen_width() - panel_w) * 0.5;
        let panel_y = self.constants.ui.hud_height + 12.0;
        draw_rectangle(
            panel_x,
            panel_y,
            panel_w,
            panel_h,
            Color::new(0.025, 0.045, 0.055, 0.98),
        );
        draw_rectangle_lines(
            panel_x,
            panel_y,
            panel_w,
            panel_h,
            1.6,
            Color::new(0.24, 0.62, 0.68, 0.95),
        );

        let (title, lines, accent) = match detail {
            HudDetail::Power => (
                t.power_grid.clone(),
                vec![
                    format!("{} {:.0}", t.gen, advice.power.generated),
                    format!("{} {:.0}", t.used, advice.power.used),
                    format!("{} {:+.0}/s", t.net, advice.power.net),
                    format!("{} {:.0}", t.battery, advice.power.battery),
                ],
                dark::POSITIVE,
            ),
            HudDetail::Threat => {
                let mut lines = Vec::new();
                if let Some(alert) = advice.alerts.first() {
                    lines.push(alert.label.clone());
                    lines.push(alert.detail.clone());
                } else {
                    lines.push(format!(
                        "{:.0}% · {}",
                        self.threat.awareness_level(),
                        self.threat.reaction_tier(&self.constants.threat).label()
                    ));
                    lines.push(super::ui_advice::format_enemy_counts(
                        &advice.wave_preview.counts,
                    ));
                }
                (
                    t.threat.clone(),
                    lines,
                    threat_color(&self.threat, &self.constants.threat),
                )
            }
            HudDetail::Beacon => (
                t.beacon_control.clone(),
                vec![
                    self.beacon_phase.label().to_uppercase(),
                    if self.beacon_active {
                        fill(
                            &t.evac_pending,
                            &[
                                ("n", &self.survivors_evacuated.to_string()),
                                ("pending", &format!("{:.0}", self.pending_evacuees)),
                            ],
                        )
                    } else {
                        fill(&t.evac, &[("n", &self.survivors_evacuated.to_string())])
                    },
                    fill(
                        &t.next_phase,
                        &[("draw", &format!("{:.0}", self.beacon_draw_to_next_phase()))],
                    ),
                ],
                beacon_color(&self.beacon_phase),
            ),
        };
        draw_ui_text(&title, panel_x + 16.0, panel_y + 24.0, 14.0, accent);
        for (index, line) in lines.iter().enumerate() {
            ui::draw_bounded_text(
                line,
                panel_x + 16.0,
                panel_y + 48.0 + index as f32 * 20.0,
                panel_w - 32.0,
                12.0,
                if index == 0 {
                    dark::TEXT_BRIGHT
                } else {
                    dark::TEXT_DIM
                },
            );
        }
        if ui::draw_console_button(
            panel_x + panel_w - 82.0,
            panel_y + panel_h - 50.0,
            66.0,
            40.0,
            &text().panels.close,
            ConsoleButtonState::Affordable,
        ) {
            self.hud_detail = None;
        }
    }

    pub(super) fn draw_edge_controls(&mut self) {
        let t = &text().hud;
        if ui::draw_edge_button(
            self.edge_button_rect(EdgeSide::Left, 0),
            ConsoleIcon::Tower,
            &t.edge_build,
            self.show_build_panel,
            dark::ACCENT,
        ) {
            self.open_only_build_panel();
        }
        if ui::draw_edge_button(
            self.edge_button_rect(EdgeSide::Left, 1),
            ConsoleIcon::Power,
            &t.edge_systems,
            self.show_sector_panel,
            dark::POSITIVE,
        ) {
            self.open_only_sector_panel();
        }
        if ui::draw_edge_button(
            self.edge_button_rect(EdgeSide::Left, 2),
            ConsoleIcon::People,
            &t.edge_shift,
            self.show_workforce,
            dark::POSITIVE,
        ) {
            self.close_dock_panels();
            self.show_workforce = true;
        }
        if self.placing_tower.is_some()
            && ui::draw_edge_button(
                self.edge_button_rect(EdgeSide::Left, 3),
                ConsoleIcon::Cancel,
                &t.cancel,
                false,
                dark::WARNING,
            )
        {
            self.placing_tower = None;
        }
        if ui::draw_edge_button(
            self.edge_button_rect(EdgeSide::Right, 0),
            ConsoleIcon::Risk,
            &t.edge_beacon,
            self.show_beacon_panel,
            dark::NEGATIVE,
        ) {
            self.open_only_beacon_panel();
        }
        if ui::draw_edge_button(
            self.edge_button_rect(EdgeSide::Right, 1),
            ConsoleIcon::Pause,
            &t.edge_pause,
            false,
            dark::WARNING,
        ) {
            self.close_dock_panels();
            self.paused = true;
        }
        if ui::draw_edge_button(
            self.camera_control_rect(0),
            ConsoleIcon::ZoomIn,
            &t.zoom_in,
            false,
            dark::ACCENT,
        ) {
            self.camera
                .zoom_at(1.2, vec2(screen_width() * 0.5, screen_height() * 0.5));
        }
        if ui::draw_edge_button(
            self.camera_control_rect(1),
            ConsoleIcon::ZoomOut,
            &t.zoom_out,
            false,
            dark::ACCENT,
        ) {
            self.camera
                .zoom_at(1.0 / 1.2, vec2(screen_width() * 0.5, screen_height() * 0.5));
        }
        if ui::draw_edge_button(
            self.camera_control_rect(2),
            ConsoleIcon::Reset,
            &t.reset_view,
            false,
            dark::ACCENT,
        ) {
            self.reset_camera_view();
        }
        if screen_height() >= 420.0 {
            ui::draw_bounded_text(
                &t.camera_hint,
                EDGE_RAIL_MARGIN + EDGE_RAIL_W + 16.0,
                screen_height() - 10.0,
                screen_width() - 2.0 * (EDGE_RAIL_MARGIN + EDGE_RAIL_W + 16.0),
                9.0,
                dark::TEXT_DIM,
            );
        }
    }

    pub(super) fn draw_wave_start_flash(&self) {
        if self.wave_flash_timer <= 0.0 {
            return;
        }
        let alpha = (self.wave_flash_timer / self.constants.ui.wave_flash_duration).clamp(0.0, 1.0);
        let t = &text().hud;
        let flash = if self.shutdown_triggered {
            t.beacon_shutdown_flash.clone()
        } else {
            fill(&t.wave_flash, &[("n", &self.last_wave_started.to_string())])
        };
        let w = 300.0;
        let h = 46.0;
        let x = (screen_width() - w) * 0.5;
        let y = self.constants.ui.hud_height + 14.0;
        draw_rectangle(x, y, w, h, Color::new(0.08, 0.07, 0.06, 0.84 * alpha));
        draw_rectangle_lines(x, y, w, h, 2.0, Color::new(1.0, 0.58, 0.18, 0.9 * alpha));
        draw_ui_text(
            &t.incoming,
            x + 18.0,
            y + 17.0,
            11.0,
            Color::new(1.0, 0.72, 0.34, alpha),
        );
        let dims = measure_ui_text(&flash, None, 24, 1.0);
        draw_ui_text(
            &flash,
            x + (w - dims.width) * 0.5,
            y + 34.0,
            24.0,
            Color::new(1.0, 0.9, 0.7, alpha),
        );
    }

    pub(super) fn draw_notifications(&self) {
        let config = NotificationRenderConfig {
            width: 340.0,
            row_height: 30.0,
            spacing: 4.0,
            padding: 8.0,
            font_size: 16.0,
            background: Color::new(0.08, 0.08, 0.1, 0.9),
            text_color: dark::TEXT_BRIGHT,
            ..Default::default()
        };
        let mut y = self.constants.ui.hud_height + 28.0;
        for note in &self.notifications {
            let notification = Notification {
                message: note.text.clone(),
                notification_type: NotificationType::Info,
                time_remaining: note.ttl,
                total_duration: self.constants.ui.notification_ttl,
            };
            draw_notification(
                &notification,
                super::ui::EDGE_RAIL_MARGIN + super::ui::EDGE_RAIL_W + 14.0,
                y,
                &config,
            );
            y += config.row_height + config.spacing;
        }
    }
}
