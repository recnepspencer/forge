# Keyed Computation

Keyed computation is for "same kind of work, different stable key."

Think:

- one computation shape
- many instances
- each instance identified by a real key

## Main surfaces

- `register_computation_family(...)`
- `keyed_node(...)`
- `ComputationFamily`
- `ComputationKey`
- `KeyedComputation`

## When to use it

Use keyed computation when:

- you have lots of nodes with the same evaluation shape
- identity comes from a stable key
- you want reuse and memoization to stay organized instead of getting hacked
  together with side maps

## Mental model

Think:

- one family
- many keyed instances
- stable runtime-managed lookup and reuse

Do not think:

- anonymous nodes plus a pile of manual bookkeeping

## Practical rule

If the system naturally says “same kind of computation, different stable key,”
use a computation family rather than hand-rolling dynamic node bookkeeping.

Concrete examples:

- one derived node per file path
- one derived node per geometry entity id
- one derived node per tenant, account, or model id
