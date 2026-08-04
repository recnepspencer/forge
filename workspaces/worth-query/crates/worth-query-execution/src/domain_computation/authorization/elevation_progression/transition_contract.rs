use worth_query_installation::facade::{
    ApplicationOperationDecisionReadTarget, ApplicationOperationProgramTarget,
};

use super::super::capability_registry::WorthQueryInstalledCapabilityPlan;

pub(super) fn lifecycle_decision_reads(
    installed: &WorthQueryInstalledCapabilityPlan,
) -> Vec<ApplicationOperationDecisionReadTarget> {
    let elevation = installed.contract.elevation().definition().unwrap();
    let review = elevation.review();
    let mut reads = [
        elevation.identity(),
        elevation.reason(),
        elevation.status(),
        elevation.validity().not_before(),
        elevation.validity().not_after(),
        review.identity(),
        review.kind().field(),
        review.status(),
    ]
    .into_iter()
    .map(|field| ApplicationOperationDecisionReadTarget::Field {
        entity: field.entity().to_string(),
        aspect: field.aspect().to_string(),
        field: field.field().to_string(),
    })
    .collect::<Vec<_>>();
    reads.extend(
        [
            elevation.requester(),
            elevation.approver(),
            elevation.grant(),
            review.relation(),
            review.scope(),
            review.reviewer(),
        ]
        .into_iter()
        .map(
            |relation| ApplicationOperationDecisionReadTarget::Relation {
                relation: relation.relation().to_string(),
                from: relation.from().to_string(),
                to: relation.to().to_string(),
            },
        ),
    );
    reads
}

pub(super) fn approval_program_targets(
    installed: &WorthQueryInstalledCapabilityPlan,
) -> Vec<ApplicationOperationProgramTarget> {
    let elevation = installed.contract.elevation().definition().unwrap();
    vec![
        write_target(elevation.status()),
        link_target(elevation.approver()),
    ]
}

pub(super) fn close_program_targets(
    installed: &WorthQueryInstalledCapabilityPlan,
) -> Vec<ApplicationOperationProgramTarget> {
    let elevation = installed.contract.elevation().definition().unwrap();
    vec![write_target(elevation.status())]
}

pub(super) fn review_program_targets(
    installed: &WorthQueryInstalledCapabilityPlan,
) -> Vec<ApplicationOperationProgramTarget> {
    let review = installed
        .contract
        .elevation()
        .definition()
        .unwrap()
        .review();
    vec![
        write_target(review.status()),
        link_target(review.reviewer()),
    ]
}

fn write_target(
    field: &worth_query_declaration::facade::application_capability::ApplicationCapabilityFieldBinding,
) -> ApplicationOperationProgramTarget {
    ApplicationOperationProgramTarget::Write {
        entity: field.entity().to_string(),
        aspect: field.aspect().to_string(),
        field: field.field().to_string(),
    }
}

fn link_target(
    relation: &worth_query_declaration::facade::application_capability::ApplicationCapabilityRelationBinding,
) -> ApplicationOperationProgramTarget {
    ApplicationOperationProgramTarget::Link {
        relation: relation.relation().to_string(),
        from: relation.from().to_string(),
        to: relation.to().to_string(),
    }
}
