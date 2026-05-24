# Declaration Family Capability Matrix

## What This Feature Is

Declaration family capability matrix is the Query-owned family support and
structural witness layer over canonical declaration families.

This is where Query decides two different things:

- which family-specific surfaces are structurally available from the family
  marker's type-level posture
- whether the admitted operating world can admit that family right now based on
  support and config posture

That is why the phase is hybrid. Some family behavior is type-level. Some is
support-dependent.

## Why You Use It

- inspect family support before canonicalization
- get typed checked-lane family admission instead of ad hoc denials
- keep structurally wrong witness surfaces absent at compile time
- let later phases consume typed witness wrappers instead of reopening taxonomy
  decisions

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

match handle.declare_checked(SplitEdgeAtMidpoint { edge_ref: "edge:42" }) {
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
struct SplitEdge;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for SplitEdge {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn required_capability_families() -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SplitEdgeAtMidpoint {
    edge_ref: &'static str,
}

impl ForgeQueryDeclarationInput<GeometryDomain> for SplitEdgeAtMidpoint {
    type Family = SplitEdge;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
    }
}

let query = ForgeQueryApplicationFacade::runtime_backed_default();
let handle = query
    .domain(GeometryDomain)
    .with_operating_context(CollaborativeWorld)
    .validate()?
    .admit()?;

let support = handle.family_support::<SplitEdge>();
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

let declaration = handle.declare(SplitEdgeAtMidpoint {
    edge_ref: "edge:42",
})?;

let truth = declaration.relational_truth();
let signal = declaration.signal_compatible();
let grouped = declaration.neighborhood_capable();

assert_eq!(truth.artifact().declaration_family_key(), "split-edge");
assert_eq!(signal.artifact().declaration_family_key(), "split-edge");
assert_eq!(grouped.artifact().declaration_family_key(), "split-edge");
```

What this example is showing:

- family support rows and declaration admission agree
- witness access is structural, not guessed later from strings
- Query stays domain-agnostic while still exposing family-shaped capability
  surfaces

## How It Relates To Other Features

- [Configured Domain Handles](./configured-domain-handles.md) provide the
  admitted support snapshot this matrix consumes
- [Declaration Family Taxonomy](./declaration-family-taxonomy.md) provides the
  family posture vocabulary this matrix turns into support rows and witness
  availability
- [Canonical Domain Declarations](./canonical-domain-declarations.md) provide
  the retained artifacts the witness wrappers point back to

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
- re-reading taxonomy values in later phases when a typed witness wrapper is
  already available
- expecting Query to enumerate every downstream family globally

## Current Limits

Family capability matrix does not yet decide:

- declaration legality
- lower-authority route planning
- grouped execution semantics
- continuation execution

It freezes the family support boundary and structural witness surface those
later phases depend on.

## Related Docs

- [Configured Domain Handles](./configured-domain-handles.md)
- [Canonical Domain Declarations](./canonical-domain-declarations.md)
- [Declaration Family Taxonomy](./declaration-family-taxonomy.md)
- [Domain Capabilities](./README.md)
