[$code-quality-qa](C:\Users\Esther\.codex\skills\code-quality-qa\SKILL.md) now
review the code quality of phase {phase.id}: {phase.title}.

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

Review only. Do not fix yet.

This is an enforced structural gate. Phase completion is not honest until this
turn passes. Concrete violations of `composition_laws.md`,
`domain_structure_laws.md`, file-size discipline, directory topology, ownership
boundaries, or public-surface shape must fail this turn and send the phase back
to repair. Do not treat code-quality QA as advisory commentary.

Warn yourself gravely before reviewing: working behavior and semantic authority
checks do not redeem bad topology. A phase that leaves proof flows in bags is
not complete. A phase that hides transition grammar behind authoritative nouns
is not aerospace-grade. A phase that makes the next correct edit harder than
the convenient edit has failed the Worth standard.

Look aggressively for:

- composition-law violations
- domain-structure-law violations
- directories with more than 10 files without an honest, documented structural
  subdivision reason
- flat root directories that encode lifecycle, authority, or truth-source axes
  only through filename prefixes
- files or modules that are too broad
- facade-shaped modules that implement rather than aggregate
- `mod.rs` files or public facade files that contain business logic,
  authority decisions, proof transitions, classification, verification, receipt
  construction, or mutation logic instead of only wiring/aggregation
- hidden helper buckets
- missed abstractions where evidence collection, classification, verification,
  transition, receipt construction, projection, diagnostics, and counter
  publication are collapsed into one function
- inline domain policy, inline decision tables, inline proof predicates, and
  branch-specific counter construction that should be named transition cases
- public export dumps that teach an ontology of nouns instead of the lifecycle
  order and next valid capability
- production authority and test/certification authority living in the same
  structural space
- file length and directory shape problems
- weak ownership boundaries
- places where the implementation works but teaches the wrong architecture

This turn reopens the phase loop when concrete structural defects exist.
Structural defects are phase defects. Do not pass the phase while recording
known topology, abstraction, facade, `mod.rs`, file-size, directory-size, or
ownership-boundary violations as "remaining" work.

If you find concrete issues, report them in chat with file/line references and
finish with:

`RUNNER_EVENT: {"event_type":"code_quality_review_failed","payload":{"notes":{"findings":["..."]}}}`

Only pass when the phase's touched structure satisfies the Worth composition
and domain-structure laws and no concrete structural-law findings remain.

Re-run the acceptance checks and record concise command evidence in
`payload.notes.verification`. Then finish with:

`RUNNER_EVENT: {"event_type":"code_quality_review_passed","payload":{"notes":{"done":["..."],"remaining":["..."],"verification":["..."]}}}`

{contract}
