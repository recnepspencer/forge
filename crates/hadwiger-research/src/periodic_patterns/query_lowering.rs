use crate::candidate_screening::{
    draft_candidate_screening_invariant_catalog_checked,
    evaluate_finite_patch_boundary_extension_screening_checked,
    evaluate_monodromy_color_holonomy_screening_checked,
    evaluate_periodic_quotient_graph_screening_checked,
    evaluate_substitution_consistency_screening_checked,
    evaluate_translation_rotation_closure_screening_checked, CandidateScreeningEvaluation,
};
use crate::domain_artifacts::core_artifact::canonical_digest_token;
use crate::domain_artifacts::HadwigerDeclaredFamilyCheckedExt;
use crate::domain_declarations::{
    declare_research_request_checked, GeneratedPatternClosureDeclaration,
    PeriodicQuotientCellDeclaration,
};
use crate::query_entry::HadwigerResearchHandle;

use super::generated_pattern_replay_suites::GeneratedPatternReplaySuite;
use super::periodic_quotient_cells::PeriodicQuotientCell;
use super::replay_counters::GeneratedPatternReplayCounters;
use super::replay_errors::{GeneratedPatternReplayError, GeneratedPatternReplayShapeError};
use super::replay_reports::{
    GeneratedPatternReplayChecked, GeneratedPatternReplayReport, PeriodicQuotientReplayChecked,
};

pub fn certify_periodic_quotient_replay_checked(
    handle: &HadwigerResearchHandle,
    suite: GeneratedPatternReplaySuite,
) -> Result<PeriodicQuotientReplayChecked, GeneratedPatternReplayError> {
    let quotient = suite
        .periodic_quotient_cell()
        .cloned()
        .ok_or(GeneratedPatternReplayShapeError::MissingPeriodicQuotientCell)?;
    let query_digest = declare_periodic_quotient_cell(handle, &quotient)?;
    let counters = GeneratedPatternReplayCounters::new(
        quotient.source_tile_count(),
        quotient.translation_rules().len(),
        0,
        0,
        0,
        0,
        0,
        1,
        0,
    );
    let report = GeneratedPatternReplayReport::checked(&suite, Vec::new(), counters, query_digest)?;
    Ok(PeriodicQuotientReplayChecked::new(quotient, report))
}

pub fn certify_generated_pattern_replay_checked(
    handle: &HadwigerResearchHandle,
    suite: GeneratedPatternReplaySuite,
) -> Result<GeneratedPatternReplayChecked, GeneratedPatternReplayError> {
    let query_digest = declare_generated_pattern_closure(handle, &suite)?;
    let catalog = draft_candidate_screening_invariant_catalog_checked(handle)?;
    let mut evaluations = Vec::new();
    for certificate in suite.periodic_quotient_conflicts() {
        evaluations.push(evaluate_periodic_quotient_graph_screening_checked(
            handle,
            &catalog,
            suite.quotient_reference().clone(),
            certificate.model(),
            certificate.certificate(),
        )?);
    }
    for certificate in suite.color_holonomy_loops() {
        let screening_certificate = certificate.to_screening_certificate()?;
        evaluations.push(evaluate_monodromy_color_holonomy_screening_checked(
            handle,
            &catalog,
            suite.quotient_reference().clone(),
            screening_certificate,
        )?);
    }
    for certificate in suite.translation_rotation_certificates() {
        evaluations.push(evaluate_translation_rotation_closure_screening_checked(
            handle,
            &catalog,
            certificate.graph(),
            certificate.certificate(),
        )?);
    }
    for certificate in suite.substitution_certificates() {
        evaluations.push(evaluate_substitution_consistency_screening_checked(
            handle,
            &catalog,
            suite.quotient_reference().clone(),
            certificate.clone(),
        )?);
    }
    for certificate in suite.finite_patch_extension_certificates() {
        evaluations.push(evaluate_finite_patch_boundary_extension_screening_checked(
            handle,
            &catalog,
            suite.quotient_reference().clone(),
            certificate.clone(),
        )?);
    }
    let counters = replay_counters(&suite, &evaluations);
    let report =
        GeneratedPatternReplayReport::checked(&suite, evaluations, counters, query_digest)?;
    Ok(GeneratedPatternReplayChecked::new(suite, report))
}

fn declare_periodic_quotient_cell(
    handle: &HadwigerResearchHandle,
    quotient: &PeriodicQuotientCell,
) -> Result<String, GeneratedPatternReplayError> {
    let checked = declare_research_request_checked(
        handle,
        PeriodicQuotientCellDeclaration::new(quotient.quotient_id())
            .try_with_lattice_basis_ref(quotient.lattice_basis().stable_token())?
            .try_with_boundary_ownership_ref("exact_periodic_replay")?,
    );
    let declaration = checked
        .admitted()
        .ok_or(GeneratedPatternReplayShapeError::EmptyField {
            field: "query_periodic_quotient_cell_declaration",
        })?;
    Ok(canonical_digest_token(declaration.declaration_digest()))
}

fn declare_generated_pattern_closure(
    handle: &HadwigerResearchHandle,
    suite: &GeneratedPatternReplaySuite,
) -> Result<String, GeneratedPatternReplayError> {
    let checked = declare_research_request_checked(
        handle,
        GeneratedPatternClosureDeclaration::new(suite.replay_suite_id(), suite.stable_token())
            .with_generator("bounded_generated_pattern_replay")?,
    );
    let declaration = checked
        .admitted()
        .ok_or(GeneratedPatternReplayShapeError::EmptyField {
            field: "query_generated_pattern_closure_declaration",
        })?;
    Ok(canonical_digest_token(declaration.declaration_digest()))
}

fn replay_counters(
    suite: &GeneratedPatternReplaySuite,
    evaluations: &[CandidateScreeningEvaluation],
) -> GeneratedPatternReplayCounters {
    let translation_rules = suite
        .periodic_quotient_cell()
        .map(|quotient| quotient.translation_rules().len())
        .unwrap_or(0);
    GeneratedPatternReplayCounters::new(
        suite
            .periodic_quotient_cell()
            .map(PeriodicQuotientCell::source_tile_count)
            .unwrap_or(0),
        translation_rules,
        suite.periodic_quotient_conflicts().len(),
        suite.color_holonomy_loops().len(),
        suite.translation_rotation_certificates().len(),
        suite.substitution_certificates().len(),
        suite.finite_patch_extension_certificates().len(),
        1 + evaluations.len(),
        evaluations.len(),
    )
}
