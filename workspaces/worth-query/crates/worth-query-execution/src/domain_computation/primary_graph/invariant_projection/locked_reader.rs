use std::marker::PhantomData;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::sync::Arc;

use worth_query_installation::facade::{
    ApplicationFieldRef, ApplicationFieldUnit, ApplicationSchema, EqualityPredicate,
    TypedApplicationReadableValue, TypedApplicationValue, WritePosture,
};

use super::work::WorthQueryInvariantProjectionWorkBudget;
use super::{
    WorthQueryApplicationInvariantProjectionAuthority,
    WorthQueryApplicationInvariantProjectionSnapshot, WorthQueryInvariantEntityIdentity,
    WorthQueryInvariantProjectionWork, WorthQueryRealizedProjectionScope,
};
use crate::domain_computation::primary_graph::{
    WorthQueryEntityResolutionDenial, WorthQueryEntityResolutionDenialKind,
    WorthQueryPrincipalResolutionMode,
};

pub struct WorthQueryApplicationInvariantProjectionReader<'runtime, Schema> {
    pub(super) runtime: &'runtime mut worth_relational::facade::runtime::RelationalRuntime,
    pub(super) layout: &'runtime super::super::schema_layout::WorthQueryPrimaryGraphLayout,
    pub(super) snapshot: &'runtime worth_relational::facade::snapshots::SnapshotHandle,
    entity_resolution: &'runtime super::super::WorthQueryInstalledEntityResolutionContext,
    pub(super) authority_identity: u64,
    pub(super) work: WorthQueryInvariantProjectionWork,
    pub(super) work_budget: WorthQueryInvariantProjectionWorkBudget,
    pub(super) realized_scope: WorthQueryRealizedProjectionScope,
    pub(super) aggregate_projections:
        Arc<std::sync::Mutex<super::super::aggregate_projection::WorthQueryAggregateProjections>>,
    _schema: PhantomData<fn() -> Schema>,
}

pub struct WorthQueryCompletedInvariantProjection<Schema, Output> {
    output: Output,
    snapshot: WorthQueryApplicationInvariantProjectionSnapshot<Schema>,
    work: WorthQueryInvariantProjectionWork,
}

pub(super) struct WorthQueryInvariantProjectionWorkLimitExceeded;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInvariantProjectionTraversalDenialKind {
    RelationNotInstalled,
    UndeclaredDecisionTarget,
    ForeignIdentity,
    EndpointUnavailable,
    WorkBudgetExceeded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInvariantProjectionTraversalDenial {
    kind: WorthQueryInvariantProjectionTraversalDenialKind,
    relation: String,
}

impl<Schema> WorthQueryApplicationInvariantProjectionAuthority<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn project<Output>(
        &self,
        projection: impl FnOnce(
            &mut WorthQueryApplicationInvariantProjectionReader<'_, Schema>,
        ) -> Output,
    ) -> WorthQueryCompletedInvariantProjection<Schema, Output> {
        match self.project_with_work_budget(
            WorthQueryInvariantProjectionWorkBudget::unbounded(),
            projection,
        ) {
            Ok(completed) => completed,
            Err(_) => unreachable!("an unbounded invariant projection cannot exhaust work"),
        }
    }

    pub(super) fn project_bounded<Output>(
        &self,
        maximum_work: usize,
        projection: impl FnOnce(
            &mut WorthQueryApplicationInvariantProjectionReader<'_, Schema>,
        ) -> Output,
    ) -> Result<
        WorthQueryCompletedInvariantProjection<Schema, Output>,
        WorthQueryInvariantProjectionWorkLimitExceeded,
    > {
        self.project_with_work_budget(
            WorthQueryInvariantProjectionWorkBudget::bounded(maximum_work),
            projection,
        )
    }

    fn project_with_work_budget<Output>(
        &self,
        work_budget: WorthQueryInvariantProjectionWorkBudget,
        projection: impl FnOnce(
            &mut WorthQueryApplicationInvariantProjectionReader<'_, Schema>,
        ) -> Output,
    ) -> Result<
        WorthQueryCompletedInvariantProjection<Schema, Output>,
        WorthQueryInvariantProjectionWorkLimitExceeded,
    > {
        let projected = self.graph.with_runtime_mut(|runtime| {
            let snapshot = runtime.snapshots().historical_snapshot();
            let projected = catch_unwind(AssertUnwindSafe(|| {
                let mut reader = WorthQueryApplicationInvariantProjectionReader {
                    runtime,
                    layout: &self.layout,
                    snapshot: &snapshot,
                    entity_resolution: &self.entity_resolution,
                    authority_identity: self.authority_identity,
                    work: WorthQueryInvariantProjectionWork::default(),
                    work_budget,
                    realized_scope: WorthQueryRealizedProjectionScope::default(),
                    aggregate_projections: Arc::clone(&self.graph.aggregate_projections),
                    _schema: PhantomData,
                };
                let output = projection(&mut reader);
                (
                    output,
                    reader.work,
                    reader.realized_scope,
                    reader.work_budget.exceeded(),
                )
            }));
            match projected {
                Ok((output, work, realized_scope, exceeded)) => {
                    if exceeded {
                        let _ = runtime.snapshots().release_snapshot(&snapshot);
                        Ok(Err(WorthQueryInvariantProjectionWorkLimitExceeded))
                    } else {
                        Ok(Ok((output, snapshot, work, realized_scope)))
                    }
                }
                Err(payload) => {
                    let _ = runtime.snapshots().release_snapshot(&snapshot);
                    Err(payload)
                }
            }
        });
        let (output, snapshot, work, realized_scope) = match projected {
            Ok(Ok(completed)) => completed,
            Ok(Err(denial)) => return Err(denial),
            Err(payload) => resume_unwind(payload),
        };
        Ok(WorthQueryCompletedInvariantProjection {
            output,
            snapshot: WorthQueryApplicationInvariantProjectionSnapshot {
                graph: self.graph.clone(),
                layout: Arc::clone(&self.layout),
                snapshot: Some(snapshot),
                runtime_authority: self.runtime_authority,
                binding_identity: self.binding_identity.clone(),
                authority_identity: self.authority_identity,
                realized_scope,
                _schema: PhantomData,
            },
            work,
        })
    }
}

impl<Schema, Output> WorthQueryCompletedInvariantProjection<Schema, Output> {
    pub const fn output(&self) -> &Output {
        &self.output
    }

    pub const fn work(&self) -> WorthQueryInvariantProjectionWork {
        self.work
    }

    pub fn into_parts(
        self,
    ) -> (
        Output,
        WorthQueryApplicationInvariantProjectionSnapshot<Schema>,
        WorthQueryInvariantProjectionWork,
    ) {
        (self.output, self.snapshot, self.work)
    }
}

impl<Schema> WorthQueryApplicationInvariantProjectionReader<'_, Schema>
where
    Schema: ApplicationSchema,
{
    pub const fn version(&self) -> worth_relational::facade::identity::VersionId {
        self.snapshot.version_id
    }

    pub fn resolve_entity<Aspect, Entity, Field, Value, Write, Unit>(
        &mut self,
        field: ApplicationFieldRef<
            Schema,
            Entity,
            Aspect,
            Field,
            Value,
            Write,
            EqualityPredicate,
            Unit,
        >,
        value: Value,
    ) -> Result<WorthQueryInvariantEntityIdentity<Schema, Entity>, WorthQueryEntityResolutionDenial>
    where
        Value: TypedApplicationValue,
        Write: WritePosture,
        Unit: ApplicationFieldUnit,
    {
        if !self.work_budget.can_afford(3) {
            return Err(WorthQueryEntityResolutionDenial::new(
                WorthQueryEntityResolutionDenialKind::ProjectionWorkBudgetExceeded,
                field.field(),
            ));
        }
        let truth = self.entity_resolution.at_snapshot(
            self.runtime,
            self.snapshot,
            WorthQueryPrincipalResolutionMode::Ordinary,
        )?;
        let (resolved, examined) = truth.resolve_with_work(
            field.entity(),
            field.aspect(),
            field.field(),
            value.into_foundational_value(),
        );
        self.work_budget.consume(1 + examined);
        self.work.record_lookup(examined);
        let resolved = resolved?;
        self.realized_scope.record(resolved.entity_id());
        Ok(WorthQueryInvariantEntityIdentity {
            entity_id: resolved.entity_id(),
            kind: resolved.entity_kind(),
            entity: Arc::from(field.entity()),
            authority_identity: self.authority_identity,
            _marker: PhantomData,
        })
    }

    pub fn resolve_optional_entity<Aspect, Entity, Field, Value, Write, Unit>(
        &mut self,
        field: ApplicationFieldRef<
            Schema,
            Entity,
            Aspect,
            Field,
            Value,
            Write,
            EqualityPredicate,
            Unit,
        >,
        value: Value,
    ) -> Result<
        Option<WorthQueryInvariantEntityIdentity<Schema, Entity>>,
        WorthQueryEntityResolutionDenial,
    >
    where
        Value: TypedApplicationValue,
        Write: WritePosture,
        Unit: ApplicationFieldUnit,
    {
        match self.resolve_entity(field, value) {
            Ok(identity) => Ok(Some(identity)),
            Err(denial) if denial.kind() == WorthQueryEntityResolutionDenialKind::UnknownEntity => {
                Ok(None)
            }
            Err(denial) => Err(denial),
        }
    }

    pub fn field<Entity, Aspect, Field, Value, Write, Equality, Unit>(
        &mut self,
        identity: &WorthQueryInvariantEntityIdentity<Schema, Entity>,
        field: ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit>,
    ) -> Option<Value>
    where
        Value: TypedApplicationReadableValue,
        Write: WritePosture,
        Unit: ApplicationFieldUnit,
    {
        if !self.identity_is_local(identity, field.entity()) {
            return None;
        }
        if !self.work_budget.can_afford(1) {
            return None;
        }
        self.work_budget.consume(1);
        self.realized_scope.record(identity.entity_id);
        let locator = self
            .layout
            .field_locator(field.entity(), field.aspect(), field.field())?
            .clone();
        self.work.record_field();
        super::super::application_attempt::observe_field_value(
            self.runtime,
            self.snapshot,
            identity.entity_id,
            identity.kind,
            &locator,
        )
        .and_then(|value| Value::from_foundational_value(&value))
    }

    pub(super) fn identity_is_local<Entity>(
        &self,
        identity: &WorthQueryInvariantEntityIdentity<Schema, Entity>,
        entity: &str,
    ) -> bool {
        identity.authority_identity == self.authority_identity && identity.entity.as_ref() == entity
    }
}

impl WorthQueryInvariantProjectionTraversalDenial {
    pub const fn kind(&self) -> WorthQueryInvariantProjectionTraversalDenialKind {
        self.kind
    }

    pub fn relation(&self) -> &str {
        &self.relation
    }

    pub(super) fn new(
        kind: WorthQueryInvariantProjectionTraversalDenialKind,
        relation: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            relation: relation.into(),
        }
    }
}

impl std::fmt::Display for WorthQueryInvariantProjectionTraversalDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "invariant projection traversal denied: {:?} ({})",
            self.kind, self.relation
        )
    }
}

impl std::error::Error for WorthQueryInvariantProjectionTraversalDenial {}
