use crate::{
    s2_readiness_facts::S2PhysicalSubstrateHandoffEvidence, S2PhysicalReadinessFacts,
    S2ReadinessDenial,
};
use worth_store_contracts::{
    RoadmapScope, S2PhysicalSubstrateReadinessSnapshot, ROADMAP_2_S1_SCOPE,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S2PhysicalSubstrateReadiness {
    scope: RoadmapScope,
    facts: S2PhysicalReadinessFacts,
    sealed: bool,
}

impl S2PhysicalSubstrateReadiness {
    pub(crate) fn from_s1_handoff_evidence(
        scope: RoadmapScope,
        evidence: S2PhysicalSubstrateHandoffEvidence,
    ) -> Result<Self, S2ReadinessDenial> {
        if scope != ROADMAP_2_S1_SCOPE {
            return Err(S2ReadinessDenial::new(
                crate::S2ReadinessDenialKind::WrongRoadmapScope,
            ));
        }
        Ok(Self {
            scope,
            facts: S2PhysicalReadinessFacts::from_handoff_evidence(evidence),
            sealed: true,
        })
    }

    pub const fn scope(&self) -> RoadmapScope {
        self.scope
    }

    pub const fn facts(&self) -> S2PhysicalReadinessFacts {
        self.facts
    }

    pub const fn is_sealed(&self) -> bool {
        self.sealed
    }

    pub const fn physical_substrate_snapshot(&self) -> S2PhysicalSubstrateReadinessSnapshot {
        S2PhysicalSubstrateReadinessSnapshot::from_exact_counts(
            self.sealed,
            self.facts.physical_reference_count(),
            self.facts.header_decode_witness_count(),
            self.facts.payload_admission_witness_count(),
            self.facts.manifest_layout_evidence_count(),
            self.facts.no_materialization_witness_count(),
            self.facts.counter_evidence_count(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        s2_readiness_facts::{
            S2PhysicalSubstrateEvidenceCounts, S2PhysicalSubstrateHandoffEvidence,
        },
        S2PhysicalSubstrateReadiness,
    };
    use worth_store_contracts::{RoadmapScope, ROADMAP_2_S1_SCOPE};
    use worth_store_physical_format::{
        PhysicalBinaryEncodingWitness, PhysicalFrameKind, PhysicalGeneration,
        PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalPageId,
        PhysicalPublicationState, PhysicalRecordSlot, PhysicalReference,
        PhysicalReferenceAuthority, PhysicalSegmentId, PHYSICAL_HEADER_LENGTH,
    };

    #[test]
    fn readiness_requires_s1_scope() {
        let evidence = complete_evidence();
        let denial = S2PhysicalSubstrateReadiness::from_s1_handoff_evidence(
            RoadmapScope::new("Roadmap 2", "S.0"),
            evidence,
        )
        .unwrap_err();

        assert_eq!(
            denial.kind(),
            crate::S2ReadinessDenialKind::WrongRoadmapScope
        );
    }

    #[test]
    fn readiness_exposes_handoff_fact_counts() {
        let readiness = S2PhysicalSubstrateReadiness::from_s1_handoff_evidence(
            ROADMAP_2_S1_SCOPE,
            complete_evidence(),
        )
        .unwrap();

        assert!(readiness.is_sealed());
        assert_eq!(readiness.facts().physical_reference_count(), 4);
        assert_eq!(readiness.facts().header_decode_witness_count(), 2);
        assert_eq!(readiness.facts().payload_admission_witness_count(), 2);
    }

    fn complete_evidence() -> S2PhysicalSubstrateHandoffEvidence {
        let first = witnessed_payload(3, b"abc");
        let second = witnessed_payload(4, b"def");
        let physical_references = [
            physical_reference(1, 1, 1, 3),
            physical_reference(1, 1, 2, 4),
            physical_reference(1, 1, 3, 5),
            physical_reference(1, 1, 4, 6),
        ];
        let header_decode_witnesses = [first.0, second.0];
        let payload_admission_witnesses = [first.1, second.1];
        S2PhysicalSubstrateHandoffEvidence::from_s1_physical_witnesses(
            &physical_references,
            &header_decode_witnesses,
            &payload_admission_witnesses,
            S2PhysicalSubstrateEvidenceCounts::from_s1_closeout_evidence(3, 1, 9),
        )
        .unwrap()
    }

    fn witnessed_payload(
        generation_value: u64,
        payload: &[u8],
    ) -> (
        worth_store_physical_format::PhysicalHeaderDecodeWitness,
        worth_store_physical_format::PhysicalPayloadViewAdmission<'static>,
    ) {
        let bytes = Box::leak(header_bytes(generation_value, payload).into_boxed_slice());
        let authority =
            PhysicalHeaderAuthority::s1(PhysicalBinaryEncodingWitness::s1_canonical().unwrap());
        let report = authority
            .decode_frame_header(
                validated_slot_reference(generation_value),
                bytes,
                PhysicalFrameKind::RecordFrame,
            )
            .unwrap();
        let payload = authority.payload_view(bytes, report.witness()).unwrap();
        (report.witness(), payload)
    }

    fn validated_slot_reference(
        generation_value: u64,
    ) -> worth_store_physical_format::PhysicalReferenceValidationWitness {
        let generations = PhysicalGenerationAuthority::s1();
        let references = PhysicalReferenceAuthority::s1();
        let cell = generations
            .slot_cell(segment(1), page(1), slot(generation_value as u16))
            .with_slot_generation(generation(generation_value));
        references
            .validate_page_slot(references.admit_page_slot(cell), cell)
            .unwrap()
    }

    fn physical_reference(
        segment_value: u64,
        page_value: u64,
        slot_value: u16,
        generation_value: u64,
    ) -> PhysicalReference {
        let cell = PhysicalGenerationAuthority::s1()
            .slot_cell(segment(segment_value), page(page_value), slot(slot_value))
            .with_slot_generation(generation(generation_value));
        PhysicalReferenceAuthority::s1()
            .admit_page_slot(cell)
            .reference()
    }

    fn header_bytes(generation_value: u64, payload: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
        bytes.push(PhysicalFrameKind::RecordFrame.tag());
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
        bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&generation_value.to_le_bytes());
        bytes.push(PhysicalPublicationState::Published.code());
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&0u64.to_le_bytes());
        bytes.extend_from_slice(payload);
        bytes
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
