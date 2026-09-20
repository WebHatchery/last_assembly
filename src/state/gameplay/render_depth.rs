//! Touch-first overlay for the operating choice at a newly reached depth.

use crate::data::strings::{fill, text};
use crate::engine::depth::DepthDirective;
use crate::ui::{self, ConsoleButtonState};
use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::{draw_ui_text, is_fully_visible, wrap_text};

use super::GameplayState;

impl GameplayState {
    pub(super) fn draw_depth_directive_overlay(&mut self) {
        let Some(depth) = self.pending_depth_directive else {
            return;
        };
        let t = text().depth_directive.clone();
        let sw = screen_width();
        let sh = screen_height();
        draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.7));

        let narrow = sw < 600.0;
        let panel_w = 820.0_f32.min(sw - 36.0);
        let panel_h = if narrow {
            520.0_f32.min(sh - 24.0)
        } else {
            360.0_f32.min(sh - 36.0)
        };
        let panel_x = (sw - panel_w) * 0.5;
        let panel_y = (sh - panel_h) * 0.5;
        let surface = macroquad_toolkit::ui::SurfaceStyle::new(Color::new(0.04, 0.06, 0.07, 0.99))
            .with_border(1.8, Color::new(0.3, 0.72, 0.78, 0.95));
        macroquad_toolkit::ui::draw_surface(
            Rect::new(panel_x, panel_y, panel_w, panel_h),
            &surface,
        );

        let title = fill(&t.title, &[("n", &format!("{depth:02}"))]);
        draw_ui_text(&title, panel_x + 28.0, panel_y + 42.0, 25.0, dark::ACCENT);
        ui::draw_bounded_text(
            &t.subtitle,
            panel_x + 28.0,
            panel_y + 68.0,
            panel_w - 56.0,
            13.0,
            dark::TEXT_DIM,
        );

        let gap = 12.0;
        let card_y = panel_y + 105.0;
        let mut chosen = None;
        let directives = DepthDirective::ALL;
        if narrow {
            let card_w = panel_w - 56.0;
            let card_h = 132.0;
            let list_view = Rect::new(panel_x + 28.0, card_y, card_w, (panel_h - 123.0).max(120.0));
            let content_h = directives.len() as f32 * (card_h + gap) - gap;
            self.depth_scroll
                .update_at(list_view, content_h, Vec2::from(mouse_position()));
            for (idx, directive) in directives.into_iter().enumerate() {
                let card_y = list_view.y + idx as f32 * (card_h + gap) - self.depth_scroll.offset();
                let card = Rect::new(panel_x + 28.0, card_y, card_w, card_h);
                if !is_fully_visible(card, list_view) {
                    continue;
                }
                let (name, detail, color) = directive_text(&t, directive);
                draw_directive_card(
                    card,
                    name,
                    detail,
                    color,
                    &t.install,
                    &mut chosen,
                    directive,
                );
            }
            self.depth_scroll.draw_scrollbar_with(
                list_view,
                content_h,
                Color::new(0.02, 0.03, 0.04, 0.42),
                Color::new(0.28, 0.56, 0.68, 0.62),
                dark::ACCENT,
            );
        } else {
            let card_w = (panel_w - 56.0 - gap * 2.0) / 3.0;
            for (idx, directive) in directives.into_iter().enumerate() {
                let x = panel_x + 28.0 + idx as f32 * (card_w + gap);
                let card = Rect::new(x, card_y, card_w, 164.0);
                let (name, detail, color) = directive_text(&t, directive);
                draw_directive_card(
                    card,
                    name,
                    detail,
                    color,
                    &t.install,
                    &mut chosen,
                    directive,
                );
            }
        }

        if let Some(directive) = chosen {
            self.choose_depth_directive(directive);
        }
    }
}

fn directive_text<'a>(
    t: &'a crate::data::strings::DepthDirectiveStrings,
    directive: DepthDirective,
) -> (&'a str, &'a str, Color) {
    match directive {
        DepthDirective::ReinforcedRoutes => (
            &t.reinforced_routes,
            &t.reinforced_routes_detail,
            Color::new(0.25, 0.65, 0.92, 1.0),
        ),
        DepthDirective::SalvageLifts => (
            &t.salvage_lifts,
            &t.salvage_lifts_detail,
            Color::new(0.84, 0.63, 0.22, 1.0),
        ),
        DepthDirective::EvacuationRelay => (
            &t.evacuation_relay,
            &t.evacuation_relay_detail,
            Color::new(0.28, 0.86, 0.65, 1.0),
        ),
    }
}

fn draw_directive_card(
    card: Rect,
    name: &str,
    detail: &str,
    color: Color,
    install: &str,
    chosen: &mut Option<DepthDirective>,
    directive: DepthDirective,
) {
    ui::draw_console_panel(card, Color::new(color.r, color.g, color.b, 0.28));
    draw_ui_text(name, card.x + 12.0, card.y + 25.0, 13.0, color);
    for (line_index, line) in wrap_text(detail, card.w - 24.0, 11.0)
        .into_iter()
        .take(3)
        .enumerate()
    {
        ui::draw_bounded_text(
            &line,
            card.x + 12.0,
            card.y + 52.0 + line_index as f32 * 13.0,
            card.w - 24.0,
            11.0,
            dark::TEXT,
        );
    }
    let button_y = if card.h > 140.0 {
        card.y + 116.0
    } else {
        card.y + card.h - 52.0
    };
    if ui::draw_console_button(
        card.x + 12.0,
        button_y,
        card.w - 24.0,
        44.0,
        install,
        ConsoleButtonState::Recommended,
    ) {
        *chosen = Some(directive);
    }
}
