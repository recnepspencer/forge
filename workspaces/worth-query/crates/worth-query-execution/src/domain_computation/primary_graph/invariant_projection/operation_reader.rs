use std::collections::BTreeSet;
use std::marker::PhantomData;
use std::sync::Arc;

use worth_query_installation::facade::{
    ApplicationFieldCurrency, ApplicationFieldRef, ApplicationOperationDecisionReadTarget,
    ApplicationRelationRef, ApplicationSchema, EqualityPredicate, OperationReads,
    TypedApplicationReadableValue, TypedApplicationValue, WritePosture,
};

mod decision_plan;

pub use decision_plan::{
    WorthQueryInvariantDecisionPlanDenial, WorthQueryInvariantDecisionPlanDenialKind,
};

use super::{
    WorthQueryApplicationInvariantProjectionAuthority,
    WorthQueryApplicationInvariantProjectionReader,
    WorthQueryApplicationInvariantProjectionSnapshot, WorthQueryCompletedInvariantProjection,
    WorthQueryInvariantAggregateDenial, WorthQueryInvariantEntityIdentity,
    WorthQueryInvariantProjectionTraversalDenial, WorthQueryInvariantProjectionWork,
    WorthQueryInvariantRelation, WorthQueryOperationProjectionDenial,
};
use crate::domain_computation::authorization::WorthQueryOperationAdmissionIdentity;
use crate::domain_computation::primary_graph::{
    application_attempt::WorthQueryApplicationFactKey, WorthQueryAdmittedApplicationOperation,
    WorthQueryEntityResolutionDenial,
};

pub struct WorthQueryApplicationOperationInvariantProjectionReader<
    'reader,
    'runtime,
    Schema,
    Operation,
> {
    reader: &'reader mut WorthQueryApplicationInvariantProjectionReader<'runtime, Schema>,
    admitted_decision_reads: Option<&'reader [ApplicationOperationDecisionReadTarget]>,
    decision_facts: &'reader mut BTreeSet<WorthQueryApplicationFactKey>,
    _operation: PhantomData<fn() -> Operation>,
}

pub struct WorthQueryCompletedOperationInvariantProjection<Schema, Operation, Output> {
    completed: WorthQueryCompletedInvariantProjection<
        Schema,
        (Output, BTreeSet<WorthQueryApplicationFactKey>),
    >,
    admission_identity: WorthQueryOperationAdmissionIdentity,
    _operation: PhantomData<fn() -> Operation>,
}

pub struct WorthQueryInspectedOperationInvariantProjection<Operation, Output> {
    output: Output,
    work: WorthQueryInvariantProjectionWork,
    _operation: PhantomData<fn() -> Operation>,
}

pub struct WorthQueryApplicationOperationInvariantProjectionSnapshot<Schema, Operation> {
    snapshot: WorthQueryApplicationInvariantProjectionSnapshot<Schema>,
    admission_identity: WorthQueryOperationAdmissionIdentity,
    decision_facts: BTreeSet<WorthQueryApplicationFactKey>,
    _operation: PhantomData<fn() -> Operation>,
}

impl<Schema> WorthQueryApplicationInvariantProjectionAuthority<Schema>
where
    Schema: ApplicationSchema,
{
    /// Runs an operation-typed inspection without retaining mutation authority.
    ///
    /// The inspection result has no snapshot extraction transition:
    ///
    /// ```compile_fail
    /// use worth_query_execution::facade::primary_graph::
    ///     WorthQueryApplicationInvariantProjectionAuthority;
    /// use worth_query_installation::facade::ApplicationSchema;
    ///
    /// struct Operation;
    ///
    /// fn inspection_cannot_become_projection<Schema: ApplicationSchema>(
    ///     authority: &WorthQueryApplicationInvariantProjectionAuthority<Schema>,
    /// ) {
    ///     let inspected = authority.project_operation::<Operation, _>(|_| ());
    ///     let _ = inspected.into_parts();
    /// }
    /// ```
    pub fn project_operation<Operation, Output>(
        &self,
        projection: impl FnOnce(
            &mut WorthQueryApplicationOperationInvariantProjectionReader<'_, '_, Schema, Operation>,
        ) -> Output,
    ) -> WorthQueryInspectedOperationInvariantProjection<Operation, Output> {
        let completed = self.project(|reader| {
            let mut decision_facts = BTreeSet::new();
            let mut operation_reader = WorthQueryApplicationOperationInvariantProjectionReader {
                reader,
                admitted_decision_reads: None,
                decision_facts: &mut decision_facts,
                _operation: PhantomData,
            };
            let output = projection(&mut operation_reader);
            (output, decision_facts)
        });
        let ((output, _), snapshot, work) = completed.into_parts();
        drop(snapshot);
        WorthQueryInspectedOperationInvariantProjection {
            output,
            work,
            _operation: PhantomData,
        }
    }

    pub fn project_admitted_operation<Operation, Input, Scope, Output>(
        &self,
        admission: &WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
        projection: impl FnOnce(
            &mut WorthQueryApplicationOperationInvariantProjectionReader<'_, '_, Schema, Operation>,
            &WorthQueryInvariantEntityIdentity<Schema, Scope>,
        ) -> Output,
    ) -> Result<
        WorthQueryCompletedOperationInvariantProjection<Schema, Operation, Output>,
        WorthQueryOperationProjectionDenial,
    > {
        admission.validate_projection_authority(self.runtime_authority, &self.binding_identity)?;
        let completed = self
            .project_bounded(
                admission.allowed_graph_contract().projection_work_budget(),
                |reader| {
                    let mut decision_facts = BTreeSet::new();
                    let mut operation_reader =
                        WorthQueryApplicationOperationInvariantProjectionReader {
                            reader,
                            admitted_decision_reads: Some(
                                admission.allowed_graph_contract().decision_reads(),
                            ),
                            decision_facts: &mut decision_facts,
                            _operation: PhantomData,
                        };
                    operation_reader
                        .reader
                        .realized_scope
                        .record(admission.scope_entity_id());
                    let scope = WorthQueryInvariantEntityIdentity {
                        entity_id: admission.scope_entity_id(),
                        kind: admission.scope_entity_kind(),
                        entity: Arc::from(admission.scope_entity_name()),
                        authority_identity: operation_reader.reader.authority_identity,
                        _marker: PhantomData,
                    };
                    let output = projection(&mut operation_reader, &scope);
                    (output, decision_facts)
                },
            )
            .map_err(|_| {
                WorthQueryOperationProjectionDenial::work_budget_exceeded(admission.operation())
            })?;
        Ok(WorthQueryCompletedOperationInvariantProjection {
            completed,
            admission_identity: admission.admission_identity(),
            _operation: PhantomData,
        })
    }
}

impl<Operation, Output> WorthQueryInspectedOperationInvariantProjection<Operation, Output> {
    pub const fn output(&self) -> &Output {
        &self.output
    }

    pub const fn work(&self) -> WorthQueryInvariantProjectionWork {
        self.work
    }

    pub fn into_output(self) -> Output {
        self.output
    }
}

impl<Schema, Operation, Output>
    WorthQueryCompletedOperationInvariantProjection<Schema, Operation, Output>
{
    pub const fn output(&self) -> &Output {
        &self.completed.output().0
    }

    pub const fn work(&self) -> WorthQueryInvariantProjectionWork {
        self.completed.work()
    }

    pub fn into_parts(
        self,
    ) -> (
        Output,
        WorthQueryApplicationOperationInvariantProjectionSnapshot<Schema, Operation>,
        WorthQueryInvariantProjectionWork,
    ) {
        let ((output, decision_facts), snapshot, work) = self.completed.into_parts();
        (
            output,
            WorthQueryApplicationOperationInvariantProjectionSnapshot {
                snapshot,
                admission_identity: self.admission_identity,
                decision_facts,
                _operation: PhantomData,
            },
            work,
        )
    }
}

impl<'reader, 'runtime, Schema, Operation>
    WorthQueryApplicationOperationInvariantProjectionReader<'reader, 'runtime, Schema, Operation>
where
    Schema: ApplicationSchema,
{
    pub const fn version(&self) -> worth_relational::facade::identity::VersionId {
        self.reader.version()
    }

    pub fn resolve_entity<Aspect, Entity, Field, Value, Write, Currency>(
        &mut self,
        field: ApplicationFieldRef<
            Schema,
            Entity,
            Aspect,
            Field,
            Value,
            Write,
            EqualityPredicate,
            Currency,
        >,
        value: Value,
    ) -> Result<WorthQueryInvariantEntityIdentity<Schema, Entity>, WorthQueryEntityResolutionDenial>
    where
        Field: OperationReads<Operation>,
        Value: TypedApplicationValue,
        Write: WritePosture,
        Currency: ApplicationFieldCurrency,
    {
        self.reader.resolve_entity(field, value)
    }

    pub fn resolve_optional_entity<Aspect, Entity, Field, Value, Write, Currency>(
        &mut self,
        field: ApplicationFieldRef<
            Schema,
            Entity,
            Aspect,
            Field,
            Value,
            Write,
            EqualityPredicate,
            Currency,
        >,
        value: Value,
    ) -> Result<
        Option<WorthQueryInvariantEntityIdentity<Schema, Entity>>,
        WorthQueryEntityResolutionDenial,
    >
    where
        Field: OperationReads<Operation>,
        Value: TypedApplicationValue,
        Write: WritePosture,
        Currency: ApplicationFieldCurrency,
    {
        self.reader.resolve_optional_entity(field, value)
    }

    pub fn field<Entity, Aspect, Field, Value, Write, Equality, Currency>(
        &mut self,
        identity: &WorthQueryInvariantEntityIdentity<Schema, Entity>,
        field: ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Currency>,
    ) -> Option<Value>
    where
        Field: OperationReads<Operation>,
        Value: TypedApplicationReadableValue,
        Write: WritePosture,
        Currency: ApplicationFieldCurrency,
    {
        self.reader.field(identity, field)
    }

    pub fn relations_from<Relation, From, To>(
        &mut self,
        relation: ApplicationRelationRef<Schema, Relation, From, To>,
        from: &WorthQueryInvariantEntityIdentity<Schema, From>,
    ) -> Result<
        Vec<WorthQueryInvariantRelation<Schema, Relation, From, To>>,
        WorthQueryInvariantProjectionTraversalDenial,
    >
    where
        Relation: OperationReads<Operation>,
    {
        self.reader.relations_from(relation, from)
    }

    pub fn relations_to<Relation, From, To>(
        &mut self,
        relation: ApplicationRelationRef<Schema, Relation, From, To>,
        to: &WorthQueryInvariantEntityIdentity<Schema, To>,
    ) -> Result<
        Vec<WorthQueryInvariantRelation<Schema, Relation, From, To>>,
        WorthQueryInvariantProjectionTraversalDenial,
    >
    where
        Relation: OperationReads<Operation>,
    {
        self.reader.relations_to(relation, to)
    }

    pub fn summarize_exclusive_incoming<
        Relation,
        From,
        To,
        Aspect,
        Field,
        Value,
        Write,
        Equality,
        Currency,
    >(
        &mut self,
        relation: ApplicationRelationRef<Schema, Relation, From, To>,
        field: ApplicationFieldRef<Schema, From, Aspect, Field, Value, Write, Equality, Currency>,
        target: &WorthQueryInvariantEntityIdentity<Schema, To>,
    ) -> Result<super::WorthQueryInvariantAggregate<Value>, WorthQueryInvariantAggregateDenial>
    where
        Relation: OperationReads<Operation>,
        Field: OperationReads<Operation>,
        Value: worth_query_installation::facade::TypedApplicationSignedAggregateValue,
        Write: WritePosture,
        Currency: ApplicationFieldCurrency,
    {
        self.reader
            .summarize_exclusive_incoming(relation, field, target)
    }
}

impl<Schema, Operation> WorthQueryApplicationOperationInvariantProjectionSnapshot<Schema, Operation>
where
    Schema: ApplicationSchema,
{
    pub fn version(&self) -> worth_relational::facade::identity::VersionId {
        self.snapshot.version()
    }

    pub(in crate::domain_computation::primary_graph) fn belongs_to(
        &self,
        runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
        binding_identity: &worth_query_installation::facade::ApplicationSchemaBindingIdentity,
        admission_identity: WorthQueryOperationAdmissionIdentity,
    ) -> bool {
        self.snapshot
            .belongs_to(runtime_authority, binding_identity)
            && self.admission_identity == admission_identity
    }

    pub(in crate::domain_computation::primary_graph) fn into_lease_and_realized_scope(
        self,
    ) -> (
        super::super::application_attempt::snapshot_lease::WorthQueryApplicationSnapshotLease,
        super::WorthQueryRealizedProjectionScope,
        BTreeSet<WorthQueryApplicationFactKey>,
    ) {
        let (lease, scope) = self.snapshot.into_lease_and_realized_scope();
        (lease, scope, self.decision_facts)
    }
}
