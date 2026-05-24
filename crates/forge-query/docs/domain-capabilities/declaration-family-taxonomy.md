# Declaration Family Taxonomy

## What This Feature Is

Declaration family taxonomy is the Query-owned classification layer over
downstream-owned family identity.

The important split is:

- your downstream domain owns the concrete family noun and semantic family key
- Query owns the closed vocabulary that classifies that family
- the family marker ties those together without making Query own your domain
  ontology

This keeps family meaning out of raw strings and host-local branch logic.

## Why You Use It

- classify declaration families without making Query own your domain nouns
- freeze authority, signal, and grouped posture before later gating and routing
- carry family posture forward as real retained declaration meaning
- let later capability surfaces derive structural witness availability from one
  sealed vocabulary

## Stable Entry Points

- `ForgeQueryDeclarationFamilyMarker`
- `ForgeQueryDeclarationFamilyTaxonomy`
- `ForgeQueryDeclarationPrimaryAuthorityFamily`
- `ForgeQueryGroupedDeclarationPosture`
- `ForgeQuerySignalCompatibilityPosture`
- `ForgeQueryDescriptiveOnlyAuthority`
- `ForgeQueryRelationalTruthAuthority`
- `ForgeQueryBridgeContinuationAuthority`
- `ForgeQueryMixedAuthority`
- `ForgeQuerySignalNotCompatiblePosture`
- `ForgeQuerySignalCompatiblePosture`
- `ForgeQuerySignalDeferredPosture`
- `ForgeQuerySingleOnlyGrouping`
- `ForgeQueryNeighborhoodCapableGrouping`
- `ForgeQueryBatchCapableGrouping`
- `ForgeQueryNeighborhoodAndBatchCapableGrouping`

Good to know:

- Query still does not ship concrete family nouns like `SplitEdge`
- the associated type tags are structural inputs for later witness gating
- the runtime taxonomy object is still the public inspection authority
- family markers may also declare required capability families and config
  sections

## Core Mental Model

Think of family identity as three connected layers:

1. **downstream semantic family identity**
   - "what family does this domain say this declaration belongs to?"
2. **Query taxonomy posture**
   - "what kind of family is it?"
3. **family-specific support requirements**
   - "what support/config posture must exist before this family can admit?"

The first layer stays domain-owned. The second and third layers stay
Query-owned.

For example, a geometry domain may define `SplitEdge` while Query classifies it
as:

- relational-truth
- signal-compatible
- neighborhood-capable
- requiring historical evaluation support

## How It Executes

1. your domain defines a family marker implementing
   `ForgeQueryDeclarationFamilyMarker<YourDomain>`
2. that marker supplies:
   - `semantic_family_key()`
   - associated posture tags
   - optional required capability families
   - optional required config sections
3. Query derives the runtime taxonomy object from those type-level tags
4. declaration inputs point at the marker through their associated `Family`
   type
5. canonical declarations retain the semantic family key and runtime taxonomy
6. later capability surfaces derive structural witness availability from the
   same family marker

The practical consequence is that taxonomy is not an advisory label. It is part
of canonical declaration meaning and later support behavior.

## Small Example

```rust
use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
};

struct GeometryDomain;
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
```

## Real Example

```rust
use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalCompatiblePosture,
};

struct GeometryDomain;

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

    fn required_config_sections() -> &'static [ForgeQueryConfigSectionFamily] {
        &[ForgeQueryConfigSectionFamily::Relational]
    }
}

struct SplitEdgeAtMidpoint {
    edge_ref: &'static str,
}

impl ForgeQueryDeclarationInput<GeometryDomain> for SplitEdgeAtMidpoint {
    type Family = SplitEdge;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
    }
}
```

What this example is showing:

- the family noun stays domain-owned
- taxonomy posture stays Query-owned and closed
- support requirements now live beside family posture instead of in ad hoc
  side tables

## How It Relates To Other Features

- [Canonical Domain Declarations](./canonical-domain-declarations.md) retain
  the family key and taxonomy in the canonical declaration artifact
- [Declaration Family Capability Matrix](./declaration-family-capability-matrix.md)
  turns family posture and requirements into support rows, checked admission,
  and witness availability
- grouped posture here is classification, not grouped execution
- signal posture here is classification, not signal execution

## Inspection And Debugging

Inspect the canonical declaration artifact to see:

- `declaration_family_key()`
- `declaration_taxonomy()`
- `declaration_primary_authority_family()`
- `declaration_grouped_posture()`
- `declaration_signal_compatibility()`

Inspect the admitted handle to see:

- `family_support::<F>()`
- `family_support_checked::<F>()`

Use those surfaces when two declarations seem similar but admit differently or
expose different follow-on family surfaces.

## Anti-Patterns

- making Query own your domain family nouns
- treating `signal` as a peer authority family instead of a compatibility
  posture
- treating grouped posture as if it already means grouped execution
- rebuilding family meaning from strings after Query has already frozen it
- putting family-specific support requirements in host-local branch logic

## Current Limits

Declaration family taxonomy does not yet decide:

- declaration legality inside one admitted operating world
- grouped execution semantics
- lower-authority route planning
- continuation participation beyond the retained family posture

It freezes the classification and support vocabulary those later phases
consume.

## Related Docs

- [Canonical Domain Declarations](./canonical-domain-declarations.md)
- [Configured Domain Handles](./configured-domain-handles.md)
- [Declaration Family Capability Matrix](./declaration-family-capability-matrix.md)
- [Platform Entry](./platform-entry.md)
