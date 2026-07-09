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
runtime boundary the phase must preserve and the weaker paths it must replace.

Review the milestone spec, this phase's scope paths, the relevant public APIs,
and the Worth UI architecture docs. Then post a concise boundary brief covering:

1. What semantic truth enters this phase.
   Name the exact incoming authority lanes, such as declaration artifacts,
   graph truth, Query projection facts, measurement evidence, mounted receipts,
   host observations, or diagnostics evidence.

2. What this phase is allowed to own.
   Name the exact artifact, witness, contract, receipt, denial, index, or
   runtime lane this phase may mint, admit, preserve, inspect, or deny.

3. What this phase must not own.
   Name which adjacent crates or lanes still own the law. In particular:
   - Query owns domain/query/runtime truth
   - host adapters own native mechanics only
   - inspection and certification prove runtime law but do not define it
   - renderer-local helpers, widgets, or tests must not become semantic owners

4. Which weaker representations must become insufficient after this phase.
   Examples include:
   - tree position
   - strings or labels standing in for identity
   - copied topology facts
   - copied receipts
   - helper-local digests
   - renderer-local booleans
   - synthetic proofs
   - fixture-only authority
   - host-local layout/state meaning
   - terminal projections re-entering runtime authority

5. Which old or adjacent paths must be cut over.
   Do not just name the new lane. Name the displaced ordinary paths, helper
   seams, wrappers, or compatibility surfaces that must stop carrying ordinary
   production meaning after this phase.

6. Which downstream handoff or next milestone must consume the new authority.
   Name the next owner, witness, adapter, inspection surface, certification
   path, or runtime lane that has to consume this phase honestly.

7. The common failure modes worth guarding against in this phase.
   Use judgment, but especially watch for:
   - WORTHable authority
   - mixed cutover
   - proxy-derived identity
   - weak equivalence contracts
   - mutable-field leakage
   - renderer/host ownership drift
   - synthetic hostile proof instead of real production derivation
   - inspection/certification owning runtime law
   - visible proof only working in bespoke fixtures

If the phase is small and none of these risks materially apply, say why briefly
and name the smaller boundary that planning must still preserve.

Finish with:

`RUNNER_EVENT: {"event_type":"boundary_review_completed","payload":{"notes":{"plan":["..."]}}}`

{contract}
