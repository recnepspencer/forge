use std::sync::{Arc, Mutex};

use worth_proof::TransitionOutcome;

use worth_query::facade::domain;

use super::{
    configured_runtime_for_package, federated_package, graph_read_material, read_definition,
    FederatedRead, GeometryDomain, ReadFamily, RemoteA, RemoteB,
};

struct CrossCallProvider {
    retained: Arc<Mutex<Option<domain::WorthQueryGraphProviderCall>>>,
    reuse_retained: bool,
}

struct ReplayedCallProvider {
    retained: Arc<Mutex<Option<domain::WorthQueryGraphProviderCall>>>,
}

impl<G> domain::WorthQueryGraphParticipationProvider<G> for ReplayedCallProvider {
    fn execution_resource_support(&self) -> domain::WorthQueryExecutionResourceSupport {
        super::super::installed_operation_fixture::execution_resource_support()
    }

    fn observe(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        Ok(call.completed("observe"))
    }

    fn project(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        let mut retained = self.retained.lock().unwrap();
        let minting_call = retained.get_or_insert_with(|| call.clone());
        minting_call.projected(
            "replayed-call-projection",
            graph_read_material("replayed-call-projection"),
        )
    }

    fn touch_effect(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        Ok(call.completed("touch"))
    }
}

struct CurrentCallProvider;

impl<G> domain::WorthQueryGraphParticipationProvider<G> for CurrentCallProvider {
    fn execution_resource_support(&self) -> domain::WorthQueryExecutionResourceSupport {
        super::super::installed_operation_fixture::execution_resource_support()
    }

    fn observe(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        Ok(call.completed("observe"))
    }

    fn project(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        call.projected("projection", graph_read_material("current-call-projection"))
    }

    fn touch_effect(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        Ok(call.completed("touch"))
    }
}

impl<G> domain::WorthQueryGraphParticipationProvider<G> for CrossCallProvider {
    fn execution_resource_support(&self) -> domain::WorthQueryExecutionResourceSupport {
        super::super::installed_operation_fixture::execution_resource_support()
    }

    fn observe(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        if self.reuse_retained {
            let retained = self.retained.lock().unwrap();
            Ok(retained
                .as_ref()
                .expect("the first graph call was retained")
                .completed("cross-call-observe"))
        } else {
            Ok(call.completed("observe"))
        }
    }

    fn project(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        let mut retained = self.retained.lock().unwrap();
        if self.reuse_retained {
            let foreign = retained
                .as_ref()
                .expect("the first graph call was retained");
            foreign.projected(
                "cross-call-projection",
                graph_read_material("cross-call-projection"),
            )
        } else {
            *retained = Some(call.clone());
            call.projected(
                "first-projection",
                graph_read_material("first-call-projection"),
            )
        }
    }

    fn touch_effect(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        Ok(call.completed("touch"))
    }
}

#[test]
fn graph_receipt_from_another_exact_call_cannot_be_reused() {
    let retained = Arc::new(Mutex::new(None));
    let mut workspace =
        configured_runtime_for_package(federated_package::<RemoteA, RemoteB>(false))
            .graph_participation(read_definition::<RemoteA>(
                "remote-a",
                domain::WorthQueryGraphProjectionPosture::NativeProjection,
            ))
            .graph_participation_provider(
                RemoteA,
                CrossCallProvider {
                    retained: Arc::clone(&retained),
                    reuse_retained: false,
                },
            )
            .graph_participation(read_definition::<RemoteB>(
                "remote-b",
                domain::WorthQueryGraphProjectionPosture::NativeProjection,
            ))
            .graph_participation_provider(
                RemoteB,
                CrossCallProvider {
                    retained,
                    reuse_retained: true,
                },
            )
            .workspace("graph-provider-cross-call")
            .unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();
    let bound = workspace
        .observe_operating_world()
        .unwrap()
        .family(ReadFamily)
        .bind(&installed, FederatedRead)
        .unwrap();
    let denial = match bound
        .admit_execution_resources(
            (),
            crate::suite::installed_operation_fixture::execution_resource_request(),
            &workspace,
        )
        .unwrap()
        .execute(&mut workspace)
    {
        TransitionOutcome::Denied(denial) => denial,
        _ => panic!("cross-call graph receipt did not produce an exact denial"),
    };
    assert_eq!(
        denial.kind(),
        &domain::WorthQueryBoundExecutionDenialKind::GraphProvider
    );
    assert_eq!(denial.counters().graph_provider_contacts, 2);
    assert_eq!(denial.counters().executor_contacts, 0);
    assert_eq!(denial.graph_receipts().len(), 1);
}

#[test]
fn graph_receipt_cannot_be_replayed_across_bound_capabilities() {
    let retained = Arc::new(Mutex::new(None));
    let mut workspace =
        configured_runtime_for_package(federated_package::<RemoteA, RemoteB>(false))
            .graph_participation(read_definition::<RemoteA>(
                "remote-a",
                domain::WorthQueryGraphProjectionPosture::NativeProjection,
            ))
            .graph_participation_provider(RemoteA, ReplayedCallProvider { retained })
            .graph_participation(read_definition::<RemoteB>(
                "remote-b",
                domain::WorthQueryGraphProjectionPosture::NativeProjection,
            ))
            .graph_participation_provider(RemoteB, CurrentCallProvider)
            .workspace("graph-provider-replayed-call")
            .unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();
    workspace
        .observe_operating_world()
        .unwrap()
        .family(ReadFamily)
        .bind(&installed, FederatedRead)
        .unwrap()
        .admit_execution_resources(
            (),
            crate::suite::installed_operation_fixture::execution_resource_request(),
            &workspace,
        )
        .unwrap()
        .execute(&mut workspace)
        .unwrap();
    let second = workspace
        .observe_operating_world()
        .unwrap()
        .family(ReadFamily)
        .bind(&installed, FederatedRead)
        .unwrap();
    let denial = match second
        .admit_execution_resources(
            (),
            crate::suite::installed_operation_fixture::execution_resource_request(),
            &workspace,
        )
        .unwrap()
        .execute(&mut workspace)
    {
        TransitionOutcome::Denied(denial) => denial,
        _ => panic!("replayed graph receipt did not produce an exact denial"),
    };
    assert_eq!(
        denial.kind(),
        &domain::WorthQueryBoundExecutionDenialKind::GraphProvider
    );
    assert_eq!(denial.counters().graph_provider_contacts, 1);
    assert_eq!(denial.counters().executor_contacts, 0);
}
