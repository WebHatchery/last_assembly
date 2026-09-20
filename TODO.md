# Outstanding work — Last Assembly

## UI_STYLE review — 2026-09-20

Audit and planning only; no UI changes implemented. Tasks are ordered by player
impact and dependencies. Previous camera/touch and responsive-panel tasks are
merged into UI-02–UI-05. All other existing work is retained; the original file
contained no checked completion history.

Evidence: read AGENTS.md, UI_STYLE.md, CODE_STANDARDS.md,
GAME_DEVELOPMENT_GUIDE.md, README.md, actual UI/render/input/camera code,
strings and capture setup. No separate GDD or PROJECT_AGENTS.md was found.
Visually inspected historical 1280x720 captures in docs/verification/:
ui_menu.png, ui_intro.png, ui_gameplay.png, ui_coach.png, ui_build_panel.png,
ui_systems_panel.png, ui_beacon_panel.png, ui_workforce.png,
ui_specialization.png, ui_vault.png, ui_report.png, ui_results.png and ui_victory.png.
These are not fresh runtime tests. Native/browser interaction, minimum-size
rendering and touch flows remain unverified. Available awakening, corruption
and sector captures still need visual review.

Preserve the useful existing design: mutually exclusive on-demand docks,
selection-only context panels, completing/skippable onboarding, temporary
notifications, world awakening feedback and the salvage report's single
return-to-work action. No unmodified template demo screen was identified;
remaining dashboard treatment is in this game's own consoles.

Verification baseline for every task: normal 1280x720 canvas; proposed compact
targets 800x600 and 640x360 landscape, plus 390x844 portrait/recovery behavior.
Compact targets are audit targets, not a claim of current support. UI-01 must
declare the supported minimum/orientation policy; test that minimum too if
different. Check actual native and embedded browser canvas dimensions, resize,
and browser/display scaling. Save evidence directly in docs/verification/,
replacing equivalent captures. After meaningful implementation, run
`.\publish.ps1` without parameters and report the result.

### Verified findings — source inspection and historical captures

- [ ] **UI-01 — Recompose gameplay around the current decision.**
  **Screen/files:** preparation, assault, takeover and selected-object views;
  README.md screen briefs; src/state/gameplay/render_hud.rs (`draw_hud`,
  `draw_compact_status`, `draw_compact_metrics`, `draw_compact_next`,
  `draw_edge_controls`), ui.rs, render.rs; assets/strings.json.
  **Problem:** ui_gameplay.png shows five bordered HUD blocks, six tiny resource
  columns, two bright edge rails and repeated WARM SIGNAL text. Beacon repeats
  phase/evacuation data when opened. Beacon and Pause share one right rail and
  the same visual treatment, mixing a tactical decision with navigation.
  **Change:** record UI_STYLE §1 briefs and supported viewport sizes in README.
  Compose a dominant battlefield, one supporting status/action area and quiet
  utilities; separate Pause spatially and visually from Beacon. Give phase one
  persistent home, disclose survival details on demand while surfacing actual
  shortages immediately, and remove redundant headings/borders before resizing.
  Subordinate FOCUS advice when the selected object's real action is available.
  Emphasize repair/build in preparation and relevant recall/shutdown/takeover
  decisions later. Preserve scrap, power constraints, wave pressure and costs.
  **Accept:** no more than 2–3 regions demand strong attention in normal play;
  the current decision is clear at a glance, utilities do not compete with it,
  and equally valid strategic choices retain equal emphasis.
  **Verify:** baseline sizes, calm/wave/shortage/takeover states, docks closed and
  one inspector open; tap tactical and utility paths independently. Establish
  this composition contract before UI-02–UI-08.

- [ ] **UI-02 — Frame the active factory and support touch camera navigation.**
  **Screen/files:** battlefield and revealed depths;
  src/state/gameplay/lifecycle.rs (`GameplayState::new`), camera.rs
  (`sync_camera_bounds`), update.rs (`handle_camera_input`), render_map.rs,
  render_map/circuit_board.rs, ui.rs; README.md and game_page.json.
  **Problem:** ui_gameplay.png devotes much of the right/bottom area to decorative
  floor while pads/towers stay small. Initialization fits hardcoded 900x560
  dimensions with large padding and a center offset, independent of the actual
  play rectangle. Camera dragging uses middle mouse; updates pass `false` for
  UI capture before checking modals.
  **Change:** derive framing from revealed actionable routes/buildings and the
  unobscured viewport; handle first reveal and resize without continuously
  overriding player framing. Add toolkit-backed touch pan/zoom plus visible
  zoom/reset-view controls. Gate camera input behind UI/modal ownership and
  teach the exact gestures. Reduce decorative floor contrast if needed after
  useful framing, rather than expanding empty map margins.
  **Accept:** first-use targets and the defended route are identifiable and easy
  to select; an inspector leaves its target visible. Every new wing and a useful
  reset view are reachable without keyboard or middle mouse.
  **Verify:** baseline sizes, first/all four depths, edge selections, zoom limits,
  resize and scaling; drag/pinch map versus each modal. No camera motion through
  UI and no tower purchase from dragging. Depends on UI-01; coordinate UI-05.

- [ ] **UI-03 — Reflow HUD and consoles without dropping content.**
  **Screen/files:** HUD, Build, Systems, selected pad/tower/building/factory;
  src/state/gameplay/render_hud.rs, ui.rs panel rectangles, ui/panels.rs
  (`draw_sector_panel`), render_panels/context_*.rs; src/ui/build_panel.rs, text.rs.
  **Problem:** HUD minimum widths total 752px with gaps/margins, pushing Beacon
  off-canvas below that width; labels use 8px text. NEXT STEP disappears below
  1180px. Systems breaks out on overflow without scrolling; Build rows ignore
  available height and contexts retain fixed widths. These compact-size issues
  are code-confirmed constraints, not observed compact renders. The normal-size
  Systems capture also contains one row in a tall, largely empty panel.
  **Change:** share responsive geometry between drawing and picking; reflow
  essential status and preserve access to the current objective. Scroll lists
  or use a dedicated compact view; keep contextual actions reachable, wrap
  consequential text, and remove unused console height and duplicate ONLINE
  facts. Do not resolve overflow by shrinking text. Keep costs, shortages,
  power prerequisites and breach warnings beside the corresponding action.
  **Accept:** every unlocked item/action is reachable, long labels/large values
  fit, and normal play still gives the world most space. Deliberate comparison
  views may take focus when needed.
  **Verify:** baseline plus 1179/1180px widths, all systems/towers unlocked,
  large resource values and every context type; touch-scroll without map clicks.
  Depends on UI-01; use its hierarchy before tuning dimensions.

- [ ] **UI-04 — Keep entry, modal choices and recovery usable on short screens.**
  **Screen/files:** menu, intro, pause/settings, workforce, depth directive,
  salvage and results; src/state/menu.rs (`MenuState::draw`), src/ui/settings.rs,
  src/state/gameplay/ui/overlays.rs, render_depth.rs, src/state/results.rs.
  **Problem:** menu placement clamps between 280 and `h-297`; below 577px height
  those bounds reverse, a source-confirmed panic path. Other overlays shrink
  only the outer panel while retaining fixed rows/text offsets. Directives
  always use three columns; results place Return after uncontained report text.
  Compact visual severity still needs runtime verification.
  **Change:** use safe content-aware bounds, wrap prose, stack narrow directive
  choices and scroll content while keeping primary/dismissal controls visible.
  Retain policy tradeoffs, irreversible-choice warnings and report escalation.
  Omit or replace browser Exit's existing no-op with meaningful recovery.
  **Accept:** launching, starting/continuing, every choice, settings, report
  dismissal and loss/victory recovery work with taps alone at supported sizes;
  no panic or unreachable button.
  **Verify:** baseline sizes, saved/no-save menu, each modal, long choice names,
  dense reports and live resize; complete menu-to-results recovery in browser.
  Depends on UI-01 and UI-03 layout patterns.

- [ ] **UI-05 — Complete touch inspection, cancellation and pointer ownership.**
  **Screen/files:** gameplay controls, coach and inspectors;
  src/state/gameplay/update.rs (`handle_input`), ui/actions.rs
  (`handle_map_click`, `click_is_over_ui`), render_hud.rs (`draw_button_hint`),
  render_panels.rs, render_hud/beacon_panel.rs, ui/overlays.rs; src/ui/widgets.rs.
  **Problem:** power/alert details rely on hover; cancel/clear uses Esc or right
  click and speed toggling uses Space. Beacon actions are 24px high, priority
  28px, Skip 22px. Hit testing reserves the bottom context rectangle even when
  no selection renders it, and omits the coach rectangle. These are source
  findings; actual touch failure paths were not exercised.
  **Change:** add visible Cancel placement, inspector Close, speed and tappable
  details with dismissal. Enlarge targets, aiming for at least 44x44 logical px
  and checking actual browser sizing. Capture input using surfaces actually
  rendered, including hints/coach/modals; remove invisible map dead zones.
  Use toolkit release semantics with the existing standards task below, rather
  than adding another local button system.
  **Accept:** no required action/fact depends on hover or keyboard; cancelling
  spends nothing; skipping/closing cannot build or select behind UI; visible
  pads under an absent context panel can be selected.
  **Verify:** baseline sizes; tap inspect/dismiss, arm/cancel/build, priority,
  speed, recall/shutdown, Skip over a map object, drag off a button to cancel,
  and picking after resize/zoom. Depends on UI-01–UI-03.

- [ ] **UI-06 — Move repeated instruction and distant unlocks into reopenable help.**
  **Screen/files:** Build and onboarding; src/ui/build_panel.rs,
  src/state/gameplay/ui/panels.rs, ui/overlays.rs (`draw_coach`, `finish_coach`),
  src/ui/settings.rs, assets/strings.json, README.md controls.
  **Problem:** Build always shows its subtitle, six-entry map key and all locked
  towers. The capture shows those taking much of the console. Coach already
  completes/skips but spans the play area. Settings replay changes tutorial_seen
  without reactivating the current state's coach.
  **Change:** add visible Help/map-key disclosure, prioritize currently useful
  towers and disclose distant locked families with prerequisites on demand.
  Keep the next meaningful unlock discoverable. Reopen instructions in the
  current run, place a compact current step near its action, and explicitly
  teach pan, zoom, Cancel and PAUSE using their visible names.
  **Accept:** normal returning play shows choices, not repeated tutorial prose;
  first-time players can understand symbols/unlocks and reopen help without
  restarting. Prices, shortages and immediate prerequisites stay visible.
  **Verify:** baseline sizes; complete/skip/reopen in one run, tap map-key
  open/close, unlock a tower and resume a saved campaign.
  Depends on UI-01/UI-03/UI-05.

- [ ] **UI-07 — Expose complete irreversible-choice consequences and urgent warnings.**
  **Screen/files:** specialization, directives, takeover and shortage states;
  src/state/gameplay/render_panels/context_tower.rs (`draw_specialization_choices`),
  context_factory.rs, render_depth.rs, render_hud.rs (`draw_compact_threat`),
  ui_advice/alerts.rs; src/ui/text.rs.
  **Problem:** specialization descriptions truncate to 152px at 10px font;
  directive details are single bounded lines. The top alert uses a 5px square
  and hover-only detail. The vault capture visibly truncates NEXT STEP risk.
  Decision-relevant information can disappear before an irreversible action.
  **Change:** provide a readable expanded comparison with complete consequences,
  price, prerequisites and permanence before commitment. Put urgent textual
  state beside the affected action with tap-to-expand details. Remove redundant
  subtitles to fund space, and keep vault lock-in/risk at its start action.
  **Accept:** no consequential text requires guessing ellipses or hovering;
  danger uses words as well as color, disabled choices explain why, and equally
  valid branches/directives keep equal emphasis.
  **Verify:** baseline sizes, both branches of each tower, all directives,
  insufficient scrap/research, power deficit and takeover lock-in. Use a genuine
  max-level capture; existing specialization image is not proof of this state.
  Depends on UI-01/UI-03/UI-05 and the fixture correction in UI-09.

### Further inspection — not visually verified defects

- [ ] **UI-08 — Exercise state/event feedback in combat and fix reproduced gaps.**
  **Screen/files:** waves, awakening, recall and shortages;
  src/state/gameplay/render.rs draw order, render_hud.rs (`draw_notifications`,
  `draw_wave_start_flash`), presentation.rs, update.rs and ui/overlays.rs.
  **Known basis/uncertainty:** notifications already expire and awakening has
  world feedback; there is no verified permanent event-log defect. Notifications
  occupy the upper-left console area and draw before docks/coach, which may
  conceal feedback. Calm captures do not prove combat readability or recovery
  after missing an event.
  **Inspect/change:** reproduce simultaneous wave start, repair/unlock, recall,
  losses and shortage. If covered, reserve an unobstructed temporary feedback
  area, coalesce routine events and prioritize actionable failures. Keep
  persistent UI for current state, with a retrievable report or affected-object
  state preserving critical outcomes after effects expire; no permanent log panel.
  **Accept:** changes remain understandable afterward, at most one transient
  emphasis competes with the decision and controls stay usable. Record a
  no-change conclusion if existing behavior passes.
  **Verify:** baseline sizes, docks/coach open, active assault/high corruption,
  awakening, destruction and post-report play; wait for effects to expire.
  Review ui_awakening.png/ui_corruption.png/ui_sector.png plus fresh scenes.
  Depends on UI-01/UI-05.

- [ ] **UI-09 — Correct capture coverage and complete visual/interaction signoff.**
  **Screen/files:** full flow; src/game.rs (`begin_capture_scene`),
  scripts/capture_ui.ps1, src/main.rs, docs/verification/, README.md evidence.
  **Problem:** ui_specialization.png shows LVL 1 and UPGRADE 25, not branches.
  The seeder appends a max-level tower but selects index 0 despite starting
  towers already existing. Vault seeding unlocks factory sectors without
  reproducing full campaign progression. Current images establish neither
  small-screen nor real late-game/touch correctness.
  **Change:** select the intended appended tower and make seeds independent of
  user settings/save state; add reachable dense/urgent/directive/pause/settings
  scenes as needed. Verify real progression as well as seeded images: menu →
  intro/coach → repair/power/build → beacon/recall/shutdown → salvage → deeper
  directive → takeover → results/recovery.
  **Accept:** each image visibly proves its named state; every UI_STYLE §9 item
  has evidence or an explicit limitation. Filenames and compilation are not
  acceptance evidence by themselves.
  **Verify:** normal and declared minimum native/browser canvas, compact stress
  targets, embedded play/scaling, long labels/large values and UI-02–UI-08 touch
  paths. Replace equivalent captures directly in docs/verification/ and run
  `.\publish.ps1` after implementation. Correct specialization early for UI-07;
  complete final signoff after the implemented UI tasks.

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
