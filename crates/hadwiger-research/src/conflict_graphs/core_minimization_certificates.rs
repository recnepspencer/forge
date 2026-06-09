use std::collections::BTreeSet;

use crate::domain_artifacts::{
    ColorabilityVerification, ColorabilityVerificationPosture, GraphVersion,
    HadwigerCanonicalArtifact,
};

use super::conflict_graph_errors::{require_conflict_non_empty, ConflictGraphError};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ConflictCoreDeletionCheckKind {
    VertexRemoval,
    EdgeRemoval,
}

impl ConflictCoreDeletionCheckKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::VertexRemoval => "vertex_removal",
            Self::EdgeRemoval => "edge_removal",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ConflictCoreDeletionCheckPosture {
    ColorableAfterDeletion,
    StillNonColorable,
    Unsupported,
}

impl ConflictCoreDeletionCheckPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ColorableAfterDeletion => "colorable_after_deletion",
            Self::StillNonColorable => "still_non_colorable",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictCoreDeletionCheck {
    kind: ConflictCoreDeletionCheckKind,
    target: String,
    posture: ConflictCoreDeletionCheckPosture,
    deletion_graph: Option<GraphVersion>,
    colorability_verification: Option<ColorabilityVerification>,
}

impl ConflictCoreDeletionCheck {
    pub fn vertex_colorable_after_deletion(
        target: impl Into<String>,
        deletion_graph: GraphVersion,
        verification: ColorabilityVerification,
    ) -> Result<Self, ConflictGraphError> {
        Self::new(
            ConflictCoreDeletionCheckKind::VertexRemoval,
            target,
            ConflictCoreDeletionCheckPosture::ColorableAfterDeletion,
            Some(deletion_graph),
            Some(verification),
        )
    }

    pub fn edge_colorable_after_deletion(
        left: impl Into<String>,
        right: impl Into<String>,
        deletion_graph: GraphVersion,
        verification: ColorabilityVerification,
    ) -> Result<Self, ConflictGraphError> {
        let target = normalized_edge_target(left, right)?;
        Self::new(
            ConflictCoreDeletionCheckKind::EdgeRemoval,
            target,
            ConflictCoreDeletionCheckPosture::ColorableAfterDeletion,
            Some(deletion_graph),
            Some(verification),
        )
    }

    pub fn unsupported(
        kind: ConflictCoreDeletionCheckKind,
        target: impl Into<String>,
    ) -> Result<Self, ConflictGraphError> {
        Self::new(
            kind,
            target,
            ConflictCoreDeletionCheckPosture::Unsupported,
            None,
            None,
        )
    }

    pub fn kind(&self) -> ConflictCoreDeletionCheckKind {
        self.kind
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn posture(&self) -> ConflictCoreDeletionCheckPosture {
        self.posture
    }

    pub fn colorability_verification(&self) -> Option<&ColorabilityVerification> {
        self.colorability_verification.as_ref()
    }

    pub fn deletion_graph(&self) -> Option<&GraphVersion> {
        self.deletion_graph.as_ref()
    }

    pub fn stable_token(&self) -> String {
        let deletion_graph = self
            .deletion_graph
            .as_ref()
            .map(|graph| graph.reference().stable_token())
            .unwrap_or_else(|| "none".to_string());
        let verification = self
            .colorability_verification
            .as_ref()
            .map(|verification| verification.reference().stable_token())
            .unwrap_or_else(|| "none".to_string());
        format!(
            "{}:{}:{}:{}:{}",
            self.kind.as_str(),
            self.target,
            self.posture.as_str(),
            deletion_graph,
            verification
        )
    }

    pub(crate) fn proves_colorable_deletion(&self) -> bool {
        self.posture == ConflictCoreDeletionCheckPosture::ColorableAfterDeletion
            && self
                .colorability_verification
                .as_ref()
                .is_some_and(|verification| {
                    verification.posture() == ColorabilityVerificationPosture::SatModelVerified
                })
    }

    fn new(
        kind: ConflictCoreDeletionCheckKind,
        target: impl Into<String>,
        posture: ConflictCoreDeletionCheckPosture,
        deletion_graph: Option<GraphVersion>,
        colorability_verification: Option<ColorabilityVerification>,
    ) -> Result<Self, ConflictGraphError> {
        Ok(Self {
            kind,
            target: require_conflict_non_empty(target, "deletion_check_target")?,
            posture,
            deletion_graph,
            colorability_verification,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictCoreMinimalityCertificate {
    certificate_id: String,
    deletion_checks: Vec<ConflictCoreDeletionCheck>,
}

impl ConflictCoreMinimalityCertificate {
    pub fn new(
        certificate_id: impl Into<String>,
        mut deletion_checks: Vec<ConflictCoreDeletionCheck>,
    ) -> Result<Self, ConflictGraphError> {
        let certificate_id = require_conflict_non_empty(certificate_id, "certificate_id")?;
        if deletion_checks.is_empty() {
            return Err(ConflictGraphError::MissingDeletionChecks {
                expected: 1,
                actual: 0,
            });
        }
        deletion_checks.sort_by_key(ConflictCoreDeletionCheck::stable_token);
        reject_duplicate_deletion_checks(&deletion_checks)?;
        Ok(Self {
            certificate_id,
            deletion_checks,
        })
    }

    pub fn certificate_id(&self) -> &str {
        &self.certificate_id
    }

    pub fn deletion_checks(&self) -> &[ConflictCoreDeletionCheck] {
        &self.deletion_checks
    }

    pub fn stable_token(&self) -> String {
        let checks = self
            .deletion_checks
            .iter()
            .map(ConflictCoreDeletionCheck::stable_token)
            .collect::<Vec<_>>()
            .join("|");
        format!("{}:{}", self.certificate_id, checks)
    }
}

fn reject_duplicate_deletion_checks(
    checks: &[ConflictCoreDeletionCheck],
) -> Result<(), ConflictGraphError> {
    let mut seen = BTreeSet::new();
    for check in checks {
        let key = format!("{}:{}", check.kind().as_str(), check.target());
        if !seen.insert(key.clone()) {
            return Err(ConflictGraphError::DuplicateDeletionCheck { target: key });
        }
    }
    Ok(())
}

fn normalized_edge_target(
    left: impl Into<String>,
    right: impl Into<String>,
) -> Result<String, ConflictGraphError> {
    let mut left = require_conflict_non_empty(left, "left_edge_target")?;
    let mut right = require_conflict_non_empty(right, "right_edge_target")?;
    if right < left {
        std::mem::swap(&mut left, &mut right);
    }
    Ok(format!("{left}:{right}"))
}
