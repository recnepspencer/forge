use schema::facade::platform::entities::TopologyEntityKind;

use super::super::admitted_handoff::TopologyPrimitiveConstructionQueryAdmittedHandoff;
use super::error::TopologyPrimitiveConstructionBirthComposeExecutionError;
use crate::topology_operators::{
    TopologyDeclaredMutationSequenceBuilder, TopologyDeclaredTouchedGraphBasis,
    TopologyDeclaredTouchedGraphBasisProof, TopologyTouchedOperatingWorld,
};

const PRIMITIVE_CONSTRUCTION_BIRTH_SEMANTIC_FAMILY_KEY: &str =
    "topology.primitive_construction_birth";

#[derive(Clone, Debug)]
pub struct TopologyPrimitiveConstructionBirthDeclaredTouchedBasis {
    proof: TopologyDeclaredTouchedGraphBasisProof,
}

impl TopologyPrimitiveConstructionBirthDeclaredTouchedBasis {
    pub fn from_admitted_handoff(
        handoff: &TopologyPrimitiveConstructionQueryAdmittedHandoff,
    ) -> Result<Self, TopologyPrimitiveConstructionBirthComposeExecutionError> {
        let sequence = primitive_construction_birth_sequence_from_handoff(handoff);
        let declared = TopologyDeclaredTouchedGraphBasis::from_sequence(
            PRIMITIVE_CONSTRUCTION_BIRTH_SEMANTIC_FAMILY_KEY,
            (),
            &sequence,
            TopologyTouchedOperatingWorld::mainline(),
        )
        .map_err(|denial| {
            TopologyPrimitiveConstructionBirthComposeExecutionError::TouchedBasisDescriptor {
                reason: format!("{denial:?}"),
            }
        })?;
        Ok(Self {
            proof: declared.proof().clone(),
        })
    }

    pub(crate) fn require_matches_handoff(
        &self,
        handoff: &TopologyPrimitiveConstructionQueryAdmittedHandoff,
    ) -> Result<(), TopologyPrimitiveConstructionBirthComposeExecutionError> {
        let expected = Self::from_admitted_handoff(handoff)?;
        if self.proof.basis_digest() != expected.proof.basis_digest() {
            return Err(
                TopologyPrimitiveConstructionBirthComposeExecutionError::TouchedBasisMismatch {
                    expected_basis_digest: expected.proof.basis_digest().to_string(),
                    actual_basis_digest: self.proof.basis_digest().to_string(),
                },
            );
        }
        Ok(())
    }
}

fn primitive_construction_birth_sequence_from_handoff(
    handoff: &TopologyPrimitiveConstructionQueryAdmittedHandoff,
) -> crate::topology_operators::TopologyDeclaredMutationSequence {
    let synopsis = handoff.topology_query_handoff().birth_synopsis();
    let mut builder = TopologyDeclaredMutationSequenceBuilder::builder();
    for index in 0..synopsis.supported_vertex_count() {
        builder.create_topology_entity(
            format!(
                "primitive-construction-birth.{}.{}.vertex-{index}",
                synopsis.family().as_str(),
                handoff.admitted_handoff_digest()
            ),
            TopologyEntityKind::Vertex,
        );
    }
    builder.finish()
}
