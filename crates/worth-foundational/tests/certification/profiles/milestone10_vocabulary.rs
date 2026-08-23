use worth_foundational::{
    derive_foundational_profile_identity, foundational_profile_milestone10_readiness_report,
    foundational_profile_progression_authority, request_foundational_profile_set,
    FoundationalProfileResolutionFamily, FoundationalProfileResolutionLedger,
    FoundationalProfileResolutionRecord, FoundationalProfileResolutionRelation,
};
use worth_proof::TransitionOutcome;

use super::support::{admit_same_profile, profile};

#[test]
fn objective_and_activation_have_independent_resolution_records() {
    let requested = profile(
        worth_foundational::DiagnosticRichnessProfile::Standard,
        worth_foundational::SupportPostureProfile::SupportReady,
        worth_foundational::CompatibilityPostureProfile::NativeOnly,
        worth_foundational::AdmissionReadinessProfile::Admitted,
        worth_foundational::RetentionDeliveryProfile::Retained,
        worth_foundational::CertificationPostureProfile::Uncertified,
    );
    let admitted = worth_foundational::FoundationalProfileSet::new(
        worth_foundational::FoundationalProfileSetInput {
            diagnostic_richness: requested.diagnostic_richness(),
            support_posture: requested.support_posture(),
            compatibility_posture: requested.compatibility_posture(),
            admission_readiness: requested.admission_readiness(),
            retention_delivery: requested.retention_delivery(),
            certification_posture: requested.certification_posture(),
            execution_objective: worth_foundational::ExecutionObjectiveProfile::Throughput,
            observation_activation: worth_foundational::ObservationActivationProfile::OnDemand,
        },
    )
    .expect("objective and activation profile should compose");

    let mut resolutions = FoundationalProfileResolutionLedger::empty();
    resolutions
        .insert(FoundationalProfileResolutionRecord::new(
            FoundationalProfileResolutionFamily::ExecutionObjective,
            FoundationalProfileResolutionRelation::ObjectiveSelection,
            "select throughput objective",
        ))
        .expect("objective family is unique");
    resolutions
        .insert(FoundationalProfileResolutionRecord::new(
            FoundationalProfileResolutionFamily::ObservationActivation,
            FoundationalProfileResolutionRelation::ActivationSelection,
            "select on-demand activation",
        ))
        .expect("activation family is unique");

    let outcome = worth_foundational::admit_requested_foundational_profile_with_resolutions(
        request_foundational_profile_set(requested),
        admitted,
        resolutions,
        foundational_profile_progression_authority(),
    );
    let admitted = match outcome {
        TransitionOutcome::Success(artifact) => artifact,
        other => panic!("expected objective/activation admission, got {other:?}"),
    };
    assert_eq!(
        admitted.payload().requested_to_admitted_resolutions().len(),
        2
    );
    assert_eq!(
        admitted
            .payload()
            .requested_to_admitted_resolutions()
            .get(FoundationalProfileResolutionFamily::ExecutionObjective)
            .expect("objective record")
            .relation(),
        FoundationalProfileResolutionRelation::ObjectiveSelection
    );
}

#[test]
fn resolution_admission_reuses_monotonic_profile_progression_rules() {
    let requested = profile(
        worth_foundational::DiagnosticRichnessProfile::Standard,
        worth_foundational::SupportPostureProfile::SupportReady,
        worth_foundational::CompatibilityPostureProfile::NativeOnly,
        worth_foundational::AdmissionReadinessProfile::Admitted,
        worth_foundational::RetentionDeliveryProfile::Retained,
        worth_foundational::CertificationPostureProfile::Uncertified,
    );
    let illegally_widened = profile(
        worth_foundational::DiagnosticRichnessProfile::Forensic,
        worth_foundational::SupportPostureProfile::SupportReady,
        worth_foundational::CompatibilityPostureProfile::NativeOnly,
        worth_foundational::AdmissionReadinessProfile::Admitted,
        worth_foundational::RetentionDeliveryProfile::Retained,
        worth_foundational::CertificationPostureProfile::Uncertified,
    );
    let mut resolutions = FoundationalProfileResolutionLedger::empty();
    resolutions
        .insert(FoundationalProfileResolutionRecord::new(
            FoundationalProfileResolutionFamily::DiagnosticRichness,
            FoundationalProfileResolutionRelation::Narrowing,
            "forged widening must not become an admitted resolution",
        ))
        .expect("diagnostic family is unique");

    let outcome = worth_foundational::admit_requested_foundational_profile_with_resolutions(
        request_foundational_profile_set(requested),
        illegally_widened,
        resolutions,
        foundational_profile_progression_authority(),
    );
    assert_eq!(
        outcome,
        TransitionOutcome::Denied(
            worth_foundational::FoundationalProfileProgressionDenial::
                RequestedAndAdmittedProfilesMayOnlyNarrow,
        )
    );
}

#[test]
fn objective_and_activation_change_profile_identity_independently() {
    let balanced_continuous = profile(
        worth_foundational::DiagnosticRichnessProfile::Standard,
        worth_foundational::SupportPostureProfile::SupportReady,
        worth_foundational::CompatibilityPostureProfile::NativeOnly,
        worth_foundational::AdmissionReadinessProfile::Admitted,
        worth_foundational::RetentionDeliveryProfile::Retained,
        worth_foundational::CertificationPostureProfile::Uncertified,
    );
    let throughput_on_demand =
        worth_foundational::FoundationalProfileSet::new(worth_foundational_profile_input(
            balanced_continuous,
            worth_foundational::ExecutionObjectiveProfile::Throughput,
            worth_foundational::ObservationActivationProfile::OnDemand,
        ))
        .expect("objective and activation profile should compose");
    let first = admit_same_profile(balanced_continuous);
    let second = admit_same_profile(throughput_on_demand);
    assert_ne!(profile_identity(&first), profile_identity(&second));
}

fn profile_identity(
    admitted: &worth_foundational::AdmittedFoundationalProfileArtifact,
) -> worth_foundational::FoundationalProfileIdentity {
    match derive_foundational_profile_identity(
        worth_foundational::CanonicalizationRuleVersion::new("m10.identity")
            .expect("identity rule"),
        admitted,
    ) {
        TransitionOutcome::Success(identity) => identity,
        other => panic!("expected profile identity, got {other:?}"),
    }
}

fn worth_foundational_profile_input(
    profile: worth_foundational::FoundationalProfileSet,
    objective: worth_foundational::ExecutionObjectiveProfile,
    activation: worth_foundational::ObservationActivationProfile,
) -> worth_foundational::FoundationalProfileSetInput {
    worth_foundational::FoundationalProfileSetInput {
        diagnostic_richness: profile.diagnostic_richness(),
        support_posture: profile.support_posture(),
        compatibility_posture: profile.compatibility_posture(),
        admission_readiness: profile.admission_readiness(),
        retention_delivery: profile.retention_delivery(),
        certification_posture: profile.certification_posture(),
        execution_objective: objective,
        observation_activation: activation,
    }
}

#[test]
fn milestone10_readiness_names_all_five_phase_gates() {
    let report = foundational_profile_milestone10_readiness_report();
    assert!(report.passes_readiness_checklist());
    assert_eq!(
        report.phase_gates(),
        &[
            worth_foundational::FoundationalProfileMilestone10PhaseGate::BoundaryFreeze,
            worth_foundational::FoundationalProfileMilestone10PhaseGate::ObjectiveAndActivationVocabulary,
            worth_foundational::FoundationalProfileMilestone10PhaseGate::ObservationDispositionAndWorkDisclosure,
            worth_foundational::FoundationalProfileMilestone10PhaseGate::FoundationalFacade,
            worth_foundational::FoundationalProfileMilestone10PhaseGate::SignalPolicyCompilerHandoff,
        ]
    );
    assert_eq!(
        report.certified_surfaces(),
        &[
            "execution-objective-profile",
            "observation-activation-profile",
            "observation-disposition-and-absence",
            "optional-observation-work-disclosure",
            "signal-policy-compiler-handoff",
        ]
    );
    assert_eq!(
        report.runtime_assumptions(),
        &[
            "adopting runtimes own execution and observation session lifecycle",
            "adopting runtimes own Signal policy execution after compiler handoff",
            "proof progression remains worth-proof-owned",
        ]
    );
    assert_eq!(
        report.runtime_non_assumptions(),
        &[
            "throughput is not a correctness or durability level",
            "on-demand activation is not permission to erase authoritative identity",
            "Foundational does not own runtime counters or persistence",
        ]
    );
    assert_eq!(
        report.hostile_pressures(),
        &[
            "missing objective or activation family",
            "multiple changed profile families hidden by one record",
            "optional observation work claimed without active disposition",
            "throughput paired with continuous observation",
        ]
    );
    assert_eq!(
        report.store_handoff(),
        "Store owns durability; Foundational owns only shared profile and work meaning"
    );
}

#[allow(dead_code)]
fn _identity_helper_is_available_for_downstream_context_tests() {
    let admitted = admit_same_profile(profile(
        worth_foundational::DiagnosticRichnessProfile::Standard,
        worth_foundational::SupportPostureProfile::SupportReady,
        worth_foundational::CompatibilityPostureProfile::NativeOnly,
        worth_foundational::AdmissionReadinessProfile::Admitted,
        worth_foundational::RetentionDeliveryProfile::Retained,
        worth_foundational::CertificationPostureProfile::Uncertified,
    ));
    let _ = derive_foundational_profile_identity(
        worth_foundational::CanonicalizationRuleVersion::new("m10.test").expect("version"),
        &admitted,
    );
}
