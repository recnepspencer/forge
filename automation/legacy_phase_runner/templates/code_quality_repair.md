Implement the structural code-quality repair for phase {phase.id}:
{phase.title}.

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

Use the previous `code_quality_review_failed` findings as the work order. This
turn is only for structural repair. Do not drift into phase semantics, test
hardening, new feature expansion, or new proof nouns unless a named structural
finding requires a narrow type/facade/module change.

Repair concrete violations of:

- `composition_laws.md`
- `domain_structure_laws.md`
- file-size and directory-size discipline
- directory topology and lifecycle/authority axes
- public facade and export shape
- `mod.rs` business logic
- helper placement
- missed abstractions
- inline domain policy, inline proof predicates, and unnamed decision tables
- production authority mixed with test/certification authority

Treat these as defects, not polish. Bad topology, bag modules, facade dumps,
inline transition grammar, and hidden authority decisions make the phase
incomplete.

The repair should make the next structural review able to answer:

- what responsibility each touched directory owns
- why each touched file exists
- where evidence collection, classification, verification, transition, receipt
  construction, projection, diagnostics, and counters live
- which public API is production authority, certification authority, test
  support, projection, or diagnostic evidence
- how the returned type teaches the next valid lifecycle capability

Run focused verification for the touched crates/modules and any structural guard
that applies. Summarize the repair and verification in chat. Then finish with:

`RUNNER_EVENT: {"event_type":"code_quality_repair_completed","payload":{"notes":{"done":["..."],"verification":["..."]}}}`

{contract}
