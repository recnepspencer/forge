use worth_query::facade::domain;

use super::super::{canonical_bundle, semantic_closure, GeometryDomain, ReadFamily, WorkflowRead};
use super::contract::{add_evidence_graph_read, evidence_contract, EvidenceGovernance};
use super::graph_receipt::EvidenceGraph;

pub(super) fn workflow_package(
    redaction: domain::WorthQueryArtifactRedactionPosture,
) -> domain::WorthQueryDomainPackage<GeometryDomain> {
    workflow_package_with_graph(redaction, false)
}

pub(super) fn workflow_graph_package(
    redaction: domain::WorthQueryArtifactRedactionPosture,
) -> domain::WorthQueryDomainPackage<GeometryDomain> {
    workflow_package_with_graph(redaction, true)
}

fn workflow_package_with_graph(
    redaction: domain::WorthQueryArtifactRedactionPosture,
    graph: bool,
) -> domain::WorthQueryDomainPackage<GeometryDomain> {
    let contract = evidence_contract(EvidenceGovernance::retained(redaction));
    let mut stages = super::super::workflow::valid_stages()
        .into_iter()
        .map(|stage| {
            if !matches!(stage.identity(), "start" | "left") {
                return stage;
            }
            let mut semantics = stage.semantics().clone();
            semantics.evidence =
                domain::WorthQueryDomainEvidenceContract::installed_artifact(contract.reference());
            stage.with_semantics(semantics)
        })
        .collect::<Vec<_>>();
    if graph {
        let mut start = stages[0].semantics().clone();
        start.graph_read_roles.push("evidence-graph".into());
        start
            .cost_roles
            .push(domain::WorthQueryWorkflowCostRole::GraphRead);
        stages[0] = stages[0].clone().with_semantics(start);
    }
    let workflow = domain::WorthQueryPortableWorkflowDefinition::new("start", stages);
    let mut semantics = semantic_closure(
        canonical_bundle("Vertex"),
        domain::WorthQuerySupportRequirement::Required,
        true,
    );
    semantics.lowering = domain::WorthQueryOperationLoweringContract {
        family: "domain-evidence-workflow-v1".into(),
        deterministic: true,
    };
    semantics.replay = domain::WorthQueryOperationReplayContract::CertReplayable {
        comparator: domain::WorthQueryOperationReplayComparatorContract {
            family: "domain-evidence-workflow-exact-v1",
        },
    };
    semantics.workflow = domain::WorthQueryOperationWorkflowContract::Declared(workflow);
    if graph {
        add_evidence_graph_read(&mut semantics.graph_reads);
        semantics.cost.execution = domain::WorthQueryOperationCostClass::ExternalBoundary;
    }
    let operation = domain::WorthQueryDomainOperationDefinition::<
        GeometryDomain,
        WorkflowRead,
        ReadFamily,
    >::new(
        domain::WorthQueryDomainOperationIdentity::new("workflow-read", 1),
        semantics,
    );
    let package = domain::WorthQueryDomainPackage::declare(
        GeometryDomain,
        domain::WorthQueryDomainIdentityDeclaration::new(
            domain::WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
            domain::WorthQueryDomainIdentityName::new("geometry").unwrap(),
            domain::WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .operation(operation)
    .artifact_contract(contract);
    if graph {
        package.operation_graph_participation::<WorkflowRead, ReadFamily, EvidenceGraph>(
            "evidence-graph",
        )
    } else {
        package
    }
}
