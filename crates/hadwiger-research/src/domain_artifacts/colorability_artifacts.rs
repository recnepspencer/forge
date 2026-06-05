use std::collections::BTreeMap;

use super::checker_artifacts::{
    checker_artifact_with_entries, HadwigerCheckerCausalEvidence, HadwigerCheckerPosture,
};
use super::core_artifact::{
    impl_hadwiger_artifact, require_non_empty, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactReference,
    HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use super::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use super::query_references::HadwigerQueryDeclarationReference;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColorabilityEncoding {
    core: HadwigerArtifactCore,
    color_count: u32,
    variable_map: Vec<(String, u32, i32)>,
    clauses: Vec<Vec<i32>>,
}

impl ColorabilityEncoding {
    pub fn new(
        graph_version_reference: HadwigerArtifactReference,
        color_count: u32,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Self::checked(graph_version_reference, color_count, Vec::new(), Vec::new())
    }

    pub(crate) fn checked(
        graph_version_reference: HadwigerArtifactReference,
        color_count: u32,
        variable_map: Vec<(String, u32, i32)>,
        clauses: Vec<Vec<i32>>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        if color_count == 0 {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "color_count",
            });
        }
        let mut payload_entries = vec![HadwigerArtifactPayloadEntry::unsigned(
            "color_count",
            color_count as u128,
        )];
        for (vertex, color, variable) in &variable_map {
            payload_entries.push(HadwigerArtifactPayloadEntry::text(
                "variable",
                format!("{vertex}:{color}:{variable}"),
            ));
        }
        for clause in &clauses {
            payload_entries.push(HadwigerArtifactPayloadEntry::text(
                "clause",
                clause
                    .iter()
                    .map(i32::to_string)
                    .collect::<Vec<_>>()
                    .join(","),
            ));
        }
        let core = artifact_core(
            HadwigerArtifactKind::ColorabilityEncoding,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "colorability_encoding".to_string(),
            },
            vec![graph_version_reference],
            payload_entries,
        )?;
        Ok(Self {
            core,
            color_count,
            variable_map,
            clauses,
        })
    }

    pub fn color_count(&self) -> u32 {
        self.color_count
    }

    pub fn variable_map(&self) -> &[(String, u32, i32)] {
        &self.variable_map
    }

    pub fn clauses(&self) -> &[Vec<i32>] {
        &self.clauses
    }

    pub fn cnf_digest_token(&self) -> &str {
        self.core.artifact_digest().stable_token()
    }
}

impl_hadwiger_artifact!(ColorabilityEncoding, core);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SolverRunPosture {
    Sat,
    Unsat,
    UnsupportedCertificateBudget,
}

impl SolverRunPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sat => "sat",
            Self::Unsat => "unsat",
            Self::UnsupportedCertificateBudget => "unsupported_certificate_budget",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolverRun {
    core: HadwigerArtifactCore,
    checker_identity: String,
    checker_version: String,
    posture: SolverRunPosture,
    model: Vec<i32>,
    causal_evidence: HadwigerCheckerCausalEvidence,
    query_declaration_reference: HadwigerQueryDeclarationReference,
}

impl SolverRun {
    pub(crate) fn checked(
        encoding_reference: HadwigerArtifactReference,
        query_declaration_reference: HadwigerQueryDeclarationReference,
        checker_identity: impl Into<String>,
        checker_version: impl Into<String>,
        posture: SolverRunPosture,
        model: Vec<i32>,
        causal_evidence: HadwigerCheckerCausalEvidence,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let checker_identity = require_non_empty(checker_identity, "checker_identity")?;
        let checker_version = require_non_empty(checker_version, "checker_version")?;
        let mut entries = vec![
            HadwigerArtifactPayloadEntry::text("checker_identity", checker_identity.clone()),
            HadwigerArtifactPayloadEntry::text("checker_version", checker_version.clone()),
            HadwigerArtifactPayloadEntry::text("solver_posture", posture.as_str()),
            HadwigerArtifactPayloadEntry::text(
                "query_declaration_reference",
                query_declaration_reference.stable_token(),
            ),
        ];
        for literal in &model {
            entries.push(HadwigerArtifactPayloadEntry::text(
                "model_literal",
                literal.to_string(),
            ));
        }
        entries.extend(causal_evidence.payload_entries());
        let core = artifact_core(
            HadwigerArtifactKind::SolverRun,
            HadwigerArtifactAuthorityOwner::Checker,
            HadwigerArtifactSourceReference::CheckerBoundary {
                checker_identity: checker_identity.clone(),
                checker_version: checker_version.clone(),
            },
            vec![encoding_reference],
            entries,
        )?;
        Ok(Self {
            core,
            checker_identity,
            checker_version,
            posture,
            model,
            causal_evidence,
            query_declaration_reference,
        })
    }

    pub fn posture(&self) -> SolverRunPosture {
        self.posture
    }

    pub fn model(&self) -> &[i32] {
        &self.model
    }

    pub fn checker_identity(&self) -> &str {
        &self.checker_identity
    }

    pub fn checker_version(&self) -> &str {
        &self.checker_version
    }

    pub fn causal_evidence(&self) -> &HadwigerCheckerCausalEvidence {
        &self.causal_evidence
    }

    pub fn query_declaration_reference(&self) -> &HadwigerQueryDeclarationReference {
        &self.query_declaration_reference
    }
}

impl_hadwiger_artifact!(SolverRun, core);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColorabilityVerificationPosture {
    SatModelVerified,
    UnsatVerified,
    Rejected,
    UnsupportedCertificateBudget,
}

impl ColorabilityVerificationPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SatModelVerified => "sat_model_verified",
            Self::UnsatVerified => "unsat_verified",
            Self::Rejected => "rejected",
            Self::UnsupportedCertificateBudget => "unsupported_certificate_budget",
        }
    }

    pub fn checker_posture(self) -> HadwigerCheckerPosture {
        match self {
            Self::SatModelVerified | Self::UnsatVerified => HadwigerCheckerPosture::Admitted,
            Self::Rejected => HadwigerCheckerPosture::Rejected,
            Self::UnsupportedCertificateBudget => HadwigerCheckerPosture::Unsupported,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColorabilityVerification {
    core: HadwigerArtifactCore,
    posture: ColorabilityVerificationPosture,
    color_count: u32,
    graph_version_reference: HadwigerArtifactReference,
    query_declaration_reference: HadwigerQueryDeclarationReference,
}

impl ColorabilityVerification {
    pub(crate) fn checked(
        solver_run_reference: HadwigerArtifactReference,
        graph_version_reference: HadwigerArtifactReference,
        query_declaration_reference: HadwigerQueryDeclarationReference,
        checker_identity: impl Into<String>,
        checker_version: impl Into<String>,
        posture: ColorabilityVerificationPosture,
        color_count: u32,
        causal_evidence: HadwigerCheckerCausalEvidence,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        if color_count == 0 {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "color_count",
            });
        }
        let core = checker_artifact_with_entries(
            HadwigerArtifactKind::ColorabilityVerification,
            "colorability_verification",
            solver_run_reference,
            checker_identity,
            checker_version,
            posture.checker_posture(),
            causal_evidence,
            query_declaration_reference.clone(),
            vec![
                HadwigerArtifactPayloadEntry::unsigned("color_count", color_count as u128),
                HadwigerArtifactPayloadEntry::text(
                    "graph_version_reference",
                    graph_version_reference.stable_token(),
                ),
            ],
        )?;
        Ok(Self {
            core,
            posture,
            color_count,
            graph_version_reference,
            query_declaration_reference,
        })
    }

    pub fn posture(&self) -> ColorabilityVerificationPosture {
        self.posture
    }

    pub fn color_count(&self) -> u32 {
        self.color_count
    }

    pub fn graph_version_reference(&self) -> &HadwigerArtifactReference {
        &self.graph_version_reference
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn query_declaration_reference(&self) -> &HadwigerQueryDeclarationReference {
        &self.query_declaration_reference
    }
}

impl_hadwiger_artifact!(ColorabilityVerification, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColoringRefutationCertificate {
    variable_count: u32,
    exhausted_assignment_count: u128,
    clauses: Vec<Vec<i32>>,
}

impl ColoringRefutationCertificate {
    pub(crate) fn checked_exhaustive(
        variable_count: u32,
        exhausted_assignment_count: u128,
        clauses: Vec<Vec<i32>>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        if variable_count == 0 {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "variable_count",
            });
        }
        if clauses.is_empty() {
            return Err(HadwigerArtifactShapeError::EmptyField { field: "clauses" });
        }
        Ok(Self {
            variable_count,
            exhausted_assignment_count,
            clauses,
        })
    }

    pub(crate) fn variable_count(&self) -> u32 {
        self.variable_count
    }

    pub(crate) fn exhausted_assignment_count(&self) -> u128 {
        self.exhausted_assignment_count
    }

    pub(crate) fn clauses(&self) -> &[Vec<i32>] {
        &self.clauses
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnsatCoreArtifact {
    core: HadwigerArtifactCore,
    unsat_core_id: String,
}

impl UnsatCoreArtifact {
    pub fn candidate(
        verification_reference: HadwigerArtifactReference,
        unsat_core_id: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let unsat_core_id = require_non_empty(unsat_core_id, "unsat_core_id")?;
        let core = artifact_core(
            HadwigerArtifactKind::UnsatCoreArtifact,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "unsat_core_artifact".to_string(),
            },
            vec![verification_reference],
            vec![HadwigerArtifactPayloadEntry::text(
                "unsat_core_id",
                unsat_core_id.clone(),
            )],
        )?;
        Ok(Self {
            core,
            unsat_core_id,
        })
    }

    pub fn unsat_core_id(&self) -> &str {
        &self.unsat_core_id
    }
}

impl_hadwiger_artifact!(UnsatCoreArtifact, core);

pub(crate) fn assignment_from_model(model: &[i32]) -> BTreeMap<i32, bool> {
    model
        .iter()
        .map(|literal| (literal.abs(), *literal > 0))
        .collect()
}
