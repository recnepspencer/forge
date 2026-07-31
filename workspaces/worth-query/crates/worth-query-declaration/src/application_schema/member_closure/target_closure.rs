use worth_foundational::facade::ScalarAspectType;

use super::ClosureIndex;
use crate::application_schema::{
    ApplicationMutationPreconditionTarget, ApplicationOperationDecisionReadTarget,
    ApplicationOperationProgramTarget, ApplicationSchemaMember,
};

impl ClosureIndex<'_> {
    pub(super) fn field_matches(
        &self,
        entity_name: &str,
        aspect_name: &str,
        field_name: &str,
        expected_family: ScalarAspectType,
        expected_value_type: &str,
        expected_writable: bool,
        equality_required: bool,
    ) -> bool {
        self.members.iter().any(|member| {
            matches!(
                member,
                ApplicationSchemaMember::Field {
                    entity,
                    aspect,
                    field,
                    scalar_family,
                    value_type,
                    writable,
                    equality_queryable,
                    ..
                } if entity == entity_name
                    && aspect == aspect_name
                    && field == field_name
                    && *scalar_family == expected_family
                    && value_type == expected_value_type
                    && *writable == expected_writable
                    && (!equality_required || *equality_queryable)
            )
        })
    }

    pub(super) fn program_target_exists(&self, target: &ApplicationOperationProgramTarget) -> bool {
        match target {
            ApplicationOperationProgramTarget::Create { entity }
            | ApplicationOperationProgramTarget::Delete { entity } => {
                self.entities.contains(entity.as_str())
            }
            ApplicationOperationProgramTarget::Write {
                entity,
                aspect,
                field,
            } => self
                .fields
                .contains(&(entity.as_str(), aspect.as_str(), field.as_str())),
            ApplicationOperationProgramTarget::Link { relation, from, to }
            | ApplicationOperationProgramTarget::Unlink { relation, from, to } => self
                .relations
                .contains(&(relation.as_str(), from.as_str(), to.as_str())),
            ApplicationOperationProgramTarget::Emit { effect } => {
                self.effects.contains(effect.as_str())
            }
        }
    }

    pub(super) fn decision_read_target_exists(
        &self,
        target: &ApplicationOperationDecisionReadTarget,
    ) -> bool {
        match target {
            ApplicationOperationDecisionReadTarget::Entity { entity } => {
                self.entities.contains(entity.as_str())
            }
            ApplicationOperationDecisionReadTarget::Field {
                entity,
                aspect,
                field,
            } => self
                .fields
                .contains(&(entity.as_str(), aspect.as_str(), field.as_str())),
            ApplicationOperationDecisionReadTarget::Relation { relation, from, to } => self
                .relations
                .contains(&(relation.as_str(), from.as_str(), to.as_str())),
        }
    }

    pub(super) fn precondition_target_is_decision_read(
        &self,
        operation: &str,
        target: &ApplicationMutationPreconditionTarget,
    ) -> bool {
        self.members.iter().any(|member| {
            matches!(
                member,
                ApplicationSchemaMember::OperationDecisionRead {
                    operation: candidate_operation,
                    target: ApplicationOperationDecisionReadTarget::Field {
                        entity,
                        aspect,
                        field,
                    },
                } if candidate_operation == operation
                    && entity == target.entity()
                    && aspect == target.aspect()
                    && field == target.field_name()
            )
        })
    }
}
