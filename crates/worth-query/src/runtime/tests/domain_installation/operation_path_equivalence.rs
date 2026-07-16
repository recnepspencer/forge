use super::*;
use crate::ordinary::read::current;
use crate::runtime::{
    WorthQueryGraphReadOperationLookup, WorthQueryGraphReadOperationRegistry,
    WorthQueryLiveGraphReadMaintenanceBudget,
};

#[test]
fn installed_operation_resolution_is_identical_across_every_runtime_path() {
    let runtime = installed_runtime();
    let handle = runtime.domain(InstalledDomain).unwrap();
    let operation = installed_operation_declaration(&handle);
    let family = operation_family(operation.clone());
    let oracle_registry = oracle_registry_from_installed_index(&runtime, &operation);
    let oracle_explanation =
        crate::runtime::explain_graph_read_access_shape_for_family_with_operation_lookup(
            &family,
            &oracle_registry,
        )
        .expect("the internal oracle must resolve the installed declaration");
    let lookup_baseline = runtime
        .domain_installation_lookup_counters()
        .indexed_operation_lookups();

    let admission = runtime
        .admit_graph_read_access_for_family(&family)
        .expect("ordinary admission must resolve the installed declaration");
    let mut workspace = runtime
        .workspace("installed-operation-equivalence")
        .unwrap();
    let ordinary_explanation = workspace
        .explain_graph_read_access_shape(&family)
        .expect("ordinary explanation must resolve the installed declaration");
    let live_plan = workspace
        .plan_live_graph_read_access(&family, WorthQueryLiveGraphReadMaintenanceBudget::bounded())
        .expect("live maintenance must resolve the installed declaration");
    let ordinary_completion = handle
        .read(|read| installed_operation_read(read, operation))
        .expect("the handle-bound ordinary read must declare")
        .using(current())
        .run(&mut workspace)
        .expect("the handle and workspace share runtime authority")
        .into_result()
        .expect("the installed operation must execute through ordinary read");

    let expected_shape = oracle_explanation.access_shape().digest().as_str();
    assert_eq!(
        admission.requirement_set().access_shape_digest(),
        expected_shape
    );
    assert_eq!(
        ordinary_explanation.access_shape().digest().as_str(),
        expected_shape
    );
    assert_eq!(live_plan.one_shot_access_shape_digest(), expected_shape);
    assert_eq!(
        ordinary_completion
            .receipt()
            .installed_authority()
            .witness_identity(),
        handle.authority_witness().witness_identity()
    );

    let counters = workspace.runtime.domain_installation_lookup_counters();
    assert_eq!(counters.indexed_operation_lookups() - lookup_baseline, 4);
    assert_eq!(counters.package_content_scans(), 0);
}

fn installed_operation_declaration(
    handle: &crate::domain_installation::WorthQueryInstalledDomainHandle<InstalledDomain>,
) -> WorthQueryGraphReadDomainOperationDeclaration {
    handle.graph_read_operation(
        &WorthQueryDomainGraphReadOperationDefinition::new(
            WorthQueryDomainIdentityName::new("neighbors").unwrap(),
            1,
        )
        .accepts_relation(RelationName::new("manager").unwrap()),
    )
}

fn oracle_registry_from_installed_index(
    runtime: &WorthQueryRuntime,
    operation: &WorthQueryGraphReadDomainOperationDeclaration,
) -> WorthQueryGraphReadOperationRegistry {
    let registration = runtime
        .installed_domain_execution_index()
        .matching_declared_operation(operation)
        .expect("the installed index must own the declared operation")
        .clone();
    let admitted = registration.admitted();
    assert_eq!(admitted.operation_name(), "neighbors");
    assert_eq!(admitted.operation_version(), 1);
    assert_eq!(admitted.domain_owner(), "WORTH.tests.installed-domain");
    assert_eq!(
        admitted.accepted_relation_names(),
        &[RelationName::new("manager").unwrap()]
    );
    assert_eq!(
        admitted.traversal_operator(),
        &WorthQueryGraphReadTraversalOperator::DeclarationTraversal
    );
    let provenance = admitted
        .installed_provenance()
        .expect("the compiled operation must retain installed provenance");
    assert_eq!(
        provenance.package_identity(),
        runtime
            .domain(InstalledDomain)
            .unwrap()
            .package_identity()
            .as_str()
    );
    WorthQueryGraphReadOperationRegistry::admit([registration])
        .expect("one installed registration is an unambiguous oracle")
}

fn installed_operation_read<Output>(
    read: WorthQueryReadBuilder<Output>,
    operation: WorthQueryGraphReadDomainOperationDeclaration,
) -> Result<Output, crate::runtime::WorthQueryReadDenial> {
    read.local_detail(
        "user",
        schema(),
        |query: DetailQueryBuilder| {
            query
                .project(AspectFieldSelector::new("identity", "id").unwrap())
                .domain_graph_operation(operation)
        },
        |shape: DetailResultShapeBuilder| {
            shape.field(
                crate::authoring::AuthoredResultShapeField::new("identity", "id", "id").unwrap(),
            )
        },
    )
}
