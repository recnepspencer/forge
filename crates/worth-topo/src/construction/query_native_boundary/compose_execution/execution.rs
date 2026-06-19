use forge_query::facade::{ForgeQueryBatchWriteReceipt, ForgeQueryWorkspace};

use super::super::admitted_handoff::TopologyPrimitiveConstructionQueryAdmittedHandoff;
use super::super::surface_vocab::TopologyConstructionQueryMutationSurface;
use super::error::TopologyPrimitiveConstructionBirthComposeExecutionError;
use super::evidence::TopologyPrimitiveConstructionBirthComposeEvidence;
use super::family_programs::build_primitive_construction_birth_compose_program;
use super::program::TopologyPrimitiveConstructionBirthComposeProgram;

#[derive(Clone, Debug)]
pub struct TopologyPrimitiveConstructionBirthComposeExecution {
    program: TopologyPrimitiveConstructionBirthComposeProgram,
    receipt: ForgeQueryBatchWriteReceipt,
    evidence: TopologyPrimitiveConstructionBirthComposeEvidence,
}

impl TopologyPrimitiveConstructionBirthComposeExecution {
    pub(crate) fn new(
        program: TopologyPrimitiveConstructionBirthComposeProgram,
        receipt: ForgeQueryBatchWriteReceipt,
    ) -> Result<Self, TopologyPrimitiveConstructionBirthComposeExecutionError> {
        let graph_obligation_envelope_digest = receipt
            .graph_obligation_envelope_digest()
            .map(str::to_string)
            .ok_or(
                TopologyPrimitiveConstructionBirthComposeExecutionError::MissingGraphObligationEvidence {
                    family: program.family(),
                },
            )?;
        let evidence = TopologyPrimitiveConstructionBirthComposeEvidence::from_receipt(
            &program,
            &receipt,
            graph_obligation_envelope_digest,
        );
        Ok(Self {
            program,
            receipt,
            evidence,
        })
    }

    pub fn mutation_surface(&self) -> TopologyConstructionQueryMutationSurface {
        self.program.mutation_surface()
    }

    pub fn program(&self) -> &TopologyPrimitiveConstructionBirthComposeProgram {
        &self.program
    }

    pub fn receipt(&self) -> &ForgeQueryBatchWriteReceipt {
        &self.receipt
    }

    pub fn evidence(&self) -> &TopologyPrimitiveConstructionBirthComposeEvidence {
        &self.evidence
    }

    pub fn graph_obligation_envelope_digest(&self) -> &str {
        self.evidence.graph_obligation_envelope_digest()
    }
}

pub fn execute_primitive_construction_birth_compose(
    workspace: &mut ForgeQueryWorkspace,
    handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
) -> Result<
    TopologyPrimitiveConstructionBirthComposeExecution,
    TopologyPrimitiveConstructionBirthComposeExecutionError,
> {
    let program = build_primitive_construction_birth_compose_program(&handoff);
    let receipt = program.execute(workspace)?;
    TopologyPrimitiveConstructionBirthComposeExecution::new(program, receipt)
}
