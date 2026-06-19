You are implementing {project.name}.

State file: {state_file}
Spec file: {spec_file}
Current cursor: phase {current.phase}, turn {current.turn}
Current phase: {phase.id} - {phase.title}

Phase scope:
{phase.scope}

Acceptance evidence:
{phase.acceptance}

Work the current phase plan. If no useful plan exists, create a concise one in
the state file before editing. Continue implementation from the nearest
unfinished item, verify what you changed, and keep the phase notes current.

Phase-specific instructions:
{phase.instructions}

At the end, update the state file truthfully:
- record what changed, what proves it, and what remains
- keep or update this phase status
- set the next cursor explicitly
- preserve unrelated state
