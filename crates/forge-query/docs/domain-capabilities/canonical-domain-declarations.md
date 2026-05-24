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
- compare declarations under an explicit equivalence basis
- keep declaration meaning rooted in an admitted operating world instead of
  ambient host state

## Stable Entry Points

- `ForgeQueryAdmittedConfiguredDomainHandle::declare(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::declare_with_version(...)`
- `ForgeQueryDeclarationInput`
- `ForgeQueryCanonicalDeclarationArtifact`
- `ForgeQueryCanonicalDeclarationArtifact::compare_under(...)`
- `ForgeQueryDeclarationCanonicalizationVersion`

Good to know:
- Query still does not own your domain families or nouns
- the ordinary lane is generic: downstream code defines the typed declaration
  input, and Query canonicalizes it
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

- names the declaration family explicitly
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
   - declaration family identity
   - declaration-local canonical components
5. Query lowers that meaning into one foundational canonical basis artifact and
   one canonical basis bundle
6. Query derives the declaration digest from the bundle
7. later phases can consume the retained canonical declaration artifact instead
   of rediscovering the declaration meaning

The ordinary lane pins the canonicalization version for you. Use the explicit
version surface only when you are doing advanced tooling, proof work, or
certification.

## Small Example

```rust
use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalEntryKind,
    ForgeQueryDeclarationCanonicalValue, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
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
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
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

impl ForgeQueryDeclarationInput<GeometryDomain> for SplitEdgeDeclaration {
    fn declaration_family(&self) -> &'static str {
        "split-edge"
    }

    fn canonical_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::new(
                "split_edge.edge_ref",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(self.edge_ref.to_string()),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "split_edge.parameter",
                ForgeQueryDeclarationCanonicalEntryKind::Field,
                ForgeQueryDeclarationCanonicalValue::ExactText(self.parameter.to_string()),
            ),
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
```

## Real Example

```rust
use forge_foundational::facade::CanonicalEquivalenceBasis;
use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalEntryKind,
    ForgeQueryDeclarationCanonicalValue, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
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
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
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

impl ForgeQueryDeclarationInput<GeometryDomain> for SplitEdgeDeclaration {
    fn declaration_family(&self) -> &'static str {
        "split-edge"
    }

    fn canonical_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::new(
            "split_edge.edge_ref",
            ForgeQueryDeclarationCanonicalEntryKind::Identity,
            ForgeQueryDeclarationCanonicalValue::ExactText(self.0.to_string()),
        )]
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

What is authoritative here:

- the admitted handle is authoritative for the stable operating world
- the canonical declaration artifact is authoritative for declaration meaning

What is derived:

- the declaration digest is derived from the canonical basis bundle
- comparison posture is derived from the chosen equivalence basis plus the two
  canonical declaration artifacts

## How It Relates To Other Features

- start with [Platform Entry](./platform-entry.md) when the domain has not
  entered Query yet
- use [Configured Domain Handles](./configured-domain-handles.md) to establish
  the admitted operating world before you declare inside it
- later legality, routing, and continuation phases build on this retained
  declaration artifact rather than replacing it

## Inspection And Debugging

Use the canonical declaration artifact to inspect:

- the admitted handle identity digest
- the declaration family
- the pinned or explicit canonicalization version
- the canonical basis bundle
- the derived declaration digest

Use `compare_under(...)` when you need explicit equivalence or mismatch posture
instead of guessing from digest equality alone.

If canonicalization fails, start by checking:

- whether the declaration input produced any canonical entries
- whether two entries collided on the same locus and kind
- whether the declaration-local meaning is accidentally depending on hidden host
  state instead of stable typed inputs

## Anti-Patterns

- treating digest equality as the only declaration-equality story
- skipping configured handles and trying to declare straight from a domain
  marker
- hiding declaration family identity and expecting Query to infer it later
- smuggling dynamic trigger conditions or exact runtime basis into the
  declaration-local canonical entries
- teaching raw canonical-entry assembly as the default product-code workflow

## Current Limits

This feature gives you canonical declaration identity. It does not yet give you:

- structural legality
- dynamic eligibility
- route planning
- runtime, preview, or historical continuation binding
- domain-specific helper verbs owned by Query

The ordinary lane is intentionally generic. If you want domain-shaped helper
verbs such as `declare_split_edge(...)`, those should live in downstream code or
later Query extension layers, not in the generic Query facade itself.

## Related Docs

- [Platform Entry](./platform-entry.md)
- [Configured Domain Handles](./configured-domain-handles.md)
- [Workflow Lanes: Common, Checked, Proof, And Raw](./workflow/workflow-lanes-common-checked-proof-raw.md)
- [Typed Query Expressions And Result Shapes](../authoring/query-expressions-and-result-shapes.md)
