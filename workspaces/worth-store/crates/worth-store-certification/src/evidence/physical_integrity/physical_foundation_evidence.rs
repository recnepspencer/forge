use worth_store_contracts::{RoadmapScope, StableArtifactId, StableDigest};
use worth_store_readiness::{
    FoundationalAdoptionDigest, FoundationalVocabularyAdoptionMap, PhysicalFoundationEvidenceField,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalFoundationEvidenceDenial {
    MissingField(PhysicalFoundationEvidenceField),
    DuplicateField(PhysicalFoundationEvidenceField),
    ArtifactDigestRequiresFoundationalProof,
}

#[derive(Debug)]
pub struct PhysicalFoundationEvidenceBundleBuilder {
    adoption: FoundationalVocabularyAdoptionMap,
    entries: Vec<PhysicalFoundationEvidenceEntry>,
}

impl PhysicalFoundationEvidenceBundleBuilder {
    pub fn with_report_evidence(
        mut self,
        field: PhysicalFoundationEvidenceField,
        artifact_id: StableArtifactId,
        digest: StableDigest,
    ) -> Result<Self, PhysicalFoundationEvidenceDenial> {
        if field == PhysicalFoundationEvidenceField::ArtifactDigest {
            return Err(PhysicalFoundationEvidenceDenial::ArtifactDigestRequiresFoundationalProof);
        }
        self.entries.push(PhysicalFoundationEvidenceEntry {
            field,
            artifact_id,
            identity: PhysicalFoundationEvidenceIdentity::StoreDigest(digest),
        });
        Ok(self)
    }

    pub fn with_canonical_artifact_digest(
        mut self,
        artifact_id: StableArtifactId,
        digest: FoundationalAdoptionDigest,
    ) -> Self {
        self.entries.push(PhysicalFoundationEvidenceEntry {
            field: PhysicalFoundationEvidenceField::ArtifactDigest,
            artifact_id,
            identity: PhysicalFoundationEvidenceIdentity::FoundationalAdoption(digest),
        });
        self
    }

    pub fn admit_without_byte_authority_promotion(
        self,
    ) -> Result<PhysicalFoundationEvidenceBundle, PhysicalFoundationEvidenceDenial> {
        reject_duplicate_evidence_fields(&self.entries)?;
        reject_missing_required_evidence_fields(self.adoption.required_evidence(), &self.entries)?;

        Ok(PhysicalFoundationEvidenceBundle {
            scope: self.adoption.scope(),
            adoption: self.adoption,
            entries: self.entries,
        })
    }
}

fn reject_duplicate_evidence_fields(
    entries: &[PhysicalFoundationEvidenceEntry],
) -> Result<(), PhysicalFoundationEvidenceDenial> {
    let mut seen = Vec::new();
    for entry in entries {
        if seen.contains(&entry.field) {
            return Err(PhysicalFoundationEvidenceDenial::DuplicateField(
                entry.field,
            ));
        }
        seen.push(entry.field);
    }
    Ok(())
}

fn reject_missing_required_evidence_fields(
    required_fields: &[PhysicalFoundationEvidenceField],
    entries: &[PhysicalFoundationEvidenceEntry],
) -> Result<(), PhysicalFoundationEvidenceDenial> {
    for required in required_fields {
        if !entries.iter().any(|entry| entry.field == *required) {
            return Err(PhysicalFoundationEvidenceDenial::MissingField(*required));
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct PhysicalFoundationEvidenceBundle {
    scope: RoadmapScope,
    adoption: FoundationalVocabularyAdoptionMap,
    entries: Vec<PhysicalFoundationEvidenceEntry>,
}

impl PhysicalFoundationEvidenceBundle {
    pub fn builder(
        adoption: FoundationalVocabularyAdoptionMap,
    ) -> PhysicalFoundationEvidenceBundleBuilder {
        PhysicalFoundationEvidenceBundleBuilder {
            adoption,
            entries: Vec::new(),
        }
    }

    pub const fn scope(&self) -> RoadmapScope {
        self.scope
    }

    pub const fn adoption(&self) -> &FoundationalVocabularyAdoptionMap {
        &self.adoption
    }

    pub fn entries(&self) -> &[PhysicalFoundationEvidenceEntry] {
        &self.entries
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalFoundationEvidenceEntry {
    field: PhysicalFoundationEvidenceField,
    artifact_id: StableArtifactId,
    identity: PhysicalFoundationEvidenceIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalFoundationEvidenceIdentity {
    StoreDigest(StableDigest),
    FoundationalAdoption(FoundationalAdoptionDigest),
}

impl PhysicalFoundationEvidenceEntry {
    pub const fn field(&self) -> PhysicalFoundationEvidenceField {
        self.field
    }

    pub const fn artifact_id(&self) -> &StableArtifactId {
        &self.artifact_id
    }

    pub const fn identity(&self) -> &PhysicalFoundationEvidenceIdentity {
        &self.identity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use worth_store_readiness::FoundationalVocabularyAdoptionMap;

    #[test]
    fn evidence_bundle_requires_every_declared_field() {
        let adoption = FoundationalVocabularyAdoptionMap::physical_format_all_public_lanes()
            .expect("all foundational lanes are available");

        let denial = PhysicalFoundationEvidenceBundle::builder(adoption)
            .admit_without_byte_authority_promotion()
            .expect_err("missing fields are rejected");

        assert_eq!(
            denial,
            PhysicalFoundationEvidenceDenial::MissingField(
                PhysicalFoundationEvidenceField::PhysicalLayoutReport
            )
        );
    }

    #[test]
    fn evidence_bundle_accepts_complete_physical_format_field_set() {
        let adoption = FoundationalVocabularyAdoptionMap::physical_format_all_public_lanes()
            .expect("all foundational lanes are available");
        let adoption_digest = adoption.proof_vocabulary().digest().clone();
        let mut builder = PhysicalFoundationEvidenceBundle::builder(adoption)
            .with_canonical_artifact_digest(
                StableArtifactId::new(PhysicalFoundationEvidenceField::ArtifactDigest.as_str())
                    .expect("static field id"),
                adoption_digest,
            );

        for field in PhysicalFoundationEvidenceField::required_for_physical_format() {
            if field != PhysicalFoundationEvidenceField::ArtifactDigest {
                builder = builder
                    .with_report_evidence(
                        field,
                        StableArtifactId::new(field.as_str()).expect("static field id"),
                        report_digest_for(field),
                    )
                    .expect("non-artifact evidence fields accept report digests");
            }
        }

        let bundle = builder
            .admit_without_byte_authority_promotion()
            .expect("complete fields are accepted");

        assert_eq!(
            bundle.entries().len(),
            PhysicalFoundationEvidenceField::required_for_physical_format().len()
        );

        for field in PhysicalFoundationEvidenceField::required_for_physical_format() {
            let matching_entries: Vec<_> = bundle
                .entries()
                .iter()
                .filter(|entry| entry.field() == field)
                .collect();
            assert_eq!(matching_entries.len(), 1);
            assert_eq!(matching_entries[0].artifact_id().as_str(), field.as_str());
        }
    }

    #[test]
    fn artifact_digest_rejects_raw_store_digest() {
        let adoption = FoundationalVocabularyAdoptionMap::physical_format_all_public_lanes()
            .expect("all foundational lanes are available");

        let denial = PhysicalFoundationEvidenceBundle::builder(adoption)
            .with_report_evidence(
                PhysicalFoundationEvidenceField::ArtifactDigest,
                StableArtifactId::new("artifact_digest").expect("static field id"),
                StableDigest::new("sha256:raw-lookalike").expect("static digest"),
            )
            .expect_err("artifact digest requires foundational proof");

        assert_eq!(
            denial,
            PhysicalFoundationEvidenceDenial::ArtifactDigestRequiresFoundationalProof
        );
    }

    #[test]
    fn evidence_bundle_rejects_duplicate_declared_field() {
        let adoption = FoundationalVocabularyAdoptionMap::physical_format_all_public_lanes()
            .expect("all foundational lanes are available");

        let denial = PhysicalFoundationEvidenceBundle::builder(adoption)
            .with_report_evidence(
                PhysicalFoundationEvidenceField::PhysicalLayoutReport,
                StableArtifactId::new("physical_layout_report").expect("static field id"),
                StableDigest::new("sha256:layout-first").expect("static digest"),
            )
            .expect("first report evidence is admitted")
            .with_report_evidence(
                PhysicalFoundationEvidenceField::PhysicalLayoutReport,
                StableArtifactId::new("physical_layout_report_conflict").expect("static field id"),
                StableDigest::new("sha256:layout-second").expect("static digest"),
            )
            .expect("duplicate evidence is rejected at bundle admission")
            .admit_without_byte_authority_promotion()
            .expect_err("duplicate evidence field is ambiguous");

        assert_eq!(
            denial,
            PhysicalFoundationEvidenceDenial::DuplicateField(
                PhysicalFoundationEvidenceField::PhysicalLayoutReport
            )
        );
    }

    fn report_digest_for(field: PhysicalFoundationEvidenceField) -> StableDigest {
        StableDigest::new(format!("sha256:phase-one-{}", field.as_str())).expect("static digest")
    }
}
