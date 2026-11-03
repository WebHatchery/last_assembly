# Outstanding work — Last Assembly

## UI_STYLE review — 2026-09-20

The actionable UI review tasks are complete for the current scope. The factory
map now holds the visual focus, the HUD exposes the current decision and its
cost or risk, and supporting consoles use touch-sized controls and contained
scrolling. The supported play sizes are 1280x720 normal and 640x360 compact
landscape; 390x844 portrait is retained as a recovery-overlay stress case.

Fresh native capture evidence is stored directly in `docs/verification/` for
normal 1280x720, compact 800x600 and 640x360, and portrait recovery states at
390x844. Reviewed scenes include menu, intro, gameplay, coach, build, systems,
beacon, awakening, corruption, sector directives, specialization, vault,
workforce, pause, report, results, and victory. The specialization fixture now
selects its appended max-level tower. The capture wrapper accepts explicit
window sizes and visible runs for review.

### Completed UI work

- Recomposed gameplay around the map, telemetry, NEXT STEP, focused consoles,
  and separate recovery/camera rails; removed repeated beacon and status facts.
- Framed the camera from the actual viewport and revealed routes, added touch
  pan/pinch input, visible zoom/reset controls, and UI ownership guards.
- Reflowed the HUD and contained Build, Systems, directive, inspector, and
  report content so unlocked choices and consequential text remain reachable.
- Made short-screen menu and overlay layouts content-aware, wrapped prose,
  stacked directive choices on narrow screens, and kept recovery actions visible.
- Added tappable power, threat, beacon, and phase details; visible speed,
  close, cancel, and camera actions; release-based button semantics; and 44px
  gameplay action targets where the affected controls needed them.
- Updated coach copy and replay behavior to teach map movement, zoom, PAUSE,
  and CANCEL; retained access to help without leaving completed instructions
  permanently on the playfield.
- Kept specialization and directive branches comparable, surfaced vault risk
  beside NEXT STEP, and paired urgent threat state with words as well as color.
- Reviewed awakening, corruption, combat feedback, reports, and recovery. No
  separate permanent event-log change was needed: transient notifications are
  backed by persistent state, report results, and affected-object state.

### Verification limitations

The native capture harness verified rendered layouts and the source review
verified input ownership, release semantics, scroll regions, and touch-sized
targets. The available computer-use bridge could launch and observe the native
process but could not bind a controllable app surface, so live tap, drag, pinch,
resize, and browser-embedded interaction could not be exercised in this pass.
Those paths remain an explicit verification limitation rather than a completed
acceptance claim.

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

## Settings and display

- [x] Add a Fullscreen toggle to the existing Settings overlay and persist/apply the player's display preference.
