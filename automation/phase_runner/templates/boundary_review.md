Before planning phase {phase.id}: {phase.title}, run a boundary review.

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

Do not write the implementation plan yet. This turn exists to sharpen the
architectural boundary the plan must respect.

Review the milestone spec, this phase's instructions, this phase's scope paths,
and the relevant APIs. Then post a concise boundary brief covering:

1. What authority, evidence, state, geometry fact, touched-graph fact,
   query/index/aspect fact, or runtime witness enters this phase.
2. What this phase is allowed to publish, witness, certify, adapt, route, or deny.
3. Which crate owns the law, which crate only adapts or proves it, and which
   callers must consume the result.
4. Which weaker representations must be insufficient after this phase, such as
   copied ids, copied receipts, copied counters, equality-only digests, strings,
   JSON, serde declarations, projections, terminal artifacts, or fixtures.
5. Which old or adjacent paths must be cut over so the new boundary is not only
   an added wrapper.
6. Which downstream handoff or next milestone must consume the new authority.
7. The common failure modes worth keeping in mind during planning, using
   judgment rather than treating this as a fixed checklist. Examples include
   forgery, public raw constructors, certification-owned law, fixture-only
   authority, copied-counter paths, downstream consumers still accepting weaker
   witnesses, query/index/aspect facts copied into Worth-local second ontologies,
   projection data re-entering runtime authority, and mixed ordinary lanes.

If none of these risks materially apply, say why briefly and name the smaller
boundary the plan should preserve.

Finish with:

`RUNNER_EVENT: {"event_type":"boundary_review_completed","payload":{"notes":{"plan":["..."]}}}`

{contract}
