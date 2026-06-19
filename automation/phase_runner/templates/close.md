You are closing {project.name}.

State file: {state_file}
Spec file: {spec_file}
Current cursor: phase {current.phase}, turn {current.turn}
Current phase: {phase.id} - {phase.title}

Acceptance evidence:
{phase.acceptance}

Verify the phase closure evidence against the spec and state notes. Summarize
the final proof, remaining explicit residue or query gaps, and verification
commands. Then update the phase status honestly.

At the end, update the state file truthfully:
- set this phase status and qa_status
- record the closeout evidence in this phase's notes/history
- set the next cursor explicitly, or set current to null if all phases are done
- preserve unrelated state
