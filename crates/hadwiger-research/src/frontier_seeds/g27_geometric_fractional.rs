use crate::candidate_screening::{
    draft_candidate_screening_invariant_catalog_checked, CandidateScreeningError,
    CandidateScreeningEvaluation, CandidateScreeningEvaluationMode,
    CandidateScreeningInvariantFamily, CandidateScreeningVerdict,
};
use crate::domain_artifacts::core_artifact::canonical_digest_token;
use crate::domain_artifacts::{
    HadwigerArtifactShapeError, HadwigerCanonicalArtifact, HadwigerDeclaredFamilyCheckedExt,
};
use crate::domain_declarations::{
    declare_research_request_checked, GeometricFractionalChromaticScreeningDeclaration,
};
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional_data::{
    g27_dimacs_edge_list_from_retained_data, replay_g27_retained_structural_certificate,
};
use super::g27_geometric_fractional_dual_replay::{
    replay_g27_retained_dual_witness, G27GeometricFractionalDualReplay,
};
use super::{
    import_frontier_graph_seed_checked, FrontierGraphSeedImport, FrontierGraphSeedImportReport,
    FrontierSeedError,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum G27GeometricFractionalError {
    Seed(FrontierSeedError),
    Screening(CandidateScreeningError),
    Artifact(HadwigerArtifactShapeError),
    MalformedData { source: &'static str },
    AdjacencyMismatch { left: usize, right: usize },
    IndependentSetMismatch,
    InvalidIsometryRow { row: usize },
    WitnessShapeMismatch,
    MatrixShapeMismatch { source: &'static str },
    MatrixZip(String),
    DualInequalityViolation { column: usize },
    QueryDeclarationNotAdmitted,
}

impl From<FrontierSeedError> for G27GeometricFractionalError {
    fn from(value: FrontierSeedError) -> Self {
        Self::Seed(value)
    }
}

impl From<CandidateScreeningError> for G27GeometricFractionalError {
    fn from(value: CandidateScreeningError) -> Self {
        Self::Screening(value)
    }
}

impl From<HadwigerArtifactShapeError> for G27GeometricFractionalError {
    fn from(value: HadwigerArtifactShapeError) -> Self {
        Self::Artifact(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27GeometricFractionalStructuralReplay {
    vertex_count: usize,
    edge_count: usize,
    independent_set_count: usize,
    isometry_count: usize,
    witness_coordinate_count: usize,
}

impl G27GeometricFractionalStructuralReplay {
    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    pub fn independent_set_count(&self) -> usize {
        self.independent_set_count
    }

    pub fn isometry_count(&self) -> usize {
        self.isometry_count
    }

    pub fn witness_coordinate_count(&self) -> usize {
        self.witness_coordinate_count
    }

    pub fn stable_token(&self) -> String {
        format!(
            "g27_structural_replay:v{}:e{}:atoms{}:isometries{}:witness{}",
            self.vertex_count,
            self.edge_count,
            self.independent_set_count,
            self.isometry_count,
            self.witness_coordinate_count
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27GeometricFractionalReproductionReport {
    seed_import: FrontierGraphSeedImportReport,
    structural_replay: G27GeometricFractionalStructuralReplay,
    dual_replay: G27GeometricFractionalDualReplay,
    evaluation: CandidateScreeningEvaluation,
}

impl G27GeometricFractionalReproductionReport {
    pub fn seed_import(&self) -> &FrontierGraphSeedImportReport {
        &self.seed_import
    }

    pub fn structural_replay(&self) -> &G27GeometricFractionalStructuralReplay {
        &self.structural_replay
    }

    pub fn dual_replay(&self) -> &G27GeometricFractionalDualReplay {
        &self.dual_replay
    }

    pub fn evaluation(&self) -> &CandidateScreeningEvaluation {
        &self.evaluation
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

pub fn reproduce_g27_geometric_fractional_witness_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27GeometricFractionalReproductionReport, G27GeometricFractionalError> {
    let seed_import = import_frontier_graph_seed_checked(
        handle,
        FrontierGraphSeedImport::g27_geometric_fractional(),
    )?;
    let structural_replay = replay_g27_structural_certificate()?;
    let dual_replay = replay_g27_retained_dual_witness()?;
    let catalog = draft_candidate_screening_invariant_catalog_checked(handle)?;
    let declaration = declare_research_request_checked(
        handle,
        GeometricFractionalChromaticScreeningDeclaration::new(
            seed_import.graph_version().reference().stable_token(),
            "4",
            "retained_g27_geometric_fractional_dual_witness",
        ),
    )
    .admitted()
    .ok_or(G27GeometricFractionalError::QueryDeclarationNotAdmitted)?;
    let query_declaration_digest = canonical_digest_token(declaration.declaration_digest());
    let evaluation = CandidateScreeningEvaluation::new(
        &catalog,
        CandidateScreeningInvariantFamily::GeometricFractionalChromaticNumber,
        seed_import.graph_version().reference(),
        CandidateScreeningVerdict::Priority,
        CandidateScreeningEvaluationMode::CheckedCertificate,
        format!(
            "query_declaration_digest={query_declaration_digest};retained_g27_geometric_fractional;{};{}",
            structural_replay.stable_token(),
            dual_replay.stable_token()
        ),
    )?;
    Ok(G27GeometricFractionalReproductionReport {
        seed_import,
        structural_replay,
        dual_replay,
        evaluation,
    })
}

pub(crate) fn g27_dimacs_edge_list() -> String {
    g27_dimacs_edge_list_from_retained_data()
}

fn replay_g27_structural_certificate(
) -> Result<G27GeometricFractionalStructuralReplay, G27GeometricFractionalError> {
    let replay = replay_g27_retained_structural_certificate()?;
    Ok(G27GeometricFractionalStructuralReplay {
        vertex_count: replay.vertex_count,
        edge_count: replay.edge_count,
        independent_set_count: replay.independent_set_count,
        isometry_count: replay.isometry_count,
        witness_coordinate_count: replay.witness_coordinate_count,
    })
}
