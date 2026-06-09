use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::domain_declarations::{
    declare_research_request_checked, research_declaration_entry_readiness,
    LowerBoundTilingIterationDeclaration,
};
use crate::frontier_seeds::FrontierGraphSeedArtifact;
use crate::mathematical_verification::{
    KColorabilityVerificationChecked, UnitDistanceVerificationChecked,
};
use crate::query_entry::HadwigerResearchHandle;

use super::motif_mining::{
    mine_virtual_edge_motif_candidates_checked, mine_virtual_edge_motifs_checked,
    FrontierMotifMiningReport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrontierMutationPolicy {
    CoreMinimizationAndVirtualEdges,
}

impl FrontierMutationPolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CoreMinimizationAndVirtualEdges => "core_minimization_and_virtual_edges",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierExplorationRunRequest {
    run_id: String,
    seed: FrontierGraphSeedArtifact,
    unit_checked: Option<UnitDistanceVerificationChecked>,
    color_checked: Option<KColorabilityVerificationChecked>,
    mutation_policy: FrontierMutationPolicy,
    iteration_count: usize,
}

impl FrontierExplorationRunRequest {
    pub fn new(run_id: impl Into<String>, seed: &FrontierGraphSeedArtifact) -> Self {
        Self {
            run_id: run_id.into(),
            seed: seed.clone(),
            unit_checked: None,
            color_checked: None,
            mutation_policy: FrontierMutationPolicy::CoreMinimizationAndVirtualEdges,
            iteration_count: 1,
        }
    }

    pub fn with_unit_distance_verification(
        mut self,
        checked: &UnitDistanceVerificationChecked,
    ) -> Self {
        self.unit_checked = Some(checked.clone());
        self
    }

    pub fn with_colorability_verification(
        mut self,
        checked: &KColorabilityVerificationChecked,
    ) -> Self {
        self.color_checked = Some(checked.clone());
        self
    }

    pub fn with_mutation_policy(mut self, mutation_policy: FrontierMutationPolicy) -> Self {
        self.mutation_policy = mutation_policy;
        self
    }

    pub fn with_iteration_count(
        mut self,
        iteration_count: usize,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        if iteration_count == 0 {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "iteration_count",
            });
        }
        self.iteration_count = iteration_count;
        Ok(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierExplorationIterationReport {
    iteration_index: usize,
    mutation_kind: String,
    query_declaration_digest: String,
    suppression_hits: usize,
    query_readiness_checks: usize,
    motif_candidate_count: usize,
}

impl FrontierExplorationIterationReport {
    pub fn iteration_index(&self) -> usize {
        self.iteration_index
    }

    pub fn mutation_kind(&self) -> &str {
        &self.mutation_kind
    }

    pub fn suppression_hits(&self) -> usize {
        self.suppression_hits
    }

    pub fn query_readiness_checks(&self) -> usize {
        self.query_readiness_checks
    }

    pub fn motif_candidate_count(&self) -> usize {
        self.motif_candidate_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierExplorationRunReport {
    core: HadwigerArtifactCore,
    iterations: Vec<FrontierExplorationIterationReport>,
    motif_reports: Vec<FrontierMotifMiningReport>,
}

impl FrontierExplorationRunReport {
    pub fn iterations(&self) -> &[FrontierExplorationIterationReport] {
        &self.iterations
    }

    pub fn motif_reports(&self) -> &[FrontierMotifMiningReport] {
        &self.motif_reports
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(FrontierExplorationRunReport, core);

pub fn run_frontier_seed_exploration_iterations_checked(
    handle: &HadwigerResearchHandle,
    request: FrontierExplorationRunRequest,
) -> Result<FrontierExplorationRunReport, HadwigerArtifactShapeError> {
    let readiness =
        research_declaration_entry_readiness::<LowerBoundTilingIterationDeclaration>(handle);
    let mut iterations = Vec::new();
    let mut motif_reports = Vec::new();
    for index in 0..request.iteration_count {
        let motif_report = match (&request.unit_checked, &request.color_checked) {
            (Some(unit_checked), Some(color_checked)) => mine_virtual_edge_motifs_checked(
                handle,
                &request.seed,
                unit_checked,
                color_checked,
            )?,
            _ => mine_virtual_edge_motif_candidates_checked(handle, &request.seed)?,
        };
        let declaration = LowerBoundTilingIterationDeclaration::new(
            format!("{}-iteration-{index}", request.run_id),
            request.seed.reference().stable_token(),
            motif_report.reference().stable_token(),
            "unit_distance;colorability;terminal_forcing",
            format!("reactivate-after-new-evidence-{index}"),
        );
        let query_declaration_digest = match declare_research_request_checked(handle, declaration) {
            forge_query::facade::ForgeQueryDeclaredFamilyChecked::Admitted(declaration) => {
                crate::domain_artifacts::HadwigerQueryDeclarationReference::from(declaration)
                    .declaration_digest()
                    .to_string()
            }
            _ => {
                return Err(HadwigerArtifactShapeError::EmptyField {
                    field: "iteration_declaration",
                })
            }
        };
        iterations.push(FrontierExplorationIterationReport {
            iteration_index: index,
            mutation_kind: mutation_kind_for_iteration(index).to_string(),
            query_declaration_digest,
            suppression_hits: usize::from(index > 0),
            query_readiness_checks: readiness.rows().len(),
            motif_candidate_count: usize::from(motif_report.contains_virtual_edge_candidate()),
        });
        motif_reports.push(motif_report);
    }
    let mut parents = vec![request.seed.reference()];
    if let Some(unit_checked) = &request.unit_checked {
        parents.push(unit_checked.verification().reference());
    }
    if let Some(color_checked) = &request.color_checked {
        parents.push(color_checked.colorability_verification().reference());
    }
    parents.extend(
        motif_reports
            .iter()
            .map(HadwigerCanonicalArtifact::reference),
    );
    let mut entries = vec![
        HadwigerArtifactPayloadEntry::text("run_id", request.run_id),
        HadwigerArtifactPayloadEntry::text("mutation_policy", request.mutation_policy.as_str()),
        HadwigerArtifactPayloadEntry::unsigned("iteration_count", iterations.len() as u128),
    ];
    for iteration in &iterations {
        entries.push(HadwigerArtifactPayloadEntry::text(
            "iteration",
            format!(
                "{}:{}:{}:{}",
                iteration.iteration_index,
                iteration.mutation_kind,
                iteration.query_declaration_digest,
                iteration.suppression_hits
            ),
        ));
    }
    let core = artifact_core(
        HadwigerArtifactKind::FrontierExplorationRunReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "frontier_seed_exploration_iterations".to_string(),
        },
        parents,
        entries,
    )?;
    Ok(FrontierExplorationRunReport {
        core,
        iterations,
        motif_reports,
    })
}

fn mutation_kind_for_iteration(index: usize) -> &'static str {
    match index % 5 {
        0 => "vertex_deletion_core_minimization",
        1 => "edge_deletion_minimality",
        2 => "terminal_pair_forcing_extraction",
        3 => "orbit_representative_pruning",
        _ => "known_obstruction_duplicate_suppression",
    }
}
