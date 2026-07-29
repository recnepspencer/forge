use worth_foundational::facade::AspectValue;
use worth_query_installation::facade::{
    ApplicationFieldCurrency, ApplicationFieldRef, ApplicationRelationRef, ApplicationSchema,
    TypedApplicationSignedAggregateValue, WritePosture,
};

use super::{WorthQueryApplicationInvariantProjectionReader, WorthQueryInvariantEntityIdentity};
use crate::domain_computation::primary_graph::aggregate_projection::WorthQueryIncomingSumKey;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInvariantAggregateDenialKind {
    RelationNotInstalled,
    FieldNotInstalled,
    ForeignIdentity,
    WorkBudgetExceeded,
    InvalidScalar,
    ArithmeticOverflow,
    SourceCountOverflow,
    AmbiguousSourceRelation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInvariantAggregateDenial {
    kind: WorthQueryInvariantAggregateDenialKind,
    member: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInvariantAggregate<Value> {
    value: Value,
    source_count: u64,
}

impl<Value> WorthQueryInvariantAggregate<Value> {
    pub const fn value(&self) -> &Value {
        &self.value
    }

    pub const fn source_count(&self) -> u64 {
        self.source_count
    }

    pub fn into_value(self) -> Value {
        self.value
    }
}

impl WorthQueryInvariantAggregateDenial {
    pub const fn kind(&self) -> WorthQueryInvariantAggregateDenialKind {
        self.kind
    }

    pub fn member(&self) -> &str {
        &self.member
    }
}

impl<Schema> WorthQueryApplicationInvariantProjectionReader<'_, Schema>
where
    Schema: ApplicationSchema,
{
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
    ) -> Result<WorthQueryInvariantAggregate<Value>, WorthQueryInvariantAggregateDenial>
    where
        Value: TypedApplicationSignedAggregateValue,
        Write: WritePosture,
        Currency: ApplicationFieldCurrency,
    {
        let relation_layout = self
            .layout
            .relation(relation.name())
            .cloned()
            .ok_or_else(|| {
                denial(
                    WorthQueryInvariantAggregateDenialKind::RelationNotInstalled,
                    relation.name(),
                )
            })?;
        let source_kind = self.layout.entity_kind(field.entity()).ok_or_else(|| {
            denial(
                WorthQueryInvariantAggregateDenialKind::FieldNotInstalled,
                field.field(),
            )
        })?;
        let field_locator = self
            .layout
            .field_locator(field.entity(), field.aspect(), field.field())
            .cloned()
            .ok_or_else(|| {
                denial(
                    WorthQueryInvariantAggregateDenialKind::FieldNotInstalled,
                    field.field(),
                )
            })?;
        if relation_layout.from != source_kind
            || relation_layout.to != target.kind
            || !self.identity_is_local(target, relation.to())
        {
            return Err(denial(
                WorthQueryInvariantAggregateDenialKind::ForeignIdentity,
                relation.name(),
            ));
        }
        if !self.work_budget.can_afford(1) {
            return Err(denial(
                WorthQueryInvariantAggregateDenialKind::WorkBudgetExceeded,
                relation.name(),
            ));
        }
        let key = WorthQueryIncomingSumKey {
            relation_kind: relation_layout.kind,
            source_kind,
            target_kind: relation_layout.to,
            field: field_locator,
        };
        let version = self.snapshot.version_id;
        let cached = self
            .aggregate_projections
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .cached_aggregate(&key, target.entity_id, version);
        self.work_budget.consume(1);
        if let Some(aggregate) = cached {
            self.work.record_aggregate_lookup(true, 0);
            self.realized_scope.record(target.entity_id);
            return Ok(WorthQueryInvariantAggregate {
                value: Value::from_aggregate_i64(aggregate.sum),
                source_count: aggregate.source_count,
            });
        }
        let read = self
            .runtime
            .read_truth()
            .bounded_incoming_relations_of_kind_at_version(
                target.entity_id,
                relation_layout.kind,
                version,
                self.work_budget.remaining(),
            )
            .map_err(|limit| {
                self.work_budget.consume(limit.consumed_work_units());
                self.work.record_adjacency(
                    limit.relation_records_examined(),
                    limit.endpoint_records_reserved(),
                );
                self.work_budget.mark_exceeded();
                denial(
                    WorthQueryInvariantAggregateDenialKind::WorkBudgetExceeded,
                    relation.name(),
                )
            })?;
        let row_count = read.relation_records_examined();
        self.work_budget.consume(read.work_units());
        self.work
            .record_adjacency(row_count, read.endpoint_records_reserved());
        let mut sum = 0_i64;
        let mut source_count = 0_u64;
        for record in read.into_records() {
            if !self.work_budget.can_afford(1) {
                self.work_budget.mark_exceeded();
                return Err(denial(
                    WorthQueryInvariantAggregateDenialKind::WorkBudgetExceeded,
                    field.field(),
                ));
            }
            self.work_budget.consume(1);
            self.work.record_field();
            let source_relations = self
                .runtime
                .read_truth()
                .bounded_outgoing_relations_of_kind_at_version(
                    record.source,
                    relation_layout.kind,
                    version,
                    self.work_budget.remaining(),
                )
                .map_err(|limit| {
                    self.work_budget.consume(limit.consumed_work_units());
                    self.work.record_adjacency(
                        limit.relation_records_examined(),
                        limit.endpoint_records_reserved(),
                    );
                    self.work_budget.mark_exceeded();
                    denial(
                        WorthQueryInvariantAggregateDenialKind::WorkBudgetExceeded,
                        relation.name(),
                    )
                })?;
            self.work_budget.consume(source_relations.work_units());
            self.work.record_adjacency(
                source_relations.relation_records_examined(),
                source_relations.endpoint_records_reserved(),
            );
            let source_relations = source_relations.into_records();
            if source_relations.as_slice() != [record.clone()] {
                return Err(denial(
                    WorthQueryInvariantAggregateDenialKind::AmbiguousSourceRelation,
                    relation.name(),
                ));
            }
            let value =
                crate::domain_computation::primary_graph::application_attempt::observe_field_value(
                    self.runtime,
                    self.snapshot,
                    record.source,
                    source_kind,
                    &key.field,
                )
                .ok_or_else(|| {
                    denial(
                        WorthQueryInvariantAggregateDenialKind::InvalidScalar,
                        field.field(),
                    )
                })?;
            let AspectValue::Int64(value) = value else {
                return Err(denial(
                    WorthQueryInvariantAggregateDenialKind::InvalidScalar,
                    field.field(),
                ));
            };
            sum = sum.checked_add(value).ok_or_else(|| {
                denial(
                    WorthQueryInvariantAggregateDenialKind::ArithmeticOverflow,
                    field.field(),
                )
            })?;
            source_count = source_count.checked_add(1).ok_or_else(|| {
                denial(
                    WorthQueryInvariantAggregateDenialKind::SourceCountOverflow,
                    relation.name(),
                )
            })?;
        }
        self.work.record_aggregate_lookup(false, row_count);
        self.realized_scope.record(target.entity_id);
        self.aggregate_projections
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .retain_aggregate(key, target.entity_id, version, sum, source_count);
        Ok(WorthQueryInvariantAggregate {
            value: Value::from_aggregate_i64(sum),
            source_count,
        })
    }
}

fn denial(
    kind: WorthQueryInvariantAggregateDenialKind,
    member: impl Into<String>,
) -> WorthQueryInvariantAggregateDenial {
    WorthQueryInvariantAggregateDenial {
        kind,
        member: member.into(),
    }
}
