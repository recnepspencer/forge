use super::projection_types::FrontierResearchProjectionGraph;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierPressureHaloHypothesis {
    hypothesis_id: String,
    motif_signature: String,
    hypothesis_statement: String,
    mutation_test_plan: String,
    required_evidence: Vec<String>,
}

impl FrontierPressureHaloHypothesis {
    fn new(
        hypothesis_id: impl Into<String>,
        motif_signature: impl Into<String>,
        hypothesis_statement: impl Into<String>,
        mutation_test_plan: impl Into<String>,
        required_evidence: Vec<String>,
    ) -> Self {
        Self {
            hypothesis_id: hypothesis_id.into(),
            motif_signature: motif_signature.into(),
            hypothesis_statement: hypothesis_statement.into(),
            mutation_test_plan: mutation_test_plan.into(),
            required_evidence,
        }
    }

    pub fn hypothesis_id(&self) -> &str {
        &self.hypothesis_id
    }

    pub fn motif_signature(&self) -> &str {
        &self.motif_signature
    }

    pub fn hypothesis_statement(&self) -> &str {
        &self.hypothesis_statement
    }

    pub fn mutation_test_plan(&self) -> &str {
        &self.mutation_test_plan
    }

    pub fn required_evidence(&self) -> &[String] {
        &self.required_evidence
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

pub fn propose_frontier_pressure_halo_hypotheses_checked(
    projection: &FrontierResearchProjectionGraph,
) -> Vec<FrontierPressureHaloHypothesis> {
    projection
        .pressure_halo_motifs()
        .iter()
        .map(|motif| {
            let spokes = motif
                .satellites()
                .iter()
                .map(|satellite| {
                    format!(
                        "{}:[{}]",
                        satellite.vertex_label(),
                        satellite.common_spokes().join("|")
                    )
                })
                .collect::<Vec<_>>()
                .join(";");
            FrontierPressureHaloHypothesis::new(
                format!("{}:spoke-pair-transfer", motif.novelty_signature()),
                motif.novelty_signature(),
                format!(
                    "Hub {} may act as a color-pressure reservoir whose paired spoke corridors \
                     transfer terminal constraints into the {} high-degree satellites.",
                    motif.hub_vertex(),
                    motif.satellites().len()
                ),
                format!(
                    "For each satellite, contract or delete one spoke in its pair, replay exact \
                     unit-distance geometry, replay k-colorability, and compare whether terminal \
                     forcing disappears asymmetrically. spoke_pairs={spokes}"
                ),
                vec![
                    "exact_unit_distance_replay".to_string(),
                    "colorability_refutation_replay".to_string(),
                    "suppression_checked_mutation_results".to_string(),
                ],
            )
        })
        .collect()
}
