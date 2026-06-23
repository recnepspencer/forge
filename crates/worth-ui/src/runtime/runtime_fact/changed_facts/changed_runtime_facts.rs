use super::super::{WorthUiRuntimeFactId, WorthUiRuntimeFactSet, WorthUiRuntimeFactSetDigest};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiChangedRuntimeFactsProof {
    basis_digest: WorthUiRuntimeFactSetDigest,
}

impl WorthUiChangedRuntimeFactsProof {
    pub fn basis_digest(self) -> WorthUiRuntimeFactSetDigest {
        self.basis_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiChangedRuntimeFacts {
    facts: WorthUiRuntimeFactSet,
    proof: WorthUiChangedRuntimeFactsProof,
}

impl WorthUiChangedRuntimeFacts {
    pub(crate) fn from_runtime(facts: WorthUiRuntimeFactSet) -> Self {
        Self {
            proof: WorthUiChangedRuntimeFactsProof {
                basis_digest: facts.digest(),
            },
            facts,
        }
    }

    pub fn facts(&self) -> &WorthUiRuntimeFactSet {
        &self.facts
    }

    pub fn proof(&self) -> WorthUiChangedRuntimeFactsProof {
        self.proof
    }

    pub fn digest(&self) -> WorthUiRuntimeFactSetDigest {
        self.proof.basis_digest()
    }

    pub fn contains_exact(&self, fact: &WorthUiRuntimeFactId) -> bool {
        self.facts.contains_exact(fact)
    }

    pub fn contains_family(&self, family: super::super::WorthUiRuntimeFactFamily) -> bool {
        self.facts.contains_family(family)
    }

    pub fn len(&self) -> usize {
        self.facts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.facts.is_empty()
    }
}
