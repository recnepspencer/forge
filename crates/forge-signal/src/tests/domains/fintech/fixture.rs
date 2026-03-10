use crate::facade::{
    AspectVersion, ChangedRegion, NodeEvaluationResult, NodeId, SignalBranchHandle, SignalGraph,
    SignalRuntime, SignalRuntimePolicy, SignalSnapshotId, SignalSnapshotV1, StageExecutor,
};

use super::audit_surface::PrimaryAuditSurface;
use super::branch_checkpoint::BranchCheckpoint;
use super::evaluation::FintechEvaluationShape;
use super::market_seed::MarketSeed;
use super::node_families::{
    build_aggregate_sources, build_bucket_exposure_nodes, build_bucket_sources, build_fx_nodes,
    build_instrument_nodes, build_partition_locality_nodes, build_scenario_nodes,
    build_scenario_sources, AggregateSourceNodes, FintechRuntime, FxNodes, InstrumentNodes,
};
use super::partition_surface::{MarketPartition, PartitionDetail, PartitionSurfaceNodes};
use super::regimes::MarketRegime;
use super::scales::FintechScale;
use super::world_handles::FintechWorldHandles;

#[derive(Debug)]
pub(super) struct InstrumentFixture {
    pub instrument_index: usize,
    pub book_index: usize,
    pub core: InstrumentNodes,
    pub buckets: Vec<NodeId>,
    pub scenarios: Vec<NodeId>,
}

pub(super) struct FintechWorld {
    pub runtime: FintechRuntime,
    pub handles: FintechWorldHandles,
    pub fx: FxNodes,
    pub aggregate_sources: Vec<AggregateSourceNodes>,
    pub curve_buckets: Vec<NodeId>,
    pub vol_surface_buckets: Vec<NodeId>,
    pub scenario_sources: Vec<NodeId>,
    pub instruments: Vec<InstrumentFixture>,
    pub book_aggregates: Vec<NodeId>,
    pub desk_aggregates: Vec<NodeId>,
    pub scenario_aggregates: Vec<NodeId>,
    pub bucket_aggregates: Vec<NodeId>,
}

pub(super) type FintechDomainFixture = FintechWorld;

impl FintechWorld {
    pub(super) fn live_node_count(&self) -> usize {
        self.runtime.graph().live_node_ids().len()
    }

    pub(super) fn seed_regime(
        &mut self,
        regime: MarketRegime,
        seed: u64,
    ) -> Result<(), crate::facade::SignalError> {
        super::market_state::seed_market_regime(self, regime, seed)
    }

    pub(super) fn seed_market(
        &mut self,
        market_seed: MarketSeed,
    ) -> Result<(), crate::facade::SignalError> {
        self.seed_regime(market_seed.regime, market_seed.seed)
    }

    pub(super) fn open_branch(
        &mut self,
        name: &str,
    ) -> Result<SignalBranchHandle, crate::facade::SignalError> {
        super::branch_history::create_branch(self, name)
    }

    pub(super) fn current_branch(&self) -> SignalBranchHandle {
        self.runtime.current_branch()
    }

    pub(super) fn switch_branch(
        &mut self,
        branch: SignalBranchHandle,
    ) -> Result<(), crate::facade::SignalError> {
        self.runtime.switch_branch(branch)
    }

    pub(super) fn branch_head_snapshot_id(
        &self,
        branch: SignalBranchHandle,
    ) -> Option<SignalSnapshotId> {
        self.runtime.branch_head_snapshot_id(branch.id)
    }

    pub(super) fn capture_world_snapshot(&mut self) -> SignalSnapshotV1 {
        super::branch_history::capture_active_snapshot(self)
    }

    pub(super) fn capture_branch_snapshot(
        &mut self,
        branch: SignalBranchHandle,
    ) -> Result<SignalSnapshotV1, crate::facade::SignalError> {
        super::branch_history::capture_branch_snapshot(self, branch)
    }

    pub(super) fn restore_saved_snapshot(
        &mut self,
        branch: SignalBranchHandle,
        snapshot: &SignalSnapshotV1,
    ) -> Result<(), crate::facade::SignalError> {
        super::branch_history::restore_branch_snapshot(self, branch, snapshot)
    }

    pub(super) fn replay_for_branch(
        &self,
        branch: SignalBranchHandle,
    ) -> crate::facade::ReplaySlice {
        super::branch_history::replay_for_branch(self, branch)
    }

    pub(super) fn replay_around_saved_snapshot(
        &self,
        snapshot: &SignalSnapshotV1,
    ) -> crate::facade::ReplaySlice {
        super::branch_history::replay_around_snapshot(self, snapshot)
    }

    pub(super) fn main_risk_lineage(&self) -> Vec<crate::facade::LineageRecord> {
        super::branch_history::lineage_for_main_risk(self)
    }

    pub(super) fn assert_shape(&self, scale: FintechScale) {
        super::world_shape::assert_world_shape(self, scale)
    }

    pub(super) fn top_desk(&self) -> crate::facade::NodeId {
        self.handles.aggregate.top_desk
    }

    pub(super) fn top_scenario(&self) -> crate::facade::NodeId {
        self.handles.aggregate.top_scenario
    }

    pub(super) fn main_risk_node(&self) -> crate::facade::NodeId {
        self.handles.primary.risk
    }

    pub(super) fn primary_threshold_node(&self) -> crate::facade::NodeId {
        self.handles.primary.threshold
    }

    pub(super) fn primary_market_source(&self) -> crate::facade::NodeId {
        self.handles.primary.market_source
    }

    pub(super) fn evaluation_shape(&self) -> FintechEvaluationShape {
        FintechEvaluationShape::from_fixture(self)
    }

    pub(super) fn node_state(
        &self,
        node: crate::facade::NodeId,
    ) -> Result<crate::facade::NodeState, crate::facade::SignalError> {
        self.runtime.graph().get_state(node)
    }

    pub(super) fn partitioned_market_source(&self) -> crate::facade::NodeId {
        self.handles.partition.market_regions
    }

    pub(super) fn rates_partition_node(&self) -> crate::facade::NodeId {
        self.handles.partition.rates_partition
    }

    pub(super) fn credit_partition_node(&self) -> crate::facade::NodeId {
        self.handles.partition.credit_partition
    }

    pub(super) fn rates_bucket_zero_node(&self) -> crate::facade::NodeId {
        self.handles.partition.rates_bucket_zero
    }

    pub(super) fn coarse_partition_book_node(&self) -> crate::facade::NodeId {
        self.handles.partition.coarse_book
    }

    pub(super) fn read_node_with_executor(
        &mut self,
        node: crate::facade::NodeId,
        executor: StageExecutor,
    ) -> Result<AspectVersion, crate::facade::SignalError> {
        let evaluation = self.evaluation_shape();
        let precompute = evaluation.precompute();
        self.runtime.read_with_executor(node, &precompute, executor)
    }

    pub(super) fn read_top_desk_with_executor(
        &mut self,
        executor: StageExecutor,
    ) -> Result<AspectVersion, crate::facade::SignalError> {
        self.read_node_with_executor(self.top_desk(), executor)
    }

    pub(super) fn read_top_scenario_with_executor(
        &mut self,
        executor: StageExecutor,
    ) -> Result<AspectVersion, crate::facade::SignalError> {
        self.read_node_with_executor(self.top_scenario(), executor)
    }

    pub(super) fn read_primary_threshold_with_executor(
        &mut self,
        executor: StageExecutor,
    ) -> Result<AspectVersion, crate::facade::SignalError> {
        self.read_node_with_executor(self.primary_threshold_node(), executor)
    }

    pub(super) fn read_primary_market_source_with_executor(
        &mut self,
        executor: StageExecutor,
    ) -> Result<AspectVersion, crate::facade::SignalError> {
        self.read_node_with_executor(self.primary_market_source(), executor)
    }

    pub(super) fn read_bucket_aggregate_with_executor(
        &mut self,
        index: usize,
        executor: StageExecutor,
    ) -> Result<AspectVersion, crate::facade::SignalError> {
        self.read_node_with_executor(self.bucket_aggregates[index], executor)
    }

    pub(super) fn read_scenario_aggregate_with_executor(
        &mut self,
        index: usize,
        executor: StageExecutor,
    ) -> Result<AspectVersion, crate::facade::SignalError> {
        self.read_node_with_executor(self.scenario_aggregates[index], executor)
    }

    pub(super) fn read_primary_audit_surface(
        &mut self,
        executor: StageExecutor,
    ) -> Result<PrimaryAuditSurface, crate::facade::SignalError> {
        let desk = self.read_top_desk_with_executor(executor)?;
        let scenario = self.read_top_scenario_with_executor(executor)?;
        Ok(PrimaryAuditSurface::new(desk, scenario))
    }

    pub(super) fn read_rates_partition_with_executor(
        &mut self,
        executor: StageExecutor,
    ) -> Result<AspectVersion, crate::facade::SignalError> {
        self.read_node_with_executor(self.rates_partition_node(), executor)
    }

    pub(super) fn read_credit_partition_with_executor(
        &mut self,
        executor: StageExecutor,
    ) -> Result<AspectVersion, crate::facade::SignalError> {
        self.read_node_with_executor(self.credit_partition_node(), executor)
    }

    pub(super) fn read_rates_bucket_zero_with_executor(
        &mut self,
        executor: StageExecutor,
    ) -> Result<AspectVersion, crate::facade::SignalError> {
        self.read_node_with_executor(self.rates_bucket_zero_node(), executor)
    }

    pub(super) fn read_coarse_partition_book_with_executor(
        &mut self,
        executor: StageExecutor,
    ) -> Result<AspectVersion, crate::facade::SignalError> {
        self.read_node_with_executor(self.coarse_partition_book_node(), executor)
    }

    pub(super) fn refresh_primary_audit_surface(
        &mut self,
        executor: StageExecutor,
    ) -> Result<PrimaryAuditSurface, crate::facade::SignalError> {
        let top_desk = self.top_desk();
        let top_scenario = self.top_scenario();
        let evaluation = self.evaluation_shape();
        let precompute = evaluation.precompute();
        self.runtime.transaction(&mut (), |tx| {
            tx.read_with_executor(top_desk, &precompute, executor)?;
            tx.read_with_executor(top_scenario, &precompute, executor)?;
            Ok(())
        })?;
        self.read_primary_audit_surface(executor)
    }

    pub(super) fn inject_primary_market_rollback(
        &mut self,
        executor: StageExecutor,
    ) -> Result<(), crate::facade::SignalError> {
        let top_desk = self.top_desk();
        let evaluation = self.evaluation_shape();
        let precompute = evaluation.precompute();
        let source = self.primary_market_source();
        let err = self.runtime.transaction(&mut (), |tx| {
            tx.mark_dirty(source, super::aspects::PRICE)?;
            tx.mark_dirty(source, super::aspects::VOL)?;
            tx.read(source, &|_node, view| {
                Ok(view.finish(
                    crate::facade::NodeEvaluationResult::from_version(
                        crate::facade::AspectVersion::from_updates([
                            (super::aspects::PRICE, 99_999),
                            (super::aspects::VOL, 99_999),
                            (super::aspects::CURVE, 99_999),
                            (super::aspects::LIQUIDITY, 99_999),
                            (super::aspects::RISK, 99_999),
                            (super::aspects::ALERT, 1),
                        ]),
                    )
                    .with_output_identity("bad-branch-correction"),
                ))
            })?;
            tx.read_with_executor(top_desk, &precompute, executor)?;
            Err(crate::facade::SignalError::invalid_input(
                "synthetic analysis rollback",
            ))
        });
        match err {
            Ok(_) => Err(crate::facade::SignalError::invalid_input(
                "synthetic rollback unexpectedly committed",
            )),
            Err(_) => Ok(()),
        }
    }

    pub(super) fn bump_primary_market(
        &mut self,
        price_delta: i64,
        vol_delta: i64,
        curve_delta: i64,
        liquidity_delta: i64,
        executor: StageExecutor,
    ) -> Result<AspectVersion, crate::facade::SignalError> {
        let source = self.primary_market_source();
        let current = self.read_node_with_executor(source, executor)?;
        let bumped = crate::facade::AspectVersion::from_updates([
            (
                super::aspects::PRICE,
                apply_signed_delta(current.get(super::aspects::PRICE), price_delta),
            ),
            (
                super::aspects::VOL,
                apply_signed_delta(current.get(super::aspects::VOL), vol_delta),
            ),
            (
                super::aspects::CURVE,
                apply_signed_delta(current.get(super::aspects::CURVE), curve_delta),
            ),
            (
                super::aspects::LIQUIDITY,
                apply_signed_delta(current.get(super::aspects::LIQUIDITY), liquidity_delta),
            ),
            (
                super::aspects::RISK,
                apply_signed_delta(current.get(super::aspects::RISK), price_delta + vol_delta),
            ),
            (
                super::aspects::ALERT,
                current.get(super::aspects::ALERT),
            ),
        ]);

        self.runtime.transaction(&mut (), |tx| {
            tx.mark_dirty(source, super::aspects::PRICE)?;
            tx.mark_dirty(source, super::aspects::VOL)?;
            tx.mark_dirty(source, super::aspects::CURVE)?;
            tx.mark_dirty(source, super::aspects::LIQUIDITY)?;
            tx.mark_dirty(source, super::aspects::RISK)?;
            tx.read(source, &|_node, view| {
                Ok(view.finish(
                    crate::facade::NodeEvaluationResult::from_version(bumped)
                        .with_output_identity("primary-market-bump"),
                ))
            })?;
            Ok(())
        })?;

        self.read_node_with_executor(source, executor)
    }

    pub(super) fn apply_partition_shock(
        &mut self,
        partition: MarketPartition,
        detail: Option<PartitionDetail>,
        price_delta: i64,
        executor: StageExecutor,
    ) -> Result<AspectVersion, crate::facade::SignalError> {
        let source = self.partitioned_market_source();
        let current = self.read_node_with_executor(source, executor)?;
        let bumped = crate::facade::AspectVersion::from_updates([
            (
                super::aspects::PRICE,
                apply_signed_delta(current.get(super::aspects::PRICE), price_delta),
            ),
            (
                super::aspects::RISK,
                apply_signed_delta(current.get(super::aspects::RISK), price_delta),
            ),
        ]);
        let changed_region = match detail {
            Some(detail) => ChangedRegion::new(partition.token()).with_detail(detail.token()),
            None => ChangedRegion::new(partition.token()),
        };

        self.runtime.transaction(&mut (), |tx| {
            tx.mark_dirty_with_regions(source, super::aspects::PRICE, &[changed_region.clone()])?;
            tx.read(source, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(bumped)
                        .with_output_identity(format!(
                            "partition-shock-{}-{}",
                            partition.token(),
                            detail.map(PartitionDetail::token).unwrap_or("whole")
                        ))
                        .with_changed_region(changed_region.clone()),
                ))
            })?;
            Ok(())
        })?;

        self.read_node_with_executor(source, executor)
    }

    pub(super) fn shock_rates_bucket_zero(
        &mut self,
        price_delta: i64,
        executor: StageExecutor,
    ) -> Result<AspectVersion, crate::facade::SignalError> {
        self.apply_partition_shock(
            MarketPartition::Rates,
            Some(PartitionDetail::Bucket0),
            price_delta,
            executor,
        )
    }

    pub(super) fn shock_credit_partition(
        &mut self,
        price_delta: i64,
        executor: StageExecutor,
    ) -> Result<AspectVersion, crate::facade::SignalError> {
        self.apply_partition_shock(MarketPartition::Credit, None, price_delta, executor)
    }

    pub(super) fn capture_active_checkpoint(
        &mut self,
        executor: StageExecutor,
    ) -> Result<BranchCheckpoint, crate::facade::SignalError> {
        let branch = self.current_branch();
        let audit = self.read_primary_audit_surface(executor)?;
        let snapshot = self.capture_branch_snapshot(branch.clone())?;
        Ok(BranchCheckpoint::new(branch, snapshot, audit))
    }

    pub(super) fn restore_checkpoint(
        &mut self,
        checkpoint: &BranchCheckpoint,
    ) -> Result<(), crate::facade::SignalError> {
        self.restore_saved_snapshot(checkpoint.branch.clone(), &checkpoint.snapshot)
    }

    pub(super) fn attempt_cross_branch_restore(
        &mut self,
        branch: SignalBranchHandle,
        snapshot: &SignalSnapshotV1,
    ) -> Result<(), crate::facade::SignalError> {
        self.restore_saved_snapshot(branch, snapshot)
    }

    pub(super) fn attempt_incompatible_profile_restore(
        &mut self,
        branch: SignalBranchHandle,
        snapshot: &SignalSnapshotV1,
    ) -> Result<(), crate::facade::SignalError> {
        let mut incompatible = snapshot.clone();
        incompatible.meta.core_storage_profile = "incompatible-fintech-test-profile".to_string();
        self.restore_saved_snapshot(branch, &incompatible)
    }
}

pub(super) fn build_fixture(scale: FintechScale) -> FintechWorld {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_tiers::<super::execution_tier::FintechTier>()
        .build();
    runtime.set_runtime_policy(
        SignalRuntimePolicy::fintech()
            .with_history_limit(8)
            .with_detail_limit(4),
    );
    let fx = build_fx_nodes(&mut runtime);
    let partition = build_partition_locality_nodes(&mut runtime);
    let curve_buckets = build_bucket_sources(&mut runtime, scale.buckets);
    let vol_surface_buckets = build_bucket_sources(&mut runtime, scale.buckets);
    let scenario_sources = build_scenario_sources(&mut runtime, scale.scenarios);
    let mut aggregate_sources = Vec::with_capacity(scale.books.max(scale.desks));
    for _ in 0..scale.books.max(scale.desks) {
        aggregate_sources.push(build_aggregate_sources(&mut runtime));
    }

    let mut instruments = Vec::with_capacity(scale.instruments);
    for instrument_index in 0..scale.instruments {
        let core = build_instrument_nodes(&mut runtime);
        let buckets = build_bucket_exposure_nodes(&mut runtime, &core, scale.buckets);
        let scenarios = build_scenario_nodes(&mut runtime, &core, &scenario_sources, scale.scenarios);
        instruments.push(InstrumentFixture {
            instrument_index,
            book_index: super::hierarchy::book_for_instrument(scale, instrument_index),
            core,
            buckets,
            scenarios,
        });
    }

    let mut book_aggregates = Vec::with_capacity(scale.books);
    for book_index in 0..scale.books {
        let aggregate = runtime
            .graph_mut()
            .node()
            .depends_on_aspects(super::aspects::full_mask())
            .tolerance(5)
            .build();
        runtime
            .graph_mut()
            .add_dependency(
                aggregate,
                aggregate_sources[book_index].book_state,
                super::aspects::RISK,
            )
            .unwrap();
        runtime
            .graph_mut()
            .add_dependency(
                aggregate,
                aggregate_sources[book_index].book_state,
                super::aspects::ALERT,
            )
            .unwrap();
        runtime
            .graph_mut()
            .add_dependency(aggregate, fx.eur_jpy, super::aspects::PRICE)
            .unwrap();
        for instrument in &instruments {
            if instrument.book_index == book_index {
                runtime
                    .graph_mut()
                    .add_dependency(aggregate, instrument.core.risk, super::aspects::RISK)
                    .unwrap();
                runtime
                    .graph_mut()
                    .add_dependency(aggregate, instrument.core.alert, super::aspects::ALERT)
                    .unwrap();
            }
        }
        book_aggregates.push(aggregate);
    }

    let mut desk_aggregates = Vec::with_capacity(scale.desks);
    for desk_index in 0..scale.desks {
        let aggregate = runtime
            .graph_mut()
            .node()
            .depends_on_aspects(super::aspects::full_mask())
            .tolerance(6)
            .build();
        runtime
            .graph_mut()
            .add_dependency(
                aggregate,
                aggregate_sources[desk_index].desk_limit,
                super::aspects::RISK,
            )
            .unwrap();
        runtime
            .graph_mut()
            .add_dependency(
                aggregate,
                aggregate_sources[desk_index].desk_limit,
                super::aspects::ALERT,
            )
            .unwrap();
        for (book_index, book_node) in book_aggregates.iter().enumerate() {
            if super::hierarchy::desk_for_book(scale, book_index) == desk_index {
                runtime
                    .graph_mut()
                    .add_dependency(aggregate, *book_node, super::aspects::RISK)
                    .unwrap();
            }
        }
        desk_aggregates.push(aggregate);
    }

    let mut scenario_aggregates = Vec::with_capacity(scale.scenarios);
    for scenario_index in 0..scale.scenarios {
        let aggregate = runtime
            .graph_mut()
            .node()
            .depends_on_aspects(super::aspects::full_mask())
            .tolerance(5)
            .build();
        for instrument in &instruments {
            runtime
                .graph_mut()
                .add_dependency(aggregate, instrument.scenarios[scenario_index], super::aspects::RISK)
                .unwrap();
        }
        scenario_aggregates.push(aggregate);
    }

    let mut bucket_aggregates = Vec::with_capacity(scale.buckets);
    for bucket_index in 0..scale.buckets {
        let aggregate = runtime
            .graph_mut()
            .node()
            .depends_on_aspects(super::aspects::full_mask())
            .tolerance(5)
            .build();
        for instrument in &instruments {
            runtime
                .graph_mut()
                .add_dependency(aggregate, instrument.buckets[bucket_index], super::aspects::RISK)
                .unwrap();
        }
        bucket_aggregates.push(aggregate);
    }

    let handles = FintechWorldHandles::new(
        instruments[0].core.market,
        instruments[0].core.threshold,
        instruments[0].core.risk,
        desk_aggregates[0],
        scenario_aggregates[0],
        PartitionSurfaceNodes {
            market_regions: partition.market_regions,
            rates_partition: partition.rates_partition,
            credit_partition: partition.credit_partition,
            rates_bucket_zero: partition.rates_bucket_zero,
            coarse_book: partition.coarse_book,
        },
    );

    runtime
        .transaction(&mut (), |tx| {
            tx.read(partition.market_regions, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(AspectVersion::from_updates([
                        (super::aspects::PRICE, 0),
                        (super::aspects::RISK, 0),
                    ]))
                    .with_output_identity("partition-market-baseline"),
                ))
            })?;
            Ok(())
        })
        .expect("partition locality source should seed cleanly");

    FintechWorld {
        runtime,
        handles,
        fx,
        aggregate_sources,
        curve_buckets,
        vol_surface_buckets,
        scenario_sources,
        instruments,
        book_aggregates,
        desk_aggregates,
        scenario_aggregates,
        bucket_aggregates,
    }
}

fn apply_signed_delta(base: u64, delta: i64) -> u64 {
    if delta >= 0 {
        base.saturating_add(delta as u64)
    } else {
        base.saturating_sub(delta.unsigned_abs())
    }
}
