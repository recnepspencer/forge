use std::collections::BTreeSet;

use serde::Serialize;
use sha2::{Digest, Sha256};

use super::{
    PreflightEvidenceIdentity, StructuralPredicateVerdict, StructuralPreflightEvidence,
    StructuralPreflightPlan,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructuralPreflightIntegrityDenial {
    UnsupportedPlanSchema(u32),
    UnsupportedEvidenceSchema(u32),
    PredicateSetMismatch,
    DuplicatePredicate,
    PredicateEvidenceMismatch,
    PredicateInputIdentityMismatch,
    PredicateToolIdentityMismatch,
    ToolExecutionMismatch,
    PlanIdentityMismatch,
    EvidenceIdentityMismatch,
    IdentityEncoding,
}

impl StructuralPreflightPlan {
    pub fn validate_integrity(&self) -> Result<(), StructuralPreflightIntegrityDenial> {
        if self.schema_version != 1 {
            return Err(StructuralPreflightIntegrityDenial::UnsupportedPlanSchema(
                self.schema_version,
            ));
        }
        let requested = self
            .request
            .predicates
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        let planned = self
            .predicates
            .iter()
            .map(|predicate| predicate.predicate)
            .collect::<BTreeSet<_>>();
        if requested.len() != self.request.predicates.len()
            || planned.len() != self.predicates.len()
        {
            return Err(StructuralPreflightIntegrityDenial::DuplicatePredicate);
        }
        if requested != planned {
            return Err(StructuralPreflightIntegrityDenial::PredicateSetMismatch);
        }
        if self.predicates.iter().any(|predicate| {
            predicate.tool.as_ref().is_some_and(|tool| {
                tool.tool_identity.trim().is_empty()
                    || tool.provenance.trim().is_empty()
                    || tool.program.trim().is_empty()
                    || tool.resolved_program_path.trim().is_empty()
                    || tool.program_sha256.len() != 64
                    || tool.program_version_identity.trim().is_empty()
                    || tool.source_scope_identity.trim().is_empty()
                    || tool.timeout_millis == 0
                    || tool.resource_posture.trim().is_empty()
                    || !predicate
                        .input_scopes
                        .iter()
                        .any(|scope| scope.scope_identity == tool.source_scope_identity)
            })
        }) {
            return Err(StructuralPreflightIntegrityDenial::ToolExecutionMismatch);
        }
        let mut unsigned = self.clone();
        unsigned.plan_identity.clear();
        if self.plan_identity != digest_serialized(&unsigned)? {
            return Err(StructuralPreflightIntegrityDenial::PlanIdentityMismatch);
        }
        Ok(())
    }
}

impl StructuralPreflightEvidence {
    pub fn validate_integrity(&self) -> Result<(), StructuralPreflightIntegrityDenial> {
        if self.schema_version != 1 {
            return Err(
                StructuralPreflightIntegrityDenial::UnsupportedEvidenceSchema(self.schema_version),
            );
        }
        self.plan.validate_integrity()?;
        let observed = self
            .predicates
            .iter()
            .map(|predicate| predicate.predicate)
            .collect::<BTreeSet<_>>();
        if observed.len() != self.predicates.len() {
            return Err(StructuralPreflightIntegrityDenial::DuplicatePredicate);
        }
        let planned = self
            .plan
            .predicates
            .iter()
            .map(|predicate| predicate.predicate)
            .collect::<BTreeSet<_>>();
        if observed != planned {
            return Err(StructuralPreflightIntegrityDenial::PredicateEvidenceMismatch);
        }
        for evidence in &self.predicates {
            let plan = self
                .plan
                .predicates
                .iter()
                .find(|candidate| candidate.predicate == evidence.predicate)
                .ok_or(StructuralPreflightIntegrityDenial::PredicateEvidenceMismatch)?;
            let expected_input = digest_serialized(
                &plan
                    .input_scopes
                    .iter()
                    .map(|scope| (&scope.scope_identity, &scope.input_identity))
                    .collect::<Vec<_>>(),
            )?;
            if evidence.input_identity != expected_input {
                return Err(StructuralPreflightIntegrityDenial::PredicateInputIdentityMismatch);
            }
            if evidence.tool_identity.is_some() != plan.tool.is_some() {
                return Err(StructuralPreflightIntegrityDenial::PredicateToolIdentityMismatch);
            }
            if let Some(tool_identity) = &evidence.tool_identity {
                let execution = self
                    .tool_executions
                    .iter()
                    .find(|execution| execution.authority_identity == *tool_identity)
                    .ok_or(
                        StructuralPreflightIntegrityDenial::PredicateToolIdentityMismatch,
                    )?;
                if matches!(
                    &evidence.verdict,
                    StructuralPredicateVerdict::Passed { .. }
                ) && !execution.successful
                {
                    return Err(
                        StructuralPreflightIntegrityDenial::PredicateToolIdentityMismatch,
                    );
                }
            }
            if let StructuralPredicateVerdict::Failed { failure } = &evidence.verdict {
                if failure.predicate != evidence.predicate {
                    return Err(StructuralPreflightIntegrityDenial::PredicateEvidenceMismatch);
                }
            }
        }
        let planned_commands = self
            .plan
            .predicates
            .iter()
            .filter_map(|predicate| predicate.tool.as_ref())
            .map(tool_command_identity)
            .collect::<Result<BTreeSet<_>, _>>()?;
        let executed_commands = self
            .tool_executions
            .iter()
            .map(|execution| execution.command_identity.clone())
            .collect::<BTreeSet<_>>();
        if planned_commands != executed_commands
            || executed_commands.len() != self.tool_executions.len()
        {
            return Err(StructuralPreflightIntegrityDenial::ToolExecutionMismatch);
        }
        for execution in &self.tool_executions {
            let mut expected = self
                .plan
                .predicates
                .iter()
                .filter_map(|predicate| predicate.tool.as_ref())
                .filter(|tool| {
                    tool_command_identity(tool)
                        .is_ok_and(|identity| identity == execution.command_identity)
                })
                .map(|tool| tool.tool_identity.clone())
                .collect::<Vec<_>>();
            expected.sort();
            expected.dedup();
            let exemplar = self
                .plan
                .predicates
                .iter()
                .filter_map(|predicate| predicate.tool.as_ref())
                .find(|tool| {
                    tool_command_identity(tool)
                        .is_ok_and(|identity| identity == execution.command_identity)
                })
                .ok_or(StructuralPreflightIntegrityDenial::ToolExecutionMismatch)?;
            if execution.declared_tool_identities != expected
                || execution.provenance != exemplar.provenance
                || execution.program != exemplar.program
                || execution.resolved_program_path != exemplar.resolved_program_path
                || execution.program_sha256 != exemplar.program_sha256
                || execution.program_version_identity != exemplar.program_version_identity
                || execution.arguments != exemplar.arguments
                || execution.timeout_millis != exemplar.timeout_millis
                || execution.resource_posture != exemplar.resource_posture
                || (execution.timed_out && execution.successful)
                || (execution.successful && execution.process_id == 0)
            {
                return Err(StructuralPreflightIntegrityDenial::ToolExecutionMismatch);
            }
        }
        let mut unsigned = self.clone();
        unsigned.evidence_identity = PreflightEvidenceIdentity(String::new());
        if self.evidence_identity.0 != digest_serialized(&unsigned)? {
            return Err(StructuralPreflightIntegrityDenial::EvidenceIdentityMismatch);
        }
        Ok(())
    }
}

impl std::fmt::Display for StructuralPreflightIntegrityDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "structural preflight integrity denied: {self:?}")
    }
}

impl std::error::Error for StructuralPreflightIntegrityDenial {}

fn digest_serialized(
    value: &impl Serialize,
) -> Result<String, StructuralPreflightIntegrityDenial> {
    serde_json::to_vec(value)
        .map(|bytes| {
            Sha256::digest(bytes)
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect()
        })
        .map_err(|_| StructuralPreflightIntegrityDenial::IdentityEncoding)
}

fn tool_command_identity(
    tool: &super::StructuralToolDeclaration,
) -> Result<String, StructuralPreflightIntegrityDenial> {
    digest_serialized(&(
        &tool.provenance,
        &tool.program,
        &tool.resolved_program_path,
        &tool.program_sha256,
        &tool.program_version_identity,
        &tool.arguments,
        tool.timeout_millis,
        &tool.resource_posture,
    ))
}
