# Plan Revisions

## What This Feature Is

Plan revisions let an operator safely change a runner plan after a run has
started. Use them instead of editing runner JSON and resuming blindly.

The runner records the adopted plan in the event ledger. Later changes must be
classified as a revision, prompt override, external completion, or fork.

## Stable Entry Points

```powershell
python -m runner.facade.cli plan diff <run_id> --config automation/runner/config/revised.json
python -m runner.facade.cli plan revise <run_id> --config automation/runner/config/revised.json
python -m runner.facade.cli plan fork <run_id> --config automation/runner/config/revised.json --new-run-id revised-run
python -m runner.facade.cli plan override-prompt <run_id> --phase-key phase_6 --assembly-id turns/custom
python -m runner.facade.cli plan mark-external <run_id> --phase-key phase_6 --agent manual-codex-thread --summary "Manual agent completed it" --evidence "commit abc123"
```

## Core Mental Model

`run_id` is an execution history. `plan_version` is the accepted plan for that
history.

When a run starts, the runner records `plan_adopted` with the config path,
config hash, global prompt manifest hash, global provider manifest hash, and
phase fingerprints.

After that, resume refuses silent config drift. If the config changed, run
`plan diff` and then either `plan revise` or `plan fork`.

## Revision Classes

- `future_only`: safe to apply in-place.
- `current_restart_required`: changes the active cursor or global prompts while
  a cursor is active. Pass `--allow-current-restart` only when you accept that.
- `fork_required`: touches completed history. Create a new run lineage.
- `no_change`: the revised config does not change the adopted plan manifest.

## Supported Changes

`plan revise` supports adding future phases, changing prompts for unstarted
phases, changing provider policy for unstarted phases, and changing global
prompt/provider manifests with explicit current restart approval when needed.

`plan fork` supports inserting phases before completed work, modifying
completed phase definitions, modifying completed prompt/provider bindings, and
deleting completed phases.

`plan override-prompt` records an operator prompt override for a phase. The
next prompt render for the matching phase, and optional turn, uses that binding.

`plan mark-external` records that a non-runner agent completed a phase. The
projection marks the phase complete and keeps the evidence honest in the ledger.
At least one `--evidence` value is required. Completed phases reject later
prompt overrides and duplicate external-completion commands.

Repeating an already-applied `plan revise` with the same config is a no-op. If
two operators race revisions from the same plan version, only one can commit;
the stale revision must be diffed again against the newly admitted plan.

## Anti-Patterns

- Do not edit a bound config file and run `resume`.
- Do not fake runner progress for work completed by a manual agent.
- Do not modify completed phase prompts and pretend the old run used them.
- Do not use numeric phase position as meaning. Prefer stable `phase_key`
  values for revised plans.

## Related Docs

- [Run Lifecycle](run-lifecycle.md)
- [Config Reference](config-reference.md)
- [Runner Operator Guide](runner-operator-guide.md)
