# Gate 8.4 Turn 3 — Boundary Review And Implementation Plan

## Stage 1: Boundary Brief

### Slice selected
Close the three honesty gaps named in the turn-3 audit: real R8.39 denial
scenarios (not enum-dedup theatre), no-write proof via undo *attempt* + graph
before/after, and RecordedInverse end-to-end that *uses* retained pre-image.
Do not open Gate 8.5.

### Semantic truth entering the slice
- Eight `WorthQueryUndoDenialKind` variants exist; irreversible cause
  classification keys off operation-slot + escaped-effect axes.
- Bank undo admission maps through `admit_*_recovery` → `admit_undo`. Expired /
  disposed handles currently surface as `Recovery(Expired|AlreadyTerminal)`,
  not as `Undo(Stale|AlreadyConsumed)` — so R8.39's public typed causes are
  unreachable on the production undo entry for those two rows.
- Conflicted is constructed only after admitted compensation enters ordinary
  reverse-journal progression (`map_ordinary_commit_conflict` /
  proposal denial).
- Irreversible denial tests in `phase8_undo_denials.rs` prove *installation*
  posture (R8.21 / Gate 8.1), not undo attempt + no write.
- `eight_undo_denial_kinds_are_distinguishable` asserts enum cardinality —
  compiler-guaranteed, non-evidence.
- Retention pipeline exists (`retain_attempt_preimage` → receipt), but
  `progression.rs` hardcodes `preimage_demand: None`, so Bank freezes never
  retain. RecordedInverse admission therefore always fails
  `RetainedPreImageRequired` on real receipts. Bank
  `progress_undo_commit_recovery` denies RecordedInverse with
  `CorrectionNotAdmitted` rather than progressing through ordinary commit.
- Freeze aftermath demands `"account-freeze-state"`; decision facts use field
  slot `"status"` — demand and observation cannot meet.

### What this slice owns
- R8.39: eight production undo scenarios, each typed cause at the public
  boundary, each with positive twin, each with before/after journal+activity
  equality (and admission-time denials proved never to open mutation).
- Map undo-path Recovery Expired → Undo Stale and AlreadyTerminal →
  AlreadyConsumed so the public undo boundary owns those causes.
- R8.2 consumption completion: attach installed RecordedInverse pre-image
  demand into attempt registration; align FreezeAccount demand to `"status"`;
  Bank RecordedInverse progression that *reads* retained pre-image to restore
  prior status through ordinary `compare_and_commit_application`.
- Divergence (T4b shape): same FreezeAccount RecordedInverse contract admits
  inverse undo and denies Compensation routing.
- Ledger R8.63 update for closed / remaining rows.

### Adjacent ownership that continues
- Recovery handle lifecycle, rail, money compensation courtroom (turn 2).
- Redo / lineage (8.5).
- Store-durable pre-image beyond the commit receipt carrier.

### Weaker representations that must become insufficient
- Enum-dedup as R8.39 coverage.
- Install-as-irreversible counted as undo denial + no-write.
- Kind-mapping unit tests alone for Stale / AlreadyConsumed / Conflicted.
- Graph-equality after write-then-rollback as no-write (admission denials must
  fail before mutation entry; conflicted progress must leave journals
  unchanged *and* not count as successful commit).
- Pre-image required by signature but never retained / never consumed into
  inverse derivation.

### Competing authorities / cutover
- Delete theatre denial tests; replace with production-path scenarios.
- Undo admission entry maps terminal/expiry recovery denials into Undo kinds.
- Effect program carries optional `InstalledPreImageDemand`; registration
  stops hardcoding `None`.
- Bank undo progression handles RecordedInverse via restore-from-preimage,
  not `CorrectionNotAdmitted`.

### Downstream handoff
- Courtroom modules observe typed `BankEstateProgressionDenial::Undo(kind)`
  and account/journal snapshots.
- Ledger rows R8.39 and R8.2 consumption move to PROVED when evidence holds.

### Dirty-edge failure modes
- Mapping all Recovery denials to Undo (would launder CurrentPolicyDenied).
- Matching demand slot by renaming facts instead of aligning demand to the
  real Status field.
- Parallel unfreeze mutator that bypasses compare_and_commit.
- Positive twin that shares the denial fixture's irreversible aftermath.

### Unresolved facts verified
- `Status` query field ref is `"status"` — demand must use that slot.
- `handle.ensure_live` returns AlreadyTerminal after dispose/consume;
  expiry evaluation + expire terminalizes the registry slot.
- Compensation conflict path already maps proposal/commit failure to
  `Undo(Conflicted)`.
- EffectProgram is the natural carrier for demand into
  `compare_and_commit_application_inner`.

---

## Stage 2: Implementation Plan

### Slice name
Gate 8.4 turn 3 — R8.39 real denials, no-write attempts, RecordedInverse use.

### Ordered steps
1. **Undo-path cause mapping** — in Bank `admit_undo_*`, map Recovery
   `Expired` → `Undo(Stale)` and `AlreadyTerminal` → `Undo(AlreadyConsumed)`.
2. **R8.39 courtroom** — rewrite `phase8_undo_denials.rs`: eight scenarios
   through production undo; before/after journals+activity; positive twins;
   delete enum-dedup and install-only tests.
3. **Pre-image demand alignment** — FreezeAccount demand/read coverage →
   `"status"`; update estate aftermath contract tests.
4. **Retention wire** — optional demand on EffectProgram; Bank freeze
   attaches installed inverse demand; progression passes it into
   registration.
5. **RecordedInverse progression** — Bank restores prior AccountStatus from
   retained pre-image through ordinary unfreeze/compare_and_commit; deny
   Compensation on the same contract (divergence twin).
6. **Verify** — standing set; ledger update; line caps.

### Module shape
```
estate_progression/recovery.rs          # undo-path Recovery→Undo mapping
estate_progression/undo.rs              # RecordedInverse restore progression
estate_progression/freeze_account.rs    # attach preimage demand
phase8_undo_denials.rs                  # eight real scenarios
phase8_undo_recorded_inverse.rs         # end-to-end + divergence
effect_program/model.rs + progression   # demand carrier
bank-domain estate/aftermath.rs         # status demand slot
```

### Out of scope
- Gate 8.5
- Non-freeze RecordedInverse operations (delegate/revoke/emergency)
- Store-durable pre-image
