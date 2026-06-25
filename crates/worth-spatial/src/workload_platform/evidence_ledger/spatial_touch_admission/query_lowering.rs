mod counters;
mod denial;
mod gap;
mod selectors;

pub use counters::SpatialEvidenceQueryLoweringCounters;
pub use denial::{
    deny_copied_receipt_fields_as_spatial_query_lowering_authority,
    deny_query_descriptor_as_spatial_query_lowering_authority,
    deny_raw_row_as_spatial_query_lowering_authority,
    deny_topology_touched_basis_as_spatial_query_lowering_authority,
    SpatialEvidenceQueryLoweringDenial, SpatialEvidenceQueryLoweringDenialKind,
};
pub use gap::{SpatialEvidenceQueryGapKind, SpatialEvidenceQueryGapRow};

use forge_query::facade::runtime::{
    ForgeQueryGraphObligationOperatingWorldDescriptor, ForgeQueryGraphReadTouchShape,
    ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchReadVerb,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use self::{
    counters::SpatialEvidenceQueryLoweringCounters as QueryLoweringCounters,
    gap::declared_mutation_query_gap_rows,
    selectors::{
        query_operating_world_descriptor, spatial_query_aspect_paths, spatial_query_read_verbs,
    },
};
use super::{
    SpatialEvidenceLookupProduct, SpatialEvidenceLookupProductDigest,
    SpatialGeometryEvidenceTouchAuthority, SpatialGeometryEvidenceTouchDigest,
};
use crate::query_aspect_contract::aspect_touches_from_paths;
use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialEvidenceQueryTouchDescriptor {
    touch_descriptor: ForgeQueryGraphTouchDescriptor,
    operating_world: ForgeQueryGraphObligationOperatingWorldDescriptor,
    product_digest: SpatialEvidenceQueryTouchDescriptorDigest,
    spatial_touch_digest: SpatialGeometryEvidenceTouchDigest,
    lookup_product_digest: SpatialEvidenceLookupProductDigest,
    evidence_stage: WorkloadEvidenceStage,
    evidence_identity: String,
    stage_index_identity: String,
    collection: String,
    relation_kind: String,
    aspect_paths: Vec<String>,
    read_verbs: Vec<ForgeQueryGraphTouchReadVerb>,
    gap_rows: Vec<SpatialEvidenceQueryGapRow>,
    milestone_five_selection_claimed: bool,
    counters: SpatialEvidenceQueryLoweringCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialEvidenceQueryTouchDescriptorDigest(String);

impl SpatialGeometryEvidenceTouchAuthority {
    pub fn query_touch_descriptor(
        &self,
        lookup: &SpatialEvidenceLookupProduct,
    ) -> Result<SpatialEvidenceQueryTouchDescriptor, SpatialEvidenceQueryLoweringDenial> {
        SpatialEvidenceQueryTouchDescriptor::from_authority_and_lookup(self, lookup)
    }
}

impl SpatialEvidenceQueryTouchDescriptor {
    fn from_authority_and_lookup(
        authority: &SpatialGeometryEvidenceTouchAuthority,
        lookup: &SpatialEvidenceLookupProduct,
    ) -> Result<Self, SpatialEvidenceQueryLoweringDenial> {
        require_lookup_matches_authority(authority, lookup)?;

        let collection = selectors::SPATIAL_QUERY_COLLECTION.to_string();
        let relation_kind = selectors::SPATIAL_QUERY_RELATION_KIND.to_string();
        let aspect_paths = spatial_query_aspect_paths(authority, lookup);
        let read_verbs = spatial_query_read_verbs(authority);
        let touch_descriptor = ForgeQueryGraphTouchDescriptor::read_family_shape(
            collection.clone(),
            read_verbs.iter().copied(),
            ForgeQueryGraphReadTouchShape::new(aspect_touches_from_paths(&aspect_paths)),
        )
        .map_err(|denial| {
            SpatialEvidenceQueryLoweringDenial::query_descriptor_substitution(
                denial.kind().as_str(),
            )
        })?;
        let operating_world = query_operating_world_descriptor(authority.operating_world());
        let gap_rows = declared_mutation_query_gap_rows();
        let counters = QueryLoweringCounters::from_descriptors(&gap_rows);
        let product_digest = SpatialEvidenceQueryTouchDescriptorDigest::from_parts(
            authority,
            lookup.product_digest(),
            &touch_descriptor,
            &operating_world,
            &gap_rows,
        );

        Ok(Self {
            touch_descriptor,
            operating_world,
            product_digest,
            spatial_touch_digest: authority.digest().clone(),
            lookup_product_digest: lookup.product_digest().clone(),
            evidence_stage: lookup.evidence_stage(),
            evidence_identity: lookup.evidence_identity().to_string(),
            stage_index_identity: lookup.lookup_key().stage_index_identity().to_string(),
            collection,
            relation_kind,
            aspect_paths,
            read_verbs,
            gap_rows,
            milestone_five_selection_claimed: false,
            counters,
        })
    }

    pub fn touch_descriptor(&self) -> &ForgeQueryGraphTouchDescriptor {
        &self.touch_descriptor
    }

    pub fn operating_world(&self) -> &ForgeQueryGraphObligationOperatingWorldDescriptor {
        &self.operating_world
    }

    pub fn product_digest(&self) -> &SpatialEvidenceQueryTouchDescriptorDigest {
        &self.product_digest
    }

    pub fn spatial_touch_digest(&self) -> &SpatialGeometryEvidenceTouchDigest {
        &self.spatial_touch_digest
    }

    pub fn lookup_product_digest(&self) -> &SpatialEvidenceLookupProductDigest {
        &self.lookup_product_digest
    }

    pub fn evidence_stage(&self) -> WorkloadEvidenceStage {
        self.evidence_stage
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn stage_index_identity(&self) -> &str {
        &self.stage_index_identity
    }

    pub fn collection(&self) -> &str {
        &self.collection
    }

    pub fn relation_kind(&self) -> &str {
        &self.relation_kind
    }

    pub fn aspect_paths(&self) -> &[String] {
        &self.aspect_paths
    }

    pub fn read_verbs(&self) -> &[ForgeQueryGraphTouchReadVerb] {
        &self.read_verbs
    }

    pub fn gap_rows(&self) -> &[SpatialEvidenceQueryGapRow] {
        &self.gap_rows
    }

    pub fn counters(&self) -> SpatialEvidenceQueryLoweringCounters {
        self.counters
    }

    pub fn claims_milestone_five_selection_closeout(&self) -> bool {
        self.milestone_five_selection_claimed
    }
}

impl SpatialEvidenceQueryTouchDescriptorDigest {
    fn from_parts(
        authority: &SpatialGeometryEvidenceTouchAuthority,
        lookup_digest: &SpatialEvidenceLookupProductDigest,
        touch_descriptor: &ForgeQueryGraphTouchDescriptor,
        operating_world: &ForgeQueryGraphObligationOperatingWorldDescriptor,
        gap_rows: &[SpatialEvidenceQueryGapRow],
    ) -> Self {
        Self(truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "spatial-evidence-query-touch-descriptor".to_string(),
                format!("spatial-touch-digest:{}", authority.digest().as_str()),
                format!("lookup-product-digest:{}", lookup_digest.as_str()),
                format!(
                    "query-descriptor-digest:{}",
                    touch_descriptor.descriptor_digest()
                ),
                format!(
                    "operating-world-digest:{}",
                    operating_world.descriptor_digest()
                ),
                format!(
                    "gap-digests:{}",
                    gap_rows
                        .iter()
                        .map(SpatialEvidenceQueryGapRow::gap_digest)
                        .collect::<Vec<_>>()
                        .join("|")
                ),
            ],
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub fn lower_spatial_touch_authority_to_query_descriptor(
    authority: &SpatialGeometryEvidenceTouchAuthority,
    lookup: &SpatialEvidenceLookupProduct,
) -> Result<SpatialEvidenceQueryTouchDescriptor, SpatialEvidenceQueryLoweringDenial> {
    authority.query_touch_descriptor(lookup)
}

fn require_lookup_matches_authority(
    authority: &SpatialGeometryEvidenceTouchAuthority,
    lookup: &SpatialEvidenceLookupProduct,
) -> Result<(), SpatialEvidenceQueryLoweringDenial> {
    if lookup.product_digest().spatial_touch_digest() != authority.digest() {
        return Err(SpatialEvidenceQueryLoweringDenial::lookup_product_mismatch(
            authority.digest().as_str(),
            lookup.product_digest().spatial_touch_digest().as_str(),
        ));
    }
    Ok(())
}
