use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaterializedGraphOldAuthorityResidueRow {
    caller: String,
    owner: String,
    blocker: String,
    removal_trigger: String,
    row_digest: String,
}

impl MaterializedGraphOldAuthorityResidueRow {
    fn new(
        caller: &'static str,
        owner: &'static str,
        blocker: &'static str,
        removal_trigger: &'static str,
    ) -> Self {
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:materialized-graph-old-authority-residue-row:v1".to_string(),
            format!("caller:{caller}"),
            format!("owner:{owner}"),
            format!("blocker:{blocker}"),
            format!("removal-trigger:{removal_trigger}"),
        ]);
        Self {
            caller: caller.to_string(),
            owner: owner.to_string(),
            blocker: blocker.to_string(),
            removal_trigger: removal_trigger.to_string(),
            row_digest,
        }
    }

    pub fn caller(&self) -> &str {
        &self.caller
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaterializedGraphOldAuthorityResidue {
    capped_whole_view_authority_count: usize,
    capped_rows: Vec<MaterializedGraphOldAuthorityResidueRow>,
    residue_digest: String,
}

impl MaterializedGraphOldAuthorityResidue {
    pub fn current_source_scan() -> Self {
        Self::new(
            required_current_source_residue_rows()
                .into_iter()
                .filter(|row| current_source_contains(row.caller()))
                .collect(),
        )
    }

    pub fn required_capped_callers() -> &'static [&'static str] {
        &[
            "TopologyMaterializer::materialize_from_rows",
            "MaterializedTopologyView::whole_view",
            "stage_topology_read_from_view",
        ]
    }

    #[cfg(test)]
    pub(crate) fn uncapped_for_tests() -> Self {
        Self::new(Vec::new())
    }

    fn new(capped_rows: Vec<MaterializedGraphOldAuthorityResidueRow>) -> Self {
        let capped_whole_view_authority_count = capped_rows.len();
        let mut parts = vec![
            "worth-topo:materialized-graph-old-authority-residue:v1".to_string(),
            format!("capped-count:{capped_whole_view_authority_count}"),
        ];
        parts.extend(
            capped_rows
                .iter()
                .map(|row| format!("row:{}", row.row_digest())),
        );
        let residue_digest = super::super::super::catalog::catalog_digest(parts);
        Self {
            capped_whole_view_authority_count,
            capped_rows,
            residue_digest,
        }
    }

    pub const fn capped_whole_view_authority_count(&self) -> usize {
        self.capped_whole_view_authority_count
    }

    pub fn capped_rows(&self) -> &[MaterializedGraphOldAuthorityResidueRow] {
        &self.capped_rows
    }

    pub fn contains_required_caps(&self) -> bool {
        Self::required_capped_callers()
            .iter()
            .all(|required| self.capped_rows.iter().any(|row| row.caller() == *required))
    }

    pub fn residue_digest(&self) -> &str {
        &self.residue_digest
    }
}

fn required_current_source_residue_rows() -> Vec<MaterializedGraphOldAuthorityResidueRow> {
    vec![
        MaterializedGraphOldAuthorityResidueRow::new(
            "TopologyMaterializer::materialize_from_rows",
            "Phase 10 materialized-graph product migration",
            "row materialization still reports WholeViewRebuild as fallback authority",
            "ordinary operator maintenance no longer calls the old row materializer",
        ),
        MaterializedGraphOldAuthorityResidueRow::new(
            "MaterializedTopologyView::whole_view",
            "Phase 10 materialized-graph product migration",
            "tests and bootstrap paths can still mint whole-view topology snapshots",
            "all product-family phases consume receipt-bound migrated outputs",
        ),
        MaterializedGraphOldAuthorityResidueRow::new(
            "stage_topology_read_from_view",
            "Projection read-stage receipt rollout",
            "projection test helper can still materialize from a full view",
            "projection reads consume DerivedInvalidationProjectionReadStageReceipt",
        ),
    ]
}

fn current_source_contains(caller: &str) -> bool {
    match caller {
        "TopologyMaterializer::materialize_from_rows" => {
            include_str!("../../../materialized_graph/mod.rs").contains("materialize_from_rows")
        }
        "MaterializedTopologyView::whole_view" => {
            include_str!("../../../materialized_graph/types.rs").contains("whole_view")
        }
        "stage_topology_read_from_view" => {
            include_str!("../../../../projection/runtime_boundary/read_stage.rs")
                .contains("stage_topology_read_from_view")
        }
        _ => false,
    }
}
