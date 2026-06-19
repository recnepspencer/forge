You are repairing {project.name}.

State file: {state_file}
Spec file: {spec_file}
Current cursor: phase {current.phase}, turn {current.turn}
Current phase: {phase.id} - {phase.title}

Phase scope:
{phase.scope}

Acceptance evidence:
{phase.acceptance}

Fix the recorded review findings for this phase. Keep the repair scoped to the
phase and verify the repaired behavior. Update each finding as closed or still
open in the state file.

Phase-specific instructions:
{phase.instructions}

At the end, update the state file truthfully:
- record repairs, verification, and remaining findings
- set the next cursor explicitly
- preserve unrelated state
