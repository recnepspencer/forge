use forge_query::facade::{ForgeQueryBatchWriteReceipt, ForgeQueryWorkspace};

use super::super::admitted_handoff::TopologyPrimitiveConstructionQueryAdmittedHandoff;
use super::super::digest_parts;
use super::super::surface_vocab::TopologyConstructionQueryMutationSurface;
use super::error::TopologyPrimitiveConstructionBirthComposeExecutionError;
use super::evidence::TopologyPrimitiveConstructionBirthComposeEvidence;
use super::family_programs::build_primitive_construction_birth_compose_program;
use super::program::TopologyPrimitiveConstructionBirthComposeProgram;
use super::touched_basis::TopologyPrimitiveConstructionBirthDeclaredTouchedBasis;

#[derive(Clone, Debug)]
pub struct TopologyPrimitiveConstructionBirthComposeExecution {
    program: TopologyPrimitiveConstructionBirthComposeProgram,
    receipt: ForgeQueryBatchWriteReceipt,
    evidence: TopologyPrimitiveConstructionBirthComposeEvidence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyPrimitiveConstructionBirthGraphAuthorityProof {
    mutation_surface: TopologyConstructionQueryMutationSurface,
    compose_program_digest: String,
    execution_evidence_digest: String,
    graph_obligation_envelope_digest: String,
    graph_obligation_selected_count: usize,
    proof_digest: String,
}

impl TopologyPrimitiveConstructionBirthGraphAuthorityProof {
    pub(crate) fn from_execution(
        execution: &TopologyPrimitiveConstructionBirthComposeExecution,
    ) -> Self {
        let evidence = execution.evidence();
        let mutation_surface = evidence.mutation_surface();
        let compose_program_digest = evidence.compose_program_digest().to_string();
        let execution_evidence_digest = evidence.evidence_digest().to_string();
        let graph_obligation_envelope_digest =
            evidence.graph_obligation_envelope_digest().to_string();
        let graph_obligation_selected_count = evidence.graph_obligation_selected_count();
        let proof_digest = digest_parts(&[
            "primitive-construction-birth-graph-authority-proof".to_string(),
            mutation_surface.as_str().to_string(),
            compose_program_digest.clone(),
            execution_evidence_digest.clone(),
            graph_obligation_envelope_digest.clone(),
            graph_obligation_selected_count.to_string(),
        ]);
        Self {
            mutation_surface,
            compose_program_digest,
            execution_evidence_digest,
            graph_obligation_envelope_digest,
            graph_obligation_selected_count,
            proof_digest,
        }
    }

    pub fn mutation_surface(&self) -> TopologyConstructionQueryMutationSurface {
        self.mutation_surface
    }

    pub fn compose_program_digest(&self) -> &str {
        &self.compose_program_digest
    }

    pub fn execution_evidence_digest(&self) -> &str {
        &self.execution_evidence_digest
    }

    pub fn graph_obligation_envelope_digest(&self) -> &str {
        &self.graph_obligation_envelope_digest
    }

    pub fn graph_obligation_selected_count(&self) -> usize {
        self.graph_obligation_selected_count
    }

    pub fn proof_digest(&self) -> &str {
        &self.proof_digest
    }
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

    pub fn graph_authority_proof(&self) -> TopologyPrimitiveConstructionBirthGraphAuthorityProof {
        TopologyPrimitiveConstructionBirthGraphAuthorityProof::from_execution(self)
    }
}

pub fn run_primitive_construction_birth_declared_touched_basis_compose(
    workspace: &mut ForgeQueryWorkspace,
    handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
    declared_touched_basis: TopologyPrimitiveConstructionBirthDeclaredTouchedBasis,
) -> Result<
    TopologyPrimitiveConstructionBirthComposeExecution,
    TopologyPrimitiveConstructionBirthComposeExecutionError,
> {
    let program = build_primitive_construction_birth_compose_program(&handoff);
    let receipt = program.execute_declared_touched_basis_checked(
        workspace,
        &handoff,
        &declared_touched_basis,
    )?;
    TopologyPrimitiveConstructionBirthComposeExecution::new(program, receipt)
}

#[cfg(test)]
pub(crate) fn execute_primitive_construction_birth_compose(
    workspace: &mut ForgeQueryWorkspace,
    handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
    declared_touched_basis: TopologyPrimitiveConstructionBirthDeclaredTouchedBasis,
) -> Result<
    TopologyPrimitiveConstructionBirthComposeExecution,
    TopologyPrimitiveConstructionBirthComposeExecutionError,
> {
    run_primitive_construction_birth_declared_touched_basis_compose(
        workspace,
        handoff,
        declared_touched_basis,
    )
}

pub fn topology_primitive_construction_birth_graph_authority_proof(
    execution: &TopologyPrimitiveConstructionBirthComposeExecution,
) -> TopologyPrimitiveConstructionBirthGraphAuthorityProof {
    execution.graph_authority_proof()
}
