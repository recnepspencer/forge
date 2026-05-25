# Declaration Foundational Evidence

## What This Feature Is

Declaration foundational evidence is the Query-owned boundary that lowers
retained declaration truth into shared `forge-foundational` evidence artifacts.

The important split is:

- legality review proves structural declaration truth
- declaration progression proves admitted, deferred, denied, stale, rebind, or
  failed progression truth
- foundational evidence describes that retained truth through provenance,
  support, receipts, and attachment bundles without re-deriving declaration
  meaning

This is the first declaration-side feature whose job is descriptive publication
rather than stronger admission or stronger legality/progression proof.

## Why You Use It

- describe declaration truth through shared foundational evidence artifacts
- preserve admitted-world identity, declaration identity, legality truth, and
  progression truth in one inspectable wrapper
- keep denied, deferred, stale, rebind-required, and failed declaration paths
  first-class instead of success-only
- hand route planning, receipts, and envelopes one stable evidence
  language instead of local report dialects

## Stable Entry Points

- `ForgeQueryDeclarationFoundationalEvidenceInput`
- `ForgeQueryDeclarationFoundationalEvidenceClass`
- `ForgeQueryDeclarationFoundationalEvidenceChecked`
- `ForgeQueryDeclarationFoundationalEvidence`
- `ForgeQueryDeclarationFoundationalEvidenceDenial`
- `ForgeQueryAdmittedConfiguredDomainHandle::describe_foundational(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::describe_foundational_checked(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::describe_foundational_with_profile(...)`

Good to know:

- the admitted handle stays the entry surface because foundational description
  still depends on retained admitted-world proof
- foundational evidence starts from legality or progression truth, not from raw
  declaration input or canonical declarations alone
- Query owns the declaration-side lowering boundary; `forge-foundational` owns
  the provenance, support, receipt, and bundle primitives

## API Reference

Evidence-input constructors:

- `ForgeQueryDeclarationFoundationalEvidenceInput::legality_evidence(evidence)`
- `ForgeQueryDeclarationFoundationalEvidenceInput::legality_denial(denial)`
- `ForgeQueryDeclarationFoundationalEvidenceInput::legality_checked(checked)`
- `ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(progressed)`
- `ForgeQueryDeclarationFoundationalEvidenceInput::progression_checked(checked)`

Admitted-handle evidence entry points:

- `describe_foundational(subject) -> Result<ForgeQueryDeclarationFoundationalEvidence<D, I>, ForgeQueryDeclarationFoundationalEvidenceDenial<D, I>>`
- `describe_foundational_checked(subject) -> ForgeQueryDeclarationFoundationalEvidenceChecked<D, I>`
- `describe_foundational_with_profile(subject, profile) -> Result<ForgeQueryDeclarationFoundationalEvidence<D, I>, ForgeQueryDeclarationFoundationalEvidenceDenial<D, I>>`

Evidence classes:

- `ForgeQueryDeclarationFoundationalEvidenceClass::LegalityAdmitted`
- `ForgeQueryDeclarationFoundationalEvidenceClass::LegalityDenied`
- `ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionAdmitted`
- `ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionDeferred`
- `ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionDenied`
- `ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionStale`
- `ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionRebindRequired`
- `ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionFailed`

Checked evidence outcomes:

- `ForgeQueryDeclarationFoundationalEvidenceChecked::Described(ForgeQueryDeclarationFoundationalEvidence<D, I>)`
- `ForgeQueryDeclarationFoundationalEvidenceChecked::ConstructionDenied(ForgeQueryDeclarationFoundationalEvidenceDenial<D, I>)`

Foundational evidence inspection:

- `class() -> ForgeQueryDeclarationFoundationalEvidenceClass`
- `handle_identity_digest() -> &str`
- `operating_context_identity_digest() -> &str`
- `declaration_family_key() -> &'static str`
- `declaration_digest() -> &str`
- `support_digest() -> &str`
- `legality_digest() -> Option<&str>`
- `progression_digest() -> Option<&str>`
- `provenance() -> &FoundationalBoundaryEvidenceProvenanceArtifact`
- `planning_receipt() -> Option<&FoundationalBoundaryEvidencePlanningReceiptArtifact>`
- `receipt() -> Option<&FoundationalBoundaryEvidenceCompletedReceiptArtifact>`
- `support_attachment() -> Option<&FoundationalBoundaryEvidenceSupportAttachment>`
- `legality_contract() -> ForgeQueryDeclarationLegalityContract`
- `progression_contract() -> Option<ForgeQueryDeclarationProgressionContract>`
- `attachment_bundle() -> &FoundationalMaterializedBoundaryEvidenceAttachmentBundle`
- `attachment_bundle_digest() -> &CanonicalDerivedDigest`
- `materialization_profile() -> FoundationalBoundaryEvidenceMaterializationProfile`
- `subject() -> &ForgeQueryDeclarationFoundationalEvidenceInput<D, I>`

Construction-denial inspection:

- `class() -> ForgeQueryDeclarationFoundationalEvidenceClass`
- `reason() -> &str`

Construction-denial variants:

- `WrongAdmittedWorld`
- `Provenance`
- `Support`
- `AttachmentCanonicalBasis`
- `AttachmentDigest`

Materialization profiles:

- `FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness`
- `FoundationalBoundaryEvidenceMaterializationProfile::ElideDiagnostics`
- `FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics`

## Core Mental Model

Think of foundational evidence as a publication boundary over retained truth:

1. configured handles retain admitted-world identity
2. canonical declarations retain declaration identity and family posture
3. legality and progression retain stronger declaration truth
4. foundational evidence lowers that retained truth into one shared evidence
   language without reopening support, taxonomy, or progression meaning

If two declarations really mean the same thing at the retained-proof level, the
foundational evidence should converge. If their retained truth differs, the
foundational evidence should stay observably different.

## How It Executes

1. start from one legality or progression artifact
2. wrap it in `ForgeQueryDeclarationFoundationalEvidenceInput`
3. call one of the admitted-handle description entry points
4. Query injects the retained:
   - handle identity digest
   - operating-context identity digest
   - declaration digest
   - support digest
   - legality digest when present
   - progression digest when present
5. Query lowers the retained truth into:
   - foundational provenance
   - foundational support attachment
   - foundational planning or completed receipt
   - one materialized attachment bundle and bundle digest
6. Query returns one evidence wrapper or one typed construction denial

The ordinary lane uses
`FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness`.
Use `describe_foundational_with_profile(...)` when you want the same retained
truth with a leaner materialized bundle.

## Small Example

```rust
use forge_query::facade::{
    ForgeQueryDeclarationFoundationalEvidenceChecked,
    ForgeQueryDeclarationFoundationalEvidenceInput,
};

match handle.describe_foundational_checked(
    ForgeQueryDeclarationFoundationalEvidenceInput::progression_checked(
        handle.progress_declaration_checked(
            handle.declare_and_review(SplitEdgeAtMidpoint { edge_ref: "edge:42" })?,
        ),
    ),
) {
    ForgeQueryDeclarationFoundationalEvidenceChecked::Described(evidence) => {
        assert_eq!(evidence.declaration_family_key(), "split-edge");
    }
    ForgeQueryDeclarationFoundationalEvidenceChecked::ConstructionDenied(denial) => {
        panic!("unexpected evidence denial: {}", denial.reason());
    }
}
```

## Real Example

```rust
use forge_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;
use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationFoundationalEvidenceClass,
    ForgeQueryDeclarationFoundationalEvidenceInput, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationProgressionContract,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str { "worth.geometry" }
    fn display_name(&self) -> &'static str { "Worth Geometry" }

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

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn progression_contract(
        _handle_identity_digest: &str,
        _operating_context_identity_digest: &str,
    ) -> ForgeQueryDeclarationProgressionContract {
        ForgeQueryDeclarationProgressionContract::admitted_current()
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

let progressed = handle.declare_review_and_progress(SplitEdgeAtMidpoint {
    edge_ref: "edge:42",
})?;

let evidence = handle.describe_foundational(
    ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(progressed),
)?;

assert_eq!(
    evidence.class(),
    ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionAdmitted,
);
assert_eq!(evidence.declaration_family_key(), "split-edge");
assert_eq!(evidence.operating_context_identity_digest(), "geometry.collaborative");
assert!(evidence.progression_digest().is_some());
assert!(evidence.support_attachment().is_some());
assert!(evidence.receipt().is_some());

let lean = handle.describe_foundational_with_profile(
    ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
        handle.declare_review_and_progress(SplitEdgeAtMidpoint {
            edge_ref: "edge:42",
        })?,
    ),
    FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics,
)?;

assert_eq!(
    lean.materialization_profile(),
    FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics,
);
```

What this example is showing:

- the foundational-evidence lane starts from retained progression truth
- the evidence wrapper preserves admitted-world identity and declaration truth
- materialization profile changes the bundle shape without changing the retained
  declaration truth being described

## How It Relates To Other Features

- [Configured Domain Handles](./configured-domain-handles.md) retain the
  admitted-world identity evidence description must carry forward
- [Declaration Legality](./declaration-legality.md) supplies legality evidence
  and legality denials that can be described directly
- [Declaration Progression](./declaration-progression.md) supplies admitted,
  deferred, denied, stale, rebind, and failed progression truths
- [Declaration Route Plans](./declaration-route-plan.md) consumes admitted
  progression plus matching foundational evidence and turns that retained
  declaration truth into one explicit route set
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md)
  records the public crossing posture that follows from retained route truth
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
  publish the one self-describing public artifact that carries retained
  evidence, route truth, and receipt truth together

## Inspection And Debugging

Use these surfaces when reviewing foundational evidence:

- `describe_foundational_checked(...)`
- `evidence.class()`
- `evidence.handle_identity_digest()`
- `evidence.operating_context_identity_digest()`
- `evidence.declaration_digest()`
- `evidence.support_digest()`
- `evidence.legality_digest()`
- `evidence.progression_digest()`
- `evidence.provenance()`
- `evidence.planning_receipt()`
- `evidence.receipt()`
- `evidence.support_attachment()`
- `evidence.attachment_bundle_digest()`

Use them to answer:

- whether the declaration truth being described is legality-admitted, denied,
  progression-admitted, deferred, stale, rebind-required, or failed
- whether two equivalent declaration paths converge to the same foundational
  evidence bundle digest
- whether a difference came from world identity, legality truth, or progression
  outcome rather than from formatting or local callback behavior

## Anti-Patterns

- attempting foundational description from raw declaration input
- attempting foundational description from canonical declarations alone
- rebuilding foundational evidence meaning from family keys or payload shape
- treating foundational descriptive receipts as if they were later Query
  boundary-crossing receipts
- using stale evidence as if it were fresh admitted progression

## Current Limits

Declaration foundational evidence now publishes legality and progression truth
through shared foundational artifacts. It still does not decide:

- route planning
- lower-authority boundary crossing
- public Query boundary receipts
- public Query envelopes
- public Query relational truth routing
- public Query bridge continuation routing
- public Query signal compatibility classification
- continuity or diagnostic attachments

## Related Docs

- [Configured Domain Handles](./configured-domain-handles.md)
- [Declaration Legality](./declaration-legality.md)
- [Declaration Progression](./declaration-progression.md)
- [Declaration Route Plans](./declaration-route-plan.md)
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md)
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
- [Declaration Relational Truth Routing](./declaration-relational-truth-routing.md)
- [Declaration Bridge Continuation Routing](./declaration-bridge-continuation-routing.md)
- [Declaration Signal Compatibility](./declaration-signal-compatibility.md)
- [Canonical Domain Declarations](./canonical-domain-declarations.md)
- [Domain Capabilities](./README.md)
