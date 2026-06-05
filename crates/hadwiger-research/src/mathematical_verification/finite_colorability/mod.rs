mod cnf_encoding;
mod model_replay;
mod refutation_replay;
mod varisat_execution;

use crate::aspect_authority::{ColorabilityAspectRecord, HadwigerAspectAuthorityError};
use crate::domain_artifacts::{
    ColorabilityEncoding, ColorabilityVerification, ColorabilityVerificationPosture, GraphVersion,
    HadwigerArtifactShapeError, HadwigerCanonicalArtifact, SolverRun, SolverRunPosture,
};
use crate::domain_declarations::ColorabilityDeclaration;
use crate::query_entry::HadwigerResearchHandle;

use super::{admitted_declaration_reference, checker_evidence};

use cnf_encoding::encode_graph_coloring;
use model_replay::model_satisfies_cnf;
use refutation_replay::{checked_exhaustive_certificate, replay_refutation};
use varisat_execution::solve_cnf;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HadwigerColorabilityError {
    EmptyGraph,
    ZeroColorCount,
    Solver(String),
    CorruptModel,
    CorruptRefutationCertificate,
    QueryDeclarationNotAdmitted,
    Artifact(HadwigerArtifactShapeError),
    Aspect(HadwigerAspectAuthorityError),
}

impl From<HadwigerArtifactShapeError> for HadwigerColorabilityError {
    fn from(value: HadwigerArtifactShapeError) -> Self {
        Self::Artifact(value)
    }
}

impl From<HadwigerAspectAuthorityError> for HadwigerColorabilityError {
    fn from(value: HadwigerAspectAuthorityError) -> Self {
        Self::Aspect(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KColorabilityVerificationChecked {
    encoding: ColorabilityEncoding,
    solver_run: SolverRun,
    colorability_verification: ColorabilityVerification,
    not_k_colorable_aspect: ColorabilityAspectRecord,
}

impl KColorabilityVerificationChecked {
    pub fn encoding(&self) -> &ColorabilityEncoding {
        &self.encoding
    }

    pub fn solver_run(&self) -> &SolverRun {
        &self.solver_run
    }

    pub fn colorability_verification(&self) -> &ColorabilityVerification {
        &self.colorability_verification
    }

    pub fn not_k_colorable_aspect(&self) -> &ColorabilityAspectRecord {
        &self.not_k_colorable_aspect
    }
}

pub fn verify_k_colorability_checked(
    handle: &HadwigerResearchHandle,
    graph_version: &GraphVersion,
    color_count: u32,
) -> Result<KColorabilityVerificationChecked, HadwigerColorabilityError> {
    if color_count == 0 {
        return Err(HadwigerColorabilityError::ZeroColorCount);
    }
    if graph_version.vertices().is_empty() {
        return Err(HadwigerColorabilityError::EmptyGraph);
    }
    let declared = handle.declare_checked(ColorabilityDeclaration::new(
        graph_version.version_id(),
        color_count,
    ));
    let query_declaration_reference = admitted_declaration_reference(declared)
        .ok_or(HadwigerColorabilityError::QueryDeclarationNotAdmitted)?;
    let query_identity = format!(
        "{}:{}",
        handle.handle_identity_digest(),
        query_declaration_reference.declaration_digest()
    );
    let (variable_map, clauses) = encode_graph_coloring(graph_version, color_count);
    let variable_count = variable_map.len() as u32;
    let encoding = ColorabilityEncoding::checked(
        graph_version.reference(),
        color_count,
        variable_map,
        clauses.clone(),
    )?;
    let (sat, model) = solve_cnf(&clauses)?;
    let (solver_posture, verification_posture, aspect) = if sat {
        if !model_satisfies_cnf(&clauses, &model) {
            return Err(HadwigerColorabilityError::CorruptModel);
        }
        (
            SolverRunPosture::Sat,
            ColorabilityVerificationPosture::SatModelVerified,
            ColorabilityAspectRecord::unsupported(
                graph_version.reference(),
                color_count,
                "graph is k-colorable, so not-k-colorable is rejected for this run",
            )?,
        )
    } else if let Some(certificate) =
        checked_exhaustive_certificate(graph_version, color_count, variable_count)
    {
        replay_refutation(&clauses, &certificate)?;
        (
            SolverRunPosture::Unsat,
            ColorabilityVerificationPosture::UnsatVerified,
            ColorabilityAspectRecord::admitted_checked(
                graph_version.reference(),
                color_count,
                "varisat unsat result independently replayed by exhaustive certificate",
            )?,
        )
    } else {
        (
            SolverRunPosture::UnsupportedCertificateBudget,
            ColorabilityVerificationPosture::UnsupportedCertificateBudget,
            ColorabilityAspectRecord::unsupported(
                graph_version.reference(),
                color_count,
                "unsat certificate budget unavailable for graph size",
            )?,
        )
    };
    let solver_run = SolverRun::checked(
        encoding.reference(),
        query_declaration_reference.clone(),
        "varisat",
        "0.2.1",
        solver_posture,
        model,
        checker_evidence("sat", &query_identity)?,
    )?;
    let colorability_verification = ColorabilityVerification::checked(
        solver_run.reference(),
        graph_version.reference(),
        query_declaration_reference,
        "hadwiger.cnf_replay",
        "0.1.0",
        verification_posture,
        color_count,
        checker_evidence("colorability-verification", &query_identity)?,
    )?;
    Ok(KColorabilityVerificationChecked {
        encoding,
        solver_run,
        colorability_verification,
        not_k_colorable_aspect: aspect,
    })
}
