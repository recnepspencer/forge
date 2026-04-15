# Forge Test Architecture

This document freezes the canonical Forge test architecture shape for serious domain crates.

The goal is a test system that scales from current topology milestone proof through analytic
geometry, trimmed carriers, booleans, chamfers, fillets, junctions, and freeform/NURBS work
without redesigning the harness grammar every milestone.

## Layer Model

Every serious Forge test surface should converge toward five layers:

1. `fixtures/`
   - owns reusable authored inputs and phase artifacts
   - no assertions
   - no milestone completeness logic
2. `phase_harness/`
   - owns focused proof at one runtime or pipeline boundary
   - examples: validation, interpretation, binding, replay, bridge, geometry solve
3. `certification_core/`
   - owns generic suite grammar
   - suite identity, canonical rows, rejection rows, parity rows, digests,
     completeness rules, and requirements enforcement
4. `domain_certification/`
   - owns crate-specific proof semantics and family definitions
   - examples: Worth primitive families, query planning lanes, store recovery programs
5. `milestones/`
   - thin declarations of required suites, required families, required outputs,
     and required closeout bars

Rule:
- shared harness layers own the grammar
- domain crates own the meaning

## Fixture Rule

Fixtures are organized by stable phase artifact boundaries, not by milestone-local convenience.

For Worth, the target fixture spine is:

- `fixtures/authored_topology`
- `fixtures/validated_topology`
- `fixtures/derived_topology`
- `fixtures/geometry_binding`
- `fixtures/analytic_surfaces`
- `fixtures/freeform_surfaces`
- `fixtures/intersection_cases`
- `fixtures/trim_networks`
- `fixtures/boolean_cases`
- `fixtures/chamfer_cases`
- `fixtures/fillet_cases`
- `fixtures/junction_cases`
- `fixtures/branch_replay_cases`
- `fixtures/bridge_cases`

Milestone 1 only needs the topology, branch/replay, and bridge fixtures concretely.

## Certification Grammar

Certification grammar must support:

- named suites
- canonical rows
- rejection rows
- parity rows
- bridge rows where applicable
- machine-checkable digests
- machine-checkable counters
- completeness reports driven by requirements
- explicit unmet requirements

Required row classes:

- equality
- inequality
- typed rejection
- replay parity
- branch parity
- bridge parity
- budget/counter expectations
- family coverage expectations
- localized failure evidence expectations

The same grammar must be able to express later geometry families such as:

- `PlanarCarrier`
- `TrimmedCarrier`
- `SharedEdgeDualTrim`
- `FreeformPatch`
- `FreeformTrimmedPatch`
- `BooleanSet`
- `ChamferSet`
- `ConstantFilletSet`
- `VariableFilletRail`
- `BlendJunction`

without changing the underlying harness model.

## Milestone Declaration Rule

Milestone suites should be thin declarations over fixtures plus certification grammar.

A milestone closeout path should primarily declare:

- which canonical families are required
- which rejection families are required
- which parity scopes are required
- which bridge scopes are required
- which outputs are required

It should not rebuild full runtime pipelines inline when reusable fixtures exist.

## Shared vs Local Extraction

Move code into `forge-harness` only when it is:

- domain-neutral
- needed by at least two serious crates
- stable enough not to churn with one domain milestone

Likely future `forge-harness` candidates:

- generic certification suite and row grammar
- generic completeness registry enforcement
- generic parity/run matrix helpers
- generic artifact and digest bundle containers

Not candidates until proven cross-domain:

- Worth primitive families
- Worth rejection semantics
- Worth bridge proof semantics
- Worth topology and geometry fixture meaning

## Future Worth Family Ladder

Worth’s certification architecture must support expansion through:

- topology families
  - `WireOpen(n)`
  - `WireClosed(n)`
  - `WireBranch(k)`
  - `SheetDisk(n)`
  - `SheetPatch(f)`
  - `SolidShell(f)`
  - `NmtEdgeFan(k)`
- geometry-binding families
  - planar binding
  - dual-trim binding
  - rebound carrier
  - UV anchoring
- analytic geometry families
  - planes, cylinders, cones, spheres, tori
  - intersection extraction
  - tangent and overlap cases
- freeform families
  - `FreeformPatch`
  - `FreeformTrimmedPatch`
  - patch networks
  - inversion and projection
- boolean families
  - planar
  - analytic
  - trimmed and freeform
- edge-treatment families
  - chamfers
  - constant fillets
  - variable fillets
  - blend junctions
- history/interaction families
  - branch/replay/merge
  - intent-preserving UI or DSL flows
  - bridge/writeback later

If adding one of those requires redesigning the harness grammar rather than adding new fixtures,
new domain suites, or new milestone declarations, the test architecture is wrong.
