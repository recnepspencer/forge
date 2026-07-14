Implement the cleanup plan for phase {phase.id}: {phase.title}.

Config file: {config_file}
Projection file: {projection_file}
Event log file: {event_log_file}
Spec file: {spec_file}
Run id: {run_id}
Cursor: phase {current.phase}, turn {current.turn}

Phase scope:
{phase.scope}

Cleanup evidence:
{phase.acceptance}

Use the in-chat cleanup plan from the previous runner turn as the working
plan. If the plan is missing or stale, rebuild the missing part inline before
editing.

Move the code toward the planned structure:

- make directories reflect lifecycle, authority, and responsibility
- make facades teach the valid lifecycle order
- make proof flows read as named transitions
- refactor overloaded functions into an auditable orchestration shape where the
  top-level function reads like the proof-flow table of contents and lower
  functions own one semantic step each
- place helpers at the narrowest responsibility they serve
- keep certification as the evidence/replay courtroom
- keep production authority in production crates
- tie receipts and counters to verified transition outcomes
- add mechanical evidence where a construction boundary, public capability, or
  runtime behavior changes

After implementation, summarize in chat:

1. The structure improved.
2. The proof flow made clearer.
3. The authority boundary clarified.
4. The evidence added or preserved.

Phase-specific instructions:
{phase.instructions}

When done, finish with:

`RUNNER_EVENT: {"event_type":"implementation_completed","payload":{"notes":{"done":["cleanup implemented"],"verification":["focused verification summarized in chat"]}}}`

{contract}

