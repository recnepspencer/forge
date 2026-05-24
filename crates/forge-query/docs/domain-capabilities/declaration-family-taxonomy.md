# Declaration Family Taxonomy

## What This Feature Is

Declaration family taxonomy is the Query-owned classification layer that sits
on top of downstream-owned declaration family identity.

The important split is:

- your downstream domain owns the family noun and semantic family key
- Query owns the closed taxonomy that classifies that family for later support,
  legality, routing, and continuation work

This keeps family meaning out of raw strings and host-local branch logic.

## Why You Use It

- classify declaration families without making Query own your domain ontology
- freeze authority posture before later gating and routing phases begin
- make grouped posture and signal compatibility explicit instead of ambient
- prevent cross-domain family-name collisions from becoming hidden semantics

## Stable Entry Points

- `ForgeQueryDeclarationFamilyMarker`
- `ForgeQueryDeclarationFamilyTaxonomy`
- `ForgeQueryDeclarationPrimaryAuthorityFamily`
- `ForgeQueryGroupedDeclarationPosture`
- `ForgeQuerySignalCompatibilityPosture`

Good to know:

- Query does not ship concrete family nouns like `SplitEdge`
- the family marker is downstream-owned
- the taxonomy is Query-owned and closed
- canonical declarations retain both the semantic family key and the taxonomy
  posture for later phases

## Core Mental Model

Think of family identity as two layers:

1. **downstream semantic family identity**
   - "what family does this domain say this declaration belongs to?"
2. **Query taxonomy posture**
   - "what kind of declaration family is that for later Query phases?"

Those are related, but they are not the same thing.

For example, a downstream geometry domain might define `SplitEdge`, while Query
classifies it as:

- `RelationalTruth`
- `NeighborhoodCapable`
- `Compatible` with later signal-oriented continuation

Query owns the second layer, not the first.

## How It Executes

1. your domain defines a family marker implementing
   `ForgeQueryDeclarationFamilyMarker<YourDomain>`
2. the family marker supplies:
   - `semantic_family_key()`
   - `taxonomy()`
3. your declaration input type points at that marker through its associated
   `Family` type
4. Query retains both the semantic family key and the taxonomy posture in the
   canonical declaration artifact
5. later phases consume that retained proof instead of rediscovering family
   meaning from strings

The important practical consequence is that family classification is not a
side table or advisory label. It is part of canonical declaration meaning.

## Small Example

```rust
use forge_query::facade::{
    ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationFamilyTaxonomy,
    ForgeQueryDeclarationPrimaryAuthorityFamily,
    ForgeQueryGroupedDeclarationPosture,
    ForgeQuerySignalCompatibilityPosture,
};

struct GeometryDomain;
struct SplitEdge;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for SplitEdge {
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
```

## Real Example

```rust
use forge_query::facade::{
    ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationFamilyTaxonomy,
    ForgeQueryDeclarationInput,
    ForgeQueryDeclarationPrimaryAuthorityFamily as PrimaryAuthority,
    ForgeQueryGroupedDeclarationPosture as GroupedPosture,
    ForgeQuerySignalCompatibilityPosture as SignalPosture,
};

struct GeometryDomain;

struct SplitEdge;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for SplitEdge {
    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn taxonomy() -> ForgeQueryDeclarationFamilyTaxonomy {
        ForgeQueryDeclarationFamilyTaxonomy::new(
            PrimaryAuthority::RelationalTruth,
            SignalPosture::Compatible,
            GroupedPosture::NeighborhoodCapable,
        )
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
- the taxonomy posture stays Query-owned
- later phases can inspect and gate on the taxonomy without stealing the
  domain's family ontology

## How It Relates To Other Features

- [Canonical Domain Declarations](./canonical-domain-declarations.md)
  retain the family key and taxonomy in the canonical declaration artifact
- later support/readiness, legality, and routing phases consume that retained
  taxonomy proof
- grouped posture here is classification only, not grouped execution
- signal compatibility here is classification only, not signal execution

## Inspection And Debugging

Inspect the canonical declaration artifact to see:

- `declaration_family_key()`
- `declaration_taxonomy()`
- `declaration_primary_authority_family()`
- `declaration_grouped_posture()`
- `declaration_signal_compatibility()`

Use those surfaces when two declarations seem similar but later support or
routing behavior differs.

## Anti-Patterns

- making Query own your domain family nouns
- treating `signal` as a peer authority family instead of a compatibility
  modifier
- treating grouped posture as if it already means grouped execution
- rebuilding family meaning from ad hoc strings after Query has already frozen
  the taxonomy
- using one coarse `mixed-authority` label as a substitute for later route
  planning detail

## Current Limits

Declaration family taxonomy does not yet decide:

- whether a family is admitted for a specific configured handle
- legality of a declaration in one admitted operating world
- grouped execution semantics
- lower-authority route planning
- continuation participation semantics beyond the retained taxonomy posture

It freezes the classification language that those later phases consume.

## Related Docs

- [Canonical Domain Declarations](./canonical-domain-declarations.md)
- [Configured Domain Handles](./configured-domain-handles.md)
- [Platform Entry](./platform-entry.md)
