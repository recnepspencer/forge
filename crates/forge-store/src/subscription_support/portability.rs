use super::{
    classification_error, cost_surface_for_program_path, stable_digest,
    CompletedSupportProgramAction, SubscriptionSupportActionOrigin, SubscriptionSupportArtifactId,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportOperationalBasis, SubscriptionSupportOperationalVerdict,
    SubscriptionSupportPlanFamily, SubscriptionSupportResultCostSurface, SubscriptionSupportRole,
    SupportActionId, SupportAffectedSetDigest, SupportProgramDensityClass, SupportProgramPathPlan,
};
use crate::failure::StoreError;
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SupportPortabilityManifestBudget {
    max_manifest_entries: u64,
    max_manifest_header_bytes: u64,
}

impl SupportPortabilityManifestBudget {
    pub fn new(
        max_manifest_entries: u64,
        max_manifest_header_bytes: u64,
    ) -> Result<Self, StoreError> {
        if max_manifest_entries == 0 || max_manifest_header_bytes == 0 {
            return Err(classification_error(
                "subscription-support portability manifest budgets must be non-zero",
            ));
        }
        Ok(Self {
            max_manifest_entries,
            max_manifest_header_bytes,
        })
    }

    pub fn admits(&self, manifest_entries: u64, manifest_header_bytes: u64) -> bool {
        manifest_entries <= self.max_manifest_entries
            && manifest_header_bytes <= self.max_manifest_header_bytes
    }

    pub fn max_manifest_entries(&self) -> u64 {
        self.max_manifest_entries
    }

    pub fn max_manifest_header_bytes(&self) -> u64 {
        self.max_manifest_header_bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportPortabilityAffectedSet {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    action_origin: SubscriptionSupportActionOrigin,
    affected_set_digest: SupportAffectedSetDigest,
    affected_bases: Vec<SubscriptionSupportOperationalBasis>,
}

impl SupportPortabilityAffectedSet {
    pub(crate) fn from_portability_bases(
        affected_bases: Vec<SubscriptionSupportOperationalBasis>,
    ) -> Result<Self, StoreError> {
        let Some(first) = affected_bases.first() else {
            return Err(classification_error(
                "subscription-support portability affected sets must not be empty",
            ));
        };
        if !matches!(
            first.action_origin(),
            SubscriptionSupportActionOrigin::ReplicationExport
                | SubscriptionSupportActionOrigin::ReplicationImport
        ) {
            return Err(classification_error(
                "subscription-support portability affected sets require export/import-origin bases",
            ));
        }
        for basis in &affected_bases {
            if basis.action_origin() != first.action_origin() {
                return Err(classification_error(
                    "subscription-support portability affected sets cannot mix export and import origins",
                ));
            }
            if basis.family_id() != first.family_id()
                || basis.family_kind() != first.family_kind()
                || basis.support_role() != first.support_role()
            {
                return Err(classification_error(
                    "subscription-support portability affected sets must be family-local",
                ));
            }
        }
        Ok(Self {
            family_id: first.family_id().clone(),
            family_kind: first.family_kind(),
            support_role: first.support_role(),
            action_origin: first.action_origin(),
            affected_set_digest: SupportAffectedSetDigest::from_bases(&affected_bases)?,
            affected_bases,
        })
    }

    pub fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub fn support_role(&self) -> SubscriptionSupportRole {
        self.support_role
    }

    pub fn action_origin(&self) -> SubscriptionSupportActionOrigin {
        self.action_origin
    }

    pub fn affected_count(&self) -> u64 {
        self.affected_bases.len() as u64
    }

    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub(crate) fn primary_basis(&self) -> &SubscriptionSupportOperationalBasis {
        &self.affected_bases[0]
    }

    pub(crate) fn affected_artifact_ids(&self) -> Vec<SubscriptionSupportArtifactId> {
        self.affected_bases
            .iter()
            .map(|basis| basis.artifact_id().clone())
            .collect()
    }

    pub(crate) fn basis_digests_for_artifact_ids(
        &self,
        basis_artifact_ids: &[SubscriptionSupportArtifactId],
    ) -> Result<Vec<String>, StoreError> {
        validate_basis_artifact_ids(self, basis_artifact_ids, &[])?;
        let included = basis_artifact_ids.iter().collect::<BTreeSet<_>>();
        Ok(self
            .affected_bases
            .iter()
            .filter(|basis| included.contains(basis.artifact_id()))
            .map(|basis| basis.basis_digest().to_string())
            .collect())
    }

    pub(crate) fn all_artifacts_omitted(&self) -> Vec<SubscriptionSupportArtifactId> {
        self.affected_artifact_ids()
    }

    pub(crate) fn contains_artifact_id(&self, artifact_id: &SubscriptionSupportArtifactId) -> bool {
        self.affected_bases
            .iter()
            .any(|basis| basis.artifact_id() == artifact_id)
    }

    pub(crate) fn portability_digests(&self) -> Vec<String> {
        self.affected_bases
            .iter()
            .map(|basis| basis.portability_digest().to_string())
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportPortabilityScopeFootprint {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    included_support_count: u64,
    required_basis_count: u64,
    omitted_support_count: u64,
    footprint_digest: String,
}

impl SupportPortabilityScopeFootprint {
    pub(crate) fn new(
        affected_set: &SupportPortabilityAffectedSet,
        included_support_count: u64,
        omitted_support_count: u64,
        omitted_artifact_ids: &[SubscriptionSupportArtifactId],
        basis_artifact_ids: &[SubscriptionSupportArtifactId],
    ) -> Result<Self, StoreError> {
        if included_support_count + omitted_support_count != affected_set.affected_count() {
            return Err(classification_error(
                "subscription-support portability footprint must account for every affected support artifact",
            ));
        }
        if omitted_artifact_ids.len() as u64 != omitted_support_count {
            return Err(classification_error(
                "subscription-support portability footprint omitted ids must match omitted count",
            ));
        }
        validate_omitted_artifact_ids(affected_set, omitted_artifact_ids)?;
        validate_basis_artifact_ids(affected_set, basis_artifact_ids, omitted_artifact_ids)?;
        let required_basis_count = basis_artifact_ids.len() as u64;
        let footprint_digest = stable_digest(&(
            affected_set.affected_set_digest(),
            affected_set.portability_digests(),
            omitted_artifact_ids,
            included_support_count,
            required_basis_count,
            omitted_support_count,
        ))?;
        Ok(Self {
            family_id: affected_set.family_id().clone(),
            family_kind: affected_set.family_kind(),
            support_role: affected_set.support_role(),
            included_support_count,
            required_basis_count,
            omitted_support_count,
            footprint_digest,
        })
    }

    pub fn included_support_count(&self) -> u64 {
        self.included_support_count
    }

    pub fn required_basis_count(&self) -> u64 {
        self.required_basis_count
    }

    pub fn omitted_support_count(&self) -> u64 {
        self.omitted_support_count
    }

    pub fn footprint_digest(&self) -> &str {
        &self.footprint_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CapsuleSupportManifest {
    affected_set_digest: SupportAffectedSetDigest,
    footprint: SupportPortabilityScopeFootprint,
    manifest_entry_count: u64,
    manifest_header_bytes: u64,
    required_basis_digests: Vec<String>,
    manifest_digest: String,
}

impl CapsuleSupportManifest {
    pub(crate) fn new(
        affected_set: &SupportPortabilityAffectedSet,
        footprint: SupportPortabilityScopeFootprint,
        budget: SupportPortabilityManifestBudget,
        manifest_header_bytes: u64,
        basis_artifact_ids: &[SubscriptionSupportArtifactId],
    ) -> Result<Self, StoreError> {
        let manifest_entry_count = footprint.included_support_count();
        if !budget.admits(manifest_entry_count, manifest_header_bytes) {
            return Err(classification_error(
                "subscription-support capsule manifest exceeds portability manifest budget before materialization",
            ));
        }
        let required_basis_digests =
            affected_set.basis_digests_for_artifact_ids(basis_artifact_ids)?;
        if required_basis_digests.len() as u64 != footprint.required_basis_count() {
            return Err(classification_error(
                "subscription-support capsule manifest required-basis accounting drift",
            ));
        }
        let manifest_digest = stable_digest(&(
            affected_set.affected_set_digest(),
            footprint.footprint_digest(),
            manifest_entry_count,
            manifest_header_bytes,
            &required_basis_digests,
        ))?;
        Ok(Self {
            affected_set_digest: affected_set.affected_set_digest().clone(),
            footprint,
            manifest_entry_count,
            manifest_header_bytes,
            required_basis_digests,
            manifest_digest,
        })
    }

    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub fn footprint(&self) -> &SupportPortabilityScopeFootprint {
        &self.footprint
    }

    pub fn manifest_entry_count(&self) -> u64 {
        self.manifest_entry_count
    }

    pub fn manifest_header_bytes(&self) -> u64 {
        self.manifest_header_bytes
    }

    pub fn required_basis_count(&self) -> u64 {
        self.footprint.required_basis_count()
    }

    pub fn omitted_support_count(&self) -> u64 {
        self.footprint.omitted_support_count()
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportImportAdmissionWitness {
    manifest_digest: String,
    footprint_digest: String,
    target_admission_digest: String,
    source_identity_preservation_digest: Option<String>,
}

impl SupportImportAdmissionWitness {
    pub(crate) fn new(
        manifest: &CapsuleSupportManifest,
        target_admission_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            manifest_digest: manifest.manifest_digest().to_string(),
            footprint_digest: manifest.footprint().footprint_digest().to_string(),
            target_admission_digest: require_non_empty(
                "target import admission",
                target_admission_digest,
            )?,
            source_identity_preservation_digest: None,
        })
    }

    pub(crate) fn exact(
        manifest: &CapsuleSupportManifest,
        target_admission_digest: impl Into<String>,
        source_identity_preservation_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            manifest_digest: manifest.manifest_digest().to_string(),
            footprint_digest: manifest.footprint().footprint_digest().to_string(),
            target_admission_digest: require_non_empty(
                "target import admission",
                target_admission_digest,
            )?,
            source_identity_preservation_digest: Some(require_non_empty(
                "source identity preservation",
                source_identity_preservation_digest,
            )?),
        })
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn footprint_digest(&self) -> &str {
        &self.footprint_digest
    }

    pub fn target_admission_digest(&self) -> &str {
        &self.target_admission_digest
    }

    pub fn source_identity_preservation_digest(&self) -> Option<&str> {
        self.source_identity_preservation_digest.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ImportedSupportSemanticAccess {
    import_admission: SupportImportAdmissionWitness,
    imported_semantic_digest: String,
}

impl ImportedSupportSemanticAccess {
    pub(crate) fn from_import_admission(
        import_admission: SupportImportAdmissionWitness,
        imported_semantic_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        if import_admission
            .source_identity_preservation_digest()
            .is_none()
        {
            return Err(classification_error(
                "subscription-support semantic import access requires source identity-preservation evidence",
            ));
        }
        Ok(Self {
            import_admission,
            imported_semantic_digest: require_non_empty(
                "imported support semantic",
                imported_semantic_digest,
            )?,
        })
    }

    pub fn import_admission(&self) -> &SupportImportAdmissionWitness {
        &self.import_admission
    }

    pub fn imported_semantic_digest(&self) -> &str {
        &self.imported_semantic_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReplicatedSupportBundle {
    manifest_digest: String,
    source_identity_digest: String,
    target_identity_digest: String,
    preserved_artifact_ids: Vec<SubscriptionSupportArtifactId>,
    identity_preservation_digest: String,
}

impl ReplicatedSupportBundle {
    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn identity_preservation_digest(&self) -> &str {
        &self.identity_preservation_digest
    }

    pub fn preserved_count(&self) -> u64 {
        self.preserved_artifact_ids.len() as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PartialSupportOmissionReport {
    manifest_digest: String,
    omission_reason: String,
    omitted_artifact_ids: Vec<SubscriptionSupportArtifactId>,
}

impl PartialSupportOmissionReport {
    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn omission_reason(&self) -> &str {
        &self.omission_reason
    }

    pub fn omitted_count(&self) -> u64 {
        self.omitted_artifact_ids.len() as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ImportedSupportNotResumableReport {
    import_admission: SupportImportAdmissionWitness,
    denial_reason: String,
    missing_basis_artifact_ids: Vec<SubscriptionSupportArtifactId>,
}

impl ImportedSupportNotResumableReport {
    pub fn import_admission(&self) -> &SupportImportAdmissionWitness {
        &self.import_admission
    }

    pub fn denial_reason(&self) -> &str {
        &self.denial_reason
    }

    pub fn missing_basis_count(&self) -> u64 {
        self.missing_basis_artifact_ids.len() as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum SubscriptionSupportPortabilityOutcome {
    FullScopeReplicated(ReplicatedSupportBundle),
    PartialScopeOmitted(PartialSupportOmissionReport),
    Imported(ImportedSupportSemanticAccess),
    ImportedNotResumable(ImportedSupportNotResumableReport),
    Rejected(SupportPortabilityRejection),
}

impl SubscriptionSupportPortabilityOutcome {
    pub fn outcome_kind(&self) -> SubscriptionSupportPortabilityDecisionKind {
        match self {
            Self::FullScopeReplicated(_) => {
                SubscriptionSupportPortabilityDecisionKind::FullScopeReplication
            }
            Self::PartialScopeOmitted(_) => {
                SubscriptionSupportPortabilityDecisionKind::PartialScopeOmission
            }
            Self::Imported(_) => SubscriptionSupportPortabilityDecisionKind::TargetImportAdmitted,
            Self::ImportedNotResumable(_) => {
                SubscriptionSupportPortabilityDecisionKind::TargetImportMissingBasisNotResumable
            }
            Self::Rejected(rejection) => rejection.rejection_kind(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportPortabilityRejection {
    rejection_kind: SubscriptionSupportPortabilityDecisionKind,
    manifest_digest: String,
    rejection_reason: String,
}

impl SupportPortabilityRejection {
    pub fn rejection_kind(&self) -> SubscriptionSupportPortabilityDecisionKind {
        self.rejection_kind
    }

    pub fn rejection_reason(&self) -> &str {
        &self.rejection_reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportPortabilityDecision {
    evidence: SubscriptionSupportPortabilityDecisionEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[allow(dead_code)]
enum SubscriptionSupportPortabilityDecisionEvidence {
    FullScopeReplication {
        source_identity_digest: String,
        target_identity_digest: String,
    },
    PartialScopeOmission {
        omitted_artifact_ids: Vec<SubscriptionSupportArtifactId>,
        omission_reason: String,
    },
    TargetImportAdmitted {
        target_admission_digest: String,
        source_identity_preservation_digest: String,
        imported_semantic_digest: String,
    },
    TargetImportMissingBasisNotResumable {
        target_admission_digest: String,
        basis_artifact_ids: Vec<SubscriptionSupportArtifactId>,
        denial_reason: String,
    },
    UnsupportedFamilyRejected {
        rejection_reason: String,
    },
}

#[allow(dead_code)]
impl SubscriptionSupportPortabilityDecision {
    pub(crate) fn full_scope_replication(
        source_identity_digest: impl Into<String>,
        target_identity_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        let source_identity_digest = require_non_empty("source identity", source_identity_digest)?;
        let target_identity_digest = require_non_empty("target identity", target_identity_digest)?;
        if source_identity_digest != target_identity_digest {
            return Err(classification_error(
                "full-scope subscription-support replication requires preserved source/target identity digests",
            ));
        }
        Ok(
            SubscriptionSupportPortabilityDecisionEvidence::FullScopeReplication {
                source_identity_digest,
                target_identity_digest,
            }
            .into(),
        )
    }

    pub(crate) fn partial_scope_omission(
        omitted_artifact_ids: Vec<SubscriptionSupportArtifactId>,
        omission_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        if omitted_artifact_ids.is_empty() {
            return Err(classification_error(
                "partial subscription-support replication requires omitted artifact ids",
            ));
        }
        Ok(
            SubscriptionSupportPortabilityDecisionEvidence::PartialScopeOmission {
                omitted_artifact_ids,
                omission_reason: require_non_empty("omission reason", omission_reason)?,
            }
            .into(),
        )
    }

    pub(crate) fn target_import_admitted(
        target_admission_digest: impl Into<String>,
        source_identity_preservation_digest: impl Into<String>,
        imported_semantic_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportPortabilityDecisionEvidence::TargetImportAdmitted {
                target_admission_digest: require_non_empty(
                    "target admission",
                    target_admission_digest,
                )?,
                source_identity_preservation_digest: require_non_empty(
                    "source identity preservation",
                    source_identity_preservation_digest,
                )?,
                imported_semantic_digest: require_non_empty(
                    "imported semantic",
                    imported_semantic_digest,
                )?,
            }
            .into(),
        )
    }

    pub(crate) fn target_import_missing_basis_not_resumable(
        target_admission_digest: impl Into<String>,
        basis_artifact_ids: Vec<SubscriptionSupportArtifactId>,
        denial_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportPortabilityDecisionEvidence::TargetImportMissingBasisNotResumable {
                target_admission_digest: require_non_empty(
                    "target admission",
                    target_admission_digest,
                )?,
                basis_artifact_ids,
                denial_reason: require_non_empty("missing basis denial", denial_reason)?,
            }
            .into(),
        )
    }

    pub(crate) fn unsupported_family_rejected(
        rejection_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportPortabilityDecisionEvidence::UnsupportedFamilyRejected {
                rejection_reason: require_non_empty("portability rejection", rejection_reason)?,
            }
            .into(),
        )
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        match &self.evidence {
            SubscriptionSupportPortabilityDecisionEvidence::FullScopeReplication { .. }
            | SubscriptionSupportPortabilityDecisionEvidence::TargetImportAdmitted { .. } => {
                SubscriptionSupportOperationalVerdict::ExactResumePreserved
            }
            SubscriptionSupportPortabilityDecisionEvidence::PartialScopeOmission { .. } => {
                SubscriptionSupportOperationalVerdict::DegradedResumePreserved
            }
            SubscriptionSupportPortabilityDecisionEvidence::TargetImportMissingBasisNotResumable {
                ..
            } => SubscriptionSupportOperationalVerdict::NotResumable,
            SubscriptionSupportPortabilityDecisionEvidence::UnsupportedFamilyRejected {
                ..
            } => SubscriptionSupportOperationalVerdict::RejectedByPolicy,
        }
    }

    pub fn kind(&self) -> SubscriptionSupportPortabilityDecisionKind {
        match &self.evidence {
            SubscriptionSupportPortabilityDecisionEvidence::FullScopeReplication { .. } => {
                SubscriptionSupportPortabilityDecisionKind::FullScopeReplication
            }
            SubscriptionSupportPortabilityDecisionEvidence::PartialScopeOmission { .. } => {
                SubscriptionSupportPortabilityDecisionKind::PartialScopeOmission
            }
            SubscriptionSupportPortabilityDecisionEvidence::TargetImportAdmitted { .. } => {
                SubscriptionSupportPortabilityDecisionKind::TargetImportAdmitted
            }
            SubscriptionSupportPortabilityDecisionEvidence::TargetImportMissingBasisNotResumable {
                ..
            } => SubscriptionSupportPortabilityDecisionKind::TargetImportMissingBasisNotResumable,
            SubscriptionSupportPortabilityDecisionEvidence::UnsupportedFamilyRejected {
                ..
            } => SubscriptionSupportPortabilityDecisionKind::UnsupportedFamilyRejected,
        }
    }

    pub(crate) fn omitted_artifact_ids_for_scope(
        &self,
        affected_set: &SupportPortabilityAffectedSet,
    ) -> Vec<SubscriptionSupportArtifactId> {
        match &self.evidence {
            SubscriptionSupportPortabilityDecisionEvidence::PartialScopeOmission {
                omitted_artifact_ids,
                ..
            } => omitted_artifact_ids.clone(),
            SubscriptionSupportPortabilityDecisionEvidence::UnsupportedFamilyRejected {
                ..
            } => affected_set.all_artifacts_omitted(),
            SubscriptionSupportPortabilityDecisionEvidence::FullScopeReplication { .. }
            | SubscriptionSupportPortabilityDecisionEvidence::TargetImportAdmitted { .. }
            | SubscriptionSupportPortabilityDecisionEvidence::TargetImportMissingBasisNotResumable {
                ..
            } => {
                Vec::new()
            }
        }
    }

    pub(crate) fn basis_artifact_ids_for_scope(
        &self,
        affected_set: &SupportPortabilityAffectedSet,
    ) -> Vec<SubscriptionSupportArtifactId> {
        match &self.evidence {
            SubscriptionSupportPortabilityDecisionEvidence::UnsupportedFamilyRejected { .. } => {
                Vec::new()
            }
            SubscriptionSupportPortabilityDecisionEvidence::TargetImportMissingBasisNotResumable {
                basis_artifact_ids,
                ..
            } => basis_artifact_ids.clone(),
            SubscriptionSupportPortabilityDecisionEvidence::FullScopeReplication { .. }
            | SubscriptionSupportPortabilityDecisionEvidence::TargetImportAdmitted { .. } => {
                affected_set.affected_artifact_ids()
            }
            SubscriptionSupportPortabilityDecisionEvidence::PartialScopeOmission { .. } => {
                let omitted = self
                    .omitted_artifact_ids_for_scope(affected_set)
                    .into_iter()
                    .collect::<BTreeSet<_>>();
                affected_set
                    .affected_artifact_ids()
                    .into_iter()
                    .filter(|artifact_id| !omitted.contains(artifact_id))
                    .collect()
            }
        }
    }
}

impl From<SubscriptionSupportPortabilityDecisionEvidence>
    for SubscriptionSupportPortabilityDecision
{
    fn from(evidence: SubscriptionSupportPortabilityDecisionEvidence) -> Self {
        Self { evidence }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SubscriptionSupportPortabilityDecisionKind {
    FullScopeReplication,
    PartialScopeOmission,
    TargetImportAdmitted,
    TargetImportMissingBasisNotResumable,
    UnsupportedFamilyRejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportPortabilityBatchPlan {
    action_id: SupportActionId,
    affected_set: SupportPortabilityAffectedSet,
    path_plan: SupportProgramPathPlan,
    footprint: SupportPortabilityScopeFootprint,
    manifest: CapsuleSupportManifest,
    decision: SubscriptionSupportPortabilityDecision,
}

impl SupportPortabilityBatchPlan {
    pub(crate) fn new(
        action_id: SupportActionId,
        affected_set: SupportPortabilityAffectedSet,
        path_plan: SupportProgramPathPlan,
        footprint: SupportPortabilityScopeFootprint,
        manifest: CapsuleSupportManifest,
        decision: SubscriptionSupportPortabilityDecision,
    ) -> Result<Self, StoreError> {
        if path_plan.density_class() != SupportProgramDensityClass::PortabilityScopeBatch {
            return Err(classification_error(
                "subscription-support portability plans require portability-scope batch density",
            ));
        }
        if path_plan.batch_width() != affected_set.affected_count() {
            return Err(classification_error(
                "subscription-support portability plan width must match affected-set breadth",
            ));
        }
        validate_decision_origin_and_path(&decision, &affected_set, &path_plan)?;
        if manifest.affected_set_digest() != affected_set.affected_set_digest() {
            return Err(classification_error(
                "subscription-support capsule manifest must bind the admitted affected set",
            ));
        }
        match decision.kind() {
            SubscriptionSupportPortabilityDecisionKind::FullScopeReplication
            | SubscriptionSupportPortabilityDecisionKind::TargetImportAdmitted => {
                if footprint.omitted_support_count() != 0
                    || manifest.omitted_support_count() != 0
                    || manifest.manifest_entry_count() != affected_set.affected_count()
                    || manifest.required_basis_count() != affected_set.affected_count()
                {
                    return Err(classification_error(
                        "exact subscription-support portability requires full-scope manifest coverage",
                    ));
                }
            }
            SubscriptionSupportPortabilityDecisionKind::TargetImportMissingBasisNotResumable => {
                if footprint.omitted_support_count() != 0
                    || manifest.omitted_support_count() != 0
                    || manifest.manifest_entry_count() != affected_set.affected_count()
                {
                    return Err(classification_error(
                        "not-resumable support import still requires full-scope support manifest coverage",
                    ));
                }
                if manifest.required_basis_count() >= manifest.manifest_entry_count() {
                    return Err(classification_error(
                        "missing-basis not-resumable support import requires missing basis evidence",
                    ));
                }
            }
            SubscriptionSupportPortabilityDecisionKind::PartialScopeOmission => {
                let omitted_artifact_ids = decision.omitted_artifact_ids_for_scope(&affected_set);
                validate_omitted_artifact_ids(&affected_set, &omitted_artifact_ids)?;
                if footprint.omitted_support_count() == 0 || manifest.omitted_support_count() == 0 {
                    return Err(classification_error(
                        "partial subscription-support portability requires a non-empty omission footprint",
                    ));
                }
            }
            SubscriptionSupportPortabilityDecisionKind::UnsupportedFamilyRejected => {
                if footprint.included_support_count() != 0
                    || manifest.manifest_entry_count() != 0
                    || manifest.required_basis_count() != 0
                {
                    return Err(classification_error(
                        "unsupported subscription-support portability rejection cannot include support or basis evidence",
                    ));
                }
            }
        }
        Ok(Self {
            action_id,
            affected_set,
            path_plan,
            footprint,
            manifest,
            decision,
        })
    }

    pub fn affected_set(&self) -> &SupportPortabilityAffectedSet {
        &self.affected_set
    }

    pub fn path_plan(&self) -> &SupportProgramPathPlan {
        &self.path_plan
    }

    pub fn footprint(&self) -> &SupportPortabilityScopeFootprint {
        &self.footprint
    }

    pub fn manifest(&self) -> &CapsuleSupportManifest {
        &self.manifest
    }

    pub fn decision(&self) -> &SubscriptionSupportPortabilityDecision {
        &self.decision
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        SupportActionId,
        SupportPortabilityAffectedSet,
        SupportProgramPathPlan,
        SupportPortabilityScopeFootprint,
        CapsuleSupportManifest,
        SubscriptionSupportPortabilityDecision,
    ) {
        (
            self.action_id,
            self.affected_set,
            self.path_plan,
            self.footprint,
            self.manifest,
            self.decision,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportPortabilityParticipationRecord {
    action_id: SupportActionId,
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    affected_set_digest: SupportAffectedSetDigest,
    decision_kind: SubscriptionSupportPortabilityDecisionKind,
    verdict: SubscriptionSupportOperationalVerdict,
    action_origin: SubscriptionSupportActionOrigin,
    manifest_digest: String,
    manifest_entry_count: u64,
    omitted_support_count: u64,
    required_basis_count: u64,
}

impl SupportPortabilityParticipationRecord {
    fn new(
        completed_action: &CompletedSupportProgramAction,
        affected_set: &SupportPortabilityAffectedSet,
        manifest: &CapsuleSupportManifest,
        decision_kind: SubscriptionSupportPortabilityDecisionKind,
    ) -> Result<Self, StoreError> {
        if completed_action.envelope().action_origin() != affected_set.action_origin() {
            return Err(classification_error(
                "subscription-support portability participation record action origin drift",
            ));
        }
        Ok(Self {
            action_id: completed_action.envelope().action_id().clone(),
            family_id: affected_set.family_id().clone(),
            family_kind: affected_set.family_kind(),
            support_role: affected_set.support_role(),
            affected_set_digest: affected_set.affected_set_digest().clone(),
            decision_kind,
            verdict: completed_action.envelope().verdict(),
            action_origin: completed_action.envelope().action_origin(),
            manifest_digest: manifest.manifest_digest().to_string(),
            manifest_entry_count: manifest.manifest_entry_count(),
            omitted_support_count: manifest.omitted_support_count(),
            required_basis_count: manifest.required_basis_count(),
        })
    }

    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub fn decision_kind(&self) -> SubscriptionSupportPortabilityDecisionKind {
        self.decision_kind
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.verdict
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn manifest_entry_count(&self) -> u64 {
        self.manifest_entry_count
    }

    pub fn omitted_support_count(&self) -> u64 {
        self.omitted_support_count
    }

    pub fn required_basis_count(&self) -> u64 {
        self.required_basis_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportPortabilityReport {
    completed_action: CompletedSupportProgramAction,
    participation_record: SupportPortabilityParticipationRecord,
    manifest: CapsuleSupportManifest,
    outcome: SubscriptionSupportPortabilityOutcome,
    cost_surface: SubscriptionSupportResultCostSurface,
}

impl SubscriptionSupportPortabilityReport {
    pub(crate) fn new(
        completed_action: CompletedSupportProgramAction,
        affected_set: SupportPortabilityAffectedSet,
        manifest: CapsuleSupportManifest,
        decision: &SubscriptionSupportPortabilityDecision,
        path_plan: &SupportProgramPathPlan,
    ) -> Result<Self, StoreError> {
        let decision_kind = decision.kind();
        let participation_record = SupportPortabilityParticipationRecord::new(
            &completed_action,
            &affected_set,
            &manifest,
            decision_kind,
        )?;
        let outcome = outcome_from_decision(&affected_set, &manifest, decision)?;
        if outcome.outcome_kind() != decision_kind {
            return Err(classification_error(
                "subscription-support portability outcome kind must match decision kind",
            ));
        }
        Ok(Self {
            completed_action,
            participation_record,
            manifest,
            outcome,
            cost_surface: cost_surface_for_program_path(
                SubscriptionSupportPlanFamily::PortabilityParticipationPlan,
                path_plan,
            ),
        })
    }

    pub fn completed_action(&self) -> &CompletedSupportProgramAction {
        &self.completed_action
    }

    pub fn participation_record(&self) -> &SupportPortabilityParticipationRecord {
        &self.participation_record
    }

    pub fn manifest(&self) -> &CapsuleSupportManifest {
        &self.manifest
    }

    pub fn outcome(&self) -> &SubscriptionSupportPortabilityOutcome {
        &self.outcome
    }

    pub fn cost_surface(&self) -> SubscriptionSupportResultCostSurface {
        self.cost_surface
    }
}

fn outcome_from_decision(
    affected_set: &SupportPortabilityAffectedSet,
    manifest: &CapsuleSupportManifest,
    decision: &SubscriptionSupportPortabilityDecision,
) -> Result<SubscriptionSupportPortabilityOutcome, StoreError> {
    match &decision.evidence {
        SubscriptionSupportPortabilityDecisionEvidence::FullScopeReplication {
            source_identity_digest,
            target_identity_digest,
        } => Ok(SubscriptionSupportPortabilityOutcome::FullScopeReplicated(
            ReplicatedSupportBundle {
                manifest_digest: manifest.manifest_digest().to_string(),
                source_identity_digest: source_identity_digest.clone(),
                target_identity_digest: target_identity_digest.clone(),
                preserved_artifact_ids: affected_set.affected_artifact_ids(),
                identity_preservation_digest: stable_digest(&(
                    manifest.manifest_digest(),
                    source_identity_digest,
                    target_identity_digest,
                    affected_set.affected_set_digest(),
                ))?,
            },
        )),
        SubscriptionSupportPortabilityDecisionEvidence::PartialScopeOmission {
            omitted_artifact_ids,
            omission_reason,
        } => {
            if omitted_artifact_ids.len() as u64 != manifest.omitted_support_count() {
                return Err(classification_error(
                    "subscription-support partial omission report must match manifest omitted count",
                ));
            }
            Ok(SubscriptionSupportPortabilityOutcome::PartialScopeOmitted(
                PartialSupportOmissionReport {
                    manifest_digest: manifest.manifest_digest().to_string(),
                    omission_reason: omission_reason.clone(),
                    omitted_artifact_ids: omitted_artifact_ids.clone(),
                },
            ))
        }
        SubscriptionSupportPortabilityDecisionEvidence::TargetImportAdmitted {
            target_admission_digest,
            source_identity_preservation_digest,
            imported_semantic_digest,
        } => {
            let import_admission = SupportImportAdmissionWitness::exact(
                manifest,
                target_admission_digest.clone(),
                source_identity_preservation_digest.clone(),
            )?;
            let semantic_access = ImportedSupportSemanticAccess::from_import_admission(
                import_admission,
                imported_semantic_digest.clone(),
            )?;
            Ok(SubscriptionSupportPortabilityOutcome::Imported(
                semantic_access,
            ))
        }
        SubscriptionSupportPortabilityDecisionEvidence::TargetImportMissingBasisNotResumable {
            target_admission_digest,
            basis_artifact_ids,
            denial_reason,
        } => {
            let import_admission =
                SupportImportAdmissionWitness::new(manifest, target_admission_digest.clone())?;
            let admitted_basis = basis_artifact_ids.iter().collect::<BTreeSet<_>>();
            let missing_basis_artifact_ids = affected_set
                .affected_artifact_ids()
                .into_iter()
                .filter(|artifact_id| !admitted_basis.contains(artifact_id))
                .collect();
            Ok(SubscriptionSupportPortabilityOutcome::ImportedNotResumable(
                ImportedSupportNotResumableReport {
                    import_admission,
                    denial_reason: denial_reason.clone(),
                    missing_basis_artifact_ids,
                },
            ))
        }
        SubscriptionSupportPortabilityDecisionEvidence::UnsupportedFamilyRejected {
            rejection_reason,
        } => Ok(SubscriptionSupportPortabilityOutcome::Rejected(
            SupportPortabilityRejection {
                rejection_kind: decision.kind(),
                manifest_digest: manifest.manifest_digest().to_string(),
                rejection_reason: rejection_reason.clone(),
            },
        )),
    }
}

fn validate_decision_origin_and_path(
    decision: &SubscriptionSupportPortabilityDecision,
    affected_set: &SupportPortabilityAffectedSet,
    path_plan: &SupportProgramPathPlan,
) -> Result<(), StoreError> {
    match decision.kind() {
        SubscriptionSupportPortabilityDecisionKind::FullScopeReplication
        | SubscriptionSupportPortabilityDecisionKind::PartialScopeOmission => {
            if affected_set.action_origin() != SubscriptionSupportActionOrigin::ReplicationExport
                || path_plan.path_class() != super::SupportPathClass::ReplicationExport
            {
                return Err(classification_error(
                    "subscription-support replication decisions require export-origin bases and replication-export paths",
                ));
            }
        }
        SubscriptionSupportPortabilityDecisionKind::TargetImportAdmitted
        | SubscriptionSupportPortabilityDecisionKind::TargetImportMissingBasisNotResumable
        | SubscriptionSupportPortabilityDecisionKind::UnsupportedFamilyRejected => {
            if affected_set.action_origin() != SubscriptionSupportActionOrigin::ReplicationImport
                || path_plan.path_class() != super::SupportPathClass::ImportAdmission
            {
                return Err(classification_error(
                    "subscription-support import decisions require import-origin bases and import-admission paths",
                ));
            }
        }
    }
    Ok(())
}

fn validate_basis_artifact_ids(
    affected_set: &SupportPortabilityAffectedSet,
    basis_artifact_ids: &[SubscriptionSupportArtifactId],
    omitted_artifact_ids: &[SubscriptionSupportArtifactId],
) -> Result<(), StoreError> {
    let omitted = omitted_artifact_ids.iter().collect::<BTreeSet<_>>();
    let mut seen = BTreeSet::new();
    for artifact_id in basis_artifact_ids {
        if !seen.insert(artifact_id) {
            return Err(classification_error(
                "subscription-support basis evidence cannot repeat artifact ids",
            ));
        }
        if !affected_set.contains_artifact_id(artifact_id) {
            return Err(classification_error(
                "subscription-support basis evidence must name only artifacts in the admitted portability scope",
            ));
        }
        if omitted.contains(artifact_id) {
            return Err(classification_error(
                "subscription-support omitted artifacts cannot also claim basis evidence",
            ));
        }
    }
    Ok(())
}

fn validate_omitted_artifact_ids(
    affected_set: &SupportPortabilityAffectedSet,
    omitted_artifact_ids: &[SubscriptionSupportArtifactId],
) -> Result<(), StoreError> {
    let mut seen = BTreeSet::new();
    for artifact_id in omitted_artifact_ids {
        if !seen.insert(artifact_id) {
            return Err(classification_error(
                "subscription-support omission reports cannot repeat omitted artifact ids",
            ));
        }
        if !affected_set.contains_artifact_id(artifact_id) {
            return Err(classification_error(
                "subscription-support omission reports must name only artifacts in the admitted portability scope",
            ));
        }
    }
    Ok(())
}

fn require_non_empty(label: &'static str, value: impl Into<String>) -> Result<String, StoreError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(classification_error(format!(
            "subscription-support portability {label} evidence must be non-empty"
        )));
    }
    Ok(value)
}
