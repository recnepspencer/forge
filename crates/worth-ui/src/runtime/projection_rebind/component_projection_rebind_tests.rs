use super::projection_rebind_test_support::{
    page_host_plan, projection_rebind_app, runtime_for_source_authored_page_host,
};
use crate::capability::ComponentId;
use crate::runtime::projection_contract::plan_contract::private::Sealed;
use crate::runtime::{
    WorthUiCapabilityChangedFacts, WorthUiCapabilityReloadEvidence,
    WorthUiCapabilityReloadFamilyCounters, WorthUiCapabilityReloadFamilyKind,
    WorthUiCapabilityReloadFamilyRow, WorthUiCapabilityReloadStatus,
    WorthUiClassifiedRuntimeChange, WorthUiComponentCompatibility, WorthUiComponentReloadReceipt,
    WorthUiComponentStatePreservation, WorthUiProjectionDependencyDeclaration,
    WorthUiProjectionDependencySet, WorthUiProjectionEquivalenceBasisKind,
    WorthUiProjectionPlanContract, WorthUiProjectionRebindPlan, WorthUiProjectionRebindStatus,
    WorthUiRuntimeFactId, WorthUiRuntimeFactSet, WorthUiRuntimeInstanceWitness,
};

#[test]
fn component_changed_fact_rebuilds_only_component_dependent_projection() {
    let runtime = runtime_for_source_authored_page_host(&projection_rebind_app("Save"));
    let component_change = admitted_component_change(
        runtime.instance_id().raw(),
        "validation.component.sample",
        WorthUiComponentCompatibility::CompatiblePreserveState(
            WorthUiComponentStatePreservation::new(
                ComponentId::new("validation.component.sample").unwrap(),
            ),
        ),
    );

    let dependent_rebind = runtime
        .prepare_projection_rebind(
            &component_change,
            runtime
                .admit_projection_plan(component_projection_plan(
                    "worth-ui.component.dependent",
                    "validation.component.sample",
                ))
                .unwrap(),
        )
        .expect("component-dependent projection admits");
    let unrelated_rebind = runtime
        .prepare_projection_rebind(
            &component_change,
            runtime
                .admit_projection_plan(page_host_plan(&runtime))
                .unwrap(),
        )
        .expect("unrelated page host still admits");

    let dependent_receipt = match dependent_rebind {
        WorthUiProjectionRebindPlan::Rebuild(activated) => {
            activated
                .complete_rebuild(
                    runtime
                        .admit_projection_plan(component_projection_plan(
                            "worth-ui.component.dependent",
                            "validation.component.sample",
                        ))
                        .unwrap(),
                )
                .1
        }
        WorthUiProjectionRebindPlan::Preserve(_) => {
            panic!("component-dependent projection must rebuild on component fact change")
        }
    };
    let unrelated_receipt = match unrelated_rebind {
        WorthUiProjectionRebindPlan::Preserve(preserved) => preserved.complete_preserved().1,
        WorthUiProjectionRebindPlan::Rebuild(_) => {
            panic!("projection without component dependency must stay preserved")
        }
    };

    assert!(matches!(
        dependent_receipt.rows()[0].component_compatibility(),
        Some(WorthUiComponentCompatibility::CompatiblePreserveState(_))
    ));
    assert_eq!(
        unrelated_receipt.rows()[0].status(),
        WorthUiProjectionRebindStatus::EquivalentAfterActivation
    );
    assert!(unrelated_receipt.rows()[0]
        .component_compatibility()
        .is_none());
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ComponentProjectionPlan {
    identity: &'static str,
    dependencies: WorthUiProjectionDependencySet,
}

impl Sealed for ComponentProjectionPlan {}

impl WorthUiProjectionPlanContract for ComponentProjectionPlan {
    fn projection_identity(&self) -> crate::runtime::WorthUiProjectionIdentity {
        crate::runtime::WorthUiProjectionIdentity::runtime(self.identity)
    }

    fn projection_family(&self) -> crate::runtime::WorthUiProjectionFamily {
        crate::runtime::WorthUiProjectionFamily::HeaderMenu
    }

    fn projection_dependency_declaration(&self) -> WorthUiProjectionDependencyDeclaration {
        WorthUiProjectionDependencyDeclaration::from_set(self.dependencies.clone())
    }

    fn projection_equivalence_digest(&self) -> u64 {
        91
    }

    fn projection_equivalence_basis_kind(&self) -> WorthUiProjectionEquivalenceBasisKind {
        WorthUiProjectionEquivalenceBasisKind::ProjectionDigest
    }
}

fn component_projection_plan(
    identity: &'static str,
    component_id: &str,
) -> ComponentProjectionPlan {
    ComponentProjectionPlan {
        identity,
        dependencies: WorthUiProjectionDependencySet::empty().depends_on(
            WorthUiRuntimeFactId::component(&ComponentId::new(component_id).unwrap()),
        ),
    }
}

fn admitted_component_change(
    runtime_instance: u64,
    component_id: &str,
    component_compatibility: WorthUiComponentCompatibility,
) -> crate::runtime::WorthUiAdmittedRuntimeChangeEvidence {
    let component_id = ComponentId::new(component_id).unwrap();
    let family_row = WorthUiCapabilityReloadFamilyRow::admitted_with_component_reload_receipt(
        WorthUiCapabilityReloadFamilyKind::Components,
        41,
        WorthUiCapabilityReloadFamilyCounters::new(1, 1, 1, 1, 1, 1),
        true,
        Some(WorthUiComponentReloadReceipt::new(
            vec![component_id.clone()],
            component_compatibility,
        )),
    );
    let evidence = WorthUiCapabilityReloadEvidence::from_family_rows(
        runtime_instance,
        WorthUiCapabilityReloadStatus::ReadyForFrameBoundary,
        10,
        Some(11),
        41,
        vec![family_row],
        WorthUiCapabilityChangedFacts::from_admitted_capability_reload(
            WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::component(&component_id)),
            10,
            11,
        ),
    )
    .mark_activated(11);
    let classified = WorthUiClassifiedRuntimeChange::from_capability_reload(&evidence);
    crate::runtime::WorthUiAdmittedRuntimeChangeEvidence::admit(
        classified,
        WorthUiRuntimeInstanceWitness::from_raw(runtime_instance),
    )
    .expect("component capability change admits after activation")
}
