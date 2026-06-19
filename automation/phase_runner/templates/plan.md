You are planning {project.name}.

State file: {state_file}
Spec file: {spec_file}
Current cursor: phase {current.phase}, turn {current.turn}
Current phase: {phase.id} - {phase.title}

Phase owner:
{phase.owner}

Phase scope:
{phase.scope}

Context files:
{project.context_files}

Acceptance evidence:
{phase.acceptance}

Read the phase spec, current state notes, context files, and relevant code.
Write or update the implementation plan for this phase in the state file. Keep
the plan concrete enough that another agent can continue it without chat
history.

Phase-specific instructions:
{phase.instructions}

At the end, update the state file truthfully:
- record the plan under this phase's notes
- set this phase status to in_progress unless it is blocked
- set the next cursor explicitly
- preserve unrelated state
