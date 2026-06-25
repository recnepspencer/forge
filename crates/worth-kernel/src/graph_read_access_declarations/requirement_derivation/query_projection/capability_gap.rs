use super::super::stable_identity_digest::stable_digest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadRequirementDerivationCapabilityGapKind {
    MissingQueryReadFamilyArtifact,
    MissingQueryAccessShapeArtifact,
    MissingQuerySelectivityShapeArtifact,
    QueryRequirementDerivationDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadRequirementDerivationCapabilityGap {
    kind: WorthGraphReadRequirementDerivationCapabilityGapKind,
    source_catalog_record_digest: String,
    query_family_anchor_digest: String,
    missing_prerequisite: &'static str,
    query_api_required: &'static str,
    blocker: String,
    query_capability_labels: Vec<&'static str>,
    removal_trigger: String,
    gap_digest: String,
}

impl WorthGraphReadRequirementDerivationCapabilityGap {
    pub(crate) fn missing_query_read_family_artifact(
        source_catalog_record_digest: impl Into<String>,
        query_family_anchor_digest: impl Into<String>,
        requirement_capability_labels: &[&'static str],
    ) -> Self {
        Self::missing_prerequisite_gap(
            WorthGraphReadRequirementDerivationCapabilityGapKind::MissingQueryReadFamilyArtifact,
            source_catalog_record_digest,
            query_family_anchor_digest,
            "ForgeQueryReadFamily",
            "explain_graph_read_access_requirements_for_family(...)",
            "Phase 2 catalog records currently carry a Query family anchor, but not a real ForgeQueryReadFamily artifact that Query can inspect.",
            "Replace this gap when the catalog lowers its anchor into a real ForgeQueryReadFamily through a public Query declaration path.",
            requirement_capability_labels,
        )
    }

    fn missing_prerequisite_gap(
        kind: WorthGraphReadRequirementDerivationCapabilityGapKind,
        source_catalog_record_digest: impl Into<String>,
        query_family_anchor_digest: impl Into<String>,
        missing_prerequisite: &'static str,
        query_api_required: &'static str,
        blocker: &'static str,
        removal_trigger: &'static str,
        requirement_capability_labels: &[&'static str],
    ) -> Self {
        let source_catalog_record_digest = source_catalog_record_digest.into();
        let query_family_anchor_digest = query_family_anchor_digest.into();
        let mut query_capability_labels = requirement_capability_labels.to_vec();
        query_capability_labels.sort_unstable();
        let gap_digest = stable_digest(&[
            "worth_graph_read_requirement_derivation_gap_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("catalog_record:{source_catalog_record_digest}"),
            format!("query_family_anchor:{query_family_anchor_digest}"),
            format!("missing_prerequisite:{missing_prerequisite}"),
            format!("query_api_required:{query_api_required}"),
            format!("labels:{}", query_capability_labels.join("|")),
            format!("blocker:{blocker}"),
            format!("removal_trigger:{removal_trigger}"),
        ]);
        Self {
            kind,
            source_catalog_record_digest,
            query_family_anchor_digest,
            missing_prerequisite,
            query_api_required,
            blocker: blocker.to_string(),
            query_capability_labels,
            removal_trigger: removal_trigger.to_string(),
            gap_digest,
        }
    }

    pub const fn kind(&self) -> WorthGraphReadRequirementDerivationCapabilityGapKind {
        self.kind
    }

    pub fn source_catalog_record_digest(&self) -> &str {
        &self.source_catalog_record_digest
    }

    pub fn query_family_anchor_digest(&self) -> &str {
        &self.query_family_anchor_digest
    }

    pub const fn missing_prerequisite(&self) -> &'static str {
        self.missing_prerequisite
    }

    pub const fn query_api_required(&self) -> &'static str {
        self.query_api_required
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn query_capability_labels(&self) -> &[&'static str] {
        &self.query_capability_labels
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub fn gap_digest(&self) -> &str {
        &self.gap_digest
    }

    pub const fn claims_query_requirement_rows_derived(&self) -> bool {
        false
    }
}

impl WorthGraphReadRequirementDerivationCapabilityGapKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingQueryReadFamilyArtifact => "missing_query_read_family_artifact",
            Self::MissingQueryAccessShapeArtifact => "missing_query_access_shape_artifact",
            Self::MissingQuerySelectivityShapeArtifact => {
                "missing_query_selectivity_shape_artifact"
            }
            Self::QueryRequirementDerivationDenied => "query_requirement_derivation_denied",
        }
    }
}
