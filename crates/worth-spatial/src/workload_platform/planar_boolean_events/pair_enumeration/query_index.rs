use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryOutcome,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalNotCompatiblePosture,
    ForgeQuerySingleOnlyGrouping,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::candidate_index::execute_aabb_sweep_candidate_index;
use super::counters::PlanarBooleanSegmentPairEnumerationCounters;
use super::denial::{
    PlanarBooleanSegmentPairEnumerationDenial, PlanarBooleanSegmentPairEnumerationDenialKind,
};
use super::product::{
    PlanarBooleanCandidateIndexFallbackPosture, PlanarBooleanCandidateIndexLifecycleOutcome,
    PlanarBooleanCandidateIndexStrategy, PlanarBooleanSegmentCandidateIndexProduct,
    PlanarBooleanSegmentCandidateIndexProductInput,
};
use crate::workload_platform::planar_boolean_events::segment_identity::PlanarBooleanCanonicalSegment;

pub(crate) const INDEX_STRATEGY: &str = "aabb-sweep-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
struct CandidateIndexQueryEvidence {
    declaration_digest: String,
    plan_digest: String,
    envelope_digest: String,
}

impl CandidateIndexQueryEvidence {
    pub(crate) fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub(crate) fn plan_digest(&self) -> &str {
        &self.plan_digest
    }

    pub(crate) fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }
}

pub(crate) fn query_candidate_index_product(
    canonical_segment_set_identity: &str,
    ordered_left: &[&PlanarBooleanCanonicalSegment],
    ordered_right: &[&PlanarBooleanCanonicalSegment],
    planned_counters: PlanarBooleanSegmentPairEnumerationCounters,
) -> Result<PlanarBooleanSegmentCandidateIndexProduct, PlanarBooleanSegmentPairEnumerationDenial> {
    let query_evidence = query_index_evidence(
        canonical_segment_set_identity,
        ordered_left.len(),
        ordered_right.len(),
    )?;
    let candidate_execution = execute_aabb_sweep_candidate_index(
        canonical_segment_set_identity,
        ordered_left,
        ordered_right,
        planned_counters,
    )?;
    let broad_phase_comparison_count = candidate_execution.broad_phase_comparison_count();
    let rows = candidate_execution.into_rows();
    let skipped_pair_count = planned_counters
        .expected_pair_breadth()
        .saturating_sub(rows.len());
    let counters = PlanarBooleanSegmentPairEnumerationCounters::from_index_counts(
        planned_counters.left_segment_count(),
        planned_counters.right_segment_count(),
        rows.len(),
        skipped_pair_count,
        rows.len(),
        skipped_pair_count,
    )
    .with_strategy_counts(rows.len(), broad_phase_comparison_count, 0, false);
    PlanarBooleanSegmentCandidateIndexProduct::new(PlanarBooleanSegmentCandidateIndexProductInput {
        canonical_segment_set_identity: canonical_segment_set_identity.to_string(),
        declaration_digest: query_evidence.declaration_digest().to_string(),
        plan_digest: query_evidence.plan_digest().to_string(),
        envelope_digest: query_evidence.envelope_digest().to_string(),
        strategy: PlanarBooleanCandidateIndexStrategy::AabbSweep,
        fallback_posture: PlanarBooleanCandidateIndexFallbackPosture::NotUsed,
        lifecycle_outcome: PlanarBooleanCandidateIndexLifecycleOutcome::Bound,
        counters,
        rows,
    })
}

fn query_index_evidence(
    canonical_segment_set_identity: &str,
    left_segment_count: usize,
    right_segment_count: usize,
) -> Result<CandidateIndexQueryEvidence, PlanarBooleanSegmentPairEnumerationDenial> {
    let entry = PlanarBooleanSegmentCandidateIndexEntry {
        canonical_segment_set_identity: canonical_segment_set_identity.to_string(),
        index_strategy: INDEX_STRATEGY.to_string(),
        left_segment_count,
        right_segment_count,
    };
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarBooleanSegmentCandidateIndexQueryDomain)
        .with_operating_context(PlanarBooleanSegmentCandidateIndexQueryWorld::new(
            canonical_segment_set_identity,
        ))
        .validate()
        .map_err(|_| {
            query_index_denial(
                canonical_segment_set_identity,
                left_segment_count,
                right_segment_count,
            )
        })?
        .admit()
        .map_err(|_| {
            query_index_denial(
                canonical_segment_set_identity,
                left_segment_count,
                right_segment_count,
            )
        })?;

    match handle.orchestrate_declaration_entry_outcome(entry) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => Ok(CandidateIndexQueryEvidence {
            declaration_digest: envelope.declaration_digest().to_string(),
            plan_digest: candidate_index_plan_digest(
                canonical_segment_set_identity,
                left_segment_count,
                right_segment_count,
            ),
            envelope_digest: format!("{:?}", envelope.envelope_digest()),
        }),
        ForgeQueryOrdinaryOutcome::Ambiguous(_)
        | ForgeQueryOrdinaryOutcome::AspectConflict(_)
        | ForgeQueryOrdinaryOutcome::AuthorityMismatch(_)
        | ForgeQueryOrdinaryOutcome::BasisMismatch(_)
        | ForgeQueryOrdinaryOutcome::Deferred(_)
        | ForgeQueryOrdinaryOutcome::Denied(_)
        | ForgeQueryOrdinaryOutcome::ExplicitNarrowingRequired(_)
        | ForgeQueryOrdinaryOutcome::Failed(_)
        | ForgeQueryOrdinaryOutcome::MissingRequiredAspect(_)
        | ForgeQueryOrdinaryOutcome::RebindRequired(_)
        | ForgeQueryOrdinaryOutcome::Refused(_)
        | ForgeQueryOrdinaryOutcome::Stale(_)
        | ForgeQueryOrdinaryOutcome::Unavailable(_)
        | ForgeQueryOrdinaryOutcome::Unsupported(_)
        | ForgeQueryOrdinaryOutcome::WrongHandle(_)
        | ForgeQueryOrdinaryOutcome::WrongWorld(_) => Err(query_index_denial(
            canonical_segment_set_identity,
            left_segment_count,
            right_segment_count,
        )),
    }
}

fn candidate_index_plan_digest(
    canonical_segment_set_identity: &str,
    left_segment_count: usize,
    right_segment_count: usize,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-segment-candidate-index-query-plan".to_string(),
            format!("canonical-segment-set:{canonical_segment_set_identity}"),
            format!("strategy:{INDEX_STRATEGY}"),
            format!("left-count:{left_segment_count}"),
            format!("right-count:{right_segment_count}"),
        ],
    )
}

fn query_index_denial(
    canonical_segment_set_identity: &str,
    left_segment_count: usize,
    right_segment_count: usize,
) -> PlanarBooleanSegmentPairEnumerationDenial {
    PlanarBooleanSegmentPairEnumerationDenial::new(
        PlanarBooleanSegmentPairEnumerationDenialKind::QueryIndexNotAdmitted,
        canonical_segment_set_identity,
        PlanarBooleanSegmentPairEnumerationCounters::new(
            left_segment_count,
            right_segment_count,
            0,
            left_segment_count.saturating_mul(right_segment_count),
        ),
        "segment-pair candidate planning requires an admitted Query-backed index declaration",
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PlanarBooleanSegmentCandidateIndexQueryDomain;

impl ForgeQueryDomainEntryMarker for PlanarBooleanSegmentCandidateIndexQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.spatial.planar_boolean_segment_candidate_index"
    }

    fn display_name(&self) -> &'static str {
        "WorthSpatialPlanarBooleanSegmentCandidateIndex"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PlanarBooleanSegmentCandidateIndexQueryWorld {
    canonical_segment_set_identity: String,
}

impl PlanarBooleanSegmentCandidateIndexQueryWorld {
    fn new(canonical_segment_set_identity: impl Into<String>) -> Self {
        Self {
            canonical_segment_set_identity: canonical_segment_set_identity.into(),
        }
    }
}

impl ForgeQueryDomainOperatingContext<PlanarBooleanSegmentCandidateIndexQueryDomain>
    for PlanarBooleanSegmentCandidateIndexQueryWorld
{
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!(
            "worth.spatial.planar_boolean_segment_candidate_index.{}",
            self.canonical_segment_set_identity
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PlanarBooleanSegmentCandidateIndexDeclarationFamily;

impl ForgeQueryDeclarationFamilyMarker<PlanarBooleanSegmentCandidateIndexQueryDomain>
    for PlanarBooleanSegmentCandidateIndexDeclarationFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "PlanarBooleanSegmentCandidateIndex"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &[
                "geometry.planar_boolean.segment_candidate_index.segment_set",
                "geometry.planar_boolean.segment_candidate_index.strategy",
                "geometry.planar_boolean.segment_candidate_index.left_count",
                "geometry.planar_boolean.segment_candidate_index.right_count",
            ],
            &["geometry.planar_boolean.segment_candidate_index.candidates"],
            &[],
            &[],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PlanarBooleanSegmentCandidateIndexEntry {
    canonical_segment_set_identity: String,
    index_strategy: String,
    left_segment_count: usize,
    right_segment_count: usize,
}

impl ForgeQueryDeclarationInput<PlanarBooleanSegmentCandidateIndexQueryDomain>
    for PlanarBooleanSegmentCandidateIndexEntry
{
    type Family = PlanarBooleanSegmentCandidateIndexDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.planar_boolean.segment_candidate_index.segment_set",
                &self.canonical_segment_set_identity,
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.planar_boolean.segment_candidate_index.strategy",
                &self.index_strategy,
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.planar_boolean.segment_candidate_index.left_count",
                self.left_segment_count.to_string(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "geometry.planar_boolean.segment_candidate_index.right_count",
                self.right_segment_count.to_string(),
            ),
        ]
    }
}
