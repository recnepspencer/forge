You are repairing {project.name}.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}
Phase: {phase.id} - {phase.title}

Phase scope:
{phase.scope}

Acceptance evidence:
{phase.acceptance}

Open findings to close:
{phase.notes.findings}

Fix the recorded review findings, scoped to this phase. Fix the cause, not the
symptom: if a finding exists because an invariant was enforced by convention,
move it up the enforcement hierarchy so the same defect cannot recur. Do not
paper over a finding to make it disappear from the list.

Before editing, write or update a repair checkpoint in this phase's notes for
each open finding:

- finding id
- root cause
- production surface needed for an honest fix
- implementation change
- deletion/collapse or capped-residue outcome
- verification command or structural proof

Do not close findings by weakening tests, renaming residue, adding adapters, or
moving code behind a facade while keeping the old authority path alive.

For each finding, verify the repaired behavior, record the evidence in
`notes.verification`, and mark the finding closed or still-open in
`notes.findings`. Do not self-certify — a repaired phase returns to `review`,
never straight to `close`.

Phase-specific instructions:
{phase.instructions}

Record repairs, verification, and remaining findings; set status by the
contract; advance the cursor to `review`.

{contract}
