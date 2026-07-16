# Effects And Recovery

Use this section when writes need to be more than "send request, wait, replace."

This is where you decide and inspect:

- optimistic versus committed-only behavior
- exact restore versus compact inverse rollback
- merge and rebase readiness
- effect-envelope proof
- profile closeout guarantees

## Start Here

- [Branch-Native Effects](./branch-native-effects.md)
  The common path for isolated optimistic resource writes.
- [Concurrent Optimistic Effects](./concurrency-and-dependencies.md)
  Siblings, parent/child dependencies, random response order, and targeted
  closeout.
- [Effect Envelopes And Closeout](./effect-envelopes-and-closeout.md)
  What the runtime records for each effect and how to inspect profile support.
- [Merge And Rebase](./merge-and-rebase.md)
  Planning or executing native merge for one concrete resource effect.
- [Rollback And Recovery](./rollback-and-recovery.md)
  Exact restore, compact inverse, committed-only, and unavailable rollback.

## The Main Mental Model

Confirmed server data is canonical. Open effect branches form a derived visible
projection. The effect profile decides which behavior is admitted, and each
runtime-issued effect identity is the handle for one request's closeout.

## Related Docs

- [Choose An Effect Profile](../updating/choose-an-effect-profile.md)
- [What Happens After A Write](../updating/what-happens-after-a-write.md)
- [Branch-Native Resource Effects](../branch-native-effects.md)
