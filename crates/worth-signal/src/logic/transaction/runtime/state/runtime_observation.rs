use std::collections::{BTreeMap, BTreeSet};
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::diagnostics::profile::DiagnosticsTier;

use super::runtime_state::SignalRuntime;
use crate::logic::transaction::runtime::transaction::{
    CommittedObservationEvent, CommittedObservationEventSummary,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ObserverId(u64);

impl ObserverId {
    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ObservationHandleId(u64);

impl ObservationHandleId {
    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObservationHandle {
    observer_id: ObserverId,
    handle_id: ObservationHandleId,
}

impl ObservationHandle {
    pub fn observer_id(self) -> ObserverId {
        self.observer_id
    }

    pub fn handle_id(self) -> ObservationHandleId {
        self.handle_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObservationTrigger {
    Touched,
    Recomputed,
    MeaningfulChange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObservationDeliveryMode {
    PerCommittedTransaction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationPolicy {
    trigger: ObservationTrigger,
    delivery_mode: ObservationDeliveryMode,
}

impl ObservationPolicy {
    pub fn new(trigger: ObservationTrigger, delivery_mode: ObservationDeliveryMode) -> Self {
        Self {
            trigger,
            delivery_mode,
        }
    }

    pub fn touched() -> Self {
        Self::new(
            ObservationTrigger::Touched,
            ObservationDeliveryMode::PerCommittedTransaction,
        )
    }

    pub fn recomputed() -> Self {
        Self::new(
            ObservationTrigger::Recomputed,
            ObservationDeliveryMode::PerCommittedTransaction,
        )
    }

    pub fn meaningful_change() -> Self {
        Self::new(
            ObservationTrigger::MeaningfulChange,
            ObservationDeliveryMode::PerCommittedTransaction,
        )
    }

    pub fn trigger(self) -> ObservationTrigger {
        self.trigger
    }

    pub fn delivery_mode(self) -> ObservationDeliveryMode {
        self.delivery_mode
    }
}

impl Default for ObservationPolicy {
    fn default() -> Self {
        Self::meaningful_change()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservedNodeSet {
    nodes: BTreeSet<NodeId>,
}

impl ObservedNodeSet {
    pub fn from_nodes(nodes: impl IntoIterator<Item = NodeId>) -> Self {
        Self {
            nodes: nodes.into_iter().collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn contains(&self, node: NodeId) -> bool {
        self.nodes.contains(&node)
    }

    pub fn iter(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.nodes.iter().copied()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MatchingObserverSet {
    observers: Vec<ObserverId>,
}

impl MatchingObserverSet {
    pub fn new(observers: Vec<ObserverId>) -> Self {
        Self { observers }
    }

    pub fn len(&self) -> usize {
        self.observers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.observers.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = ObserverId> + '_ {
        self.observers.iter().copied()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationRegistrySummary {
    pub active_observer_count: usize,
    pub indexed_node_count: usize,
    pub ordered_observer_ids: Vec<ObserverId>,
}

pub struct ObservationNotice<'a> {
    observer_id: ObserverId,
    handle_id: ObservationHandleId,
    policy: ObservationPolicy,
    observed_nodes: &'a ObservedNodeSet,
    matched_nodes: &'a ObservedNodeSet,
    touched: bool,
    recomputed: bool,
    meaningful_change: bool,
    trigger_matched: bool,
}

impl ObservationNotice<'_> {
    pub fn observer_id(&self) -> ObserverId {
        self.observer_id
    }

    pub fn handle_id(&self) -> ObservationHandleId {
        self.handle_id
    }

    pub fn policy(&self) -> ObservationPolicy {
        self.policy
    }

    pub fn observed_nodes(&self) -> &ObservedNodeSet {
        self.observed_nodes
    }

    pub fn matched_nodes(&self) -> &ObservedNodeSet {
        self.matched_nodes
    }

    pub fn touched(&self) -> bool {
        self.touched
    }

    pub fn recomputed(&self) -> bool {
        self.recomputed
    }

    pub fn meaningful_change(&self) -> bool {
        self.meaningful_change
    }

    pub fn trigger_matched(&self) -> bool {
        self.trigger_matched
    }
}

pub trait ObservationListener<D, I, E, Ctx, T>: Send + Sync
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn on_observation(
        &self,
        ctx: ObservationReadContext<'_, D, I, E, Ctx, T>,
        notice: &ObservationNotice<'_>,
    );
}

pub struct ObservationReadContext<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    graph: &'a SignalGraph,
    _marker: PhantomData<(D, I, E, Ctx, T)>,
}

impl<'a, D, I, E, Ctx, T> ObservationReadContext<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(crate) fn new(runtime: &'a SignalRuntime<D, I, E, Ctx, T>) -> Self {
        Self {
            graph: runtime.graph(),
            _marker: PhantomData,
        }
    }

    pub(in crate::logic::transaction::runtime) fn from_graph(graph: &'a SignalGraph) -> Self {
        Self {
            graph,
            _marker: PhantomData,
        }
    }

    pub fn graph(&self) -> crate::data::graph::GraphObserver<'a> {
        self.graph.observe()
    }

    pub fn diagnostics_profile(&self) -> DiagnosticsTier {
        self.graph.observe().diagnostics_profile()
    }

    pub fn runtime_policy(&self) -> crate::diagnostics::policy::SignalRuntimePolicy {
        self.graph.observe().runtime_policy()
    }

    pub fn current_branch(&self) -> crate::state::SignalBranchHandle {
        self.graph.current_branch()
    }
}

struct ObserverRegistration<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    handle_id: ObservationHandleId,
    policy: ObservationPolicy,
    observed_nodes: ObservedNodeSet,
    listener: Box<dyn ObservationListener<D, I, E, Ctx, T>>,
}

pub struct RuntimeObservationRegistry<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    next_observer_id: u64,
    next_handle_id: u64,
    registrations: BTreeMap<ObserverId, ObserverRegistration<D, I, E, Ctx, T>>,
    observers_by_node: BTreeMap<NodeId, BTreeSet<ObserverId>>,
}

impl<D, I, E, Ctx, T> Default for RuntimeObservationRegistry<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn default() -> Self {
        Self {
            next_observer_id: 1,
            next_handle_id: 1,
            registrations: BTreeMap::new(),
            observers_by_node: BTreeMap::new(),
        }
    }
}

impl<D, I, E, Ctx, T> RuntimeObservationRegistry<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn register_nodes(
        &mut self,
        policy: ObservationPolicy,
        observed_nodes: ObservedNodeSet,
        listener: Box<dyn ObservationListener<D, I, E, Ctx, T>>,
    ) -> ObservationHandle {
        let observer_id = ObserverId(self.next_observer_id);
        self.next_observer_id += 1;
        let handle_id = ObservationHandleId(self.next_handle_id);
        self.next_handle_id += 1;

        for node in observed_nodes.iter() {
            self.observers_by_node
                .entry(node)
                .or_default()
                .insert(observer_id);
        }

        self.registrations.insert(
            observer_id,
            ObserverRegistration {
                handle_id,
                policy,
                observed_nodes,
                listener,
            },
        );

        ObservationHandle {
            observer_id,
            handle_id,
        }
    }

    pub fn unsubscribe(&mut self, handle: ObservationHandle) -> bool {
        let Some(existing) = self.registrations.get(&handle.observer_id) else {
            return false;
        };
        if existing.handle_id != handle.handle_id {
            return false;
        }
        let registration = self
            .registrations
            .remove(&handle.observer_id)
            .expect("observation registration should still exist");
        for node in registration.observed_nodes.iter() {
            let remove_entry = if let Some(set) = self.observers_by_node.get_mut(&node) {
                set.remove(&handle.observer_id);
                set.is_empty()
            } else {
                false
            };
            if remove_entry {
                self.observers_by_node.remove(&node);
            }
        }
        true
    }

    pub fn summary(&self) -> ObservationRegistrySummary {
        ObservationRegistrySummary {
            active_observer_count: self.registrations.len(),
            indexed_node_count: self.observers_by_node.len(),
            ordered_observer_ids: self.registrations.keys().copied().collect(),
        }
    }

    pub fn matching_observers_for_node(&self, node: NodeId) -> MatchingObserverSet {
        let observers = self
            .observers_by_node
            .get(&node)
            .map(|ids| ids.iter().copied().collect())
            .unwrap_or_default();
        MatchingObserverSet::new(observers)
    }

    pub fn has_matching_observers_for_node(&self, node: NodeId) -> bool {
        self.observers_by_node
            .get(&node)
            .is_some_and(|ids| !ids.is_empty())
    }

    pub fn for_each_matching_observer_for_node<F>(&self, node: NodeId, mut visit: F)
    where
        F: FnMut(ObserverId),
    {
        let Some(observer_ids) = self.observers_by_node.get(&node) else {
            return;
        };
        for &observer_id in observer_ids {
            visit(observer_id);
        }
    }

    pub fn registration_count(&self) -> usize {
        self.registrations.len()
    }

    pub fn registration_for(
        &self,
        observer_id: ObserverId,
    ) -> Option<(ObservationHandleId, ObservationPolicy, &ObservedNodeSet)> {
        self.registrations.get(&observer_id).map(|registration| {
            (
                registration.handle_id,
                registration.policy,
                &registration.observed_nodes,
            )
        })
    }

    pub fn notify_preview(
        &self,
        runtime: &SignalRuntime<D, I, E, Ctx, T>,
        observer_id: ObserverId,
    ) -> bool {
        let Some(registration) = self.registrations.get(&observer_id) else {
            return false;
        };
        let notice = ObservationNotice {
            observer_id,
            handle_id: registration.handle_id,
            policy: registration.policy,
            observed_nodes: &registration.observed_nodes,
            matched_nodes: &registration.observed_nodes,
            touched: false,
            recomputed: false,
            meaningful_change: false,
            trigger_matched: false,
        };
        registration
            .listener
            .on_observation(ObservationReadContext::new(runtime), &notice);
        true
    }

    pub(in crate::logic::transaction::runtime) fn deliver_committed(
        &self,
        graph: &SignalGraph,
        deliveries: &[CommittedObservationEvent],
    ) -> usize {
        let mut delivered = 0;
        for delivery in deliveries {
            let Some(registration) = self.registrations.get(&delivery.observer_id()) else {
                continue;
            };
            let ctx = ObservationReadContext::from_graph(graph);
            let notice = ObservationNotice {
                observer_id: delivery.observer_id(),
                handle_id: delivery.handle_id(),
                policy: delivery.policy(),
                observed_nodes: delivery.observed_nodes(),
                matched_nodes: delivery.matched_nodes(),
                touched: delivery.touched(),
                recomputed: delivery.recomputed(),
                meaningful_change: delivery.meaningful_change(),
                trigger_matched: delivery.trigger_matched(),
            };
            registration.listener.on_observation(ctx, &notice);
            delivered += 1;
        }
        delivered
    }

    pub(crate) fn deliver_boundary_summaries(
        &self,
        graph: &SignalGraph,
        deliveries: &[CommittedObservationEventSummary],
    ) -> usize {
        let mut delivered = 0;
        for delivery in deliveries {
            let Some(registration) = self.registrations.get(&delivery.observer_id) else {
                continue;
            };
            let ctx = ObservationReadContext::from_graph(graph);
            let notice = ObservationNotice {
                observer_id: delivery.observer_id,
                handle_id: delivery.handle_id,
                policy: delivery.policy,
                observed_nodes: &delivery.observed_nodes,
                matched_nodes: &delivery.matched_nodes,
                touched: delivery.touched,
                recomputed: delivery.recomputed,
                meaningful_change: delivery.meaningful_change,
                trigger_matched: delivery.trigger_matched,
            };
            registration.listener.on_observation(ctx, &notice);
            delivered += 1;
        }
        delivered
    }
}
