use super::{
    latch_counter_backed_performance_receipt, LatchAcquisitionDenial, LatchCounterEvidenceDenial,
    LatchCounterPerformanceReceipt, LatchDeniedBeforeWaitEvidence, LatchWaitCounterSnapshot,
    PhysicalLatchKey,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalLatchWaitEdge {
    waiter: u64,
    holder: u64,
    key: PhysicalLatchKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatchWaitForGraph {
    edges: Vec<PhysicalLatchWaitEdge>,
    max_edges: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadlockDetectionReport {
    cycle_edges: Vec<PhysicalLatchWaitEdge>,
    counters: LatchWaitCounterSnapshot,
    counter_receipt: LatchCounterPerformanceReceipt,
}

#[derive(Debug)]
pub enum LatchWaitForGraphAdmissionDenial {
    BoundExceeded(Box<LatchDeniedBeforeWaitEvidence>),
    Evidence(LatchCounterEvidenceDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LatchWaitForGraphDenial {
    BoundExceeded,
    Evidence,
    NoCycleDetected,
}

impl PhysicalLatchWaitEdge {
    pub const fn new(waiter: u64, holder: u64, key: PhysicalLatchKey) -> Self {
        Self {
            waiter,
            holder,
            key,
        }
    }

    pub const fn waiter(self) -> u64 {
        self.waiter
    }

    pub const fn holder(self) -> u64 {
        self.holder
    }

    pub const fn key(self) -> PhysicalLatchKey {
        self.key
    }
}

impl LatchWaitForGraph {
    pub fn bounded(
        edges: Vec<PhysicalLatchWaitEdge>,
        max_edges: usize,
    ) -> Result<Self, LatchAcquisitionDenial> {
        if edges.len() > max_edges {
            return Err(LatchAcquisitionDenial::WaitForGraphBoundExceeded);
        }
        Ok(Self { edges, max_edges })
    }

    pub fn bounded_with_evidence(
        edges: Vec<PhysicalLatchWaitEdge>,
        max_edges: usize,
    ) -> Result<Self, LatchWaitForGraphAdmissionDenial> {
        if edges.len() > max_edges {
            return Err(LatchWaitForGraphAdmissionDenial::BoundExceeded(Box::new(
                wait_graph_bound_denial_evidence(edges.len())?,
            )));
        }
        Ok(Self { edges, max_edges })
    }

    pub fn detect_cycle(&self) -> Result<DeadlockDetectionReport, LatchWaitForGraphDenial> {
        let cycle_edges =
            first_cycle_edges(&self.edges).ok_or(LatchWaitForGraphDenial::NoCycleDetected)?;
        let counters = cycle_counter_snapshot(cycle_edges.len());
        let counter_receipt = latch_counter_backed_performance_receipt(counters)
            .map_err(|_| LatchWaitForGraphDenial::Evidence)?;
        Ok(DeadlockDetectionReport {
            cycle_edges,
            counters,
            counter_receipt,
        })
    }

    pub fn pre_wait_cycle_denial(
        &self,
    ) -> Result<Option<LatchDeniedBeforeWaitEvidence>, LatchCounterEvidenceDenial> {
        match self.detect_cycle() {
            Ok(report) => LatchDeniedBeforeWaitEvidence::new(
                LatchAcquisitionDenial::CyclicPlan,
                report.counters(),
            )
            .map(Some),
            Err(LatchWaitForGraphDenial::NoCycleDetected) => Ok(None),
            Err(LatchWaitForGraphDenial::BoundExceeded | LatchWaitForGraphDenial::Evidence) => {
                LatchDeniedBeforeWaitEvidence::new(
                    LatchAcquisitionDenial::WaitForGraphBoundExceeded,
                    LatchWaitCounterSnapshot::empty(),
                )
                .map(Some)
            }
        }
    }

    pub fn edges(&self) -> &[PhysicalLatchWaitEdge] {
        &self.edges
    }

    pub const fn max_edges(&self) -> usize {
        self.max_edges
    }
}

impl DeadlockDetectionReport {
    pub fn cycle_edges(&self) -> &[PhysicalLatchWaitEdge] {
        &self.cycle_edges
    }

    pub const fn counters(&self) -> LatchWaitCounterSnapshot {
        self.counters
    }

    pub fn counter_receipt(&self) -> &LatchCounterPerformanceReceipt {
        &self.counter_receipt
    }
}

fn first_cycle_edges(edges: &[PhysicalLatchWaitEdge]) -> Option<Vec<PhysicalLatchWaitEdge>> {
    let actors = sorted_wait_graph_actors(edges);
    let mut visiting = Vec::new();
    let mut visited = Vec::new();
    for actor in actors {
        if visited.contains(&actor) {
            continue;
        }
        if let Some(cycle) = visit_actor_for_cycle(actor, edges, &mut visiting, &mut visited) {
            return Some(cycle);
        }
    }
    None
}

fn sorted_wait_graph_actors(edges: &[PhysicalLatchWaitEdge]) -> Vec<u64> {
    let mut actors = Vec::with_capacity(edges.len().saturating_mul(2));
    for edge in edges {
        actors.push(edge.waiter());
        actors.push(edge.holder());
    }
    actors.sort_unstable();
    actors.dedup();
    actors
}

fn visit_actor_for_cycle(
    actor: u64,
    edges: &[PhysicalLatchWaitEdge],
    visiting: &mut Vec<u64>,
    visited: &mut Vec<u64>,
) -> Option<Vec<PhysicalLatchWaitEdge>> {
    if let Some(cycle_start) = visiting.iter().position(|candidate| *candidate == actor) {
        return cycle_edges_from_stack(&visiting[cycle_start..], edges);
    }
    if visited.contains(&actor) {
        return None;
    }
    visiting.push(actor);
    for edge in outgoing_edges(actor, edges) {
        if let Some(cycle) = visit_actor_for_cycle(edge.holder(), edges, visiting, visited) {
            return Some(cycle);
        }
    }
    visiting.pop();
    visited.push(actor);
    None
}

fn outgoing_edges(
    actor: u64,
    edges: &[PhysicalLatchWaitEdge],
) -> impl Iterator<Item = &PhysicalLatchWaitEdge> {
    edges.iter().filter(move |edge| edge.waiter() == actor)
}

fn cycle_edges_from_stack(
    cycle_actors: &[u64],
    edges: &[PhysicalLatchWaitEdge],
) -> Option<Vec<PhysicalLatchWaitEdge>> {
    let mut cycle_edges = Vec::with_capacity(cycle_actors.len());
    for index in 0..cycle_actors.len() {
        let waiter = cycle_actors[index];
        let holder = cycle_actors[(index + 1) % cycle_actors.len()];
        cycle_edges.push(edge_between(waiter, holder, edges)?);
    }
    Some(cycle_edges)
}

fn edge_between(
    waiter: u64,
    holder: u64,
    edges: &[PhysicalLatchWaitEdge],
) -> Option<PhysicalLatchWaitEdge> {
    edges
        .iter()
        .copied()
        .find(|edge| edge.waiter() == waiter && edge.holder() == holder)
}

fn cycle_counter_snapshot(edge_count: usize) -> LatchWaitCounterSnapshot {
    LatchWaitCounterSnapshot::empty()
        .with_attempts(edge_count as u64)
        .with_waits(edge_count as u64)
        .with_detected_cycle()
}

fn wait_graph_bound_denial_evidence(
    edge_count: usize,
) -> Result<LatchDeniedBeforeWaitEvidence, LatchWaitForGraphAdmissionDenial> {
    LatchDeniedBeforeWaitEvidence::new(
        LatchAcquisitionDenial::WaitForGraphBoundExceeded,
        LatchWaitCounterSnapshot::empty()
            .with_attempts(edge_count as u64)
            .with_waits(edge_count as u64),
    )
    .map_err(LatchWaitForGraphAdmissionDenial::Evidence)
}
