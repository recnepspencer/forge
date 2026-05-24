# Canonical Domain Declarations

## What This Feature Is

Canonical domain declarations are the first point where an admitted configured
domain handle is allowed to express **declaration-local meaning** through
Query.

The important boundary is:

- the admitted configured handle carries the stable operating world
- the declaration input carries family-local meaning inside that world
- Query canonicalizes the combined meaning into one authoritative declaration
  artifact

That declaration artifact is basis-first. Its digest is derived from the
canonical basis bundle; the digest is not the source of truth.

## Why You Use It

- declare domain-local intent without rebuilding a local pre-Query authoring
  layer
- freeze one canonical declaration identity before legality, routing, or
  continuation begin
- retain declaration family key and Query-owned taxonomy posture as part of the
  canonical declaration artifact
- compare declarations under an explicit equivalence basis
- keep declaration meaning rooted in an admitted operating world instead of
  ambient host state

## Stable Entry Points

- `ForgeQueryAdmittedConfiguredDomainHandle::declare(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::declare_with_version(...)`
- `ForgeQueryDeclarationInput`
- `ForgeQueryDeclarationFamilyMarker`
- `ForgeQueryDeclarationFamilyTaxonomy`
- `ForgeQueryCanonicalDeclarationArtifact`
- `ForgeQueryCanonicalDeclarationArtifact::compare_under(...)`
- `ForgeQueryDeclarationCanonicalizationVersion`

Good to know:
- Query still does not own your domain families or nouns
- the ordinary lane is generic: downstream code defines the typed declaration
  input, and Query canonicalizes it
- declaration family identity is carried through the input type's associated
  family marker, and that family meaning participates in canonical declaration
  identity
- raw declaration normalization stays behind Query-owned front doors

## Core Mental Model

Think of declaration work as the next layer after configured handles:

- the handle says which admitted world you are operating in
- the declaration input says what you mean inside that world
- Query turns that combined meaning into one canonical declaration basis

That split matters because two declarations with identical local syntax are not
necessarily the same declaration if the admitted operating world changed
meaningfully.

The declaration input type does two things:

- identifies the declaration family for your domain through its associated
  family marker
- binds that family to one Query-owned taxonomy posture
- exposes the canonical declaration-local components that Query will lower into
  foundational basis entries

Query owns the canonical artifact, the basis bundle, the digest derivation, and
the comparison surface.

## How It Executes

1. define a downstream declaration input type that implements
   `ForgeQueryDeclarationInput<YourDomain>`
2. start from an admitted configured handle
3. call `declare(...)` or `declare_with_version(...)`
4. Query combines:
   - admitted handle identity
   - declaration family key
   - declaration taxonomy posture
   - declaration-local canonical components
5. Query lowers that meaning into one foundational canonical basis artifact and
   one canonical basis bundle
6. Query derives the declaration digest from the bundle
7. later phases can consume the retained canonical declaration artifact instead
   of rediscovering the declaration meaning

The ordinary lane pins the canonicalization version for you. Use the explicit
version surface only when you are doing advanced tooling, proof work, or
certification.

The important practical point is that Query retains the canonical declaration
artifact for later phases. Capability gating, legality, routing, and
continuation should consume that retained artifact rather than re-deriving
family meaning from host glue.

## Small Example

```rust
use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationFamilyTaxonomy,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationPrimaryAuthorityFamily,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryGroupedDeclarationPosture, ForgeQuerySignalCompatibilityPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "example.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext;

impl ForgeQueryDomainOperatingContext<GeometryDomain> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryContext]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[ForgeQueryConfigSectionFamily::Query]
    }

    fn context_identity_digest(&self) -> String {
        "geometry.collaborative".to_string()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SplitEdgeDeclaration {
    edge_ref: &'static str,
    parameter: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SplitEdgeFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for SplitEdgeFamily {
    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn taxonomy() -> ForgeQueryDeclarationFamilyTaxonomy {
        ForgeQueryDeclarationFamilyTaxonomy::new(
            ForgeQueryDeclarationPrimaryAuthorityFamily::RelationalTruth,
            ForgeQuerySignalCompatibilityPosture::Compatible,
            ForgeQueryGroupedDeclarationPosture::NeighborhoodCapable,
        )
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for SplitEdgeDeclaration {
    type Family = SplitEdgeFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref),
            ForgeQueryDeclarationCanonicalEntry::text("parameter", self.parameter),
        ]
    }
}

let query = ForgeQueryApplicationFacade::runtime_backed_default();
let handle = query
    .domain(GeometryDomain)
    .with_operating_context(GeometryOperatingContext)
    .validate()?
    .admit()?;

let declaration = handle.declare(SplitEdgeDeclaration {
    edge_ref: "edge:42",
    parameter: "midpoint",
})?;

let digest = declaration.declaration_digest();
let family = declaration.declaration_family_key();
```

## Real Example

```rust
use forge_foundational::facade::CanonicalEquivalenceBasis;
use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationFamilyTaxonomy,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationPrimaryAuthorityFamily,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryGroupedDeclarationPosture, ForgeQuerySignalCompatibilityPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.geometry"
    }

    fn display_name(&self) -> &'static str {
        "Worth Geometry"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext {
    regime: &'static str,
}

impl GeometryOperatingContext {
    fn collaborative() -> Self {
        Self {
            regime: "collaborative-authoritative",
        }
    }
}

impl ForgeQueryDomainOperatingContext<GeometryDomain> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryContext,
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("geometry-regime:{}", self.regime)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SplitEdgeDeclaration(&'static str);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SplitEdgeFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for SplitEdgeFamily {
    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn taxonomy() -> ForgeQueryDeclarationFamilyTaxonomy {
        ForgeQueryDeclarationFamilyTaxonomy::new(
            ForgeQueryDeclarationPrimaryAuthorityFamily::RelationalTruth,
            ForgeQuerySignalCompatibilityPosture::Compatible,
            ForgeQueryGroupedDeclarationPosture::NeighborhoodCapable,
        )
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for SplitEdgeDeclaration {
    type Family = SplitEdgeFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.0)]
    }
}

let query = ForgeQueryApplicationFacade::runtime_backed_default();
let handle = query
    .domain(GeometryDomain)
    .with_operating_context(GeometryOperatingContext::collaborative())
    .validate()?
    .admit()?;

let left = handle.declare(SplitEdgeDeclaration("edge:42"))?;
let right = handle.declare(SplitEdgeDeclaration("edge:42"))?;

let comparison = left.compare_under(&right, CanonicalEquivalenceBasis::ExactCanonicalBasis)?;
let digest = left.declaration_digest();
let basis_bundle = left.canonical_basis_bundle();
```

What this example is showing:

- the operating context changes the admitted world the declaration belongs to
- the declaration input contributes family-local meaning
- Query retains a canonical declaration artifact you can compare, inspect, and
  hand to later phases without reopening the original authoring context

## How It Relates To Other Features

- [Platform Entry](./platform-entry.md)
  gives you the typed domain front door
- [Configured Domain Handles](./configured-domain-handles.md)
  give you the admitted operating world declaration work depends on
- [Declaration Family Taxonomy](./declaration-family-taxonomy.md)
  defines the Query-owned classification carried by the declaration family
  marker
- later capability gating, legality, and routing should consume the retained
  canonical declaration artifact rather than rediscovering family meaning from
  host-local strings or control flow

## Inspection And Debugging

The main inspection surfaces are on the canonical declaration artifact:

- `declaration_family_key()`
- `declaration_taxonomy()`
- `declaration_primary_authority_family()`
- `declaration_grouped_posture()`
- `declaration_signal_compatibility()`
- `declaration_digest()`
- `handle_identity_digest()`
- `canonical_basis_bundle()`
- `canonicalization_version()`
- `compare_under(...)`

Use them to answer:

- whether two authoring paths produced the same canonical declaration
- whether a declaration changed because the admitted operating world changed
- which family a retained declaration artifact belongs to
- which Query-owned taxonomy posture that family carries
- which canonicalization version was used

If two declarations look the same locally but compare differently, check the
admitted configured handle identity first. In Query, operating-world proof is
part of declaration identity.

## Anti-Patterns

- treating the declaration input as a bag of host-local data instead of a
  canonical declaration-local meaning surface
- hashing or comparing downstream declaration structs directly instead of using
  the retained canonical declaration artifact
- rebuilding family meaning from ad hoc strings or branch logic after Query has
  already canonicalized the declaration
- smuggling dynamic eligibility, preview tokens, or runtime continuation setup
  into declaration authoring
- treating the declaration digest as the authority instead of the canonical
  basis bundle it was derived from

## Current Limits

Canonical domain declarations do not yet decide:

- whether a declaration family is admitted for a given handle
- legality of the declaration inside one admitted operating world
- lower-authority routing into relational, bridge, or signal surfaces
- grouped execution semantics
- runtime, preview, or historical continuation

This feature freezes declaration-local meaning and gives later phases one
retained canonical artifact to build on.

## Related Docs

- [Platform Entry](./platform-entry.md)
- [Configured Domain Handles](./configured-domain-handles.md)
- [Declaration Family Taxonomy](./declaration-family-taxonomy.md)
- [Domain Capabilities](./README.md)

What is authoritative here:

- the admitted handle is authoritative for the stable operating world
- the canonical declaration artifact is authoritative for declaration meaning

What is derived:

- the declaration digest is derived from the canonical basis bundle
- comparison posture is derived from the chosen equivalence basis plus the two
  canonical declaration artifacts
