use crate::BackendFamily;
use worth_store_contracts::RoadmapScope;
use worth_store_physical_format::InMemoryPhysicalFormatModelEvidence;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformGradeClaimWitness {
    backend_family: BackendFamily,
    scope: RoadmapScope,
}

impl PlatformGradeClaimWitness {
    pub fn from_facade_evidence(
        _evidence: &InMemoryPhysicalFormatModelEvidence,
    ) -> Result<Self, crate::ClaimPromotionRejection> {
        Err(crate::ClaimPromotionRejection::PhysicalFoundationReconstructionOpen)
    }

    pub const fn backend_family(&self) -> BackendFamily {
        self.backend_family
    }

    pub const fn scope(&self) -> RoadmapScope {
        self.scope
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use worth_store_contracts::{
        AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
    };
    use worth_store_physical_format::{
        InMemoryPhysicalFormatModel, InMemoryPhysicalFormatModelRequest, PhysicalGeneration,
        PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot, PhysicalSegmentId,
        PlatformPhysicalAppendRequest,
    };

    #[test]
    fn heap_facade_evidence_cannot_promote_during_physical_reconstruction() {
        let mut facade = InMemoryPhysicalFormatModel::start_empty_model(
            readiness(),
            InMemoryPhysicalFormatModelRequest::physical_format_canonical(),
        )
        .expect("facade opens from accepted readiness");
        let append = facade
            .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
                slot_cell(),
                b"claim",
            ))
            .expect("facade append");
        facade
            .page_access()
            .locate_record(append.reference())
            .expect("facade locate");
        facade.publish_physical_root().expect("facade root publish");
        let scan = facade.scan_physical_layout().expect("facade verifier scan");

        let denial = PlatformGradeClaimWitness::from_facade_evidence(&scan.platform_evidence())
            .expect_err("heap-only facade evidence cannot promote");

        assert_eq!(
            denial,
            crate::ClaimPromotionRejection::PhysicalFoundationReconstructionOpen
        );
    }

    #[test]
    fn platform_grade_witness_rejects_incomplete_facade_evidence() {
        let mut facade = InMemoryPhysicalFormatModel::start_empty_model(
            readiness(),
            InMemoryPhysicalFormatModelRequest::physical_format_canonical(),
        )
        .expect("facade opens from accepted readiness");
        facade
            .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
                slot_cell(),
                b"claim",
            ))
            .expect("facade append");
        facade.publish_physical_root().expect("facade root publish");
        let scan = facade.scan_physical_layout().expect("facade verifier scan");

        let rejection = PlatformGradeClaimWitness::from_facade_evidence(&scan.platform_evidence())
            .expect_err("read or locate evidence is mandatory");

        assert_eq!(
            rejection,
            crate::ClaimPromotionRejection::PhysicalFoundationReconstructionOpen
        );
    }

    fn readiness() -> AcceptedHandoffReadiness {
        AcceptedHandoffReadiness::from_foundational_handoff_artifacts(
            ROADMAP_2_S1_SCOPE,
            digest_set(),
        )
        .expect("S.1 readiness")
    }

    fn digest_set() -> HandoffEvidenceDigestSet {
        HandoffEvidenceDigestSet::new(
            digest("backend"),
            digest("deferred"),
            digest("harness"),
            digest("terms"),
            digest("audit"),
            digest("complexity"),
            digest("provenance"),
        )
    }

    fn digest(name: &str) -> StableDigest {
        StableDigest::new(format!("sha256:{name}")).expect("non-empty digest")
    }

    fn slot_cell() -> worth_store_physical_format::SlotGenerationCell {
        PhysicalGenerationAuthority::for_canonical_physical_format()
            .slot_cell(segment(1), page(1), slot(1))
            .with_slot_generation(generation(5))
    }

    fn segment(value: u64) -> PhysicalSegmentId {
        PhysicalSegmentId::from_raw(value).unwrap()
    }

    fn page(value: u64) -> PhysicalPageId {
        PhysicalPageId::from_raw(value).unwrap()
    }

    fn slot(value: u16) -> PhysicalRecordSlot {
        PhysicalRecordSlot::from_raw(value).unwrap()
    }

    fn generation(value: u64) -> PhysicalGeneration {
        PhysicalGeneration::from_raw(value).unwrap()
    }
}
