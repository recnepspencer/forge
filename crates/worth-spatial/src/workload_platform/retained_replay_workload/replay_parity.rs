use crate::facade::planar_retained_facts::RetainedPlanarFactsReceipt;
use crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsReceipt;
use crate::planar_contracts::retained_planar_facts::RetainedPlanarHistoricalInspection;
use crate::spatial_compiled_product_family::{
    current_spatial_compiled_product_family_catalog, select_spatial_compiled_product_family,
    SpatialCompiledProductConsumer, SpatialCompiledProductFamilyIdentity,
};
use crate::workload_platform::compiled_product_admission::{
    admit_spatial_compiled_product_input, SpatialCompiledProductAdmissionErrorKind,
    SpatialCompiledProductAdmissionRequest, SpatialCompiledProductAdmissionWitness,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayParityAdmissionProvenance {
    source_authority_digest: String,
    locality_footprint_digest: String,
    evidence_support_digest: String,
    family_digest: String,
    authority_truth_identity_digest: String,
    equivalence_policy_identity_digest: String,
    prior_proof_identity_digest: Option<String>,
    compiled_product_identity_digest: String,
}

impl ReplayParityAdmissionProvenance {
    pub fn source_authority_digest(&self) -> &str {
        &self.source_authority_digest
    }

    pub fn locality_footprint_digest(&self) -> &str {
        &self.locality_footprint_digest
    }

    pub fn evidence_support_digest(&self) -> &str {
        &self.evidence_support_digest
    }

    pub fn family_digest(&self) -> &str {
        &self.family_digest
    }

    pub fn authority_truth_identity_digest(&self) -> &str {
        &self.authority_truth_identity_digest
    }

    pub fn equivalence_policy_identity_digest(&self) -> &str {
        &self.equivalence_policy_identity_digest
    }

    pub fn prior_proof_identity_digest(&self) -> Option<&str> {
        self.prior_proof_identity_digest.as_deref()
    }

    pub fn compiled_product_identity_digest(&self) -> &str {
        &self.compiled_product_identity_digest
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayParitySpatialAdmissionCause {
    BroadEvidenceScanDenied,
    FamilyCatalogDenied,
    WrongAuthorityBasis,
    WrongReceiptFamily,
    WrongSupportPosture,
}

impl From<SpatialCompiledProductAdmissionErrorKind> for ReplayParitySpatialAdmissionCause {
    fn from(value: SpatialCompiledProductAdmissionErrorKind) -> Self {
        match value {
            SpatialCompiledProductAdmissionErrorKind::BroadEvidenceScanDenied => {
                Self::BroadEvidenceScanDenied
            }
            SpatialCompiledProductAdmissionErrorKind::FamilyCatalogDenied => {
                Self::FamilyCatalogDenied
            }
            SpatialCompiledProductAdmissionErrorKind::WrongAuthorityBasis => {
                Self::WrongAuthorityBasis
            }
            SpatialCompiledProductAdmissionErrorKind::WrongReceiptFamily => {
                Self::WrongReceiptFamily
            }
            SpatialCompiledProductAdmissionErrorKind::WrongSupportPosture => {
                Self::WrongSupportPosture
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayParityErrorKind {
    SpatialAdmission,
    FamilySelection,
    IdentityLowering,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayParityError {
    kind: ReplayParityErrorKind,
    spatial_admission_cause: Option<ReplayParitySpatialAdmissionCause>,
    detail: String,
}

impl ReplayParityError {
    fn new(
        kind: ReplayParityErrorKind,
        spatial_admission_cause: Option<ReplayParitySpatialAdmissionCause>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            spatial_admission_cause,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> ReplayParityErrorKind {
        self.kind
    }

    pub fn spatial_admission_cause(&self) -> Option<ReplayParitySpatialAdmissionCause> {
        self.spatial_admission_cause
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayParityKind {
    LiveRetainedReplayedProjectionMatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayParityRow {
    kind: ReplayParityKind,
    parity_identity: String,
    human_parity: String,
}

impl ReplayParityRow {
    pub(crate) fn new(
        kind: ReplayParityKind,
        parity_identity: impl Into<String>,
        human_parity: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            parity_identity: parity_identity.into(),
            human_parity: human_parity.into(),
        }
    }

    pub fn kind(&self) -> ReplayParityKind {
        self.kind
    }

    pub fn parity_identity(&self) -> &str {
        &self.parity_identity
    }

    pub fn human_parity(&self) -> &str {
        &self.human_parity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayParityReport {
    admitted_consumer: SpatialCompiledProductConsumer,
    selected_family_identity: SpatialCompiledProductFamilyIdentity,
    admission_witness: SpatialCompiledProductAdmissionWitness,
    admission_provenance: ReplayParityAdmissionProvenance,
    rows: Vec<ReplayParityRow>,
}

impl ReplayParityReport {
    pub(crate) fn from_retained_projection_match(
        retained: &RetainedPlanarFactsReceipt,
        historical: &RetainedPlanarHistoricalInspection,
        projection: &ProjectionConsumedPlanarFactsReceipt,
    ) -> Self {
        Self::try_from_retained_projection_match(retained, historical, projection)
            .expect("retained replay admitted spatial family input")
    }

    pub(crate) fn try_from_retained_projection_match(
        retained: &RetainedPlanarFactsReceipt,
        historical: &RetainedPlanarHistoricalInspection,
        projection: &ProjectionConsumedPlanarFactsReceipt,
    ) -> Result<Self, ReplayParityError> {
        let catalog = current_spatial_compiled_product_family_catalog();
        let admitted = admit_spatial_compiled_product_input(
            &catalog,
            SpatialCompiledProductAdmissionRequest::for_retained_replay(
                historical, retained, projection,
            ),
        )
        .map_err(|error| {
            ReplayParityError::new(
                ReplayParityErrorKind::SpatialAdmission,
                Some(error.kind().into()),
                format!("{:?}", error.kind()),
            )
        })?;
        let selected =
            select_spatial_compiled_product_family(&catalog, admitted.family_admitted_input())
                .map_err(|error| {
                    ReplayParityError::new(
                        ReplayParityErrorKind::FamilySelection,
                        None,
                        format!("{:?}", error.kind()),
                    )
                })?;
        let parity_identity = selected.compile_product_identity().map_err(|error| {
            ReplayParityError::new(
                ReplayParityErrorKind::IdentityLowering,
                None,
                format!("{:?}", error.kind()),
            )
        })?;
        let admitted_input = selected.admitted_input();
        let compiled_product_identity_digest = parity_identity
            .compiled_product_identity()
            .identity_digest()
            .to_string();

        Ok(Self {
            admitted_consumer: SpatialCompiledProductConsumer::RetainedReplayParity,
            selected_family_identity: selected.declaration().identity(),
            admission_witness: admitted.witness().clone(),
            admission_provenance: ReplayParityAdmissionProvenance {
                source_authority_digest: admitted_input.source_authority_digest().to_string(),
                locality_footprint_digest: admitted_input.locality_footprint_digest().to_string(),
                evidence_support_digest: admitted_input.evidence_support_digest().to_string(),
                family_digest: parity_identity.family_digest().to_string(),
                authority_truth_identity_digest: parity_identity
                    .authority_truth_identity()
                    .identity_digest()
                    .to_string(),
                equivalence_policy_identity_digest: parity_identity
                    .equivalence_policy_identity()
                    .identity_digest()
                    .to_string(),
                prior_proof_identity_digest: parity_identity
                    .prior_proof_identity()
                    .map(|identity| identity.identity_digest().to_string()),
                compiled_product_identity_digest: compiled_product_identity_digest.clone(),
            },
            rows: vec![ReplayParityRow::new(
                ReplayParityKind::LiveRetainedReplayedProjectionMatch,
                compiled_product_identity_digest,
                "Live retained facts, retained replay, and projection-consumed facts agree.",
            )],
        })
    }

    pub fn rows(&self) -> &[ReplayParityRow] {
        &self.rows
    }

    pub fn admitted_consumer(&self) -> SpatialCompiledProductConsumer {
        self.admitted_consumer
    }

    pub fn selected_family_identity(&self) -> SpatialCompiledProductFamilyIdentity {
        self.selected_family_identity
    }

    pub(crate) fn admission_witness(&self) -> &SpatialCompiledProductAdmissionWitness {
        &self.admission_witness
    }

    pub fn admission_provenance(&self) -> &ReplayParityAdmissionProvenance {
        &self.admission_provenance
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ReplayParityErrorKind, ReplayParityKind, ReplayParityReport,
        ReplayParitySpatialAdmissionCause,
    };
    use crate::facade::spatial_compiled_product_family::SpatialCompiledProductConsumer;
    use crate::public_api_planar_projection_consumption::contract_subject::projection_consumed_planar_parts;
    use crate::public_api_planar_projection_consumption::runtime_handles::projection_consumption_handle;
    use crate::workload_platform::compiled_product_admission::{
        admit_spatial_compiled_product_input, SpatialCompiledProductAdmissionRequest,
    };
    use worth_spatial::facade::planar_projection_consumption::{
        ProjectionConsumedPlanarFacts, ProjectionConsumedPlanarFactsContracts,
    };

    #[test]
    fn replay_parity_is_rerun_stable_for_same_real_receipts() {
        run_with_real_workload_stack(|| {
            let (retained, projected) =
                retained_and_projected_receipts("phase-2-replay-parity-stable");

            let left = ReplayParityReport::from_retained_projection_match(
                &retained,
                &retained
                    .historical_replay(&retained.replay_subject())
                    .expect("historical replay"),
                &projected,
            );
            let right = ReplayParityReport::from_retained_projection_match(
                &retained,
                &retained
                    .historical_replay(&retained.replay_subject())
                    .expect("historical replay"),
                &projected,
            );

            assert_eq!(left.row_count(), 1);
            assert_eq!(
                left.rows()[0].kind(),
                ReplayParityKind::LiveRetainedReplayedProjectionMatch
            );
            assert_eq!(
                left.rows()[0].parity_identity(),
                right.rows()[0].parity_identity()
            );
            assert_eq!(left.admission_provenance(), right.admission_provenance());
            assert_eq!(left.admission_witness(), right.admission_witness());
            assert_eq!(
                left.admission_witness().admission_token(),
                right.admission_witness().admission_token()
            );
        });
    }

    #[test]
    fn replay_parity_identity_changes_with_retained_authority_or_projection_locality() {
        run_with_real_workload_stack(|| {
            let (retained, projected) =
                retained_and_projected_receipts("phase-2-replay-parity-baseline");
            let (foreign_retained, _) =
                retained_and_projected_receipts("phase-2-replay-parity-foreign-retained");
            let (_, projection_changed_receipt) =
                retained_and_projected_receipts_with_projection_world(
                    "phase-2-replay-parity-baseline",
                    "phase-2-replay-parity-foreign-projection",
                );

            let baseline_report = ReplayParityReport::from_retained_projection_match(
                &retained,
                &retained
                    .historical_replay(&retained.replay_subject())
                    .expect("historical replay"),
                &projected,
            );
            assert_eq!(baseline_report.row_count(), 1);

            let retained_changed = ReplayParityReport::try_from_retained_projection_match(
                &foreign_retained,
                &foreign_retained
                    .historical_replay(&foreign_retained.replay_subject())
                    .expect("foreign historical replay"),
                &projected,
            )
            .expect_err("foreign retained authority must deny at replay parity consumer boundary");
            assert_eq!(
                retained_changed.kind(),
                ReplayParityErrorKind::SpatialAdmission
            );
            assert!(
                retained_changed.spatial_admission_cause()
                    == Some(ReplayParitySpatialAdmissionCause::WrongAuthorityBasis),
                "foreign retained denial must preserve the typed spatial admission cause"
            );

            let projection_changed = ReplayParityReport::from_retained_projection_match(
                &retained,
                &retained
                    .historical_replay(&retained.replay_subject())
                    .expect("historical replay"),
                &projection_changed_receipt,
            );
            assert_ne!(
                baseline_report.rows()[0].parity_identity(),
                projection_changed.rows()[0].parity_identity()
            );
            assert_eq!(
                baseline_report
                    .admission_provenance()
                    .source_authority_digest(),
                projection_changed
                    .admission_provenance()
                    .source_authority_digest()
            );
            assert_ne!(
                baseline_report
                    .admission_provenance()
                    .locality_footprint_digest(),
                projection_changed
                    .admission_provenance()
                    .locality_footprint_digest()
            );
        });
    }

    #[test]
    fn replay_parity_report_carries_admission_derived_provenance() {
        run_with_real_workload_stack(|| {
            let (retained, projected) =
                retained_and_projected_receipts("phase-7-replay-parity-provenance");
            let historical = retained
                .historical_replay(&retained.replay_subject())
                .expect("historical replay");
            let expected = admit_spatial_compiled_product_input(
                &crate::spatial_compiled_product_family::current_spatial_compiled_product_family_catalog(),
                SpatialCompiledProductAdmissionRequest::for_retained_replay(
                    &historical,
                    &retained,
                    &projected,
                ),
            )
            .expect("retained replay admission");

            let report = ReplayParityReport::from_retained_projection_match(
                &retained,
                &historical,
                &projected,
            );

            assert_eq!(report.admission_witness(), expected.witness());
            assert_eq!(
                report.admission_witness().admission_token(),
                expected.witness().admission_token()
            );
            assert_eq!(
                report.admission_provenance().source_authority_digest(),
                historical.historical_digest()
            );
            assert_eq!(
                report.admission_witness().consumer(),
                SpatialCompiledProductConsumer::RetainedReplayParity
            );
            assert_eq!(
                report.admission_witness().family_identity(),
                report.selected_family_identity()
            );
            assert!(
                !report
                    .admission_witness()
                    .admission_token()
                    .trim()
                    .is_empty(),
                "replay parity provenance must carry a boundary-minted admission token"
            );
            assert_eq!(
                report.admission_provenance().locality_footprint_digest(),
                projected.projection_consumption_digest()
            );
            assert!(
                !report
                    .admission_provenance()
                    .family_digest()
                    .trim()
                    .is_empty(),
                "replay parity provenance must carry the lowered family digest"
            );
            assert_ne!(
                report.admission_provenance().authority_truth_identity_digest(),
                historical.historical_digest(),
                "replay parity provenance must carry lowered authority truth, not raw source digest"
            );
            assert!(
                !report
                    .admission_provenance()
                    .equivalence_policy_identity_digest()
                    .trim()
                    .is_empty(),
                "replay parity provenance must carry a real equivalence policy identity"
            );
            assert_eq!(
                report.admission_provenance().prior_proof_identity_digest(),
                None
            );
            assert_eq!(
                report
                    .admission_provenance()
                    .compiled_product_identity_digest(),
                report.rows()[0].parity_identity()
            );
            assert!(
                !report
                    .admission_provenance()
                    .evidence_support_digest()
                    .trim()
                    .is_empty(),
                "replay parity provenance must carry a real admitted support digest"
            );
        });
    }

    fn run_with_real_workload_stack(test: impl FnOnce() + Send + 'static) {
        std::thread::Builder::new()
            .name("retained-replay-parity-tests".to_string())
            .stack_size(16 * 1024 * 1024)
            .spawn(test)
            .expect("retained replay parity test thread")
            .join()
            .expect("retained replay parity test passed");
    }

    fn retained_and_projected_receipts(
        world: &'static str,
    ) -> (
        crate::facade::planar_retained_facts::RetainedPlanarFactsReceipt,
        crate::facade::planar_projection_consumption::ProjectionConsumedPlanarFactsReceipt,
    ) {
        retained_and_projected_receipts_with_projection_world(world, world)
    }

    fn retained_and_projected_receipts_with_projection_world(
        retained_world: &'static str,
        projection_world: &'static str,
    ) -> (
        crate::facade::planar_retained_facts::RetainedPlanarFactsReceipt,
        crate::facade::planar_projection_consumption::ProjectionConsumedPlanarFactsReceipt,
    ) {
        let parts = projection_consumed_planar_parts(retained_world);
        let retained = parts.retained;
        let projected = ProjectionConsumedPlanarFacts::from_retained_planar_facts(retained.clone())
            .consume_bundle_projection_receipts(parts.projections)
            .compile(&ProjectionConsumedPlanarFactsContracts::new(
                projection_consumption_handle(projection_world),
            ))
            .expect("projection-consumed plan")
            .consume()
            .expect("projection-consumed receipt");
        (retained, projected)
    }
}
