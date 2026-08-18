use std::collections::BTreeMap;

use worth_foundational::facade::{
    AspectValue, CanonicalFieldPath, ContractValidatedAspectValueView, FieldKey,
};
use worth_query::facade::runtime::WorthQueryPrimaryGraphSourceProjection;
use worth_query::facade::{domain, foundation, runtime};

pub struct FinancialSourceProjection {
    record: worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts,
    secondary_record: Option<worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts>,
    derive_risk_from_curve: bool,
    unrelated_portfolio_rows: usize,
}

impl FinancialSourceProjection {
    pub const fn curve_risk(
        record: worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts,
    ) -> Self {
        Self {
            record,
            secondary_record: None,
            derive_risk_from_curve: true,
            unrelated_portfolio_rows: 0,
        }
    }

    pub const fn committed_risk(
        record: worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts,
    ) -> Self {
        Self {
            record,
            secondary_record: None,
            derive_risk_from_curve: false,
            unrelated_portfolio_rows: 0,
        }
    }

    pub const fn portfolio(
        record: worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts,
        secondary_record: worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts,
        unrelated_portfolio_rows: usize,
    ) -> Self {
        Self {
            record,
            secondary_record: Some(secondary_record),
            derive_risk_from_curve: false,
            unrelated_portfolio_rows,
        }
    }
}

impl WorthQueryPrimaryGraphSourceProjection for FinancialSourceProjection {
    fn project_live_target(
        &self,
        graph: &worth_query_execution::facade::integration::WorthQueryPrimaryGraphIntegrationHandle,
        _target: &runtime::WorthQueryLiveArtifactTarget,
    ) -> Vec<foundation::WorthQueryEntity> {
        let mut rows = project_financial_record(graph, self.record, self.derive_risk_from_curve)
            .into_iter()
            .collect::<Vec<_>>();
        rows.extend(self.secondary_record.and_then(|record| {
            project_financial_record(graph, record, self.derive_risk_from_curve)
        }));
        rows.extend(unrelated_portfolio_rows(
            self.record,
            self.unrelated_portfolio_rows,
        ));
        rows
    }

    fn project_granular_scope(
        &self,
        graph: &worth_query_execution::facade::integration::WorthQueryPrimaryGraphIntegrationHandle,
        _target: &runtime::WorthQueryLiveArtifactTarget,
        scope: &domain::WorthQueryMaintenanceScope,
    ) -> Result<Vec<foundation::WorthQueryEntity>, foundation::WorthQueryWorkspaceError> {
        let requested = match scope {
            domain::WorthQueryMaintenanceScope::ExactSourceRecord {
                partition_id,
                local_slot,
                generation,
            } => worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts::entity(
                *partition_id,
                *local_slot,
                *generation,
            ),
            domain::WorthQueryMaintenanceScope::SourcePartition(partition)
                if partition == "usd-rates" =>
            {
                return Ok(self.project_records(graph))
            }
            domain::WorthQueryMaintenanceScope::WholeLogicalGraph => {
                return Ok(self.project_records(graph));
            }
            domain::WorthQueryMaintenanceScope::SourcePartition(_) => return Ok(Vec::new()),
        };
        if requested != self.record && self.secondary_record != Some(requested) {
            return Ok(Vec::new());
        }
        Ok(
            project_financial_record(graph, requested, self.derive_risk_from_curve)
                .into_iter()
                .collect(),
        )
    }
}

impl FinancialSourceProjection {
    fn project_records(
        &self,
        graph: &worth_query_execution::facade::integration::WorthQueryPrimaryGraphIntegrationHandle,
    ) -> Vec<foundation::WorthQueryEntity> {
        std::iter::once(self.record)
            .chain(self.secondary_record)
            .filter_map(|record| {
                project_financial_record(graph, record, self.derive_risk_from_curve)
            })
            .collect()
    }
}

fn unrelated_portfolio_rows(
    primary: worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts,
    count: usize,
) -> Vec<foundation::WorthQueryEntity> {
    (0..count)
        .filter_map(|offset| {
            let record = worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts::entity(
                primary.partition_id(),
                100_000 + offset as u64,
                1,
            );
            Some(foundation::WorthQueryEntity::from_native_field_values(
                foundation::WorthQueryEntityIdentity::from_bridge_record_projection(record),
                BTreeMap::from([
                    (
                        path("PortfolioFacts", "PortfolioValueField")?,
                        AspectValue::UInt64(1_000 + offset as u64),
                    ),
                    (
                        path("PortfolioFacts", "PortfolioDeskField")?,
                        AspectValue::String("rates".into()),
                    ),
                    (
                        path("PortfolioFacts", "PortfolioRankField")?,
                        AspectValue::UInt64(if offset == 0 {
                            2
                        } else {
                            10_000 + offset as u64
                        }),
                    ),
                ]),
            ))
        })
        .collect()
}

fn project_financial_record(
    graph: &worth_query_execution::facade::integration::WorthQueryPrimaryGraphIntegrationHandle,
    record: worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts,
    derive_risk_from_curve: bool,
) -> Option<foundation::WorthQueryEntity> {
    graph.with_runtime(|runtime| {
        let version = runtime
            .history()
            .historical_branch_head(&worth_relational::facade::history::BranchId("main".into()))?
            .version_id;
        let entity = worth_relational::facade::identity::EntityId::new(
            worth_relational::facade::identity::PartitionId::new(record.partition_id()),
            record.local_slot(),
            record.generation(),
        );
        let authoritative = runtime
            .read_truth()
            .visible_entity_at_version(entity, version)?;
        let mut fields = BTreeMap::new();
        for (aspect, value) in authoritative
            .authoritative_aspect_state?
            .aspects()
            .entries()
        {
            let ContractValidatedAspectValueView::Struct(value) = value.view() else {
                continue;
            };
            let aspect_key = FieldKey::new(aspect.as_str().to_owned())?;
            fields.extend(value.fields().filter_map(|(field, value)| {
                CanonicalFieldPath::new(vec![aspect_key.clone(), field.clone()])
                    .map(|path| (path, value.clone()))
            }));
        }
        if derive_risk_from_curve {
            let curve_path = path("CurveFacts", "CurveZeroRateField")?;
            let AspectValue::UInt64(curve_rate) = fields.get(&curve_path)? else {
                return None;
            };
            fields.insert(
                path("RiskFacts", "RiskValueField")?,
                AspectValue::UInt64(5_100 + curve_rate.saturating_sub(4_250) * 2),
            );
        }
        Some(foundation::WorthQueryEntity::from_native_field_values(
            foundation::WorthQueryEntityIdentity::from_bridge_record_projection(record),
            fields,
        ))
    })
}

fn path(aspect: &str, field: &str) -> Option<CanonicalFieldPath> {
    CanonicalFieldPath::new(vec![FieldKey::new(aspect)?, FieldKey::new(field)?])
}
