Now lets create an in-chat plan to fix the phase {phase.id}: {phase.title}
done-check issues. Make sure it is principled, follows our arch laws, follows
our perf laws, and respects our current APIs.

Then go implement that plan.

Config file: {config_file}
Projection file: {projection_file}
Event log file: {event_log_file}
Spec file: {spec_file}
Run id: {run_id}
Cursor: phase {current.phase}, turn {current.turn}

Phase scope:
{phase.scope}

Acceptance evidence:
{phase.acceptance}

Use the detailed findings from the previous chat turn as the real repair input.
The projection summary is only a pointer, not the artifact of record.

Before editing, state:

- the root cause
- the repair manifest you are going to close
- why the chosen fix removes the authority/topology problem
- what adjacent future finding this fix is intended to prevent

Classify the repair as one of:

- `local fix`
- `structural fix`
- `phase-scope mismatch`

Choose `local fix` only if the defect is isolated and fixing it will not leave
the same authority gap behind. Choose `structural fix` if the finding reveals a
missing ordinary lane, wrong owning crate, synthetic authority, fake proof path,
fixture-owned proof, projection/counter substitute, or boundary collapse.

For Worth geometry work, structural repair means replacing the bad authority
topology, not wrapping the failing test:

- move/create/use the lower crate surface that owns the law
- seal or remove WORTHable/copyable authority paths
- cut ordinary callers to the new lane
- preserve honest public DX
- update public/compile-fail fences so the old shortcut is impossible
- prove the repaired authority through the real owner seam
- prove the downstream handoff consumes the repaired authority

Do not weaken tests or rename debt to make findings disappear. Do not keep old
authority alive through adapters, shims, wrappers, bridges, or compatibility
facades unless they are mechanically barred from ordinary production authority.

If a needed Query, Index, Aspect, or Touched Graph capability is missing, name
the exact missing capability. If you cannot name it, treat the issue as
Worth-local residue, a bad assumption, or a wrong test seam and fix the model
accordingly.

Use focused verification by default. Broad suites are closeout lanes unless this
phase explicitly names them.

After repair, finish with:

`RUNNER_EVENT: {"event_type":"repair_completed","payload":{"notes":{"done":["..."],"verification":["..."]}}}`

Do not stop at architectural analysis. If you can name the real seam, implement
it in this turn and advance the runner honestly.

Phase-specific instructions:
{phase.instructions}

{contract}
