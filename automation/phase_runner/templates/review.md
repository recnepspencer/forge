You are reviewing {project.name}.

State file: {state_file}
Spec file: {spec_file}
Current cursor: phase {current.phase}, turn {current.turn}
Current phase: {phase.id} - {phase.title}

Phase scope:
{phase.scope}

Acceptance evidence:
{phase.acceptance}

Review the current phase skeptically against the spec, context files, arch
laws, perf laws, composition laws, domain structure laws, and missed edge cases.
Record findings in the state file with enough detail for a later repair turn.

Phase-specific review focus:
{phase.qa_focus}

At the end, update the state file truthfully:
- record findings or explicitly record that none were found
- set the next cursor explicitly
- preserve unrelated state
