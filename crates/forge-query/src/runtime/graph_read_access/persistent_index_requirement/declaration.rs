use super::{
    ForgeQueryPersistentGraphIndexRequirementCounters,
    ForgeQueryPersistentGraphIndexRequirementReceipt, ForgeQueryPersistentGraphIndexRequirementRow,
};
use crate::identity::hash_parts;
use crate::runtime::{
    ForgeQueryGraphIndexInventoryMatchReport, ForgeQueryGraphReadAccessAdmissionPosture,
    ForgeQueryGraphReadAccessCostEstimate, ForgeQueryGraphReadAccessRequirementSet,
    ForgeQueryGraphReadRequiredCapabilityOwner,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPersistentGraphIndexRequirementDeclaration {
    digest: String,
    read_graph_digest: String,
    access_shape_digest: String,
    selectivity_shape_digest: String,
    requirement_set_digest: String,
    inventory_match_report_digest: String,
    estimated_index_bytes: usize,
    estimated_result_bytes: usize,
    required_owner: ForgeQueryGraphReadRequiredCapabilityOwner,
    requirement_rows: Vec<ForgeQueryPersistentGraphIndexRequirementRow>,
    counters: ForgeQueryPersistentGraphIndexRequirementCounters,
}

impl ForgeQueryPersistentGraphIndexRequirementDeclaration {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn access_shape_digest(&self) -> &str {
        &self.access_shape_digest
    }

    pub fn selectivity_shape_digest(&self) -> &str {
        &self.selectivity_shape_digest
    }

    pub fn requirement_set_digest(&self) -> &str {
        &self.requirement_set_digest
    }

    pub fn inventory_match_report_digest(&self) -> &str {
        &self.inventory_match_report_digest
    }

    pub fn estimated_index_bytes(&self) -> usize {
        self.estimated_index_bytes
    }

    pub fn estimated_result_bytes(&self) -> usize {
        self.estimated_result_bytes
    }

    pub fn required_owner(&self) -> &ForgeQueryGraphReadRequiredCapabilityOwner {
        &self.required_owner
    }

    pub fn requirement_rows(&self) -> &[ForgeQueryPersistentGraphIndexRequirementRow] {
        &self.requirement_rows
    }

    pub fn counters(&self) -> &ForgeQueryPersistentGraphIndexRequirementCounters {
        &self.counters
    }

    pub fn receipt(&self) -> ForgeQueryPersistentGraphIndexRequirementReceipt {
        ForgeQueryPersistentGraphIndexRequirementReceipt::new(
            self.digest.clone(),
            self.counters.clone(),
        )
    }

    pub(crate) fn from_admission_parts(
        requirements: &ForgeQueryGraphReadAccessRequirementSet,
        estimate: &ForgeQueryGraphReadAccessCostEstimate,
        report: &ForgeQueryGraphIndexInventoryMatchReport,
    ) -> Option<Self> {
        let requirement_rows = report
            .matches()
            .iter()
            .filter(|row| {
                row.resolved_admission_posture()
                    == &ForgeQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired
            })
            .map(ForgeQueryPersistentGraphIndexRequirementRow::from_inventory_match)
            .collect::<Vec<_>>();
        if requirement_rows.is_empty() {
            return None;
        }
        let read_graph_digest = requirements.read_graph_digest().to_string();
        let access_shape_digest = requirements.access_shape_digest().to_string();
        let selectivity_shape_digest = requirements.selectivity_shape_digest().to_string();
        let requirement_set_digest = requirements.digest().as_str().to_string();
        let inventory_match_report_digest = report.digest().to_string();
        let estimated_index_bytes = estimate.supported().index_bytes();
        let estimated_result_bytes = estimate.supported().result_bytes();
        let required_owner = ForgeQueryGraphReadRequiredCapabilityOwner::PersistentStore;
        let counters =
            ForgeQueryPersistentGraphIndexRequirementCounters::new(requirement_rows.len());
        let digest = hash_parts(
            &[
                "forge_query_persistent_graph_index_requirement_declaration_v1".to_string(),
                format!("read_graph:{read_graph_digest}"),
                format!("access_shape:{access_shape_digest}"),
                format!("selectivity_shape:{selectivity_shape_digest}"),
                format!("requirements:{requirement_set_digest}"),
                format!("inventory_match_report:{inventory_match_report_digest}"),
                format!("estimated_index_bytes:{estimated_index_bytes}"),
                format!("estimated_result_bytes:{estimated_result_bytes}"),
                format!("required_owner:{}", required_owner.as_str()),
                counters.digest_part(),
            ]
            .into_iter()
            .chain(
                requirement_rows
                    .iter()
                    .map(ForgeQueryPersistentGraphIndexRequirementRow::digest_part),
            )
            .collect::<Vec<_>>(),
        );
        Some(Self {
            digest,
            read_graph_digest,
            access_shape_digest,
            selectivity_shape_digest,
            requirement_set_digest,
            inventory_match_report_digest,
            estimated_index_bytes,
            estimated_result_bytes,
            required_owner,
            requirement_rows,
            counters,
        })
    }
}
