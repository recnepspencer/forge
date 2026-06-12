use std::collections::BTreeSet;

use worth_kernel::workload_composition::{BuiltWorkloadCatalogRecipe, WorkloadCatalog};
use worth_spatial::facade::mixed_surface_kill_box::{
    MixedSurfaceFamilyRun, MixedSurfaceKillBoxDenial, MixedSurfaceKillBoxOutcomeMatrix,
    MixedSurfaceKillBoxReceipt, MixedSurfaceKillBoxWorkload,
};
use worth_spatial::facade::surface_support::{SurfaceFamily, UnsupportedSurfaceSupportReasonCode};
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserResponseSource, WorthUserResponseWorkload,
};

pub(crate) struct MixedSurfaceKillBoxSubject {
    pub catalog: BuiltWorkloadCatalogRecipe,
    pub receipt: MixedSurfaceKillBoxReceipt,
    pub outcome_matrix: MixedSurfaceKillBoxOutcomeMatrix,
}

pub(crate) fn mixed_surface_kill_box_subject(stem: &str) -> MixedSurfaceKillBoxSubject {
    let catalog = WorkloadCatalog::mixed_surface_kill_box()
        .declared(format!("{stem} stable topology carrier"))
        .build()
        .expect("stable topology carrier must build through catalog");
    let receipt = MixedSurfaceKillBoxWorkload::for_bound_geometry(catalog.bound_geometry().clone())
        .declared(format!("{stem} mixed surface kill box"))
        .with_surface_family_matrix(SurfaceFamily::ALL)
        .certify()
        .expect("mixed surface kill box must certify complete family matrix");
    let outcome_matrix =
        MixedSurfaceKillBoxOutcomeMatrix::from_receipt(&receipt).expect("outcome matrix");

    MixedSurfaceKillBoxSubject {
        catalog,
        receipt,
        outcome_matrix,
    }
}

pub(crate) fn mixed_surface_kill_box_denial_for_family_matrix(
    stem: &str,
    families: impl IntoIterator<Item = SurfaceFamily>,
) -> MixedSurfaceKillBoxDenial {
    let catalog = WorkloadCatalog::mixed_surface_kill_box()
        .declared(format!("{stem} stable topology carrier"))
        .build()
        .expect("stable topology carrier must build through catalog");

    MixedSurfaceKillBoxWorkload::for_bound_geometry(catalog.bound_geometry().clone())
        .declared(format!("{stem} mixed surface kill box"))
        .with_surface_family_matrix(families)
        .certify()
        .expect_err("invalid mixed surface family matrix must deny before certification")
}

pub(crate) fn unsupported_runs(
    receipt: &MixedSurfaceKillBoxReceipt,
) -> Vec<&MixedSurfaceFamilyRun> {
    receipt.unsupported_family_runs().collect()
}

pub(crate) fn unsupported_digest_set(receipt: &MixedSurfaceKillBoxReceipt) -> BTreeSet<String> {
    unsupported_runs(receipt)
        .into_iter()
        .map(|run| run.support_evidence_digest().to_string())
        .collect()
}

pub(crate) fn unsupported_reason_set(receipt: &MixedSurfaceKillBoxReceipt) -> BTreeSet<String> {
    unsupported_runs(receipt)
        .into_iter()
        .map(|run| run.human_reason().to_string())
        .collect()
}

pub(crate) fn plane_receipt_smuggling_denials(
    receipt: &MixedSurfaceKillBoxReceipt,
) -> Vec<MixedSurfaceKillBoxDenial> {
    let plane = receipt.plane_control().expect("plane control");
    unsupported_runs(receipt)
        .into_iter()
        .map(|run| {
            run.attempt_with_plane_support_receipt(plane)
                .expect_err("non-plane family must reject plane support receipt")
        })
        .collect()
}

pub(crate) fn wrong_family_response_denial(
    receipt: &MixedSurfaceKillBoxReceipt,
) -> MixedSurfaceKillBoxDenial {
    let generated = receipt
        .run_for_family(SurfaceFamily::GeneratedFeature)
        .expect("generated feature run");
    let freeform = receipt
        .run_for_family(SurfaceFamily::Freeform)
        .expect("freeform run");
    generated
        .attempt_with_user_response(freeform)
        .expect_err("generated feature run must reject freeform response evidence")
}

pub(crate) fn generated_feature_smuggling_denial(
    receipt: &MixedSurfaceKillBoxReceipt,
) -> MixedSurfaceKillBoxDenial {
    receipt
        .attempt_generated_feature_partial_admission()
        .expect_err("generated feature must not partially admit through kill box")
}

pub(crate) fn kernel_summary_substitution_outcome() -> WorthUserOutcome {
    response_from_denial(MixedSurfaceKillBoxDenial::KernelSummarySubstitution)
}

pub(crate) fn missing_surface_support_outcome(family: SurfaceFamily) -> WorthUserOutcome {
    response_from_denial(MixedSurfaceKillBoxDenial::MissingSurfaceSupportEvidence { family })
}

pub(crate) fn assert_family_is_unsupported(run: &MixedSurfaceFamilyRun) {
    assert!(!run.is_acceptable_m7_input());
    assert!(run.attempt_readiness().is_err());
    assert_eq!(
        run.unsupported_reason_code(),
        Some(UnsupportedSurfaceSupportReasonCode::FamilyNotAdmitted)
    );
    assert!(!run.support_evidence_digest().is_empty());
    assert!(!run.user_response_digest().is_empty());
    assert_eq!(run.user_response_digest(), run.support_evidence_digest());
}

fn response_from_denial(denial: MixedSurfaceKillBoxDenial) -> WorthUserOutcome {
    WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_mixed_surface_kill_box_denial(&denial),
    )
    .declared("mixed surface kill box denial response")
    .respond()
    .expect("mixed surface denial response")
    .outcome()
    .clone()
}
