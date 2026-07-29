use worth_query_installation::facade::{
    ApplicationOperationProgramTarget, ApplicationRelationRef, OperationLinks, OperationUnlinks,
};

use super::{
    canonical_key, denial, WorthQueryApplicationAttemptDenial,
    WorthQueryApplicationAttemptDenialKind, WorthQueryApplicationEffectEntity,
    WorthQueryApplicationEffectProgramBuilder, WorthQueryApplicationRealizedEffect,
};
use crate::domain_computation::primary_graph::WorthQueryObservedApplicationRelation;

impl<Schema, Operation, Input, Scope>
    WorthQueryApplicationEffectProgramBuilder<Schema, Operation, Input, Scope>
{
    pub fn link<Relation, From, To>(
        &mut self,
        relation: ApplicationRelationRef<Schema, Relation, From, To>,
        key: impl Into<String>,
        from: &WorthQueryApplicationEffectEntity<Schema, From>,
        to: &WorthQueryApplicationEffectEntity<Schema, To>,
    ) -> Result<(), WorthQueryApplicationAttemptDenial>
    where
        Relation: OperationLinks<Operation>,
    {
        self.validate_target(from, relation.from())?;
        self.validate_target(to, relation.to())?;
        self.admit_program_target(&ApplicationOperationProgramTarget::Link {
            relation: relation.name().to_string(),
            from: relation.from().to_string(),
            to: relation.to().to_string(),
        })?;
        let layout = self.layout.relation(relation.name()).ok_or_else(|| {
            denial(
                WorthQueryApplicationAttemptDenialKind::UndeclaredEffect,
                relation.name(),
            )
        })?;
        let key = canonical_key(key.into(), relation.name())?;
        if !self.keys.insert((layout.kind, key.clone())) {
            return Err(denial(
                WorthQueryApplicationAttemptDenialKind::DuplicateEffectKey,
                relation.name(),
            ));
        }
        self.effects
            .push(WorthQueryApplicationRealizedEffect::CreateRelation {
                kind: layout.kind,
                key,
                from: from.reference.clone(),
                to: to.reference.clone(),
            });
        Ok(())
    }

    pub fn unlink<Relation, From, To>(
        &mut self,
        relation: ApplicationRelationRef<Schema, Relation, From, To>,
        observed: WorthQueryObservedApplicationRelation<Schema, Relation, From, To>,
    ) -> Result<(), WorthQueryApplicationAttemptDenial>
    where
        Relation: OperationUnlinks<Operation>,
    {
        self.admit_program_target(&ApplicationOperationProgramTarget::Unlink {
            relation: relation.name().to_string(),
            from: relation.from().to_string(),
            to: relation.to().to_string(),
        })?;
        for relation_id in observed.matching_relations {
            self.effects
                .push(WorthQueryApplicationRealizedEffect::DeleteRelation { relation_id });
        }
        Ok(())
    }
}
