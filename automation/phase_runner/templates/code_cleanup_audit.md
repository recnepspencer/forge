Review phase {phase.id}: {phase.title} as an architectural cleanup phase.

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

Read the milestone spec and relevant code, then identify the concrete
structural opportunities for this phase.

Look for places where the code can become more auditable by making these
qualities explicit:

- lifecycle-shaped directories
- narrow public facades
- localized authority
- named proof transitions
- clear production vs certification/test authority
- named classifiers or decision tables
- receipt and counter construction tied to verified outcomes
- helper placement that reflects the responsibility it serves
- overloaded functions decomposed into named semantic steps, especially where a
  function mixes evidence collection, classification, verification, mutation,
  receipt construction, counter updates, and denial/result assembly

Return in chat:

1. The phase cleanup boundary.
2. The main surfaces involved.
3. The architectural improvement each surface needs.
4. The evidence that would prove the cleanup is complete.
5. The behavior and public compatibility that should be preserved.

Phase-specific instructions:
{phase.instructions}

Finish with:

`RUNNER_EVENT: {"event_type":"boundary_review_completed","payload":{"notes":{"plan":["cleanup audit completed"]}}}`

{contract}

