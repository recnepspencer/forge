# Invariant and Capability Contributions

## What This Feature Is

Invariant and capability contributions let domains attach **invariant/capability-gap posture** through the domain-capabilities proof integration lane (`WorthQueryInvariantCapabilityContribution*` payloads)â€”distinct from **registering** invariant catalogs via the ordinary runtime builder ([registering domain invariants](registering-domain-invariants-through-query.md)).

## Why You Use It

- declare capability gaps or invariant posture alongside other contribution lanes
- materialize trace and support reports typed on `WorthQueryInvariantCapabilityContributionPayload`
- keep registration authority on Query/relational builder paths without duplicating catalogs in contributions

## Core Mental Model

Two pathsâ€”do not merge them:

| Path | Purpose |
|------|---------|
| **Registration** | `register_invariant_catalog` / builder artifacts â†’ relational authority |
| **Contribution** | Authoring + materialize contribution payload â†’ support/trace reports |

```text
worth_query_domain(...).for_intent(...)
  â†’ invariant/capability contribution authoring
  â†’ materialize()
  â†’ WorthQueryInvariantCapabilityContributionSupportReport / trace artifact
```

Registration hands artifacts to `WorthQueryRuntime::builder()`; contributions record **posture** for declaration-scoped orchestration.

## Main Entry Points

- `domain_capabilities/proof_integration/artifacts.rs` â€” `WorthQueryInvariantCapabilityContributionPayload`
- `WorthQueryInvariantCapabilityContributionAuthoring` â€” e.g. `capability_gap(...)` (tests)
- `domain_capabilities/support/reports.rs` â€” `WorthQueryInvariantCapabilityContributionSupportReport`
- `domain_capabilities/trace/materializers.rs` â€” trace artifacts
- Tests: `canonical_runtime_invariant_registration_tests.rs`, `domain_capabilities/tests.rs`

## Typical Flow

1. If you need **catalog truth** in the runtime: use [registration doc](registering-domain-invariants-through-query.md) first.
2. For declaration-scoped posture: author invariant/capability contribution on `worth_query_domain`.
3. `materialize()` and read support report rows (admitted vs deferred neighbors).
4. Compose with other lanes via [contributions hub](../contributions/README.md) when needed.

## How It Relates

- [Registering domain invariants through Query](registering-domain-invariants-through-query.md) â€” registration vs contribution
- [Contributions hub](../contributions/README.md) â€” lane map
- [Contribution composed orchestration](../contribution-composed-orchestration.md) â€” multi-lane flows
- [Declaration scoped support](../support/declaration-scoped-support-and-traceability.md) â€” support traceability neighbors

## Good to Know

- Payload types participate in the same proof-integration artifact family as explanation/admission contributions.
- `capability_gap` authoring is test-coveredâ€”use tests as behavior proof for gap posture shapes.
- Support reports are generic over payload typeâ€”read the invariant-specific report alias in `reports.rs`.

## Anti-Patterns

- Using contribution materialize as a silent substitute for `register_invariant` on the builder.
- Assuming contribution posture implies invariant enforcement at runtime without registration artifacts.
- Duplicating relational invariant authority inside domain modules.

## Current Limits

| Concern | Status |
|---------|--------|
| Runtime-backed contribution materialize + support reports | **Verified** on certified paths |
| Store-backed contribution durability | **Deferred** per domain-capability matrix neighbors |
| Full lane â€œcloseoutâ€ for every orchestration row | **Not claimed**â€”see [public doc coverage](../public-doc-coverage.md) |

## Related Docs

- [Registering domain invariants through Query](registering-domain-invariants-through-query.md)
- [Contributions hub](../contributions/README.md)
- [Support matrix and admission](../../foundations/support-matrix-and-admission.md)
