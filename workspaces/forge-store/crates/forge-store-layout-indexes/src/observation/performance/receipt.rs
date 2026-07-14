use forge_foundational::{
    performance, performance_api, FoundationalAuthoritativePerformanceClaim,
    FoundationalCounterBackedPerformanceReceipt, FoundationalPerformanceAccessPatternPosture,
    FoundationalPerformanceBoundary, FoundationalPerformanceBreadthLocalityPosture,
    FoundationalPerformanceContractName, FoundationalPerformanceCounterName,
    FoundationalPerformanceCounterRow, FoundationalPerformanceCounterSpec,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceSupportingEvidenceCode, FoundationalPerformanceSupportingEvidenceRow,
    FoundationalPerformanceWorkClass,
};

use crate::BaselineBTreeLookupCounterReceipt;

/// Foundational performance evidence derived from an executed layout lookup.
///
/// The input receipt is owner-issued and opaque. Raw counter snapshots cannot
/// enter this boundary or independently claim executed layout work.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutAccessPerformanceReceipt {
    plan_binding: crate::AccessPlanIdentity,
    counter_backed:
        FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>,
}

impl LayoutAccessPerformanceReceipt {
    pub fn from_btree_lookup(executed: &BaselineBTreeLookupCounterReceipt) -> Self {
        let rows = counter_rows(executed.observed());
        let claim = performance()
            .claim()
            .authoritative_execution()
            .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
            .evidence_strength(
                FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt,
            )
            .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::PointLocal)
            .access_pattern(FoundationalPerformanceAccessPatternPosture::PointLookup)
            .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
            .freshness_retention(
                FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent,
            )
            .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
            .include_work(FoundationalPerformanceWorkClass::ValidationPlanning)
            .exclude_work(FoundationalPerformanceWorkClass::ReplayReconstruction)
            .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
            .exclude_work(FoundationalPerformanceWorkClass::ForensicParity)
            .finish()
            .expect("layout lookup performance claim is legal");
        let mut bundle = performance_api::lower_lane::basis::performance_bundle(claim)
            .attach_contract_name(
                FoundationalPerformanceContractName::new("store.layout.btree.point_lookup")
                    .expect("static layout performance contract name"),
            );
        for row in &rows {
            bundle = bundle.attach_counter_spec(FoundationalPerformanceCounterSpec::new(
                row.name().clone(),
                FoundationalPerformanceWorkClass::ValidationPlanning,
                row.observed_count(),
            ));
        }
        bundle = bundle.attach_supporting_evidence_row(
            FoundationalPerformanceSupportingEvidenceRow::new(
                FoundationalPerformanceSupportingEvidenceCode::new(
                    "store.layout.btree.point_lookup.counter_evidence",
                )
                .expect("static layout support evidence code"),
                FoundationalPerformanceWorkClass::SupportReportAssembly,
            ),
        );
        let bundle = bundle
            .finish()
            .expect("layout performance counters have unique names");
        let mut receipt =
            performance_api::lower_lane::receipts::counter_backed_performance_receipt(bundle);
        for row in rows {
            receipt = receipt.attach_counter_row(row);
        }
        Self {
            plan_binding: executed.plan_binding().clone(),
            counter_backed: receipt
                .finish()
                .expect("layout execution rows match their counter specifications"),
        }
    }

    pub const fn plan_binding(&self) -> &crate::AccessPlanIdentity {
        &self.plan_binding
    }

    pub const fn counter_backed(
        &self,
    ) -> &FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>
    {
        &self.counter_backed
    }
}

fn counter_rows(
    snapshot: crate::AccessPathCounterSnapshot,
) -> Vec<FoundationalPerformanceCounterRow> {
    [
        ("point_lookups", u64::from(snapshot.point_lookups())),
        ("range_lookups", u64::from(snapshot.range_lookups())),
        ("wal_replays", u64::from(snapshot.wal_replays())),
        ("publications", u64::from(snapshot.publications())),
        ("maintenance_reads", u64::from(snapshot.maintenance_reads())),
        ("page_touches", u64::from(snapshot.page_touches())),
        ("index_probes", u64::from(snapshot.index_probes())),
        ("key_comparisons", u64::from(snapshot.key_comparisons())),
        ("range_steps", u64::from(snapshot.range_steps())),
        ("prefix_steps", u64::from(snapshot.prefix_steps())),
        (
            "chunk_tree_node_reads",
            u64::from(snapshot.chunk_tree_node_reads()),
        ),
        ("manifest_reads", u64::from(snapshot.manifest_reads())),
        ("bytes_read", snapshot.bytes_read()),
        ("bytes_written", snapshot.bytes_written()),
        ("write_fanout", u64::from(snapshot.write_fanout())),
        (
            "read_amplification",
            u64::from(snapshot.read_amplification()),
        ),
        (
            "write_amplification",
            u64::from(snapshot.write_amplification()),
        ),
        ("allocation_events", snapshot.allocation_events()),
    ]
    .into_iter()
    .map(|(name, observed)| {
        FoundationalPerformanceCounterRow::new(
            FoundationalPerformanceCounterName::new(format!("store.layout.{name}"))
                .expect("static layout counter name"),
            observed,
        )
    })
    .collect()
}
