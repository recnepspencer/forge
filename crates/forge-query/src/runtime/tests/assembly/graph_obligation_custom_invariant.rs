use super::super::support::*;
use forge_relational::facade::runtime::{
    CustomInvariantDescriptor, CustomInvariantExecutionContext, CustomInvariantExecutionError,
    CustomInvariantOperationalMetadata, CustomInvariantPreparationError,
    CustomInvariantRegistration, CustomInvariantRule, CustomInvariantRuleId,
    CustomInvariantScopePlanner, CustomInvariantSemanticIdentity, CustomInvariantSemanticVersion,
    CustomInvariantVerdict, InvariantCostClass, InvariantExecutionPoint, InvariantFailureEffect,
    InvariantGroup, InvariantGroupSet,
};
use std::sync::Arc;

#[derive(Clone, Copy)]
struct CustomGraphObligationRule {
    rule_id: &'static str,
    failure_effect: InvariantFailureEffect,
}

impl CustomInvariantRule for CustomGraphObligationRule {
    type Scope = ();

    fn descriptor(&self) -> CustomInvariantDescriptor {
        CustomInvariantDescriptor {
            identity: CustomInvariantSemanticIdentity {
                rule_id: CustomInvariantRuleId::new(self.rule_id),
                semantic_version: CustomInvariantSemanticVersion::new(1, 2),
            },
            display_name: Arc::from(self.rule_id),
            operational: CustomInvariantOperationalMetadata {
                execution_point: InvariantExecutionPoint::CommitBoundary,
                groups: InvariantGroupSet::of(InvariantGroup::SchemaCompliance),
                cost_class: InvariantCostClass::Touched,
                failure_effect: self.failure_effect,
            },
        }
    }

    fn prepare_scope(
        &self,
        _planner: &mut CustomInvariantScopePlanner<'_>,
    ) -> Result<Self::Scope, CustomInvariantPreparationError> {
        Ok(())
    }

    fn evaluate(
        &self,
        _context: &CustomInvariantExecutionContext<'_>,
        _scope: &Self::Scope,
    ) -> Result<CustomInvariantVerdict, CustomInvariantExecutionError> {
        Ok(CustomInvariantVerdict::Pass)
    }
}

#[test]
fn custom_invariant_registration_requires_explicit_graph_touch_selector() {
    let custom_invariant = CustomInvariantRegistration::new(CustomGraphObligationRule {
        rule_id: "topology.custom.loop-wiring",
        failure_effect: InvariantFailureEffect::BlockCommit,
    })
    .unwrap();
    let graph_obligation = ForgeQueryGraphObligationRegistration::custom_invariant(
        &custom_invariant,
        ForgeQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );

    assert_eq!(graph_obligation.kind().as_str(), "blocking-invariant");
    assert!(graph_obligation
        .rule_identity()
        .domain_invariant_family()
        .contains("topology.custom.loop-wiring"));
}

#[test]
fn query_builder_registers_graph_scoped_custom_invariant_and_obligation_together() {
    let runtime = complete_backend_from_parts_builder()
        .graph_scoped_custom_invariant(ForgeQueryGraphScopedCustomInvariantRegistration::new(
            CustomInvariantRegistration::new(CustomGraphObligationRule {
                rule_id: "topology.custom.builder-scope",
                failure_effect: InvariantFailureEffect::BlockCommit,
            })
            .unwrap(),
            ForgeQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap(),
            ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
        ))
        .build_backend_from_parts()
        .build()
        .expect("query runtime should assemble graph-scoped custom invariant registrations");

    let catalog = runtime.graph_obligation_registration_catalog();
    assert_eq!(catalog.registration_count(), 1);
    let registration = &catalog.registrations()[0];
    assert_eq!(registration.kind().as_str(), "blocking-invariant");
    assert_eq!(
        registration.rule_identity().namespace(),
        "relational-custom-invariant"
    );
    assert_eq!(
        registration.rule_identity().name(),
        "topology.custom.builder-scope"
    );
    assert_eq!(registration.touch_selector().selector_kind(), "collection");
    assert_eq!(
        registration.touch_selector().selector_value().as_deref(),
        Some("topology.loop_successor")
    );
}

#[test]
fn audit_only_custom_invariant_registers_as_advisory_obligation() {
    let custom_invariant = CustomInvariantRegistration::new(CustomGraphObligationRule {
        rule_id: "topology.custom.audit",
        failure_effect: InvariantFailureEffect::AuditOnly,
    })
    .unwrap();
    let graph_obligation = ForgeQueryGraphObligationRegistration::custom_invariant(
        &custom_invariant,
        ForgeQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );

    assert_eq!(graph_obligation.kind().as_str(), "advisory-obligation");
}
