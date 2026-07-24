use worth_proof::TransitionOutcome;
use worth_query::facade::{domain, runtime};

use super::super::installed_operation_fixture::{
    evidence_workspace, EvidenceFamily, EvidenceRead, EvidenceScenario, GeometryDomain,
};

pub(super) fn admitted_receipt(
    name: &str,
    scenario: EvidenceScenario,
    redaction: domain::WorthQueryArtifactRedactionPosture,
) -> domain::WorthQueryBoundExecutionReceipt {
    let mut workspace = evidence_workspace(name, scenario, redaction).unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();
    let bound = workspace
        .observe_operating_world()
        .unwrap()
        .family(EvidenceFamily)
        .bind(&installed, EvidenceRead)
        .unwrap();
    bound.execute((), &mut workspace).unwrap().receipt().clone()
}

pub(super) fn denied_execution(
    name: &str,
    scenario: EvidenceScenario,
) -> domain::WorthQueryBoundExecutionDenial {
    let mut workspace = evidence_workspace(
        name,
        scenario,
        domain::WorthQueryArtifactRedactionPosture::NotRequired,
    )
    .unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();
    let bound = workspace
        .observe_operating_world()
        .unwrap()
        .family(EvidenceFamily)
        .bind(&installed, EvidenceRead)
        .unwrap();
    match bound.execute((), &mut workspace) {
        TransitionOutcome::Denied(denial) => denial,
        _ => panic!("dishonest domain evidence did not produce an admission denial"),
    }
}

pub(super) fn settled_honest_execution(
    name: &str,
) -> domain::WorthQuerySettledDomainProjection<
    GeometryDomain,
    EvidenceRead,
    EvidenceFamily,
    worth_query::facade::foundation::ObservationLaneWitness,
> {
    let mut workspace = evidence_workspace(
        name,
        EvidenceScenario::Honest,
        domain::WorthQueryArtifactRedactionPosture::NotRequired,
    )
    .unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();
    let bound = workspace
        .observe_operating_world()
        .unwrap()
        .family(EvidenceFamily)
        .bind(&installed, EvidenceRead)
        .unwrap();
    let consumer = bound.consumer_projection_contract().unwrap();
    bound
        .execute((), &mut workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume(
            consumer,
            worth_query::facade::read::project_facts().entity_identities(),
        )
        .unwrap()
        .settle()
        .unwrap()
}

pub(super) fn evidence(
    receipt: &domain::WorthQueryBoundExecutionReceipt,
) -> &domain::WorthQueryAdmittedDomainEvidence {
    receipt
        .domain_evidence()
        .expect("the installed contract requires admitted domain evidence")
}

pub(super) fn inspection(
    evidence: &domain::WorthQueryAdmittedDomainEvidence,
    policy: runtime::CausalInspectionRedactionPolicy,
) -> runtime::WorthQueryDomainEvidenceInspectionCopy {
    runtime::WorthQueryDomainEvidenceInspectionCopy::derive(evidence, policy)
}
