use worth_proof::{Artifact, PhaseMarker};

use crate::aspects::contracts::AspectContract;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AspectEvolutionKind {
    Unchanged,
    Additive,
    Widening,
    Narrowing,
    Incompatible,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AspectEvolutionVerdict {
    kind: AspectEvolutionKind,
    reason: &'static str,
}

impl AspectEvolutionVerdict {
    pub fn new(kind: AspectEvolutionKind, reason: &'static str) -> Self {
        Self { kind, reason }
    }

    pub fn kind(&self) -> AspectEvolutionKind {
        self.kind
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AspectEvolutionClassifiedContracts {
    previous: AspectContract,
    next: AspectContract,
    verdict: AspectEvolutionVerdict,
}

impl AspectEvolutionClassifiedContracts {
    pub fn previous(&self) -> &AspectContract {
        &self.previous
    }

    pub fn next(&self) -> &AspectContract {
        &self.next
    }

    pub fn verdict(&self) -> &AspectEvolutionVerdict {
        &self.verdict
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AspectEvolutionClassified;

impl PhaseMarker for AspectEvolutionClassified {}

pub type AspectEvolutionClassifiedContractArtifact =
    Artifact<AspectEvolutionClassified, AspectEvolutionClassifiedContracts>;

pub fn classify_aspect_contract_evolution(
    previous: AspectContract,
    next: AspectContract,
) -> AspectEvolutionClassifiedContractArtifact {
    let verdict = previous.classify_evolution_to(&next);
    Artifact::new(AspectEvolutionClassifiedContracts {
        previous,
        next,
        verdict,
    })
}
