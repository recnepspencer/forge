Create a repair plan for Phase {phase.id}: {phase.title}. Do not edit yet.

Spec: {spec_file}
Scope: {phase.scope}
Acceptance: {phase.acceptance}
Review finding: {phase.notes.findings}

Treat the review finding as a hypothesis, not a conclusion. Read the relevant
code and use repository search to determine whether it is local or structural
before planning. Post the plan in chat, not a file. It must cover:

1. Finding, relevant sources, and constraints.
2. Confirmed root/adversarial constraint and canonical owning artifact, or why
   the finding is genuinely local.
3. Intended Rust/DX boundary and required module/directory changes.
4. Every constructor, export, caller, test seam, duplicate state, and ordinary
   production path in the authority family.
5. Complete cutover steps: each edit, removed authority, proof, and blockers.

Treat existing substrate as part of the plan. If the phase needs a graph,
planning, receipt, Query, host, or evidence capability that is absent or
broken, plan the principled repair at its owning seam. Do not dismiss it as
out-of-scope or create a local proxy. Keep genuinely later-phase behavior as a
narrow typed handoff.
End with exactly:
`RUNNER_EVENT: {"event_type":"repair_plan_posted","payload":{"notes":{"plan":["..."]}}}`
