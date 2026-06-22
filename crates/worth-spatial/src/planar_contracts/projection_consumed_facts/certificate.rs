use super::{
    projection_consumed_planar_fact_authority_entries, projection_consumed_planar_fact_digest,
    ProjectionConsumedPlanarFactsBasis, ProjectionConsumedPlanarFactsCounters,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionConsumedPlanarFactKind {
    RetainedPlanarClassification,
}

impl ProjectionConsumedPlanarFactKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RetainedPlanarClassification => "retained_planar_classification",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectionConsumedPlanarFactsReceipt {
    basis: ProjectionConsumedPlanarFactsBasis,
    projected_fact_kind: ProjectionConsumedPlanarFactKind,
    declaration_digest: String,
    progression_digest: String,
    route_plan_digest: String,
    query_receipt_digest: String,
    envelope_digest: String,
    retained_planar_fact_digest: String,
    structural_identity_digest: String,
    motion_posture_digest: String,
    topology_contract_digest: String,
    materialization_digest: String,
    projection_consumption_digest: String,
    counters: ProjectionConsumedPlanarFactsCounters,
}

impl ProjectionConsumedPlanarFactsReceipt {
    pub(crate) fn new(
        basis: ProjectionConsumedPlanarFactsBasis,
        declaration_digest: String,
        progression_digest: String,
        route_plan_digest: String,
        query_receipt_digest: String,
        envelope_digest: String,
        materialization_digest: String,
        projection_consumption_digest: String,
        counters: ProjectionConsumedPlanarFactsCounters,
    ) -> Self {
        Self {
            retained_planar_fact_digest: basis.retained_planar_fact_digest().to_string(),
            structural_identity_digest: basis.structural_identity_digest().to_string(),
            motion_posture_digest: basis.motion_posture_digest().to_string(),
            topology_contract_digest: basis.topology_contract_digest().to_string(),
            basis,
            projected_fact_kind: ProjectionConsumedPlanarFactKind::RetainedPlanarClassification,
            declaration_digest,
            progression_digest,
            route_plan_digest,
            query_receipt_digest,
            envelope_digest,
            materialization_digest,
            projection_consumption_digest,
            counters,
        }
    }

    pub(crate) fn digest_parts(
        basis: &ProjectionConsumedPlanarFactsBasis,
        declaration_digest: &str,
        progression_digest: &str,
        route_plan_digest: &str,
        query_receipt_digest: &str,
        envelope_digest: &str,
        materialization_digest: &str,
    ) -> Vec<String> {
        let mut parts = projection_consumed_planar_fact_authority_entries(basis)
            .into_iter()
            .map(|entry| entry.digest_part())
            .collect::<Vec<_>>();
        parts.push(format!(
            "projected_fact_kind:{}",
            ProjectionConsumedPlanarFactKind::RetainedPlanarClassification.as_str()
        ));
        parts.push(format!("declaration:{declaration_digest}"));
        parts.push(format!("progression:{progression_digest}"));
        parts.push(format!("route_plan:{route_plan_digest}"));
        parts.push(format!("query_receipt:{query_receipt_digest}"));
        parts.push(format!("envelope:{envelope_digest}"));
        parts.push(format!("materialization:{materialization_digest}"));
        parts
    }

    pub(crate) fn projection_consumption_digest_for(
        basis: &ProjectionConsumedPlanarFactsBasis,
        declaration_digest: &str,
        progression_digest: &str,
        route_plan_digest: &str,
        query_receipt_digest: &str,
        envelope_digest: &str,
        materialization_digest: &str,
    ) -> String {
        projection_consumed_planar_fact_digest(&Self::digest_parts(
            basis,
            declaration_digest,
            progression_digest,
            route_plan_digest,
            query_receipt_digest,
            envelope_digest,
            materialization_digest,
        ))
    }

    pub fn basis(&self) -> &ProjectionConsumedPlanarFactsBasis {
        &self.basis
    }

    pub fn projected_fact_kind(&self) -> ProjectionConsumedPlanarFactKind {
        self.projected_fact_kind
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn progression_digest(&self) -> &str {
        &self.progression_digest
    }

    pub fn route_plan_digest(&self) -> &str {
        &self.route_plan_digest
    }

    pub fn query_receipt_digest(&self) -> &str {
        &self.query_receipt_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn retained_planar_fact_digest(&self) -> &str {
        &self.retained_planar_fact_digest
    }

    pub fn structural_identity_digest(&self) -> &str {
        &self.structural_identity_digest
    }

    pub fn motion_posture_digest(&self) -> &str {
        &self.motion_posture_digest
    }

    pub fn topology_contract_digest(&self) -> &str {
        &self.topology_contract_digest
    }

    pub fn materialization_digest(&self) -> &str {
        &self.materialization_digest
    }

    pub fn projection_consumption_digest(&self) -> &str {
        &self.projection_consumption_digest
    }

    pub fn counters(&self) -> ProjectionConsumedPlanarFactsCounters {
        self.counters
    }
}
