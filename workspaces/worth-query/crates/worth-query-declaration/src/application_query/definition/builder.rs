use std::marker::PhantomData;

use crate::application_query::{
    ApplicationQueryAuthorizationRequirement, ApplicationQueryContinuationTarget,
    ApplicationQueryDefinitionDenial, ApplicationQueryLiveCauseBinding,
    ApplicationQueryLiveCauseContract, ApplicationQueryLiveResourceContract,
    ApplicationQueryOrderingDirection, ApplicationQueryOrderingTerm,
    ApplicationQueryParameterDefinition, ApplicationQueryParameterRef, ApplicationQueryReference,
    ApplicationQueryResultFieldRef, ApplicationQueryResultRelationRef,
    ApplicationQueryResultTraversal, ApplicationQueryRootPath, ManyResults,
};
use crate::application_schema::{
    ApplicationFieldRef, ApplicationFieldUnit, EqualityCapable, EqualityPredicate,
    TypedApplicationValue,
};

use super::authoring::{ApplicationQueryDefinitionParts, ApplicationQueryRootAuthoring};
use super::{ApplicationQueryDefinition, ApplicationQueryPredicate};

pub struct ApplicationQueryDefinitionBuilder<Schema, Query, Parameters, QueryResult, Scope> {
    definition: ApplicationQueryDefinition<Schema, Query, Parameters, QueryResult, Scope>,
}

impl<Schema, Query, Parameters, QueryResult, Scope>
    ApplicationQueryDefinitionBuilder<Schema, Query, Parameters, QueryResult, Scope>
{
    pub fn declare(
        reference: ApplicationQueryReference<Schema, Query, Parameters, QueryResult, Scope>,
    ) -> ApplicationQueryRootAuthoring<Schema, Query, Parameters, QueryResult, Scope> {
        ApplicationQueryRootAuthoring::new(reference)
    }

    pub(super) fn with_authorization<Root>(
        parts: ApplicationQueryDefinitionParts<Schema, Query, Parameters, QueryResult, Scope, Root>,
        authorization: ApplicationQueryAuthorizationRequirement,
    ) -> Self {
        Self {
            definition: ApplicationQueryDefinition {
                name: parts.reference.name(),
                root_entity: parts.root.name(),
                scope_entity: parts.scope.name(),
                parameters: Vec::new(),
                result_shape: parts.result_shape.into_erased(),
                root_paths: Vec::new(),
                cardinality: parts.cardinality,
                predicates: Vec::new(),
                ordering: Vec::new(),
                continuation: None,
                live_cause: None,
                dependency_ceiling: parts.dependency_ceiling,
                disclosure: parts.disclosure,
                authorization,
                basis_support: parts.basis_support,
                lanes: parts.lanes,
                _marker: PhantomData,
            },
        }
    }

    pub fn parameter<Parameter, Value>(
        mut self,
        parameter: ApplicationQueryParameterRef<Query, Parameter, Value>,
    ) -> Self
    where
        Value: TypedApplicationValue,
    {
        self.definition
            .parameters
            .push(ApplicationQueryParameterDefinition::typed(parameter));
        self
    }

    pub fn root_path<Root>(mut self, path: ApplicationQueryRootPath<Schema, Scope, Root>) -> Self {
        self.definition.root_paths.push(path.into_meaning());
        self
    }

    pub fn where_equal<Root, Aspect, Field, Value, Write, Unit, Parameter>(
        mut self,
        field: ApplicationFieldRef<
            Schema,
            Root,
            Aspect,
            Field,
            Value,
            Write,
            EqualityPredicate,
            Unit,
        >,
        parameter: ApplicationQueryParameterRef<Query, Parameter, Value>,
    ) -> Self
    where
        Value: TypedApplicationValue,
        Unit: ApplicationFieldUnit,
        EqualityPredicate: EqualityCapable,
    {
        self.definition.predicates.push(ApplicationQueryPredicate {
            entity: field.entity(),
            aspect: field.aspect(),
            field: field.field(),
            parameter: parameter.name(),
            scalar_family: field.scalar_family(),
        });
        self
    }

    pub fn order_by<Slot, Entity, Aspect, Field, Value, Write, Equality, Unit>(
        mut self,
        field: ApplicationQueryResultFieldRef<
            Query,
            Slot,
            Schema,
            Entity,
            Aspect,
            Field,
            Value,
            Write,
            Equality,
            Unit,
        >,
        direction: ApplicationQueryOrderingDirection,
    ) -> Self
    where
        Value: TypedApplicationValue,
        Unit: ApplicationFieldUnit,
    {
        self.definition
            .ordering
            .push(ApplicationQueryOrderingTerm::from_result_field(
                field, direction,
            ));
        self
    }

    pub fn continue_by<Slot, Relation, From, To, Direction>(
        mut self,
        relation: ApplicationQueryResultRelationRef<
            Query,
            Slot,
            Schema,
            Relation,
            From,
            To,
            Direction,
            ManyResults,
        >,
    ) -> Self
    where
        Direction: ApplicationQueryResultTraversal,
    {
        self.definition.continuation = Some(
            ApplicationQueryContinuationTarget::from_many_relation(relation),
        );
        self
    }

    #[allow(clippy::type_complexity)]
    pub fn live_by<
        Target,
        Binding,
        ScopeSlot,
        ScopeAspect,
        ScopeField,
        ScopeUnit,
        TargetSlot,
        TargetAspect,
        TargetField,
        TargetUnit,
    >(
        mut self,
        scope_identity: ApplicationQueryResultFieldRef<
            Query,
            ScopeSlot,
            Schema,
            Scope,
            ScopeAspect,
            ScopeField,
            Binding::ScopeIdentity,
            crate::application_schema::ReadOnly,
            crate::application_schema::EqualityPredicate,
            ScopeUnit,
        >,
        target_identity: ApplicationQueryResultFieldRef<
            Query,
            TargetSlot,
            Schema,
            Target,
            TargetAspect,
            TargetField,
            Binding::TargetIdentity,
            crate::application_schema::ReadOnly,
            crate::application_schema::EqualityPredicate,
            TargetUnit,
        >,
        resources: ApplicationQueryLiveResourceContract,
    ) -> Self
    where
        Binding: ApplicationQueryLiveCauseBinding<Schema, Query, Scope, Target>,
        ScopeUnit: ApplicationFieldUnit,
        TargetUnit: ApplicationFieldUnit,
    {
        self.definition.live_cause = Some(ApplicationQueryLiveCauseContract::typed::<
            Schema,
            Query,
            Scope,
            Target,
            Binding,
            ScopeSlot,
            ScopeAspect,
            ScopeField,
            ScopeUnit,
            TargetSlot,
            TargetAspect,
            TargetField,
            TargetUnit,
        >(scope_identity, target_identity, resources));
        self
    }

    pub fn build(
        mut self,
    ) -> Result<
        ApplicationQueryDefinition<Schema, Query, Parameters, QueryResult, Scope>,
        ApplicationQueryDefinitionDenial,
    > {
        self.definition.parameters.sort();
        self.definition.predicates.sort();
        self.definition.root_paths.sort();
        self.definition.root_paths.dedup();
        crate::application_query::validation::validate_definition(&self.definition)?;
        Ok(self.definition)
    }
}
