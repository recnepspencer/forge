use forge_query::facade::{ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial};
use std::marker::PhantomData;

use crate::topology_operators::TopologyDeclaredMutationSequence;

use super::lowering::{
    topology_operator_touch_descriptor_from_touched_graph_basis,
    topology_touched_graph_basis_from_mutation_sequence,
};
use super::{
    TopologyTouchedGraphBasis, TopologyTouchedGraphCounters, TopologyTouchedOperatingWorld,
};

#[derive(Debug, Clone)]
pub struct TopologyDeclaredTouchedGraphBasisProof {
    semantic_family_key: &'static str,
    basis: TopologyTouchedGraphBasis,
    touch_descriptor: ForgeQueryGraphTouchDescriptor,
}

#[derive(Debug, Clone)]
pub struct TopologyDeclaredTouchedGraphBasis<I> {
    proof: TopologyDeclaredTouchedGraphBasisProof,
    _declaration: PhantomData<I>,
}

impl TopologyDeclaredTouchedGraphBasisProof {
    #[cfg(any(test, feature = "test-support-lowering"))]
    pub(crate) fn from_basis_for_tests(
        semantic_family_key: &'static str,
        basis: TopologyTouchedGraphBasis,
    ) -> Result<Self, ForgeQueryGraphTouchDescriptorDenial> {
        Self::from_basis(semantic_family_key, basis)
    }

    pub(crate) fn from_basis(
        semantic_family_key: &'static str,
        basis: TopologyTouchedGraphBasis,
    ) -> Result<Self, ForgeQueryGraphTouchDescriptorDenial> {
        let touch_descriptor = topology_operator_touch_descriptor_from_touched_graph_basis(&basis)?;
        Ok(Self::from_basis_with_touch_descriptor(
            semantic_family_key,
            basis,
            touch_descriptor,
        ))
    }

    pub(crate) fn from_basis_with_touch_descriptor(
        semantic_family_key: &'static str,
        basis: TopologyTouchedGraphBasis,
        touch_descriptor: ForgeQueryGraphTouchDescriptor,
    ) -> Self {
        Self {
            semantic_family_key,
            basis,
            touch_descriptor,
        }
    }

    pub(crate) fn from_mutation_sequence(
        semantic_family_key: &'static str,
        sequence: &TopologyDeclaredMutationSequence,
        operating_world: TopologyTouchedOperatingWorld,
    ) -> Result<Self, ForgeQueryGraphTouchDescriptorDenial> {
        let basis = topology_touched_graph_basis_from_mutation_sequence(sequence, operating_world);
        Self::from_basis(semantic_family_key, basis)
    }

    pub fn semantic_family_key(&self) -> &'static str {
        self.semantic_family_key
    }

    pub fn basis(&self) -> &TopologyTouchedGraphBasis {
        &self.basis
    }

    pub fn basis_digest(&self) -> &str {
        self.basis.digest()
    }

    pub const fn counters(&self) -> TopologyTouchedGraphCounters {
        self.basis.counters()
    }

    pub fn operating_world(&self) -> &TopologyTouchedOperatingWorld {
        self.basis.operating_world()
    }

    pub fn touch_descriptor(&self) -> &ForgeQueryGraphTouchDescriptor {
        &self.touch_descriptor
    }

    pub(crate) fn covers_sequence(&self, sequence: &TopologyDeclaredMutationSequence) -> bool {
        let replayed_basis = topology_touched_graph_basis_from_mutation_sequence(
            sequence,
            self.basis.operating_world().clone(),
        );
        self.basis.digest() == replayed_basis.digest()
            && self.basis.counters() == replayed_basis.counters()
    }
}

impl<I> TopologyDeclaredTouchedGraphBasis<I> {
    pub(crate) fn from_sequence(
        semantic_family_key: &'static str,
        _declaration: I,
        sequence: &TopologyDeclaredMutationSequence,
        operating_world: TopologyTouchedOperatingWorld,
    ) -> Result<Self, ForgeQueryGraphTouchDescriptorDenial> {
        let basis = topology_touched_graph_basis_from_mutation_sequence(sequence, operating_world);
        let proof = TopologyDeclaredTouchedGraphBasisProof::from_basis(semantic_family_key, basis)?;
        Ok(Self {
            proof,
            _declaration: PhantomData,
        })
    }

    pub fn proof(&self) -> &TopologyDeclaredTouchedGraphBasisProof {
        &self.proof
    }
}
