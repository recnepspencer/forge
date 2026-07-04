use crate::facade::planar_retained_facts::RetainedPlanarFactsReceipt;
use crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsReceipt;
use crate::planar_contracts::retained_planar_facts::RetainedPlanarHistoricalInspection;
use crate::spatial_compiled_product_family::{
    current_spatial_compiled_product_family_catalog, select_spatial_compiled_product_family,
    SpatialCompiledProductConsumer,
};
use crate::workload_platform::compiled_product_admission::{
    admit_spatial_compiled_product_input, SpatialCompiledProductAdmissionRequest,
};
use crate::workload_platform::retained_replay_workload::{
    AdmittedRetainedReplayCapture, CapturedRetainedWorkload, ReplayParityAdmissionProvenance,
    ReplayParityError, ReplayParityErrorKind, ReplayParityKind, ReplayParityReport,
    ReplayParityRow, RetainedArtifactCaptureReceipt, UnsupportedReplayReasonCode,
    UnsupportedReplayWorkload,
};

pub fn build_retained_replay_parity_report(
    retained: &RetainedPlanarFactsReceipt,
    historical: &RetainedPlanarHistoricalInspection,
    projection: &ProjectionConsumedPlanarFactsReceipt,
) -> Result<ReplayParityReport, ReplayParityError> {
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

    Ok(ReplayParityReport::from_lowered(
        SpatialCompiledProductConsumer::RetainedReplayParity,
        selected.declaration().identity(),
        admitted.witness().clone(),
        ReplayParityAdmissionProvenance::new(
            admitted_input.source_authority_digest().to_string(),
            admitted_input.locality_footprint_digest().to_string(),
            admitted_input.evidence_support_digest().to_string(),
            parity_identity.family_digest().to_string(),
            parity_identity
                .authority_truth_identity()
                .identity_digest()
                .to_string(),
            parity_identity
                .equivalence_policy_identity()
                .identity_digest()
                .to_string(),
            parity_identity
                .prior_proof_identity()
                .map(|identity| identity.identity_digest().to_string()),
            compiled_product_identity_digest.clone(),
        ),
        vec![ReplayParityRow::new(
            ReplayParityKind::LiveRetainedReplayedProjectionMatch,
            compiled_product_identity_digest,
            "Live retained facts, retained replay, and projection-consumed facts agree.",
        )],
    ))
}

pub fn require_retained_capture_receipt(
    retained_capture_receipt: Option<RetainedArtifactCaptureReceipt>,
) -> Result<RetainedArtifactCaptureReceipt, UnsupportedReplayWorkload> {
    retained_capture_receipt.ok_or_else(|| {
        UnsupportedReplayWorkload::new(
            UnsupportedReplayReasonCode::MissingRetainedCaptureReceipt,
            "Retained replay workload requires a retained capture receipt from the shared cutover lane.",
        )
    })
}

pub fn admit_retained_replay_capture(
    captured_retained_workload: CapturedRetainedWorkload,
) -> AdmittedRetainedReplayCapture {
    AdmittedRetainedReplayCapture::from_captured_retained_workload(captured_retained_workload)
}
