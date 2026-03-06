//! Deterministic DAG-ordered lifecycle event bus.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::marker::PhantomData;

use forge_core::KernelError;

use crate::data::checkpoint::CheckpointBarrier;
use crate::data::event_subscriber::{EventSubscriber, SubscriberId};
use crate::data::subscriber_context::SubscriberContext;

/// Registration-time DAG validation failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscriberRegistryError<D: Copy + Ord + std::fmt::Debug + 'static> {
    DuplicateSubscriberId {
        id: SubscriberId,
        first: &'static str,
        second: &'static str,
    },
    DuplicateProvider {
        data_id: D,
        first: &'static str,
        second: &'static str,
    },
    MissingProvider {
        subscriber: &'static str,
        data_id: D,
    },
    CycleDetected {
        cycle_chain: Vec<&'static str>,
    },
}

/// Flush-time failure for event bus execution.
#[derive(Debug)]
pub enum EventFlushError<D: Copy + Ord + std::fmt::Debug + 'static> {
    Registry(SubscriberRegistryError<D>),
    Subscriber {
        subscriber_id: SubscriberId,
        subscriber_name: &'static str,
        source: KernelError,
    },
}

impl<D: Copy + Ord + std::fmt::Debug + 'static> std::fmt::Display for EventFlushError<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Registry(err) => write!(f, "subscriber registry invalid during flush: {err:?}"),
            Self::Subscriber {
                subscriber_id,
                subscriber_name,
                source,
            } => {
                write!(
                    f,
                    "subscriber {} (id={}) failed: {}",
                    subscriber_name,
                    subscriber_id.get(),
                    source
                )
            }
        }
    }
}

impl<D: Copy + Ord + std::fmt::Debug + 'static> std::error::Error for EventFlushError<D> {}

struct SubscriberEntry<E, D, C>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
{
    id: SubscriberId,
    name: &'static str,
    requires: &'static [D],
    provides: &'static [D],
    sub: Box<dyn EventSubscriber<Event = E, DataId = D, RuntimeContext = C>>,
}

/// Typed deterministic event bus.
///
/// Events are buffered in-order and delivered at checkpoint flush.
pub struct EventBus<E, D, C = ()>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
{
    pending: Vec<E>,
    subscribers: Vec<SubscriberEntry<E, D, C>>,
    order: Vec<usize>,
    context: SubscriberContext<D>,
    finalized: bool,
    context_marker: PhantomData<fn(&mut C)>,
}

impl<E, D, C> Default for EventBus<E, D, C>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<E, D, C> EventBus<E, D, C>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
{
    /// Create an empty bus.
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
            subscribers: Vec::new(),
            order: Vec::new(),
            context: SubscriberContext::new(),
            finalized: false,
            context_marker: PhantomData,
        }
    }

    /// Access committed subscriber context.
    pub fn context(&self) -> &SubscriberContext<D> {
        &self.context
    }

    /// Access committed subscriber context mutably.
    pub fn context_mut(&mut self) -> &mut SubscriberContext<D> {
        &mut self.context
    }

    /// Register one subscriber (must call `finalize_registration` before use).
    pub fn subscribe(
        &mut self,
        sub: Box<dyn EventSubscriber<Event = E, DataId = D, RuntimeContext = C>>,
    ) -> Result<(), SubscriberRegistryError<D>> {
        let id = sub.id();
        let name = sub.name();
        let requires = sub.requires();
        let provides = sub.provides();

        if let Some(existing) = self.subscribers.iter().find(|e| e.id == id) {
            return Err(SubscriberRegistryError::DuplicateSubscriberId {
                id,
                first: existing.name,
                second: name,
            });
        }
        self.subscribers.push(SubscriberEntry {
            id,
            name,
            requires,
            provides,
            sub,
        });
        self.finalized = false;
        Ok(())
    }

    /// Validate dependency DAG and compute deterministic order.
    pub fn finalize_registration(&mut self) -> Result<(), SubscriberRegistryError<D>> {
        let mut provider_for: BTreeMap<D, usize> = BTreeMap::new();
        for (idx, entry) in self.subscribers.iter().enumerate() {
            for &data in entry.provides {
                if let Some(prev_idx) = provider_for.get(&data).copied() {
                    return Err(SubscriberRegistryError::DuplicateProvider {
                        data_id: data,
                        first: self.subscribers[prev_idx].name,
                        second: entry.name,
                    });
                }
                provider_for.insert(data, idx);
            }
        }

        let n = self.subscribers.len();
        let mut indegree = vec![0usize; n];
        let mut edges: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (consumer_idx, entry) in self.subscribers.iter().enumerate() {
            for &required in entry.requires {
                let Some(provider_idx) = provider_for.get(&required).copied() else {
                    return Err(SubscriberRegistryError::MissingProvider {
                        subscriber: entry.name,
                        data_id: required,
                    });
                };
                if provider_idx != consumer_idx {
                    edges[provider_idx].push(consumer_idx);
                    indegree[consumer_idx] += 1;
                }
            }
        }

        let mut ready: BTreeSet<(SubscriberId, usize)> = BTreeSet::new();
        for (idx, entry) in self.subscribers.iter().enumerate() {
            if indegree[idx] == 0 {
                ready.insert((entry.id, idx));
            }
        }

        let mut resolved = Vec::with_capacity(n);
        while let Some((_, idx)) = ready.pop_first() {
            resolved.push(idx);
            for &dst in &edges[idx] {
                indegree[dst] -= 1;
                if indegree[dst] == 0 {
                    ready.insert((self.subscribers[dst].id, dst));
                }
            }
        }

        if resolved.len() != n {
            let cycle = build_cycle_chain(&edges, &self.subscribers);
            return Err(SubscriberRegistryError::CycleDetected { cycle_chain: cycle });
        }

        self.order = resolved;
        self.finalized = true;
        Ok(())
    }

    fn ensure_finalized(&mut self) -> Result<(), SubscriberRegistryError<D>> {
        if self.finalized {
            return Ok(());
        }
        self.finalize_registration()
    }

    /// Signal start of an operation lifecycle.
    pub fn begin(&mut self, runtime: &mut C) -> Result<(), SubscriberRegistryError<D>> {
        self.ensure_finalized()?;
        self.context.clear_staged();
        for &idx in &self.order {
            self.subscribers[idx]
                .sub
                .on_begin(&mut self.context, runtime);
        }
        Ok(())
    }

    /// Record one event for later checkpoint delivery.
    pub fn emit(&mut self, event: E) {
        self.pending.push(event);
    }

    /// Flush pending events and run checkpoint lifecycle in deterministic order.
    pub fn flush(
        &mut self,
        barrier: CheckpointBarrier,
        runtime: &mut C,
    ) -> Result<(), EventFlushError<D>> {
        self.ensure_finalized().map_err(EventFlushError::Registry)?;

        for &idx in &self.order {
            let entry = &mut self.subscribers[idx];
            for event in &self.pending {
                entry.sub.on_event(event);
            }
            if let Err(source) = entry.sub.on_checkpoint(barrier, &mut self.context, runtime) {
                self.context.clear_staged();
                return Err(EventFlushError::Subscriber {
                    subscriber_id: entry.id,
                    subscriber_name: entry.name,
                    source,
                });
            }
        }

        self.pending.clear();
        self.context.finalize();
        Ok(())
    }

    /// Roll back lifecycle state in reverse deterministic order.
    pub fn rollback(&mut self, runtime: &mut C) {
        self.pending.clear();
        self.context.clear_staged();
        for &idx in self.order.iter().rev() {
            self.subscribers[idx].sub.on_rollback(runtime);
        }
    }

    /// Deterministic resolved order for diagnostics/tests.
    pub fn resolved_order(&self) -> Vec<SubscriberId> {
        self.order
            .iter()
            .map(|&idx| self.subscribers[idx].id)
            .collect()
    }
}

fn build_cycle_chain<E, D, C>(
    edges: &[Vec<usize>],
    entries: &[SubscriberEntry<E, D, C>],
) -> Vec<&'static str>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
{
    let n = edges.len();
    let mut color = vec![0u8; n]; // 0=white,1=gray,2=black
    let mut parent = vec![usize::MAX; n];

    fn dfs<E, D, C>(
        u: usize,
        edges: &[Vec<usize>],
        color: &mut [u8],
        parent: &mut [usize],
    ) -> Option<(usize, usize)>
    where
        D: Copy + Ord + std::fmt::Debug + 'static,
    {
        color[u] = 1;
        for &v in &edges[u] {
            if color[v] == 0 {
                parent[v] = u;
                if let Some(c) = dfs::<E, D, C>(v, edges, color, parent) {
                    return Some(c);
                }
            } else if color[v] == 1 {
                return Some((u, v));
            }
        }
        color[u] = 2;
        None
    }

    for i in 0..n {
        if color[i] != 0 {
            continue;
        }
        if let Some((from, to)) = dfs::<E, D, C>(i, edges, &mut color, &mut parent) {
            let mut chain_idx = VecDeque::new();
            chain_idx.push_front(to);
            let mut cur = from;
            chain_idx.push_front(cur);
            while cur != to {
                let p = parent[cur];
                if p == usize::MAX {
                    break;
                }
                cur = p;
                chain_idx.push_front(cur);
            }
            return chain_idx.into_iter().map(|idx| entries[idx].name).collect();
        }
    }

    vec!["<unknown-cycle>"]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::event_subscriber::{EventSubscriber, SubscriberId};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum Data {
        A,
        B,
        C,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Ev {
        Tick(u32),
    }

    struct RecSub {
        id: SubscriberId,
        name: &'static str,
        req: &'static [Data],
        prov: &'static [Data],
        out: std::sync::Arc<std::sync::Mutex<Vec<&'static str>>>,
    }

    impl EventSubscriber for RecSub {
        type Event = Ev;
        type DataId = Data;
        type RuntimeContext = ();

        fn id(&self) -> SubscriberId {
            self.id
        }
        fn name(&self) -> &'static str {
            self.name
        }
        fn requires(&self) -> &'static [Data] {
            self.req
        }
        fn provides(&self) -> &'static [Data] {
            self.prov
        }
        fn on_event(&mut self, event: &Ev) {
            let Ev::Tick(_v) = event;
        }
        fn on_checkpoint(
            &mut self,
            _barrier: CheckpointBarrier,
            _ctx: &mut SubscriberContext<Data>,
            _runtime: &mut Self::RuntimeContext,
        ) -> Result<(), KernelError> {
            self.out.lock().unwrap().push(self.name);
            Ok(())
        }
        fn on_rollback(&mut self, _runtime: &mut Self::RuntimeContext) {
            self.out.lock().unwrap().push(self.name);
        }
    }

    #[test]
    fn deterministic_order_independent_of_registration() {
        let out1 = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let out2 = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let mk = |id, name, req, prov, out| RecSub {
            id: SubscriberId::new(id),
            name,
            req,
            prov,
            out,
        };

        let mut b1: EventBus<Ev, Data> = EventBus::new();
        b1.subscribe(Box::new(mk(30, "c", &[Data::B], &[Data::C], out1.clone())))
            .unwrap();
        b1.subscribe(Box::new(mk(10, "a", &[], &[Data::A], out1.clone())))
            .unwrap();
        b1.subscribe(Box::new(mk(20, "b", &[Data::A], &[Data::B], out1.clone())))
            .unwrap();
        b1.finalize_registration().unwrap();

        let mut b2: EventBus<Ev, Data> = EventBus::new();
        b2.subscribe(Box::new(mk(10, "a", &[], &[Data::A], out2.clone())))
            .unwrap();
        b2.subscribe(Box::new(mk(20, "b", &[Data::A], &[Data::B], out2.clone())))
            .unwrap();
        b2.subscribe(Box::new(mk(30, "c", &[Data::B], &[Data::C], out2.clone())))
            .unwrap();
        b2.finalize_registration().unwrap();

        assert_eq!(b1.resolved_order(), b2.resolved_order());
        assert_eq!(
            b1.resolved_order(),
            vec![
                SubscriberId::new(10),
                SubscriberId::new(20),
                SubscriberId::new(30)
            ]
        );
    }

    #[test]
    fn cycle_error_contains_chain() {
        let out = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let mk = |id, name, req, prov| RecSub {
            id: SubscriberId::new(id),
            name,
            req,
            prov,
            out: out.clone(),
        };

        let mut bus: EventBus<Ev, Data> = EventBus::new();
        bus.subscribe(Box::new(mk(10, "a", &[Data::C], &[Data::A])))
            .unwrap();
        bus.subscribe(Box::new(mk(20, "b", &[Data::A], &[Data::B])))
            .unwrap();
        bus.subscribe(Box::new(mk(30, "c", &[Data::B], &[Data::C])))
            .unwrap();

        let err = bus.finalize_registration().unwrap_err();
        match err {
            SubscriberRegistryError::CycleDetected { cycle_chain } => {
                assert!(!cycle_chain.is_empty());
                assert!(cycle_chain.contains(&"a"));
                assert!(cycle_chain.contains(&"b"));
                assert!(cycle_chain.contains(&"c"));
            }
            _ => panic!("expected cycle error"),
        }
    }

    #[test]
    fn duplicate_provider_and_missing_provider_errors() {
        let out = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let mk = |id, name, req, prov| RecSub {
            id: SubscriberId::new(id),
            name,
            req,
            prov,
            out: out.clone(),
        };

        let mut dup: EventBus<Ev, Data> = EventBus::new();
        dup.subscribe(Box::new(mk(10, "a", &[], &[Data::A])))
            .unwrap();
        dup.subscribe(Box::new(mk(20, "b", &[], &[Data::A])))
            .unwrap();
        let err = dup.finalize_registration().unwrap_err();
        assert!(matches!(
            err,
            SubscriberRegistryError::DuplicateProvider { .. }
        ));

        let mut miss: EventBus<Ev, Data> = EventBus::new();
        miss.subscribe(Box::new(mk(10, "a", &[Data::C], &[Data::A])))
            .unwrap();
        let err = miss.finalize_registration().unwrap_err();
        assert!(matches!(
            err,
            SubscriberRegistryError::MissingProvider { .. }
        ));
    }

    #[test]
    fn rollback_runs_reverse_order() {
        let out = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let mk = |id, name, req, prov| RecSub {
            id: SubscriberId::new(id),
            name,
            req,
            prov,
            out: out.clone(),
        };

        let mut bus: EventBus<Ev, Data> = EventBus::new();
        bus.subscribe(Box::new(mk(10, "a", &[], &[Data::A])))
            .unwrap();
        bus.subscribe(Box::new(mk(20, "b", &[Data::A], &[Data::B])))
            .unwrap();
        bus.subscribe(Box::new(mk(30, "c", &[Data::B], &[Data::C])))
            .unwrap();
        bus.finalize_registration().unwrap();
        let mut runtime = ();
        bus.rollback(&mut runtime);
        assert_eq!(&*out.lock().unwrap(), &["c", "b", "a"]);
    }

    #[test]
    fn flush_auto_finalizes_and_delivers_events() {
        let out = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let mk = |id, name, req, prov| RecSub {
            id: SubscriberId::new(id),
            name,
            req,
            prov,
            out: out.clone(),
        };

        let mut bus: EventBus<Ev, Data> = EventBus::new();
        bus.subscribe(Box::new(mk(10, "a", &[], &[Data::A])))
            .unwrap();
        bus.emit(Ev::Tick(1));
        let mut runtime = ();
        bus.flush(CheckpointBarrier::PerOperation, &mut runtime)
            .unwrap();

        assert_eq!(&*out.lock().unwrap(), &["a"]);
        assert_eq!(bus.resolved_order(), vec![SubscriberId::new(10)]);
    }
}
