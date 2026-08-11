use super::super::artifacts::BackendCapabilityMatrix;
use super::super::capability::{Roadmap2SequenceId, StoreBackendCapabilityTier};
use super::super::deferred::DeferredPhysicalGuaranteeMap;
use super::super::harness::{
    HarnessMaturityLevel, HarnessSubsystemMaturity, S1CompileTimeBoundaryFixture,
    S1ForbiddenShortcut,
};
use super::compile_time_boundary_rows::{
    compile_time_fixture_rows, S1CompileTimeBoundaryFixtureStatusRow, S1NonPlatformGradeDebtRow,
};
use super::handoff_validation::S0S1HandoffBuildRejection;

pub(super) struct HandoffRequirementsInput<'a> {
    pub(super) backend_matrix: &'a BackendCapabilityMatrix,
    pub(super) deferred_map: &'a DeferredPhysicalGuaranteeMap,
    pub(super) available_fixtures: &'a [S1CompileTimeBoundaryFixture],
}

pub(super) struct DerivedHandoffRequirements {
    pub(super) required_forbidden_shortcuts: Vec<S1ForbiddenShortcut>,
    pub(super) required_harness_subsystems: Vec<SequenceHarnessDependency>,
    pub(super) allowed_backend_candidates: Vec<String>,
    pub(super) legacy_backend_fences: Vec<String>,
    pub(super) compile_time_boundary_fixtures: Vec<S1CompileTimeBoundaryFixtureStatusRow>,
    pub(super) non_platform_grade_debt_rows: Vec<S1NonPlatformGradeDebtRow>,
    pub(super) blocking_predicates: Vec<S1BlockingPredicateRow>,
}

pub(super) fn derive_handoff_requirements(
    inputs: HandoffRequirementsInput<'_>,
) -> Result<DerivedHandoffRequirements, S0S1HandoffBuildRejection> {
    let required_forbidden_shortcuts = required_forbidden_shortcuts();
    let required_harness_subsystems = required_harness_subsystems()?;
    let allowed_backend_candidates = allowed_backend_candidates(inputs.backend_matrix);
    if allowed_backend_candidates.is_empty() {
        return Err(S0S1HandoffBuildRejection::MissingAllowedBackendCandidate);
    }
    let legacy_backend_fences = legacy_backend_fences(inputs.backend_matrix);
    let compile_time_boundary_fixtures = compile_time_fixture_rows(inputs.available_fixtures);
    let non_platform_grade_debt_rows = non_platform_grade_debt_rows(inputs.backend_matrix);
    let blocking_predicates = blocking_predicates(inputs.deferred_map);
    Ok(DerivedHandoffRequirements {
        required_forbidden_shortcuts,
        required_harness_subsystems,
        allowed_backend_candidates,
        legacy_backend_fences,
        compile_time_boundary_fixtures,
        non_platform_grade_debt_rows,
        blocking_predicates,
    })
}
use super::s1_blocking_predicate::{
    S1BlockingPredicate, S1BlockingPredicateRow, S1BlockingPredicateStatus,
};
use super::sequence_harness_dependency::SequenceHarnessDependency;

pub(super) fn required_forbidden_shortcuts() -> Vec<S1ForbiddenShortcut> {
    vec![
        S1ForbiddenShortcut::OverclaimedPhysicalPosture,
        S1ForbiddenShortcut::BackendTierMismatch,
        S1ForbiddenShortcut::UnmappedDeferredGuarantee,
        S1ForbiddenShortcut::MissingMilestonePhysicalStatusRow,
    ]
}

pub(super) fn required_harness_subsystems(
) -> Result<Vec<SequenceHarnessDependency>, S0S1HandoffBuildRejection> {
    Ok(vec![
        SequenceHarnessDependency::new(
            Roadmap2SequenceId::new("S1")
                .map_err(|_| S0S1HandoffBuildRejection::EmptyRequiredField)?,
            HarnessSubsystemMaturity::TerminologyClaimGate,
            HarnessMaturityLevel::Exists,
        ),
        SequenceHarnessDependency::new(
            Roadmap2SequenceId::new("S1")
                .map_err(|_| S0S1HandoffBuildRejection::EmptyRequiredField)?,
            HarnessSubsystemMaturity::DeferredGuaranteeValidation,
            HarnessMaturityLevel::Exists,
        ),
        SequenceHarnessDependency::new(
            Roadmap2SequenceId::new("S1")
                .map_err(|_| S0S1HandoffBuildRejection::EmptyRequiredField)?,
            HarnessSubsystemMaturity::CompileTimeBoundaryFixtures,
            HarnessMaturityLevel::Exists,
        ),
    ])
}

pub(super) fn allowed_backend_candidates(matrix: &BackendCapabilityMatrix) -> Vec<String> {
    matrix
        .rows()
        .iter()
        .filter(|row| {
            matches!(
                row.capability_tier(),
                StoreBackendCapabilityTier::PhysicalFoundation
                    | StoreBackendCapabilityTier::PlatformGrade
            )
        })
        .map(|row| row.subject_path_or_symbol().to_string())
        .collect::<Vec<_>>()
}

pub(super) fn legacy_backend_fences(matrix: &BackendCapabilityMatrix) -> Vec<String> {
    matrix
        .rows()
        .iter()
        .filter(|row| {
            matches!(
                row.capability_tier(),
                StoreBackendCapabilityTier::Bootstrap
                    | StoreBackendCapabilityTier::SemanticCertification
                    | StoreBackendCapabilityTier::Compatibility
            )
        })
        .map(|row| row.subject_path_or_symbol().to_string())
        .collect::<Vec<_>>()
}

pub(super) fn non_platform_grade_debt_rows(
    matrix: &BackendCapabilityMatrix,
) -> Vec<S1NonPlatformGradeDebtRow> {
    matrix
        .rows()
        .iter()
        .filter(|row| row.capability_tier() != StoreBackendCapabilityTier::PlatformGrade)
        .filter(|row| !row.deferred_s_sequences().is_empty())
        .map(|row| S1NonPlatformGradeDebtRow {
            subject: row.subject_path_or_symbol().to_string(),
            deferred_s_sequences: row.deferred_s_sequences().to_vec(),
            required_wording:
                "Legal only as explicit non-platform-grade debt until Roadmap 2 closes."
                    .to_string(),
        })
        .collect::<Vec<_>>()
}

pub(super) fn blocking_predicates(
    deferred_map: &DeferredPhysicalGuaranteeMap,
) -> Vec<S1BlockingPredicateRow> {
    vec![
        S1BlockingPredicateRow {
            predicate: S1BlockingPredicate::MissingBackendTierMatrix,
            status: S1BlockingPredicateStatus::Satisfied,
        },
        S1BlockingPredicateRow {
            predicate: S1BlockingPredicate::MissingDeferredGuaranteeMap,
            status: if deferred_map.rows().is_empty() {
                S1BlockingPredicateStatus::Blocking
            } else {
                S1BlockingPredicateStatus::Satisfied
            },
        },
        S1BlockingPredicateRow {
            predicate: S1BlockingPredicate::MissingTerminologyScanDigest,
            status: S1BlockingPredicateStatus::Satisfied,
        },
        S1BlockingPredicateRow {
            predicate: S1BlockingPredicate::MissingForbiddenShortcutList,
            status: S1BlockingPredicateStatus::Satisfied,
        },
        S1BlockingPredicateRow {
            predicate: S1BlockingPredicate::MissingHarnessReadinessRows,
            status: S1BlockingPredicateStatus::Satisfied,
        },
        S1BlockingPredicateRow {
            predicate: S1BlockingPredicate::OverclaimedPhysicalPosturePresent,
            status: S1BlockingPredicateStatus::Satisfied,
        },
        S1BlockingPredicateRow {
            predicate: S1BlockingPredicate::UnmappedDeferredGuaranteePresent,
            status: S1BlockingPredicateStatus::Satisfied,
        },
    ]
}
