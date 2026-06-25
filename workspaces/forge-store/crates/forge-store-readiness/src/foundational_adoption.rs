use crate::{
    FoundationalAdoptionDenial, FoundationalPublicLaneSet, PhysicalFoundationEvidenceField,
    ProofVocabularyAdoptionMap,
};
use forge_store_contracts::{RoadmapScope, ROADMAP_2_S1_SCOPE};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalAdoptionFamily {
    Canonicalization,
    Diagnostics,
    Profiles,
    BoundaryEvidence,
    ProvenanceReceipts,
    Performance,
}

impl FoundationalAdoptionFamily {
    pub const fn public_lane(self) -> &'static str {
        match self {
            Self::Canonicalization => "forge_foundational::canonicalization_api::lower_lane",
            Self::Diagnostics => "forge_foundational::facade::FoundationalDiagnostic*",
            Self::Profiles => "forge_foundational::profiles_api::lower_lane",
            Self::BoundaryEvidence => "forge_foundational::boundary_evidence_api::lower_lane",
            Self::ProvenanceReceipts => "forge_foundational::boundary_evidence_api::lower_lane",
            Self::Performance => "forge_foundational::performance_api::lower_lane",
        }
    }

    pub const fn canonical_locus(self) -> &'static str {
        match self {
            Self::Canonicalization => "foundational_adoption.canonicalization",
            Self::Diagnostics => "foundational_adoption.diagnostics",
            Self::Profiles => "foundational_adoption.profiles",
            Self::BoundaryEvidence => "foundational_adoption.boundary_evidence",
            Self::ProvenanceReceipts => "foundational_adoption.provenance_receipts",
            Self::Performance => "foundational_adoption.performance",
        }
    }

    pub const fn required_for_s1() -> [Self; 6] {
        [
            Self::Canonicalization,
            Self::Diagnostics,
            Self::Profiles,
            Self::BoundaryEvidence,
            Self::ProvenanceReceipts,
            Self::Performance,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalAdoptionStatus {
    AdoptedPublicLane,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalAdoptionRow {
    family: FoundationalAdoptionFamily,
    status: FoundationalAdoptionStatus,
    public_lane: &'static str,
}

impl FoundationalAdoptionRow {
    fn adopted(family: FoundationalAdoptionFamily) -> Self {
        Self {
            family,
            status: FoundationalAdoptionStatus::AdoptedPublicLane,
            public_lane: family.public_lane(),
        }
    }

    pub const fn family(&self) -> FoundationalAdoptionFamily {
        self.family
    }

    pub const fn status(&self) -> FoundationalAdoptionStatus {
        self.status
    }

    pub const fn public_lane(&self) -> &'static str {
        self.public_lane
    }
}

pub struct FoundationalVocabularyAdoptionMapBuilder {
    scope: RoadmapScope,
    families: Vec<FoundationalAdoptionFamily>,
}

impl FoundationalVocabularyAdoptionMapBuilder {
    pub fn adopt_public_lane(
        mut self,
        family: FoundationalAdoptionFamily,
    ) -> Result<Self, FoundationalAdoptionDenial> {
        if self.families.contains(&family) {
            return Err(FoundationalAdoptionDenial::DuplicateFamily(family));
        }
        self.families.push(family);
        Ok(self)
    }

    pub fn prove_with_foundational_public_lanes(
        self,
    ) -> Result<FoundationalVocabularyAdoptionMap, FoundationalAdoptionDenial> {
        if self.scope != ROADMAP_2_S1_SCOPE {
            return Err(FoundationalAdoptionDenial::WrongRoadmapScope);
        }

        for family in FoundationalAdoptionFamily::required_for_s1() {
            if !self.families.contains(&family) {
                return Err(FoundationalAdoptionDenial::MissingRequiredFamily(family));
            }
        }

        let rows: Vec<_> = self
            .families
            .into_iter()
            .map(FoundationalAdoptionRow::adopted)
            .collect();
        let proof_vocabulary = ProofVocabularyAdoptionMap::from_adoption_rows(&rows)?;

        Ok(FoundationalVocabularyAdoptionMap {
            scope: self.scope,
            rows,
            lane_set: FoundationalPublicLaneSet::from_public_foundational_apis(),
            proof_vocabulary,
            required_evidence: PhysicalFoundationEvidenceField::required_for_s1().to_vec(),
        })
    }
}

#[derive(Debug)]
pub struct FoundationalVocabularyAdoptionMap {
    scope: RoadmapScope,
    rows: Vec<FoundationalAdoptionRow>,
    lane_set: FoundationalPublicLaneSet,
    proof_vocabulary: ProofVocabularyAdoptionMap,
    required_evidence: Vec<PhysicalFoundationEvidenceField>,
}

impl FoundationalVocabularyAdoptionMap {
    pub fn builder(scope: RoadmapScope) -> FoundationalVocabularyAdoptionMapBuilder {
        FoundationalVocabularyAdoptionMapBuilder {
            scope,
            families: Vec::new(),
        }
    }

    pub fn s1_all_public_lanes() -> Result<Self, FoundationalAdoptionDenial> {
        let mut builder = Self::builder(ROADMAP_2_S1_SCOPE);
        for family in FoundationalAdoptionFamily::required_for_s1() {
            builder = builder.adopt_public_lane(family)?;
        }
        builder.prove_with_foundational_public_lanes()
    }

    pub const fn scope(&self) -> RoadmapScope {
        self.scope
    }

    pub fn rows(&self) -> &[FoundationalAdoptionRow] {
        &self.rows
    }

    pub fn required_evidence(&self) -> &[PhysicalFoundationEvidenceField] {
        &self.required_evidence
    }

    pub const fn public_lane_set(&self) -> &FoundationalPublicLaneSet {
        &self.lane_set
    }

    pub const fn proof_vocabulary(&self) -> &ProofVocabularyAdoptionMap {
        &self.proof_vocabulary
    }

    pub fn covers_family(&self, family: FoundationalAdoptionFamily) -> bool {
        self.rows.iter().any(|row| row.family == family)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_store_contracts::RoadmapScope;

    #[test]
    fn adoption_map_covers_every_required_family() {
        let map = FoundationalVocabularyAdoptionMap::s1_all_public_lanes()
            .expect("all required public lanes are declared");

        for family in FoundationalAdoptionFamily::required_for_s1() {
            assert!(map.covers_family(family));
        }
    }

    #[test]
    fn adoption_map_digest_converges_across_independent_construction_paths() {
        let first = FoundationalVocabularyAdoptionMap::s1_all_public_lanes()
            .expect("first construction path succeeds");
        let mut second_builder = FoundationalVocabularyAdoptionMap::builder(ROADMAP_2_S1_SCOPE);
        for family in [
            FoundationalAdoptionFamily::Performance,
            FoundationalAdoptionFamily::ProvenanceReceipts,
            FoundationalAdoptionFamily::BoundaryEvidence,
            FoundationalAdoptionFamily::Profiles,
            FoundationalAdoptionFamily::Diagnostics,
            FoundationalAdoptionFamily::Canonicalization,
        ] {
            second_builder = second_builder
                .adopt_public_lane(family)
                .expect("unique family is accepted");
        }
        let second = second_builder
            .prove_with_foundational_public_lanes()
            .expect("second construction path succeeds");

        assert_eq!(
            first
                .proof_vocabulary()
                .digest()
                .canonical_digest()
                .value()
                .bytes(),
            second
                .proof_vocabulary()
                .digest()
                .canonical_digest()
                .value()
                .bytes()
        );
        assert_eq!(first.proof_vocabulary().canonical_entry_count(), 6);
    }

    #[test]
    fn missing_family_is_typed_denial() {
        let denial = FoundationalVocabularyAdoptionMap::builder(ROADMAP_2_S1_SCOPE)
            .adopt_public_lane(FoundationalAdoptionFamily::Canonicalization)
            .expect("first family is accepted")
            .prove_with_foundational_public_lanes()
            .expect_err("incomplete adoption is denied");

        assert_eq!(
            denial,
            FoundationalAdoptionDenial::MissingRequiredFamily(
                FoundationalAdoptionFamily::Diagnostics
            )
        );
    }

    #[test]
    fn wrong_scope_is_typed_denial() {
        let denial =
            FoundationalVocabularyAdoptionMap::builder(RoadmapScope::new("Roadmap 2", "S.0"))
                .prove_with_foundational_public_lanes()
                .expect_err("wrong scope is rejected before adoption");

        assert_eq!(denial, FoundationalAdoptionDenial::WrongRoadmapScope);
    }
}
