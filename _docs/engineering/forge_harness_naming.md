# Forge Harness Naming

## Goal

Keep the harness vocabulary explicit, distinct, and stable across runtimes.

## Preferred Terms

- `ScenarioPlan`: a readable scenario definition before fixture compilation
- `ScenarioFixture`: a reusable compiled scenario payload
- `MutationBatch`: one named group of input changes
- `ExecutionRequest`: a concrete request to evaluate targets
- `ExecutionProfile`: a named execution and capture policy
- `RunRecord`: the primary captured result of one execution
- `SnapshotRecord`: a captured state view before or after execution
- `ComparisonRecord`: the result of comparing harness outputs
- `RecordArchive`: a grouped export bundle for captured records
- `EventSubscription`: a typed event selection request
- `RunMatrix`: one request executed across multiple profiles
- `ParitySuite`: a profile-parity tool over harness runs
- `BudgetUsage`: the budget consumption summary for one run
- `AdapterDouble`: a fake adapter for harness tests

## Terms To Avoid

- `Recipe`
- `Seeder`
- `Helper`
- `Manager`
- `Stuff`

These terms are either too ambiguous, too implementation-shaped, or too close to one another for stable cross-runtime use.
