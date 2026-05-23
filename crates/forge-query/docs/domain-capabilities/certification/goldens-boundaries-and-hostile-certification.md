# Goldens, Boundaries, And Hostile Certification

## What This Feature Is

This doc explains the three proof surfaces that keep the domain-capability seam
honest:

- compile-checked golden transcripts
- compile-fail boundary suites
- hostile executable certification tests

## Why You Use It

- you want to know which proof surface should catch a regression
- you need to extend the seam without weakening its public-lane guarantees
- you want confidence that geometry-kernel usage is protected against pseudo-
  Query shortcuts

## Stable Entry Points

Golden and boundary manifests:

- `forge_query_domain_capability_golden_transcripts()`
- `forge_query_domain_capability_compile_fail_boundaries()`
- `forge_query_domain_capability_compile_fail_boundary_digest()`

Executable closeout:

- `certify_domain_capabilities()`
- `domain_capabilities::certification_closeout_tests`

## Core Mental Model

Goldens prove the public path compiles the way the docs teach it.

Compile-fail boundaries prove illegal progression, illegal degradation, or
illegal construction does not compile.

Hostile certification proves the public, checked, proof, and raw lanes converge
or stay distinct in the ways the spec requires.

## How It Executes

1. goldens compile the intended ordinary examples
2. compile-fail tests reject illegal usage
3. hostile tests compare equivalent and intentionally different surfaces
4. the certification bundle digests those live proof surfaces

## Small Example

```rust
let digest = forge_query_domain_capability_compile_fail_boundary_digest();
```

## Real Example

When you add a new domain-capability feature, the honest path is:

1. add or update the feature doc
2. add or update a golden transcript
3. add or update DX compile-fail boundaries
4. extend hostile certification if the feature changes canonicalization,
   support posture, or lane behavior

## How It Relates To Other Features

- [Certification Surface And Closeout Bundle](./certification-surface-and-closeout-bundle.md)
  explains the public readout that summarizes these proofs
- every category doc in this tree should have a matching golden or boundary
  story through these surfaces

## Inspection And Debugging

- if the docs teach a path that no longer compiles, the golden should fail
- if a forbidden shortcut becomes legal, the compile-fail boundary should fail
- if two lanes drift semantically, hostile certification should fail

## Anti-Patterns

- adding public examples without a golden
- relying on runtime denials when the boundary should be compile-fail
- treating certification as a one-time exercise instead of a living guardrail

## Current Limits

- these proof surfaces are only as good as their synchronization with the live
  public API
- they do not replace product-facing docs; they prove the docs are still honest

## Related Docs

- [Certification Surface And Closeout Bundle](./certification-surface-and-closeout-bundle.md)
- [Domain Capabilities](../README.md)
