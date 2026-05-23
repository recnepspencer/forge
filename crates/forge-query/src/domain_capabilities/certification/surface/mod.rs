use crate::domain_capabilities::ForgeQueryDomainCapabilityCategory;
use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainCapabilityCertifiedSurfaceRow {
    category: ForgeQueryDomainCapabilityCategory,
    ordinary_lane: &'static str,
    inspectable_lane: &'static str,
    proof_lane: &'static str,
    raw_lane: &'static str,
    implementation_path: &'static str,
}

impl ForgeQueryDomainCapabilityCertifiedSurfaceRow {
    pub(crate) const fn new(
        category: ForgeQueryDomainCapabilityCategory,
        ordinary_lane: &'static str,
        inspectable_lane: &'static str,
        proof_lane: &'static str,
        raw_lane: &'static str,
        implementation_path: &'static str,
    ) -> Self {
        Self {
            category,
            ordinary_lane,
            inspectable_lane,
            proof_lane,
            raw_lane,
            implementation_path,
        }
    }

    pub fn category(&self) -> ForgeQueryDomainCapabilityCategory {
        self.category
    }

    pub fn ordinary_lane(&self) -> &'static str {
        self.ordinary_lane
    }

    pub fn inspectable_lane(&self) -> &'static str {
        self.inspectable_lane
    }

    pub fn proof_lane(&self) -> &'static str {
        self.proof_lane
    }

    pub fn raw_lane(&self) -> &'static str {
        self.raw_lane
    }

    pub fn implementation_path(&self) -> &'static str {
        self.implementation_path
    }

    pub fn row_digest(&self) -> String {
        hash_parts(&[
            self.category.as_str().to_string(),
            self.ordinary_lane.to_string(),
            self.inspectable_lane.to_string(),
            self.proof_lane.to_string(),
            self.raw_lane.to_string(),
            self.implementation_path.to_string(),
        ])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainCapabilityCertifiedSurfaceInventory {
    rows: &'static [ForgeQueryDomainCapabilityCertifiedSurfaceRow],
}

impl ForgeQueryDomainCapabilityCertifiedSurfaceInventory {
    pub(crate) const fn new(
        rows: &'static [ForgeQueryDomainCapabilityCertifiedSurfaceRow],
    ) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &'static [ForgeQueryDomainCapabilityCertifiedSurfaceRow] {
        self.rows
    }

    pub fn public_surface_digest(&self) -> String {
        hash_parts(
            &self
                .rows
                .iter()
                .map(ForgeQueryDomainCapabilityCertifiedSurfaceRow::row_digest)
                .collect::<Vec<_>>(),
        )
    }
}

pub fn forge_query_domain_capability_public_surface_inventory(
) -> ForgeQueryDomainCapabilityCertifiedSurfaceInventory {
    ForgeQueryDomainCapabilityCertifiedSurfaceInventory::new(CERTIFIED_SURFACE_ROWS)
}

const CERTIFIED_SURFACE_ROWS: &[ForgeQueryDomainCapabilityCertifiedSurfaceRow] = &[
    ForgeQueryDomainCapabilityCertifiedSurfaceRow::new(
        ForgeQueryDomainCapabilityCategory::Admission,
        "forge_query_domain(...).for_admitted_intent_plan(...).advises(...).because(...).materialize()",
        "forge_query_domain(...).for_admitted_intent_plan(...).advises(...).because(...).try_materialize()",
        "ForgeQueryAdmissionContributionAuthoring::advisory(...).for_admitted_intent_plan(...)",
        "materialize_runtime_admission_decision(...)",
        "crates/forge-query/src/domain_capabilities/dx/common/intent_admission.rs",
    ),
    ForgeQueryDomainCapabilityCertifiedSurfaceRow::new(
        ForgeQueryDomainCapabilityCategory::SupportTraceability,
        "forge_query_domain(...).for_intent(...).supports_traceability(...).because(...).materialize()",
        "forge_query_domain(...).for_intent(...).supports_traceability(...).because(...).try_materialize()",
        "ForgeQuerySupportContributionAuthoring::declaration_traceability(...).for_intent_declaration(...)",
        "materialize_intent_declaration_support_traceability_artifact(...)",
        "crates/forge-query/src/domain_capabilities/dx/common/intent.rs",
    ),
    ForgeQueryDomainCapabilityCertifiedSurfaceRow::new(
        ForgeQueryDomainCapabilityCategory::InvariantCapability,
        "forge_query_domain(...).for_intent(...).register_invariant_catalog(...).because(...).materialize()",
        "forge_query_domain(...).for_intent(...).register_invariant_catalog(...).because(...).try_materialize()",
        "ForgeQueryInvariantCapabilityContributionAuthoring::invariant_registration(...).for_intent_declaration(...)",
        "materialize_query_invariant_catalog_registration_artifact(...)",
        "crates/forge-query/src/domain_capabilities/dx/common/intent.rs",
    ),
    ForgeQueryDomainCapabilityCertifiedSurfaceRow::new(
        ForgeQueryDomainCapabilityCategory::WorkflowPreview,
        "forge_query_domain(...).for_intent(...).plans_preview_mutation(...).because(...).materialize()",
        "forge_query_domain(...).for_intent(...).plans_preview_mutation(...).because(...).try_materialize()",
        "ForgeQueryWorkflowContributionAuthoring::promotion_eligible_mutation_lowering(...).for_intent_declaration(...)",
        "materialize_query_workflow_declaration(...)",
        "crates/forge-query/src/domain_capabilities/dx/common/intent_workflow.rs",
    ),
    ForgeQueryDomainCapabilityCertifiedSurfaceRow::new(
        ForgeQueryDomainCapabilityCategory::ContinuityLineage,
        "forge_query_domain(...).for_admitted_intent_plan(...).preserves_continuity(...).because(...).materialize()",
        "forge_query_domain(...).for_admitted_intent_plan(...).preserves_continuity(...).because(...).try_materialize()",
        "ForgeQueryContinuityContributionAuthoring::split(...).for_admitted_intent_plan(...)",
        "materialize_canonical_continuity_artifact(...)",
        "crates/forge-query/src/domain_capabilities/dx/common/admitted_plan.rs",
    ),
    ForgeQueryDomainCapabilityCertifiedSurfaceRow::new(
        ForgeQueryDomainCapabilityCategory::ConsequenceAftermath,
        "forge_query_domain(...).for_admitted_intent_plan(...).consumes_projection_contract(...).because(...).materialize()",
        "forge_query_domain(...).for_admitted_intent_plan(...).consumes_projection_contract(...).because(...).review()",
        "ForgeQueryAftermathContributionAuthoring::projection_contract_consumption(...).for_admitted_intent_plan(...)",
        "materialize_projection_consumption_contract(...)",
        "crates/forge-query/src/domain_capabilities/dx/common/aftermath.rs",
    ),
    ForgeQueryDomainCapabilityCertifiedSurfaceRow::new(
        ForgeQueryDomainCapabilityCategory::ExplanationInspection,
        "forge_query_domain(...).for_lower_runtime_boundary_envelope(...).explains_cross_runtime_fallback(...).because(...).materialize_artifact()",
        "forge_query_domain(...).for_lower_runtime_boundary_envelope(...).explains_cross_runtime_fallback(...).because(...).review()",
        "ForgeQueryExplanationContributionAuthoring::cross_runtime_fallback_explanation(...).for_lower_runtime_boundary_envelope(...)",
        "materialize_query_causal_inspection_artifact(...)",
        "crates/forge-query/src/domain_capabilities/dx/common/lower_runtime.rs",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn certified_surface_inventory_covers_each_named_phase_six_category_once() {
        let inventory = forge_query_domain_capability_public_surface_inventory();
        let categories = inventory
            .rows()
            .iter()
            .map(|row| row.category().as_str())
            .collect::<Vec<_>>();

        assert_eq!(inventory.rows().len(), 7);
        assert_eq!(
            categories.len(),
            categories.iter().copied().collect::<BTreeSet<_>>().len()
        );
        for category in [
            "admission",
            "support-traceability",
            "invariant-capability",
            "workflow-preview",
            "continuity-lineage",
            "consequence-aftermath",
            "explanation-inspection",
        ] {
            assert!(categories.iter().any(|seen| *seen == category));
        }
    }

    #[test]
    fn certified_surface_rows_show_one_lane_at_a_time_degradation() {
        for row in forge_query_domain_capability_public_surface_inventory().rows() {
            assert!(!row.ordinary_lane().is_empty());
            assert!(!row.inspectable_lane().is_empty());
            assert!(!row.proof_lane().is_empty());
            assert!(!row.raw_lane().is_empty());
            assert_ne!(row.ordinary_lane(), row.inspectable_lane());
            assert_ne!(row.inspectable_lane(), row.proof_lane());
            assert_ne!(row.proof_lane(), row.raw_lane());
            assert!(row.implementation_path().contains("domain_capabilities"));
        }
    }

    #[test]
    fn certified_surface_inventory_digest_is_row_order_stable() {
        let inventory = forge_query_domain_capability_public_surface_inventory();
        let expected = hash_parts(
            &inventory
                .rows()
                .iter()
                .map(ForgeQueryDomainCapabilityCertifiedSurfaceRow::row_digest)
                .collect::<Vec<_>>(),
        );

        assert_eq!(inventory.public_surface_digest(), expected);
    }
}
