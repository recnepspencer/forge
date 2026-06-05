use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, require_non_empty, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactShapeError,
    HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileConstraintSignature {
    tile_id: String,
    signature_token: String,
}

impl TileConstraintSignature {
    pub(crate) fn new(
        tile_id: impl Into<String>,
        signature_token: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            tile_id: require_non_empty(tile_id, "tile_id")?,
            signature_token: require_non_empty(signature_token, "signature_token")?,
        })
    }

    pub fn tile_id(&self) -> &str {
        &self.tile_id
    }

    pub fn signature_token(&self) -> &str {
        &self.signature_token
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileContactGraphSignature {
    inner: TileConstraintSignature,
}

impl TileContactGraphSignature {
    pub fn from_edges(
        tile_id: impl Into<String>,
        edges: impl IntoIterator<Item = (&'static str, &'static str)>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let mut edge_tokens = edges
            .into_iter()
            .map(|(left, right)| normalized_edge_token(left, right))
            .collect::<Result<Vec<_>, _>>()?;
        if edge_tokens.is_empty() {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "contact_edges",
            });
        }
        edge_tokens.sort();
        edge_tokens.dedup();
        Ok(Self {
            inner: TileConstraintSignature::new(tile_id, edge_tokens.join("|"))?,
        })
    }

    pub fn tile_id(&self) -> &str {
        self.inner.tile_id()
    }

    pub fn signature_token(&self) -> &str {
        self.inner.signature_token()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileMetricThresholdSignature {
    inner: TileConstraintSignature,
}

impl TileMetricThresholdSignature {
    pub fn from_thresholds(
        tile_id: impl Into<String>,
        thresholds: impl IntoIterator<Item = &'static str>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let mut tokens = thresholds
            .into_iter()
            .map(|threshold| require_non_empty(threshold, "metric_threshold"))
            .collect::<Result<Vec<_>, _>>()?;
        if tokens.is_empty() {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "metric_thresholds",
            });
        }
        tokens.sort();
        tokens.dedup();
        Ok(Self {
            inner: TileConstraintSignature::new(tile_id, tokens.join("|"))?,
        })
    }

    pub fn signature_token(&self) -> &str {
        self.inner.signature_token()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeriodicColorRuleSignature {
    modulus: u32,
    color_a: i128,
    color_b: i128,
}

impl PeriodicColorRuleSignature {
    pub fn new(
        modulus: u32,
        color_a: i128,
        color_b: i128,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        if modulus == 0 {
            return Err(HadwigerArtifactShapeError::EmptyField { field: "modulus" });
        }
        Ok(Self {
            modulus,
            color_a,
            color_b,
        })
    }

    pub fn signature_token(&self) -> String {
        format!(
            "mod={}:a={}:b={}",
            self.modulus,
            self.color_a.rem_euclid(self.modulus as i128),
            self.color_b.rem_euclid(self.modulus as i128)
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TileEquivalenceScope {
    ContactConstraint,
    MetricThreshold,
    PeriodicColorRule,
}

impl TileEquivalenceScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ContactConstraint => "contact_constraint",
            Self::MetricThreshold => "metric_threshold",
            Self::PeriodicColorRule => "periodic_color_rule",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TileEquivalencePosture {
    BlocksDuplicateCheckerWork,
    Unsupported,
}

impl TileEquivalencePosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BlocksDuplicateCheckerWork => "blocks_duplicate_checker_work",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileEquivalenceWitness {
    core: HadwigerArtifactCore,
    witness_id: String,
    scope: TileEquivalenceScope,
    posture: TileEquivalencePosture,
    left_signature_token: String,
    right_signature_token: String,
}

impl TileEquivalenceWitness {
    pub fn builder(
        witness_id: impl Into<String>,
        scope: TileEquivalenceScope,
    ) -> TileEquivalenceWitnessBuilder {
        TileEquivalenceWitnessBuilder {
            witness_id: witness_id.into(),
            scope,
            left_signature_token: None,
            right_signature_token: None,
        }
    }

    fn new(
        witness_id: String,
        scope: TileEquivalenceScope,
        left_signature_token: String,
        right_signature_token: String,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let witness_id = require_non_empty(witness_id, "witness_id")?;
        let posture = if left_signature_token == right_signature_token {
            TileEquivalencePosture::BlocksDuplicateCheckerWork
        } else {
            TileEquivalencePosture::Unsupported
        };
        let core = artifact_core(
            HadwigerArtifactKind::TileEquivalenceWitness,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "tile_equivalence_witness".to_string(),
            },
            Vec::new(),
            vec![
                HadwigerArtifactPayloadEntry::text("witness_id", witness_id.clone()),
                HadwigerArtifactPayloadEntry::text("scope", scope.as_str()),
                HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
                HadwigerArtifactPayloadEntry::text("left_signature", left_signature_token.clone()),
                HadwigerArtifactPayloadEntry::text(
                    "right_signature",
                    right_signature_token.clone(),
                ),
            ],
        )?;
        Ok(Self {
            core,
            witness_id,
            scope,
            posture,
            left_signature_token,
            right_signature_token,
        })
    }

    pub fn witness_id(&self) -> &str {
        &self.witness_id
    }

    pub fn scope(&self) -> TileEquivalenceScope {
        self.scope
    }

    pub fn posture(&self) -> TileEquivalencePosture {
        self.posture
    }

    pub fn blocks_duplicate_checker_work(&self) -> bool {
        self.posture == TileEquivalencePosture::BlocksDuplicateCheckerWork
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn admits_checker_authority(&self) -> bool {
        false
    }

    pub fn equivalence_token(&self) -> String {
        format!(
            "{}:{}:{}",
            self.scope.as_str(),
            self.left_signature_token,
            self.right_signature_token
        )
    }
}

impl_hadwiger_artifact!(TileEquivalenceWitness, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileEquivalenceWitnessBuilder {
    witness_id: String,
    scope: TileEquivalenceScope,
    left_signature_token: Option<String>,
    right_signature_token: Option<String>,
}

impl TileEquivalenceWitnessBuilder {
    pub fn with_left_contact_signature(mut self, signature: TileContactGraphSignature) -> Self {
        self.left_signature_token = Some(signature.signature_token().to_string());
        self
    }

    pub fn with_right_contact_signature(mut self, signature: TileContactGraphSignature) -> Self {
        self.right_signature_token = Some(signature.signature_token().to_string());
        self
    }

    pub fn with_left_metric_signature(mut self, signature: TileMetricThresholdSignature) -> Self {
        self.left_signature_token = Some(signature.signature_token().to_string());
        self
    }

    pub fn with_right_metric_signature(mut self, signature: TileMetricThresholdSignature) -> Self {
        self.right_signature_token = Some(signature.signature_token().to_string());
        self
    }

    pub fn with_left_color_rule_signature(mut self, signature: PeriodicColorRuleSignature) -> Self {
        self.left_signature_token = Some(signature.signature_token());
        self
    }

    pub fn with_right_color_rule_signature(
        mut self,
        signature: PeriodicColorRuleSignature,
    ) -> Self {
        self.right_signature_token = Some(signature.signature_token());
        self
    }

    pub fn finish(self) -> Result<TileEquivalenceWitness, HadwigerArtifactShapeError> {
        TileEquivalenceWitness::new(
            self.witness_id,
            self.scope,
            self.left_signature_token
                .ok_or(HadwigerArtifactShapeError::EmptyField {
                    field: "left_signature",
                })?,
            self.right_signature_token
                .ok_or(HadwigerArtifactShapeError::EmptyField {
                    field: "right_signature",
                })?,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileEquivalenceWitnessChecked {
    witness: TileEquivalenceWitness,
    query_readiness_rows: usize,
}

impl TileEquivalenceWitnessChecked {
    pub(crate) fn new(witness: TileEquivalenceWitness, query_readiness_rows: usize) -> Self {
        Self {
            witness,
            query_readiness_rows,
        }
    }

    pub fn witness(&self) -> &TileEquivalenceWitness {
        &self.witness
    }

    pub fn query_readiness_rows(&self) -> usize {
        self.query_readiness_rows
    }

    pub fn blocks_duplicate_checker_work(&self) -> bool {
        self.witness.blocks_duplicate_checker_work()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

fn normalized_edge_token(left: &str, right: &str) -> Result<String, HadwigerArtifactShapeError> {
    let left = require_non_empty(left, "contact_left")?;
    let right = require_non_empty(right, "contact_right")?;
    if left <= right {
        Ok(format!("{left}--{right}"))
    } else {
        Ok(format!("{right}--{left}"))
    }
}
