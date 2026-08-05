//! Contract-derived reads and effect target for capability revocation.

use crate::application_schema::{
    ApplicationOperationDecisionReadTarget, ApplicationOperationProgramTarget,
};

use super::{
    ApplicationCapabilityFieldBinding, ApplicationCapabilityRelationBinding,
    ErasedApplicationCapabilityContract,
};

#[doc(hidden)]
pub fn application_capability_revocation_program_target(
    contract: &ErasedApplicationCapabilityContract,
) -> Option<ApplicationOperationProgramTarget> {
    let revocation = contract.delegation().revocation()?;
    Some(write_target(revocation.revoked_status().field()))
}

#[doc(hidden)]
pub fn application_capability_revocation_decision_reads(
    contract: &ErasedApplicationCapabilityContract,
) -> Option<Vec<ApplicationOperationDecisionReadTarget>> {
    let revocation = contract.delegation().revocation()?;
    Some(vec![
        field_read(revocation.identity()),
        field_read(revocation.revoked_status().field()),
        relation_read(contract.target().resource()),
    ])
}

fn write_target(field: &ApplicationCapabilityFieldBinding) -> ApplicationOperationProgramTarget {
    ApplicationOperationProgramTarget::Write {
        entity: field.entity().to_owned(),
        aspect: field.aspect().to_owned(),
        field: field.field().to_owned(),
    }
}

fn field_read(field: &ApplicationCapabilityFieldBinding) -> ApplicationOperationDecisionReadTarget {
    ApplicationOperationDecisionReadTarget::Field {
        entity: field.entity().to_owned(),
        aspect: field.aspect().to_owned(),
        field: field.field().to_owned(),
    }
}

fn relation_read(
    relation: &ApplicationCapabilityRelationBinding,
) -> ApplicationOperationDecisionReadTarget {
    ApplicationOperationDecisionReadTarget::Relation {
        relation: relation.relation().to_owned(),
        from: relation.from().to_owned(),
        to: relation.to().to_owned(),
    }
}
