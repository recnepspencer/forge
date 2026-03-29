# Conditions And Comparators

This reference exists because these concepts are easy to blur together:

- aspects
- evaluation conditions
- comparator policy
- tolerance
- custom conditions

They are related, but they are not the same thing.

## Main Rule

If your question is:

- "Should this node run at all?"
  - think condition

- "Did the upstream change matter enough to count?"
  - think comparator

- "What does this aspect slot mean?"
  - that is your host-domain contract

## Main Surfaces

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

## Real-World Meaning

In a commerce system:

- price changed is an aspect
- "only rerun when asked" is a condition
- "ignore tiny changes" is comparator policy
- "same checkout summary identity, no downstream churn" is output identity behavior
