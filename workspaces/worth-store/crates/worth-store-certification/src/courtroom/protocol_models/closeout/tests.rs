use std::collections::BTreeSet;
use std::num::NonZeroU64;

use worth_foundational::{
    materialize_diagnostic_explanation_bundle, materialize_diagnostic_support_report,
    AdmissionReadinessProfile, BoundaryArtifactField, BoundaryArtifactId,
    CertificationPostureProfile, CompatibilityPostureProfile, DiagnosticRichnessProfile,
    ExecutionObjectiveProfile, FoundationalDiagnosticCounterSnapshot,
    FoundationalDiagnosticDeliveryClass, FoundationalDiagnosticExplanationInput,
    FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticPartiality,
    FoundationalDiagnosticSubject, FoundationalDiagnosticSupportClaimStrength,
    FoundationalDiagnosticSupportInput, FoundationalDiagnosticSurfaceAvailability,
    FoundationalProfileSet, FoundationalProfileSetInput, ObservationActivationProfile,
    RetentionDeliveryProfile, SupportPostureProfile,
};
use worth_store_aspect_native::{
    StoreDiagnosticExplanationBundleEvidence, StoreDiagnosticSupportReportEvidence,
    StorePhysicalBoundaryWitness,
};
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use worth_store_formal_models::assumptions::TornWriteAssumption;
use worth_store_formal_models::runner::ProtocolCheckBounds;
use worth_store_formal_models::{ProtocolFamily, ProtocolLivenessContract};
use worth_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet, BackendDurabilityProfile,
    BackendMediaAssumptionSet, BackendRebindTriggers, PhysicalBackendCapabilityAdmissionAuthority,
    PosixFileFsyncDirFsyncProfile, WindowsFlushFileBuffersProfile,
};

#[cfg(not(windows))]
type HostDurabilityProfile = PosixFileFsyncDirFsyncProfile;
#[cfg(windows)]
type HostDurabilityProfile = WindowsFlushFileBuffersProfile;
#[cfg(not(windows))]
type MismatchedDurabilityProfile = WindowsFlushFileBuffersProfile;
#[cfg(windows)]
type MismatchedDurabilityProfile = PosixFileFsyncDirFsyncProfile;

use super::legal_execution::structural_checked_protocol_fixture_for_closeout_tests;
use super::{
    adjudicate_protocol_law_closeout, CounterexampleDiagnosticEvidence, ExactOwnerMappingEvidence,
    ProtocolResidualRisk,
};
use crate::courtroom::protocol_models::mutants::structural_mutation_fixture_for_closeout_tests;

mod fixtures;
use fixtures::*;

#[test]
fn exact_closeout_matrix_binds_every_evidence_dimension_by_protocol() {
    let bounds = ProtocolCheckBounds::new(
        NonZeroU64::new(10_000).unwrap(),
        NonZeroU64::new(32).unwrap(),
    );
    let report = adjudicate_protocol_law_closeout(
        structural_checked_protocol_fixture_for_closeout_tests::<HostDurabilityProfile>(
            formal_model_crate_root(),
            bounds,
            &admitted_backend(),
        ),
        structural_mutation_fixture_for_closeout_tests(),
    )
    .expect("the exact catalog and ordinary executions compose");
    let protocols = report
        .rows()
        .iter()
        .map(|row| row.protocol())
        .collect::<BTreeSet<_>>();
    assert_eq!(protocols, BTreeSet::from(ProtocolFamily::all()));

    for row in report.rows() {
        let protocol = row.protocol();
        assert_eq!(row.checked_execution().protocol(), protocol);
        assert_eq!(row.model_contract().protocol(), protocol);
        assert!(!row.model_contract().finite_abstractions().is_empty());
        assert_eq!(
            row.model_contract().liveness(),
            ProtocolLivenessContract::SafetyOnlyNoFairnessAssumed
        );
        assert!(row.checked_execution().invocation().model_path().is_file());
        assert!(row
            .checked_execution()
            .invocation()
            .configuration_path()
            .is_file());
        assert!(row.checked_execution().statistics().distinct_states() > 0);
        assert_eq!(row.ordinary_execution().protocol(), protocol);
        assert!(!row.ordinary_execution().coverage_actions().is_empty());
        assert!(!row.ordinary_execution().legal_traces().is_empty());
        assert_eq!(
            row.ordinary_execution().legal_traces().len(),
            row.ordinary_execution().validation_receipts().len()
        );
        assert_eq!(
            row.backend_assumptions().profile().row().protocol(),
            protocol
        );
        assert_eq!(
            row.backend_assumptions().profile().row().torn_write(),
            TornWriteAssumption::TornPagePossible
        );
        assert_eq!(row.controlled_defect().mutant().protocol(), protocol);
        let replay = row.controlled_defect().physical_replay();
        assert_eq!(replay.mapped_transcript().protocol(), protocol);
        if protocol == ProtocolFamily::DurabilityRecovery {
            assert_eq!(
                replay.backend_profile(),
                Some(
                    row.backend_assumptions()
                        .profile()
                        .durability()
                        .runtime_profile()
                )
            );
        }
        assert!(row
            .controlled_defect()
            .localization()
            .counterexample()
            .states()
            .iter()
            .any(|state| state
                .valuation("mutantEdge")
                .is_some_and(|edge| edge.trim_matches('"') == replay.illegal_edge())));
        let shrunk = replay
            .shrink_preserving_guard()
            .expect("shrinking preserves the concrete counterexample guard");
        assert_eq!(shrunk.identity(), replay.identity());
        assert_eq!(shrunk.owner(), replay.owner());
        assert_eq!(shrunk.backend_profile(), replay.backend_profile());
        assert_eq!(shrunk.illegal_edge(), replay.illegal_edge());
        assert_eq!(shrunk.concrete_guard(), replay.concrete_guard());
        let shrink_trace = shrunk
            .schedule_shrink()
            .expect("the minimized replay carries the physical schedule shrink trace");
        assert!(shrunk.schedule_identity().is_some());
        assert_eq!(
            shrink_trace.failure_class(),
            worth_store_physical_certification::ScheduleFailureClass::CounterMismatch
        );
        assert_eq!(
            shrink_trace.oracle_verdict().verdict(),
            worth_store_physical_certification::OracleVerdictKind::Satisfied
        );
        assert!(!shrink_trace.minimized_steps().is_empty());
        assert!(
            shrunk.mapped_transcript().actions().len()
                <= replay.mapped_transcript().actions().len()
        );
        let coverage = match row.owner_mapping() {
            ExactOwnerMappingEvidence::ExactOwnerCaseCoverage(receipt)
            | ExactOwnerMappingEvidence::SharedFrontierComposition(receipt) => receipt,
        };
        assert_eq!(
            row.counters()
                .checked()
                .get(worth_store_formal_models::runner::ProtocolRunnerCounter::ReceiptEmission),
            coverage.ordinary_executed_cases()
        );
        for counter in [
            worth_store_formal_models::runner::ProtocolRunnerCounter::OwnerCasesDeclared,
            worth_store_formal_models::runner::ProtocolRunnerCounter::OwnerCasesExecuted,
            worth_store_formal_models::runner::ProtocolRunnerCounter::OwnerCasesMapped,
            worth_store_formal_models::runner::ProtocolRunnerCounter::TypedOutcomePosturesObserved,
        ] {
            assert_eq!(
                row.counters().checked().get(counter),
                coverage.ordinary_executed_cases()
            );
        }
        assert_eq!(row.counters().checked().identity().protocol(), protocol);
        assert_eq!(
            row.counters().checked().identity().bounds(),
            row.checked_execution().invocation().bounds()
        );
        assert_eq!(
            row.counters().checked().identity().backend_profile(),
            row.backend_assumptions()
                .profile()
                .durability()
                .runtime_profile()
        );
        assert!(
            row.counters().checked().get(
                worth_store_formal_models::runner::ProtocolRunnerCounter::InvariantChecksExecuted
            ) > 0
        );
        assert_eq!(
            row.counters()
                .checked()
                .get(worth_store_formal_models::runner::ProtocolRunnerCounter::StateExploration),
            row.checked_execution().statistics().distinct_states()
        );
        assert_eq!(
            row.counters().checked().get(
                worth_store_formal_models::runner::ProtocolRunnerCounter::TransitionExploration
            ),
            row.checked_execution().statistics().generated_states()
                - row.checked_execution().statistics().initial_states()
        );
        for counter in [
            worth_store_formal_models::runner::ProtocolRunnerCounter::OmissionClassification,
            worth_store_formal_models::runner::ProtocolRunnerCounter::MappingRejection,
            worth_store_formal_models::runner::ProtocolRunnerCounter::DeadlockDetection,
            worth_store_formal_models::runner::ProtocolRunnerCounter::BoundExhaustion,
            worth_store_formal_models::runner::ProtocolRunnerCounter::RuntimeObservationsRejected,
            worth_store_formal_models::runner::ProtocolRunnerCounter::OwnerCasesMissing,
            worth_store_formal_models::runner::ProtocolRunnerCounter::DuplicateMappings,
            worth_store_formal_models::runner::ProtocolRunnerCounter::NormalizationRejections,
            worth_store_formal_models::runner::ProtocolRunnerCounter::UnsupportedBackendMismatch,
        ] {
            assert_eq!(row.counters().checked().get(counter), 0);
        }
        assert_eq!(
            row.counters().controlled_defect().get(
                worth_store_formal_models::runner::ProtocolRunnerCounter::CounterexampleLocalization
            ),
            1
        );
        assert_eq!(
            row.counters().checked().get(
                worth_store_formal_models::runner::ProtocolRunnerCounter::CounterexamplesProduced
            ),
            0
        );
        assert_eq!(
            row.counters().controlled_defect().get(
                worth_store_formal_models::runner::ProtocolRunnerCounter::CounterexamplesProduced
            ),
            1
        );
        assert_ne!(
            row.counters()
                .checked()
                .identity()
                .artifacts()
                .configuration_sha256(),
            row.counters()
                .controlled_defect()
                .identity()
                .artifacts()
                .configuration_sha256()
        );
        let mutant = row.controlled_defect().mutant().invocation(bounds);
        assert!(mutant.model_path().is_file());
        assert!(mutant.configuration_path().is_file());
        assert_eq!(
            row.checked_operator_bindings().len(),
            row.ordinary_execution().coverage_actions().len(),
            "every executed projection must name a declared operator in the checked artifact"
        );
        assert!(row
            .checked_operator_bindings()
            .iter()
            .all(|binding| !binding.operator().is_empty()));
        match (protocol, row.owner_mapping()) {
            (
                ProtocolFamily::SharedFrontiers,
                ExactOwnerMappingEvidence::SharedFrontierComposition(receipt),
            ) => assert!(receipt.declared_owner_cases() > 0),
            (_, ExactOwnerMappingEvidence::ExactOwnerCaseCoverage(receipt)) => {
                assert_eq!(
                    receipt.declared_owner_cases(),
                    receipt.ordinary_executed_cases()
                );
                assert_eq!(
                    receipt.declared_owner_cases(),
                    receipt.mapped_model_actions()
                );
            }
            _ => panic!("owner mapping evidence did not match {protocol:?}"),
        }
    }
}
#[test]
fn closeout_names_every_residual_risk_instead_of_claiming_generic_proof() {
    let bounds = ProtocolCheckBounds::new(
        NonZeroU64::new(10_000).unwrap(),
        NonZeroU64::new(32).unwrap(),
    );
    let report = adjudicate_protocol_law_closeout(
        structural_checked_protocol_fixture_for_closeout_tests::<HostDurabilityProfile>(
            formal_model_crate_root(),
            bounds,
            &admitted_backend(),
        ),
        structural_mutation_fixture_for_closeout_tests(),
    )
    .unwrap();
    assert_eq!(
        report.residual_risks(),
        &BTreeSet::from([
            ProtocolResidualRisk::BoundedCheckingIsNotUnboundedProof,
            ProtocolResidualRisk::FairnessAndLivenessRemainUnclaimed,
            ProtocolResidualRisk::BackendClaimsRequireAdmittedRuntimeProfile,
            ProtocolResidualRisk::ReplicationProgressRequiresIntactDurableStore,
        ])
    );
}

#[test]
fn localized_counterexample_binds_to_support_and_explanation_evidence() {
    let bounds = ProtocolCheckBounds::new(
        NonZeroU64::new(10_000).unwrap(),
        NonZeroU64::new(32).unwrap(),
    );
    let report = adjudicate_protocol_law_closeout(
        structural_checked_protocol_fixture_for_closeout_tests::<HostDurabilityProfile>(
            formal_model_crate_root(),
            bounds,
            &admitted_backend(),
        ),
        structural_mutation_fixture_for_closeout_tests(),
    )
    .unwrap();
    let physical = physical_witness();
    let evidence = CounterexampleDiagnosticEvidence::bind(
        report.rows()[0].controlled_defect().clone(),
        StoreDiagnosticSupportReportEvidence::new(diagnostic_support_report(), physical),
        StoreDiagnosticExplanationBundleEvidence::new(diagnostic_explanation(), physical),
    )
    .unwrap();

    assert_eq!(
        evidence
            .controlled_defect()
            .localization()
            .failing_lane()
            .as_str(),
        report.rows()[0]
            .controlled_defect()
            .mutant()
            .certification_lane()
    );
    assert_eq!(
        evidence.support_report().physical_witness(),
        evidence.explanation_bundle().physical_witness()
    );
}

#[test]
fn closeout_rejects_a_checked_profile_not_executed_by_the_host_scenario() {
    let bounds = ProtocolCheckBounds::new(
        NonZeroU64::new(10_000).unwrap(),
        NonZeroU64::new(32).unwrap(),
    );
    let denial = adjudicate_protocol_law_closeout(
        structural_checked_protocol_fixture_for_closeout_tests::<MismatchedDurabilityProfile>(
            formal_model_crate_root(),
            bounds,
            &admitted_backend_for::<MismatchedDurabilityProfile>(),
        ),
        structural_mutation_fixture_for_closeout_tests(),
    )
    .unwrap_err();
    assert!(matches!(
        denial,
        super::ProtocolCloseoutDenial::BackendProfileMismatch { .. }
    ));
}
