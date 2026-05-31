# Declaration Family Capability Matrix

## What This Feature Is

Declaration family capability matrix is the Query-owned family support and
structural witness layer over canonical declaration families.

This is where Query decides two different things:

- which family-specific surfaces are structurally available from the family
  marker's type-level posture
- whether the admitted operating world can admit that family right now based on
  support and config posture

That is why the boundary is hybrid. Some family behavior is type-level. Some is
support-dependent.

## Why You Use It

- inspect family support before canonicalization
- get typed checked-lane family admission instead of ad hoc denials
- keep structurally wrong witness surfaces absent at compile time
- let other Query declaration features consume typed witness wrappers instead of reopening taxonomy
  decisions or redoing family support gating
- let later progression and route-planning work start from one already-admitted
  family boundary instead of rediscovering support posture

## Stable Entry Points

- `ForgeQueryAdmittedConfiguredDomainHandle::family_support::<F>()`
- `ForgeQueryAdmittedConfiguredDomainHandle::family_support_checked::<F>()`
- `ForgeQueryAdmittedConfiguredDomainHandle::declare_checked(...)`
- `ForgeQueryDeclarationCapabilityVerb`
- `ForgeQueryDeclarationCapabilityStatus`
- `ForgeQueryDeclarationFamilySupportRow`
- `ForgeQueryDeclarationFamilySupportReport`
- `ForgeQueryDeclarationFamilySupportChecked`
- `ForgeQueryDeclaredFamilyChecked`
- `ForgeQueryDeclarationAdmissionError`
- `ForgeQueryRelationalTruthDeclaration`
- `ForgeQueryBridgeContinuationDeclaration`
- `ForgeQuerySignalCompatibleDeclaration`
- `ForgeQueryNeighborhoodCapableDeclaration`
- `ForgeQueryBatchCapableDeclaration`

Good to know:

- Query does not try to encode the runtime support snapshot in types
- `handle.declare(input)` remains the ordinary lane
- support-dependent denial happens before canonicalization
- witness methods exist only when the family marker's posture tags make them
  structurally valid

## API Reference

The main public capability surfaces are:

Admitted-handle family support:
- `family_support::<F>() -> ForgeQueryDeclarationFamilySupportReport<D, F>`
- `family_support_checked::<F>() -> ForgeQueryDeclarationFamilySupportChecked<D, F>`
- `declare_checked(input) -> ForgeQueryDeclaredFamilyChecked<D, I>`

Support report inspection:
- `domain_key() -> &'static str`
- `declaration_family_key() -> &'static str`
- `declaration_taxonomy() -> ForgeQueryDeclarationFamilyTaxonomy`
- `aspect_contract() -> &ForgeQueryDeclarationAspectContract`
- `aspect_coverage() -> &ForgeQueryDeclarationAspectCoverage`
- `required_capability_families() -> &[ForgeQueryCapabilityFamily]`
- `required_config_sections() -> &[ForgeQueryConfigSectionFamily]`
- `rows() -> &[ForgeQueryDeclarationFamilySupportRow]`
- `row(verb) -> Option<&ForgeQueryDeclarationFamilySupportRow>`
- `declare_status() -> ForgeQueryDeclarationCapabilityStatus`
- `support_digest() -> &str`

Support row inspection:
- `verb() -> ForgeQueryDeclarationCapabilityVerb`
- `status() -> ForgeQueryDeclarationCapabilityStatus`
- `aspect_fit() -> ForgeQueryDeclarationAspectFit`
- `reason() -> &'static str`

Capability enum helpers:
- `ForgeQueryDeclarationCapabilityVerb::as_str() -> &'static str`
- `ForgeQueryDeclarationCapabilityStatus::as_str() -> &'static str`

Checked support outcomes:
- `ForgeQueryDeclarationFamilySupportChecked::Admitted(ForgeQueryDeclarationFamilySupportReport<D, F>)`
- `ForgeQueryDeclarationFamilySupportChecked::Deferred(ForgeQueryDeclarationFamilySupportReport<D, F>)`
- `ForgeQueryDeclarationFamilySupportChecked::Unsupported(ForgeQueryDeclarationFamilySupportReport<D, F>)`
- `ForgeQueryDeclarationFamilySupportChecked::InvalidContext(ForgeQueryDeclarationFamilySupportReport<D, F>)`

Checked admission outcomes:
- `ForgeQueryDeclaredFamilyChecked::Admitted(ForgeQueryCanonicalDeclarationArtifact<D, I>)`
- `ForgeQueryDeclaredFamilyChecked::Deferred(ForgeQueryDeclarationCapabilityDenial<D, I::Family>)`
- `ForgeQueryDeclaredFamilyChecked::Unsupported(ForgeQueryDeclarationCapabilityDenial<D, I::Family>)`
- `ForgeQueryDeclaredFamilyChecked::InvalidContext(ForgeQueryDeclarationCapabilityDenial<D, I::Family>)`
- `ForgeQueryDeclaredFamilyChecked::Canonicalization(ForgeQueryDeclarationCanonicalizationError)`

Admission-denial inspection:
- `capability_status() -> ForgeQueryDeclarationCapabilityStatus`
- `support_report() -> &ForgeQueryDeclarationFamilySupportReport<D, F>`

Admission error family:
- `ForgeQueryDeclarationAdmissionError::Deferred(ForgeQueryDeclarationCapabilityDenial<D, I::Family>)`
- `ForgeQueryDeclarationAdmissionError::Unsupported(ForgeQueryDeclarationCapabilityDenial<D, I::Family>)`
- `ForgeQueryDeclarationAdmissionError::InvalidContext(ForgeQueryDeclarationCapabilityDenial<D, I::Family>)`
- `ForgeQueryDeclarationAdmissionError::Canonicalization(ForgeQueryDeclarationCanonicalizationError)`

Witness wrappers:
- `ForgeQueryRelationalTruthDeclaration<'a, D, I>`
- `ForgeQueryBridgeContinuationDeclaration<'a, D, I>`
- `ForgeQuerySignalCompatibleDeclaration<'a, D, I>`
- `ForgeQueryNeighborhoodCapableDeclaration<'a, D, I>`
- `ForgeQueryBatchCapableDeclaration<'a, D, I>`

Witness-wrapper inspection:
- `artifact() -> &ForgeQueryCanonicalDeclarationArtifact<D, I>`

## Core Mental Model

Think of family capability as two filters:

1. **structural availability**
   - does this family kind even admit a given follow-on surface?
   - example: a non-relational family has no `.relational_truth()`
2. **operating-world admission**
   - does the admitted handle currently support this family's declared
     requirements?
   - example: a durable-artifacts family defers even if its posture is valid

The first filter is compile-time. The second filter is a checked-lane runtime
decision backed by the support snapshot.

## How It Executes

1. your family marker chooses Query-owned posture tags and optional capability
   requirements
2. `family_support::<F>()` derives one family-scoped report from:
   - admitted handle identity
   - support snapshot
   - required capability families
   - required config sections
   - family taxonomy posture
3. `declare_checked(...)` evaluates that family support first
4. only admitted families reach canonical declaration formation
5. admitted canonical declarations expose only the structural witness methods
   their family posture allows

The report includes one row per verb:

- `Declare`
- `RelationalTruthWitness`
- `BridgeContinuationWitness`
- `SignalCompatibilityWitness`
- `NeighborhoodGroupingWitness`
- `BatchGroupingWitness`

Each row is classified as:

- `Admitted`
- `DeferredDebt`
- `Unsupported`
- `InvalidContext`

Each row also carries an aspect-fit posture for that witness surface:

- `Exact`
- `CompatibleSuperset`
- `Partial`
- `MissingRequired`
- `Conflict`

That row-level aspect fit is the semantic answer to "could this witness surface
support the family's declared slices if support posture were otherwise
admitted?", while the row `status()` remains the operating-world answer about
support, config, and structural availability.

The important detail is that aspect coverage is not inferred back
from the contract itself. The family can now report declared semantic coverage
separately, including masked slices, so a family may remain broadly admitted
while still telling later consumers that a particular semantic slice is not
currently bindable.

## Small Example

```rust
use forge_query::facade::{
    ForgeQueryDeclarationCapabilityStatus, ForgeQueryDeclaredFamilyChecked,
};

let support = handle.family_support::<SplitEdge>();
assert_eq!(
    support.declare_status(),
    ForgeQueryDeclarationCapabilityStatus::Admitted,
);

match handle.declare_checked(AttachMaterialForActiveFaceSelection) {
    ForgeQueryDeclaredFamilyChecked::Admitted(declaration) => {
        let _truth = declaration.relational_truth();
    }
    ForgeQueryDeclaredFamilyChecked::Deferred(denial) => {
        assert_eq!(
            denial.capability_status(),
            ForgeQueryDeclarationCapabilityStatus::DeferredDebt,
        );
    }
    ForgeQueryDeclaredFamilyChecked::Unsupported(denial) => {
        assert_eq!(
            denial.capability_status(),
            ForgeQueryDeclarationCapabilityStatus::Unsupported,
        );
    }
    ForgeQueryDeclaredFamilyChecked::InvalidContext(denial) => {
        assert_eq!(
            denial.capability_status(),
            ForgeQueryDeclarationCapabilityStatus::InvalidContext,
        );
    }
    ForgeQueryDeclaredFamilyChecked::Canonicalization(error) => {
        return Err(error.into());
    }
}
```

## Real Example

```rust
use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationCapabilityStatus, ForgeQueryDeclarationCapabilityVerb,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
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
struct CollaborativeWorld;

impl ForgeQueryDomainOperatingContext<GeometryDomain> for CollaborativeWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[ForgeQueryConfigSectionFamily::Relational]
    }

    fn context_identity_digest(&self) -> String {
        "geometry.collaborative".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AttachFaceMaterial;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for AttachFaceMaterial {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "attach-face-material"
    }

    fn required_capability_families() -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AttachMaterialForActiveFaceSelection;

impl ForgeQueryDeclarationInput<GeometryDomain> for AttachMaterialForActiveFaceSelection {
    type Family = AttachFaceMaterial;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::text(
                "selection_scope",
                "active-face-selection",
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "material_edit_intent",
                "attach-material-from-current-selection",
            ),
        ]
    }
}

let query = ForgeQueryApplicationFacade::runtime_backed_default();
let handle = query
    .domain(GeometryDomain)
    .with_operating_context(CollaborativeWorld)
    .validate()?
    .admit()?;

let support = handle.family_support::<AttachFaceMaterial>();
assert_eq!(
    support.declare_status(),
    ForgeQueryDeclarationCapabilityStatus::Admitted,
);
assert_eq!(
    support
        .row(ForgeQueryDeclarationCapabilityVerb::RelationalTruthWitness)
        .unwrap()
        .status(),
    ForgeQueryDeclarationCapabilityStatus::Admitted,
);

let declaration = handle.declare(AttachMaterialForActiveFaceSelection)?;

let truth = declaration.relational_truth();
let signal = declaration.signal_compatible();
let grouped = declaration.neighborhood_capable();

assert_eq!(truth.artifact().declaration_family_key(), "attach-face-material");
assert_eq!(signal.artifact().declaration_family_key(), "attach-face-material");
assert_eq!(grouped.artifact().declaration_family_key(), "attach-face-material");
```

What this example is showing:

- family support rows and declaration admission agree
- witness access is structural, not guessed later from source-order folklore
- Query stays domain-agnostic while still exposing family-shaped capability
  surfaces
- the primary app-facing geometry story is dynamic context, while the canonical
  declaration entries remain an internal retained shape

## Aspect Semantics

This matrix is the first aspect-aware declaration-entry gate.
Family admission remains the outer shape, but the retained support surface must
be allowed to say "this family is admitted while this semantic slice is
unsupported, masked, permission-limited, or invariant-sensitive." Later
legality and progression surfaces consume that narrower support truth instead of
pretending broad family admission already proved everything they need.

## How It Relates To Other Features

- [Configured Domain Handles](./configured-domain-handles.md) provide the
  admitted support snapshot this matrix consumes
- [Declaration Family Taxonomy](./declaration-family-taxonomy.md) provides the
  family posture vocabulary this matrix turns into support rows and witness
  availability
- [Canonical Domain Declarations](./canonical-domain-declarations.md) provide
  the retained artifacts the witness wrappers point back to
- [Declaration Legality](./declaration-legality.md) begins only after this
  matrix has already admitted the family
- [Declaration Progression](./declaration-progression.md) begins only after the
  family is admitted and legality evidence exists
- [Declaration Route Plans](./declaration-route-plan.md) begins only after the
  family is admitted, progression is admitted, and matching foundational
  evidence exists

## Inspection And Debugging

Use `family_support::<F>()` when you want the full family-scoped report,
including:

- the family key
- the family taxonomy
- required capability families
- required config sections
- row-by-row verb status
- support digest

Use `family_support_checked::<F>()` or `declare_checked(...)` when you want the
result already classified as:

- admitted
- deferred
- unsupported
- invalid context

When a declaration is admitted but a witness method is absent, that is a
structural posture issue, not a support snapshot issue.

## Anti-Patterns

- treating all family denial as if it were compile-time
- canonicalizing declarations before checking family support
- using raw family strings as the primary support lookup key
- re-reading taxonomy values in other Query declaration features when a typed
  witness wrapper is
  already available
- expecting Query to enumerate every downstream family globally

## Current Limits

Family capability matrix now decides family admission before legality begins.
It still does not decide:

- lower-authority route planning
- Query boundary receipts
- Query boundary envelopes
- Query relational truth routing
- Query bridge continuation routing
- grouped execution semantics
- continuation execution

It freezes the family support boundary and structural witness surface other
Query declaration features depend on.

## Related Docs

- [Configured Domain Handles](./configured-domain-handles.md)
- [Canonical Domain Declarations](./canonical-domain-declarations.md)
- [Declaration Family Taxonomy](./declaration-family-taxonomy.md)
- [Declaration Legality](./declaration-legality.md)
- [Declaration Progression](./declaration-progression.md)
- [Declaration Route Plans](./declaration-route-plan.md)
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md)
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
- [Declaration Relational Truth Routing](./declaration-relational-truth-routing.md)
- [Declaration Bridge Continuation Routing](./declaration-bridge-continuation-routing.md)
- [Declaration Signal Compatibility](./declaration-signal-compatibility.md)
- [Domain Capabilities](./README.md)
