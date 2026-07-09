# Runtime Policy

This guide explains how runtime policy shapes bridge behavior without changing
bridge meaning.

Policy is where the bridge decides questions like:

- how rich diagnostics should be
- what replay artifacts are retained
- which execution posture is in force
- how route planning policy constrains runtime behavior

## What Policy Is For

Policy is not supposed to redefine truth semantics or compute semantics.

It exists to control:

- execution posture
- diagnostics richness
- artifact retention
- replay compatibility

Those are bridge concerns, not host folklore.

## Everyday Default

Most users should rely on the runtime default:

- current truth view
- standard diagnostics tier
- canonical routing mode
- replay-safe ordinary behavior

That is why the standard path does not force policy work into first success.

## Where Policy Lives In The Public Surface

Policy belongs primarily in:

- `worth_runtime_bridge::facade`

That is because policy matters, but it is not supposed to clutter the everyday
memory model.

Relevant advanced types include:

- `BridgeRuntimePolicy`
- `BridgePolicyDeclaration`
- `BridgeDiagnosticsTier`
- `BridgeExecutionPolicyBaseline`
- `BridgeRoutePlanningPolicy`

## Diagnostics Tier

The most visible policy knob is diagnostics richness.

The bridge should allow richer or leaner retained explanations without changing:

- routing meaning
- truth-view meaning
- speculative isolation meaning
- writeback meaning
- replay meaning

If diagnostics-tier changes alter semantic bridge outcome, the policy boundary
is broken.

## Policy And Replay

Replay policy is especially important because it affects what evidence is
retained and what compatibility guarantees can be honored later.

The bridge should keep these distinctions explicit:

- no replay artifacts available
- replay artifacts retained
- replay attempted under incompatible policy

Those are typed bridge concerns, not ambiguous support situations.

## Policy As Advanced Control

Reach for explicit policy when the job requires things like:

- narrowing diagnostics for a leaner runtime posture
- broadening retained artifacts for proof or certification
- testing policy-sensitive replay behavior
- controlling route-planning posture in a deliberate way

If you do not need one of those, the default path is usually the correct path.

## Product Rule

Policy should change richness and posture, not truth.

That is the standard the bridge has to preserve.
