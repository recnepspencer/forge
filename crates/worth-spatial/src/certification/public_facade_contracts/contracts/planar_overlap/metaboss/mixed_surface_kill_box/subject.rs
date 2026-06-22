use std::collections::BTreeSet;

use topology::facade::{NmtTopologyScopeKind, NmtTopologyScopeSet};
use worth_kernel::workload_composition::{BuiltWorkloadCatalogRecipe, WorkloadCatalog};
use worth_spatial::facade::mixed_surface_kill_box::{
    MixedSurfaceFamilyRun, MixedSurfaceKillBoxDenial, MixedSurfaceKillBoxOutcomeMatrix,
    MixedSurfaceKillBoxReceipt, MixedSurfaceKillBoxWorkload,
};
use worth_spatial::facade::nmt_certification_context::{
    NmtBossOutcomeMatrixEvidence, NmtCertifiedScopeSet,
};
use worth_spatial::facade::surface_support::{SurfaceFamily, UnsupportedSurfaceSupportReasonCode};
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserResponseSource, WorthUserResponseWorkload,
};

pub(crate) struct MixedSurfaceKillBoxSubject {
    pub catalog: BuiltWorkloadCatalogRecipe,
    pub certified_scopes: NmtCertifiedScopeSet,
    pub receipt: MixedSurfaceKillBoxReceipt,
    pub outcome_matrix: MixedSurfaceKillBoxOutcomeMatrix,
}

pub(crate) struct MixedSurfaceKillBoxCloseoutEvidence {
    pub certified_scopes: NmtCertifiedScopeSet,
    pub matrix: NmtBossOutcomeMatrixEvidence,
}

pub(crate) fn mixed_surface_kill_box_subject(stem: &str) -> MixedSurfaceKillBoxSubject {
    let catalog = WorkloadCatalog::mixed_surface_kill_box()
        .declared(format!("{stem} stable topology carrier"))
        .build()
        .expect("stable topology carrier must build through catalog");
    let certified = certified_scope_set(&catalog);
    let scope = certified
        .single_scope(NmtTopologyScopeKind::OpenSheet)
        .expect("mixed surface kill box must expose one open sheet scope");
    let receipt =
        MixedSurfaceKillBoxWorkload::for_certified_scope(scope, catalog.bound_geometry().clone())
            .declared(format!("{stem} mixed surface kill box"))
            .with_surface_family_matrix(SurfaceFamily::ALL)
            .certify()
            .expect("mixed surface kill box must certify complete family matrix");
    assert_eq!(
        receipt.certified_scope_identity(),
        Some(scope.topology_scope().scope_identity())
    );
    let outcome_matrix =
        MixedSurfaceKillBoxOutcomeMatrix::from_receipt(&receipt).expect("outcome matrix");

    MixedSurfaceKillBoxSubject {
        catalog,
        certified_scopes: certified,
        receipt,
        outcome_matrix,
    }
}

fn certified_scope_set(catalog: &BuiltWorkloadCatalogRecipe) -> NmtCertifiedScopeSet {
    let topology = catalog
        .topology_construction()
        .expect("mixed surface catalog must expose topology construction");
    let scopes = NmtTopologyScopeSet::from_construction(topology)
        .expect("mixed surface NMT scopes must compile");
    NmtCertifiedScopeSet::from_platform_evidence(
        topology,
        catalog.workload().evidence_ledger(),
        catalog.bound_geometry(),
        catalog.projected_workload(),
        catalog.transform_receipts(),
        catalog
            .replay_receipts()
            .expect("mixed surface catalog must expose retained replay receipts"),
        scopes,
    )
    .compile()
    .expect("mixed surface certified scopes must compile")
}

pub(crate) fn mixed_surface_kill_box_denial_for_family_matrix(
    stem: &str,
    families: impl IntoIterator<Item = SurfaceFamily>,
) -> MixedSurfaceKillBoxDenial {
    let catalog = WorkloadCatalog::mixed_surface_kill_box()
        .declared(format!("{stem} stable topology carrier"))
        .build()
        .expect("stable topology carrier must build through catalog");
    let certified = certified_scope_set(&catalog);
    let scope = certified
        .single_scope(NmtTopologyScopeKind::OpenSheet)
        .expect("mixed surface invalid-matrix scope");

    MixedSurfaceKillBoxWorkload::for_certified_scope(scope, catalog.bound_geometry().clone())
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

pub(crate) fn mixed_surface_closeout_evidence(stem: &str) -> MixedSurfaceKillBoxCloseoutEvidence {
    let subject = mixed_surface_kill_box_subject(stem);
    let plane = subject.receipt.plane_control().expect("plane support run");
    let unsupported = subject
        .receipt
        .run_for_family(SurfaceFamily::Freeform)
        .expect("freeform unsupported run");
    let outcomes = vec![
        plane.user_outcome().clone(),
        unsupported.user_outcome().clone(),
        response_from_denial(wrong_family_response_denial(&subject.receipt)),
        response_from_denial(generated_feature_smuggling_denial(&subject.receipt)),
        missing_surface_support_outcome(SurfaceFamily::Freeform),
    ];
    MixedSurfaceKillBoxCloseoutEvidence {
        certified_scopes: subject.certified_scopes,
        matrix: NmtBossOutcomeMatrixEvidence::from_outcomes(outcomes),
    }
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
