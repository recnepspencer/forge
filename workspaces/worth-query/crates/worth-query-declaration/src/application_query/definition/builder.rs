use std::marker::PhantomData;

use crate::application_query::{
    ApplicationQueryAuthorizationRequirement, ApplicationQueryBasisSupport,
    ApplicationQueryContinuationTarget, ApplicationQueryDefinitionDenial,
    ApplicationQueryDisclosureContract, ApplicationQueryLaneEligibility,
    ApplicationQueryLiveCauseBinding, ApplicationQueryLiveCauseContract,
    ApplicationQueryLiveResourceContract, ApplicationQueryOrderingDirection,
    ApplicationQueryOrderingTerm, ApplicationQueryParameterDefinition,
    ApplicationQueryParameterRef, ApplicationQueryReference, ApplicationQueryResultFieldRef,
    ApplicationQueryResultRelationRef, ApplicationQueryResultTraversal, ApplicationQueryRootPath,
    ManyResults, TypedApplicationQueryResultShape,
};
use crate::application_schema::{
    ApplicationAbilityRef, ApplicationEntityRef, ApplicationFieldCurrency, ApplicationFieldRef,
    EqualityCapable, EqualityPredicate, TypedApplicationValue,
};

use super::{
    ApplicationQueryCardinality, ApplicationQueryDefinition, ApplicationQueryDependencyCeiling,
    ApplicationQueryPredicate,
};

pub struct ApplicationQueryDefinitionBuilder<Schema, Query, Parameters, QueryResult, Scope> {
    definition: ApplicationQueryDefinition<Schema, Query, Parameters, QueryResult, Scope>,
}

impl<Schema, Query, Parameters, QueryResult, Scope>
    ApplicationQueryDefinitionBuilder<Schema, Query, Parameters, QueryResult, Scope>
{
    #[allow(clippy::too_many_arguments)]
    fn with_authorization<Root>(
        reference: ApplicationQueryReference<Schema, Query, Parameters, QueryResult, Scope>,
        root: ApplicationEntityRef<Schema, Root>,
        scope: ApplicationEntityRef<Schema, Scope>,
        result_shape: TypedApplicationQueryResultShape<Schema, Query, Root, QueryResult>,
        cardinality: ApplicationQueryCardinality,
        dependency_ceiling: ApplicationQueryDependencyCeiling,
        disclosure: ApplicationQueryDisclosureContract,
        basis_support: ApplicationQueryBasisSupport,
        lanes: ApplicationQueryLaneEligibility,
        authorization: ApplicationQueryAuthorizationRequirement,
    ) -> Self {
        Self {
            definition: ApplicationQueryDefinition {
                name: reference.name(),
                root_entity: root.name(),
                scope_entity: scope.name(),
                parameters: Vec::new(),
                result_shape: result_shape.into_erased(),
                root_paths: Vec::new(),
                cardinality,
                predicates: Vec::new(),
                ordering: Vec::new(),
                continuation: None,
                live_cause: None,
                dependency_ceiling,
                disclosure,
                authorization,
                basis_support,
                lanes,
                _marker: PhantomData,
            },
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn public<Root>(
        reference: ApplicationQueryReference<Schema, Query, Parameters, QueryResult, Scope>,
        root: ApplicationEntityRef<Schema, Root>,
        scope: ApplicationEntityRef<Schema, Scope>,
        result_shape: TypedApplicationQueryResultShape<Schema, Query, Root, QueryResult>,
        cardinality: ApplicationQueryCardinality,
        dependency_ceiling: ApplicationQueryDependencyCeiling,
        disclosure: ApplicationQueryDisclosureContract,
        basis_support: ApplicationQueryBasisSupport,
        lanes: ApplicationQueryLaneEligibility,
    ) -> Self {
        Self::with_authorization(
            reference,
            root,
            scope,
            result_shape,
            cardinality,
            dependency_ceiling,
            disclosure,
            basis_support,
            lanes,
            ApplicationQueryAuthorizationRequirement::public(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn requires_ability<Root, Ability>(
        reference: ApplicationQueryReference<Schema, Query, Parameters, QueryResult, Scope>,
        root: ApplicationEntityRef<Schema, Root>,
        scope: ApplicationEntityRef<Schema, Scope>,
        result_shape: TypedApplicationQueryResultShape<Schema, Query, Root, QueryResult>,
        cardinality: ApplicationQueryCardinality,
        dependency_ceiling: ApplicationQueryDependencyCeiling,
        disclosure: ApplicationQueryDisclosureContract,
        basis_support: ApplicationQueryBasisSupport,
        lanes: ApplicationQueryLaneEligibility,
        ability: ApplicationAbilityRef<Schema, Ability, Scope>,
    ) -> Self {
        Self::with_authorization(
            reference,
            root,
            scope,
            result_shape,
            cardinality,
            dependency_ceiling,
            disclosure,
            basis_support,
            lanes,
            ApplicationQueryAuthorizationRequirement::for_ability(ability),
        )
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

    pub fn where_equal<Root, Aspect, Field, Value, Write, Currency, Parameter>(
        mut self,
        field: ApplicationFieldRef<
            Schema,
            Root,
            Aspect,
            Field,
            Value,
            Write,
            EqualityPredicate,
            Currency,
        >,
        parameter: ApplicationQueryParameterRef<Query, Parameter, Value>,
    ) -> Self
    where
        Value: TypedApplicationValue,
        Currency: ApplicationFieldCurrency,
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

    pub fn order_by<Slot, Entity, Aspect, Field, Value, Write, Equality, Currency>(
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
            Currency,
        >,
        direction: ApplicationQueryOrderingDirection,
    ) -> Self
    where
        Value: TypedApplicationValue,
        Currency: ApplicationFieldCurrency,
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
        ScopeCurrency,
        TargetSlot,
        TargetAspect,
        TargetField,
        TargetCurrency,
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
            ScopeCurrency,
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
            TargetCurrency,
        >,
        resources: ApplicationQueryLiveResourceContract,
    ) -> Self
    where
        Binding: ApplicationQueryLiveCauseBinding<Schema, Query, Scope, Target>,
        ScopeCurrency: ApplicationFieldCurrency,
        TargetCurrency: ApplicationFieldCurrency,
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
            ScopeCurrency,
            TargetSlot,
            TargetAspect,
            TargetField,
            TargetCurrency,
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
