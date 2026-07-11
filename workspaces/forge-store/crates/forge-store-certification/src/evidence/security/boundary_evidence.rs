use forge_foundational::{
    attachment, prepare_boundary_evidence_attachment_bundle_for_canonical_basis,
    prepare_counter_backed_performance_receipt_for_canonical_basis, provenance, receipt,
    BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator,
    CanonicalBasisReadyArtifact, CanonicalizationRuleVersion,
    FoundationalBoundaryEvidenceAttachmentBundle, FoundationalBoundaryEvidenceAttachmentTargetKind,
    FoundationalBoundaryEvidenceFreshnessPosture,
    FoundationalBoundaryEvidenceMaterializationProfile,
    FoundationalBoundaryEvidenceReceiptBoundary, FoundationalBoundaryEvidenceSourceBasis,
    FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
};
use forge_proof::TransitionOutcome;
use forge_store_readiness::PhysicalFoundationEvidenceField;

use super::{
    S51CertificationCloseoutDenial, S51CertificationCloseoutInput, S51CloseoutCounterMatrix,
    S51CloseoutPerformanceReceipts,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S51CloseoutFoundationalLane {
    NativeAspectValues,
    Canonicalization,
    BoundaryArtifact,
    BoundaryEvidence,
    Profile,
    CounterBackedPerformance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S51CloseoutFoundationalBoundaryPackage {
    required_fields: [PhysicalFoundationEvidenceField; 6],
    attachment_bundle: FoundationalBoundaryEvidenceAttachmentBundle,
    materialized_bundle: FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
    performance_canonical_basis: CanonicalBasisReadyArtifact,
    boundary_canonical_basis: CanonicalBasisReadyArtifact,
    consumed_lower_store_evidence_rows: u64,
    native_aspect_evidence_rows: u64,
    receipt_counter_family_count: u64,
}

#[derive(Debug)]
pub struct S51CloseoutBoundaryEvidencePublication {
    package: S51CloseoutFoundationalBoundaryPackage,
}

impl S51CloseoutBoundaryEvidencePublication {
    pub(crate) fn from_input_and_counter_matrix(
        input: &S51CertificationCloseoutInput,
        counter_matrix: &S51CloseoutCounterMatrix,
        performance_receipts: &S51CloseoutPerformanceReceipts,
    ) -> Result<Self, S51CertificationCloseoutDenial> {
        let package = S51CloseoutFoundationalBoundaryPackage::from_counter_matrix_and_receipts(
            counter_matrix,
            performance_receipts,
        )?;
        Ok(Self { package })
    }

    pub const fn package(&self) -> &S51CloseoutFoundationalBoundaryPackage {
        &self.package
    }

    pub fn is_foundational_boundary_evidence(&self) -> bool {
        self.package.proves_required_security_scope_lanes()
    }
}

impl S51CloseoutFoundationalBoundaryPackage {
    const REQUIRED_LANES: [S51CloseoutFoundationalLane; 6] = [
        S51CloseoutFoundationalLane::NativeAspectValues,
        S51CloseoutFoundationalLane::Canonicalization,
        S51CloseoutFoundationalLane::BoundaryArtifact,
        S51CloseoutFoundationalLane::BoundaryEvidence,
        S51CloseoutFoundationalLane::Profile,
        S51CloseoutFoundationalLane::CounterBackedPerformance,
    ];

    pub(crate) fn from_counter_matrix_and_receipts(
        counter_matrix: &S51CloseoutCounterMatrix,
        performance_receipts: &S51CloseoutPerformanceReceipts,
    ) -> Result<Self, S51CertificationCloseoutDenial> {
        let consumed_lower_store_evidence_rows =
            counter_matrix.consumed_lower_store_evidence_rows();
        let native_aspect_evidence_rows = counter_matrix.lower_store_current_authority_checks();
        if native_aspect_evidence_rows != consumed_lower_store_evidence_rows {
            return Err(S51CertificationCloseoutDenial::CounterMismatch {
                counter: "store.s5_1.closeout.native_aspect_evidence_rows",
                expected: consumed_lower_store_evidence_rows,
                observed: native_aspect_evidence_rows,
            });
        }
        let locator = closeout_artifact_locator();
        let source_basis = FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(locator);
        let provenance = match provenance()
            .current(source_basis)
            .with_freshness(FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained)
        {
            TransitionOutcome::Success(provenance) => provenance,
            TransitionOutcome::Denied(denial) => return Err(denial.into()),
        };
        let completed_receipt = receipt()
            .publication(FoundationalBoundaryEvidenceReceiptBoundary::boundary_artifact(locator))
            .with_provenance(provenance.clone());
        let attachment_bundle = attachment()
            .for_boundary_artifact(locator)
            .with_provenance_attachment(provenance)
            .with_receipt_attachment(completed_receipt.completed_receipt().clone());
        let materialized_bundle = attachment_bundle.materialize_under(
            FoundationalBoundaryEvidenceMaterializationProfile::ElideDiagnostics,
        );
        let version = closeout_rule_version();
        let performance_canonical_basis =
            match prepare_counter_backed_performance_receipt_for_canonical_basis(
                version.clone(),
                performance_receipts.counter_backed_receipt(),
            ) {
                TransitionOutcome::Success(basis) => basis,
                TransitionOutcome::Denied(denial) => return Err(denial.into()),
            };
        let boundary_canonical_basis =
            match prepare_boundary_evidence_attachment_bundle_for_canonical_basis(
                version,
                &materialized_bundle,
            ) {
                TransitionOutcome::Success(basis) => basis,
                TransitionOutcome::Denied(denial) => return Err(denial.into()),
            };
        Ok(Self {
            required_fields: required_security_scope_foundational_fields(),
            attachment_bundle,
            materialized_bundle,
            performance_canonical_basis,
            boundary_canonical_basis,
            consumed_lower_store_evidence_rows,
            native_aspect_evidence_rows,
            receipt_counter_family_count: performance_receipts
                .counter_backed_receipt()
                .counter_rows()
                .len() as u64,
        })
    }

    pub const fn required_fields(&self) -> &[PhysicalFoundationEvidenceField; 6] {
        &self.required_fields
    }

    pub const fn attachment_bundle(&self) -> &FoundationalBoundaryEvidenceAttachmentBundle {
        &self.attachment_bundle
    }

    pub const fn materialized_bundle(
        &self,
    ) -> &FoundationalMaterializedBoundaryEvidenceAttachmentBundle {
        &self.materialized_bundle
    }

    pub const fn performance_canonical_basis(&self) -> &CanonicalBasisReadyArtifact {
        &self.performance_canonical_basis
    }

    pub const fn boundary_canonical_basis(&self) -> &CanonicalBasisReadyArtifact {
        &self.boundary_canonical_basis
    }

    pub const fn consumed_lower_store_evidence_rows(&self) -> u64 {
        self.consumed_lower_store_evidence_rows
    }

    pub const fn native_aspect_evidence_rows(&self) -> u64 {
        self.native_aspect_evidence_rows
    }

    pub const fn receipt_counter_family_count(&self) -> u64 {
        self.receipt_counter_family_count
    }

    pub fn covers_lane(&self, lane: S51CloseoutFoundationalLane) -> bool {
        match lane {
            S51CloseoutFoundationalLane::NativeAspectValues => {
                self.native_aspect_evidence_rows == self.consumed_lower_store_evidence_rows
                    && self.consumed_lower_store_evidence_rows > 0
            }
            S51CloseoutFoundationalLane::Canonicalization => {
                !self
                    .performance_canonical_basis
                    .payload()
                    .entries()
                    .is_empty()
                    && !self.boundary_canonical_basis.payload().entries().is_empty()
            }
            S51CloseoutFoundationalLane::BoundaryArtifact => {
                self.attachment_bundle.target_kind()
                    == FoundationalBoundaryEvidenceAttachmentTargetKind::BoundaryArtifact
            }
            S51CloseoutFoundationalLane::BoundaryEvidence => {
                self.attachment_bundle.provenance().is_some()
                    && self.attachment_bundle.receipt().is_some()
            }
            S51CloseoutFoundationalLane::Profile => {
                self.materialized_bundle.materialization_profile()
                    == FoundationalBoundaryEvidenceMaterializationProfile::ElideDiagnostics
            }
            S51CloseoutFoundationalLane::CounterBackedPerformance => {
                self.receipt_counter_family_count > 0
            }
        }
    }

    pub fn carries_field(&self, field: PhysicalFoundationEvidenceField) -> bool {
        self.required_fields.contains(&field)
    }

    pub fn proves_required_security_scope_lanes(&self) -> bool {
        Self::REQUIRED_LANES
            .into_iter()
            .all(|lane| self.covers_lane(lane))
            && required_security_scope_foundational_fields()
                .into_iter()
                .all(|field| self.carries_field(field))
            && self.native_aspect_evidence_rows == self.consumed_lower_store_evidence_rows
            && self.receipt_counter_family_count > 0
    }
}

fn required_security_scope_foundational_fields() -> [PhysicalFoundationEvidenceField; 6] {
    [
        PhysicalFoundationEvidenceField::FoundationalCanonicalBasisBundle,
        PhysicalFoundationEvidenceField::FoundationalProfileMaterializationPlan,
        PhysicalFoundationEvidenceField::FoundationalBoundaryEvidenceBundle,
        PhysicalFoundationEvidenceField::FoundationalProvenanceSupportTruth,
        PhysicalFoundationEvidenceField::FoundationalCounterBackedPerformanceReceipt,
        PhysicalFoundationEvidenceField::CounterSnapshot,
    ]
}

fn closeout_artifact_locator() -> BoundaryArtifactLocator {
    BoundaryArtifactLocator::new(
        BoundaryArtifactId::new(5_101),
        BoundaryArtifactField::Proofs,
    )
}

fn closeout_rule_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("store.s5_1.certification.closeout")
        .expect("static closeout canonicalization version is valid")
}
