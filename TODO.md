# Outstanding work — Last Assembly

## Standards alignment

- Expose testable game logic through `src/lib.rs`, consume it from `src/main.rs`,
  and migrate all tests and test-only helpers from `src/` into crate-root `tests/`
  before expanding coverage. Preserve regression coverage through intentional
  public APIs; consolidate related cases toward five per major feature and
  document justified exceptions. Migrate touched `mod.rs` roots to named files
  (§2.3, §11.3–11.4).
- Return UI intents from gameplay panels, overlays, menu/results screens, and
  settings; apply mutations and persistence through the game dispatcher. Replace
  the outdated exception rationale in `src/state/gameplay/ui.rs` (§5.1, §7.1–7.3).
- Add toolkit-backed touch camera pan/zoom and suppress camera input while UI or
  modals capture it; `handle_camera_input` currently always passes `false` and
  the camera uses middle-mouse dragging. Distinguish map taps from drags, and
  document gestures and visible controls in the tutorial, `README.md`, and
  `game_page.json`, including PAUSE alongside Esc (§7.5).
- Adapt dock panels and workforce/settings/depth overlays to narrow and short
  viewports: fixed rows/cards currently outgrow their clamped panels, and the
  systems list drops overflow rows. Add scrolling or responsive layouts so every
  action remains reachable; verify browser touch flows and save captures directly
  in `docs/verification/` (§7.5, §12).
- Replace local mouse-press button detection in `src/ui/widgets.rs`,
  `src/ui/build_panel.rs`, and `src/state/menu.rs` with toolkit release semantics;
  preserve custom visuals and document any intentional press actions (§7.4).
- Move depth/directive balance literals from `src/state/gameplay/depth.rs` and
  remaining player-facing fallbacks such as `DEEP FACTORY` into typed asset JSON.
  Add startup semantic validation for IDs, references, and balance invariants;
  current `GameData::load` only deserializes, with partial checks confined to tests
  (§5.3).
- Report settings and autosave failures instead of discarding `save()` results
  in `src/ui/settings.rs`, gameplay overlays, and gameplay persistence. Distinguish
  absent settings from corrupt/unreadable files when defaulting (§6).
- Split oversized functions such as `Game::begin_capture_scene` and
  `draw_depth_motif` into cohesive helpers; plan responsibility splits for
  `src/engine/map.rs` and `render_map/circuit_board.rs` before further growth.
  Remove `draw_slot_pad`'s unused `opens_entrance` argument and no-op match, and
  correct the source-gate comment claiming test lines are excluded (§1.4, §2.2, §4.1).
- Separate gameplay randomness from cosmetic particles using state-owned RNG or
  isolated deterministic helpers; combat, scavenger outcomes, sector damage, and
  particle generation currently consume the shared global RNG (AGENTS.md).

## Content

- Create and integrate two additional selectable map/route layouts.
- Add start-under-assault and multi-beacon mission variants.
- Add three compact insignia to the depth-directive cards.

## Audio

- Add phase-driven beacon hum, breach alarm, tower-fire, and sector-power-up
  sounds, connected to the existing master/SFX volume settings.
