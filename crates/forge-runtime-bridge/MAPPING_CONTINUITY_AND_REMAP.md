# Mapping, Continuity, And Remap

This guide covers the bridge surfaces that preserve identity and route meaning
when truth changes are not trivial one-to-one updates.

These topics matter when you need the bridge to stay honest under:

- fanout
- fallback
- continuity lineage
- remap or identity drift
- structural ambiguity

## Mapping

Mapping is the first bridge step from truth to compute invalidation.

The public mapping surface includes:

- `BridgeMappingRegistration`
- `BridgeMappingId`
- `TruthPatchScope`
- `SignalInvalidationScope`
- `CoarseRoutingMode`
- `BridgeAspectRegistration`

This is where the bridge learns what truth scopes correspond to which compute
targets.

## Continuity

Continuity answers harder questions:

- is this still the same thing after history changed?
- did lineage remain stable?
- is this ambiguous, unsupported, or merge-like?

Relevant public surfaces include:

- `BridgeContinuityArtifact`
- `BridgeContinuityOutcomeClass`
- `BridgeContinuityRejectionClass`
- `ResolvedLineageContinuity`
- `BridgeHistoricalLineagePacket`

Continuity exists so the bridge can classify identity-preserving versus
identity-breaking history explicitly instead of hiding it in host heuristics.

## Remap

Remap matters when the bridge can no longer treat a new structure as a trivial
continuation of the old one.

That is why the bridge also exposes structural comparison and remap surfaces
for more deliberate identity reasoning.

## Why This Matters

If mapping and continuity are shallow, the bridge may still appear to work in
simple cases while failing the real trust question:

- does the same bridge target still mean the same thing after history moves?

This guide exists because correctness here is foundational, not decorative.
