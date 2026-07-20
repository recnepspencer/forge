# Conditions And Comparators

Conditions, comparators, triggers, and aspects answer different questions.
Keeping them separate is what lets Signal avoid unnecessary work without
misreporting what changed.

## Main Rule

- “Which part of this node changed?” is an aspect question.
- “Is this node eligible to evaluate?” is a condition question.
- “Was evaluation explicitly requested?” is a trigger question.
- “Did the dependency or output change meaningfully?” is a comparator question.
- “May an earlier artifact be reused?” is an artifact-equivalence question.

## Standalone Signal Surfaces

- `Aspect`
- `EvaluationCondition`
- `ComparatorPolicy`
- `.on_demand()`
- `.debounce(...)`
- `.aspect_filter(...)`
- `.delta_threshold(...)`
- `.custom_condition(...)`
- `.tolerance(...)`
- `.output_identity()`

Use these while constructing a standalone Signal graph.

## Installed Conditional Contracts

Query-hosted Signal work uses an installed contract:

- `InstalledSignalConditionalContract`
- `InstalledSignalConditionIdentity`
- `InstalledSignalComparatorIdentity`
- `InstalledSignalConditionDecision`
- `SignalConditionalExecutionRequest`
- `SignalConditionalDecisionEvidence`

Query authors portable semantic dependencies, condition families, trigger
families, thresholds and units, comparison requirements, maintenance posture,
and output relationships. Runtime Bridge admits those declarations against the
actual Signal graph and lowers them into `InstalledSignalConditionalContract`.

Signal remains the only owner of the runtime decision.

## Decision Semantics

The installed execution path preserves the exact decision classes:

- `ComputedChanged`
- `ComputedRevertedClean`
- `DependencyUnchanged`
- `SuppressedBeforeCompute`
- `DeferredByCondition`
- `DeferredTemporal`
- `DeferredOnDemand`

Dependency-unchanged, suppressed, and deferred decisions do not invoke
compute. Reverted-clean decisions preserve compute cost but must not emit a
semantic change. Query and Runtime Bridge carry this evidence; they do not
infer or reclassify it.

## Delta Thresholds

Portable Query thresholds carry typed unit identity, value family, comparison
domain, and inclusive/exclusive boundary. Runtime Bridge lowers that meaning;
Signal resolves the threshold against installed dependency observations.

Do not reimplement a threshold in a Query executor or Bridge callback. A
provider is appropriate only for a declared typed comparator family.

## Custom Conditions And Triggers

Standalone Signal graphs may use host-local custom conditions. Query-installed
operations instead declare typed portable families. Runtime construction
registers the matching Bridge provider.

A string-dispatch callback is not a typed family. A provider cannot select a
different family or redefine the portable condition.

## Aspects

Signal `Aspect` values are runtime-local node slots. Their meaning is scoped to
the installed graph, node, partition, and lowering owner.

Foundational aspect contracts and Relational aspect bindings are portable
semantic meaning. Runtime Bridge installs the correspondence between the two.
Do not persist a Signal aspect number as domain identity.

## Anti-Patterns

- Treating a condition and comparator as interchangeable.
- Running compute before the condition decision.
- Treating any output as a meaningful change.
- Re-evaluating a Signal decision in Query or Runtime Bridge.
- Using string dispatch for an installed condition or trigger family.
- Treating a numeric Signal aspect as portable semantic identity.

## Related Docs

- [Aspects And Dependencies](../core-concepts/aspects-and-dependencies.md)
- [Defining Computation](../guides/defining-computation.md)
- [Query Conditional Installed Operations](../../../../workspaces/worth-query/crates/worth-query/docs/domain-capabilities/conditional-installed-operations.md)
