# Domain Capability Contributions Hub

## What This Page Is

A **navigation hub** for contribution lanes—not a full lifecycle feature doc. Each lane has its own page; use [contribution composed orchestration](../contribution-composed-orchestration.md) when multiple lanes compose in one declaration flow.

Contributions declare posture (admission, support, workflow, continuity, aftermath, explanation, invariants) **without** replacing Query admission or support matrices. Not every lane is “fully closed” in certification—check per-lane support reports.

## Contribution Lanes

| Lane | Doc |
|------|-----|
| Admission (advisory / violation) | [advisory-and-violation-contributions.md](../admission/advisory-and-violation-contributions.md) |
| Admission targets | [declaration-vs-admitted-plan-targets.md](../admission/declaration-vs-admitted-plan-targets.md) |
| Support (declaration-scoped) | [declaration-scoped-support-and-traceability.md](../support/declaration-scoped-support-and-traceability.md) |
| Support (lower-runtime traceability) | [lower-runtime-support-and-boundary-traceability.md](../support/lower-runtime-support-and-traceability.md) |
| Support (admission-local reports) | [admission-local-support-reports.md](../support/admission-local-support-reports.md) |
| Workflow | [workflow/README.md](../workflow/README.md) |
| Continuity | [continuity-contributions-and-authoritative-successors.md](../continuity/continuity-contributions-and-authoritative-successors.md) |
| Continuity vs correspondence | [continuity-vs-correspondence.md](../continuity/continuity-vs-correspondence.md) |
| Aftermath | [aftermath-review-support-eligibility-and-materialization.md](../aftermath/aftermath-review-support-eligibility-and-materialization.md) |
| Explanation | [lower-runtime-explanation-contributions.md](../explanation/lower-runtime-explanation-contributions.md) |
| Explanation gaps | [cross-runtime-fallback-vs-store-backed-replay-gap.md](../explanation/cross-runtime-fallback-vs-store-backed-replay-gap.md) |
| Invariants (contribution posture) | [invariant-and-capability-contributions.md](../invariants/invariant-and-capability-contributions.md) |
| Invariants (registration) | [registering-domain-invariants-through-query.md](../invariants/registering-domain-invariants-through-query.md) |

## Composed Entry

- [Contribution composed orchestration](../contribution-composed-orchestration.md) — multi-lane materialization and orchestration inventory neighbors
- [Grouped contributions](../grouped-contributions.md) — neighborhood-scoped contribution shapes
- [Public doc coverage](../public-doc-coverage.md) — which orchestration rows require docs

## Quick Rules

- **Admission contributions** annotate legality and violations—they do not bypass `admit_*` APIs.
- **Support contributions** attach traceability to declarations; read `domain_capabilities/support/reports.rs` patterns in code when debugging posture rows.
- **Explanation contributions** ≠ [cross-runtime causal inspection](../../capabilities/cross-runtime-causal-inspection.md)—runtime causal lane is separate.
- **Invariant contributions** ≠ invariant **registration**—registration doc owns catalog/builder path.

## Related Docs

- [Domain capabilities README](../README.md)
- [Choosing the right surface](../choosing/README.md)
- [Contribution composed orchestration](../contribution-composed-orchestration.md)
