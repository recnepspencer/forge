use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{SpatialGeometryEvidenceTouchAuthority, SpatialGeometryEvidenceTouchDigest};
use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceStageKind, CompleteWorkloadEvidenceLedger, WorkloadEvidenceStage,
    WorkloadEvidenceStageCounters, WorkloadEvidenceStageLookupCounters, WorkloadEvidenceSupport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialEvidenceLookupKey {
    key: String,
    boolean_stage: BooleanEvidenceStageKind,
    evidence_stage: WorkloadEvidenceStage,
    evidence_identity: String,
    stage_index_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialEvidenceLookupProductDigest {
    digest: String,
    spatial_touch_digest: SpatialGeometryEvidenceTouchDigest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialEvidenceLookupProduct {
    lookup_key: SpatialEvidenceLookupKey,
    product_digest: SpatialEvidenceLookupProductDigest,
    boolean_stage: BooleanEvidenceStageKind,
    evidence_stage: WorkloadEvidenceStage,
    evidence_identity: String,
    support: WorkloadEvidenceSupport,
    counters: WorkloadEvidenceStageCounters,
    lookup_counters: WorkloadEvidenceStageLookupCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialEvidenceLookupExpectation {
    boolean_stage: BooleanEvidenceStageKind,
    evidence_identity: String,
    stage_index_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialEvidenceLookupDenial {
    kind: SpatialEvidenceLookupDenialKind,
    detail: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialEvidenceLookupDenialKind {
    WrongBooleanStage,
    WrongEvidenceIdentity,
    WrongStageIndexIdentity,
    UnsupportedSupportPosture,
    QueryDescriptorDigestSubstitution,
    MissingEvidenceStage,
}

impl SpatialEvidenceLookupExpectation {
    pub fn from_authority(authority: &SpatialGeometryEvidenceTouchAuthority) -> Self {
        Self {
            boolean_stage: authority.boolean_stage(),
            evidence_identity: authority.evidence_identity().to_string(),
            stage_index_identity: authority.stage_index_identity().to_string(),
        }
    }

    pub fn with_boolean_stage(mut self, boolean_stage: BooleanEvidenceStageKind) -> Self {
        self.boolean_stage = boolean_stage;
        self
    }

    pub fn with_evidence_identity(mut self, evidence_identity: impl Into<String>) -> Self {
        self.evidence_identity = evidence_identity.into();
        self
    }

    pub fn with_stage_index_identity(mut self, stage_index_identity: impl Into<String>) -> Self {
        self.stage_index_identity = stage_index_identity.into();
        self
    }
}

impl SpatialGeometryEvidenceTouchAuthority {
    pub fn spatial_evidence_lookup(
        &self,
        ledger: &CompleteWorkloadEvidenceLedger,
    ) -> Result<SpatialEvidenceLookupProduct, SpatialEvidenceLookupDenial> {
        self.spatial_evidence_lookup_matching(
            ledger,
            SpatialEvidenceLookupExpectation::from_authority(self),
        )
    }

    pub fn spatial_evidence_lookup_matching(
        &self,
        ledger: &CompleteWorkloadEvidenceLedger,
        expectation: SpatialEvidenceLookupExpectation,
    ) -> Result<SpatialEvidenceLookupProduct, SpatialEvidenceLookupDenial> {
        require_expected_authority_identity(self, &expectation)?;
        require_ledger_stage_index_identity(self, ledger)?;
        require_ledger_evidence_identity(self, ledger)?;
        SpatialEvidenceLookupProduct::from_authority(self)
    }
}

impl SpatialEvidenceLookupProduct {
    fn from_authority(
        authority: &SpatialGeometryEvidenceTouchAuthority,
    ) -> Result<Self, SpatialEvidenceLookupDenial> {
        require_admitted_support(authority)?;
        let lookup_key = SpatialEvidenceLookupKey::from_authority(authority);
        let product_digest =
            SpatialEvidenceLookupProductDigest::from_authority_and_key(authority, &lookup_key);
        Ok(Self {
            lookup_key,
            product_digest,
            boolean_stage: authority.boolean_stage(),
            evidence_stage: authority.evidence_stage(),
            evidence_identity: authority.evidence_identity().to_string(),
            support: authority.support(),
            counters: authority.evidence_counters(),
            lookup_counters: authority.lookup_counters(),
        })
    }

    pub fn lookup_key(&self) -> &SpatialEvidenceLookupKey {
        &self.lookup_key
    }

    pub fn product_digest(&self) -> &SpatialEvidenceLookupProductDigest {
        &self.product_digest
    }

    pub fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        self.boolean_stage
    }

    pub fn evidence_stage(&self) -> WorkloadEvidenceStage {
        self.evidence_stage
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn support(&self) -> WorkloadEvidenceSupport {
        self.support
    }

    pub fn counters(&self) -> WorkloadEvidenceStageCounters {
        self.counters
    }

    pub fn lookup_counters(&self) -> WorkloadEvidenceStageLookupCounters {
        self.lookup_counters
    }
}

impl SpatialEvidenceLookupKey {
    fn from_authority(authority: &SpatialGeometryEvidenceTouchAuthority) -> Self {
        let key = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "spatial-evidence-lookup-key".to_string(),
                format!("boolean-stage:{:?}", authority.boolean_stage()),
                format!("evidence-stage:{}", authority.evidence_stage().human_name()),
                format!("evidence-identity:{}", authority.evidence_identity()),
                format!("stage-index-identity:{}", authority.stage_index_identity()),
            ],
        );
        Self {
            key,
            boolean_stage: authority.boolean_stage(),
            evidence_stage: authority.evidence_stage(),
            evidence_identity: authority.evidence_identity().to_string(),
            stage_index_identity: authority.stage_index_identity().to_string(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.key
    }

    pub fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        self.boolean_stage
    }

    pub fn evidence_stage(&self) -> WorkloadEvidenceStage {
        self.evidence_stage
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn stage_index_identity(&self) -> &str {
        &self.stage_index_identity
    }
}

impl SpatialEvidenceLookupProductDigest {
    fn from_authority_and_key(
        authority: &SpatialGeometryEvidenceTouchAuthority,
        lookup_key: &SpatialEvidenceLookupKey,
    ) -> Self {
        let digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "spatial-evidence-lookup-product".to_string(),
                format!("lookup-key:{}", lookup_key.as_str()),
                format!("spatial-touch-digest:{}", authority.digest().as_str()),
                format!("support:{:?}", authority.support()),
                format!("counters:{:?}", authority.evidence_counters()),
                format!("lookup-counters:{:?}", authority.lookup_counters()),
                format!(
                    "stage-link-set-identity:{}",
                    authority.stage_link_set_identity()
                ),
            ],
        );
        Self {
            digest,
            spatial_touch_digest: authority.digest().clone(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.digest
    }

    pub fn spatial_touch_digest(&self) -> &SpatialGeometryEvidenceTouchDigest {
        &self.spatial_touch_digest
    }
}

impl SpatialEvidenceLookupDenial {
    fn wrong_boolean_stage(
        expected: BooleanEvidenceStageKind,
        actual: BooleanEvidenceStageKind,
    ) -> Self {
        Self::new(
            SpatialEvidenceLookupDenialKind::WrongBooleanStage,
            format!("expected {expected:?} lookup authority, got {actual:?}"),
        )
    }

    fn wrong_evidence_identity(expected: &str, actual: &str) -> Self {
        Self::new(
            SpatialEvidenceLookupDenialKind::WrongEvidenceIdentity,
            format!("expected evidence identity {expected}, got {actual}"),
        )
    }

    fn wrong_stage_index_identity(expected: &str, actual: &str) -> Self {
        Self::new(
            SpatialEvidenceLookupDenialKind::WrongStageIndexIdentity,
            format!("expected stage-index identity {expected}, got {actual}"),
        )
    }

    fn unsupported_support_posture(
        stage: WorkloadEvidenceStage,
        support: WorkloadEvidenceSupport,
    ) -> Self {
        Self::new(
            SpatialEvidenceLookupDenialKind::UnsupportedSupportPosture,
            format!("{} has support {support:?}", stage.human_name()),
        )
    }

    fn missing_evidence_stage(stage: WorkloadEvidenceStage) -> Self {
        Self::new(
            SpatialEvidenceLookupDenialKind::MissingEvidenceStage,
            format!(
                "{} is absent from the complete ledger stage index",
                stage.human_name()
            ),
        )
    }

    pub fn query_descriptor_digest_substitution(digest: &str) -> Self {
        Self::new(
            SpatialEvidenceLookupDenialKind::QueryDescriptorDigestSubstitution,
            format!(
                "Query descriptor digest {digest} cannot construct spatial evidence lookup authority"
            ),
        )
    }

    fn new(kind: SpatialEvidenceLookupDenialKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> SpatialEvidenceLookupDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

pub fn deny_query_descriptor_digest_as_spatial_evidence_lookup_authority(
    digest: &str,
) -> SpatialEvidenceLookupDenial {
    SpatialEvidenceLookupDenial::query_descriptor_digest_substitution(digest)
}

fn require_expected_authority_identity(
    authority: &SpatialGeometryEvidenceTouchAuthority,
    expectation: &SpatialEvidenceLookupExpectation,
) -> Result<(), SpatialEvidenceLookupDenial> {
    if authority.boolean_stage() != expectation.boolean_stage {
        return Err(SpatialEvidenceLookupDenial::wrong_boolean_stage(
            expectation.boolean_stage,
            authority.boolean_stage(),
        ));
    }
    if authority.evidence_identity() != expectation.evidence_identity {
        return Err(SpatialEvidenceLookupDenial::wrong_evidence_identity(
            &expectation.evidence_identity,
            authority.evidence_identity(),
        ));
    }
    if authority.stage_index_identity() != expectation.stage_index_identity {
        return Err(SpatialEvidenceLookupDenial::wrong_stage_index_identity(
            &expectation.stage_index_identity,
            authority.stage_index_identity(),
        ));
    }
    Ok(())
}

fn require_ledger_stage_index_identity(
    authority: &SpatialGeometryEvidenceTouchAuthority,
    ledger: &CompleteWorkloadEvidenceLedger,
) -> Result<(), SpatialEvidenceLookupDenial> {
    let ledger_identity = ledger.stage_index().index_identity();
    if authority.stage_index_identity() != ledger_identity {
        return Err(SpatialEvidenceLookupDenial::wrong_stage_index_identity(
            authority.stage_index_identity(),
            ledger_identity,
        ));
    }
    Ok(())
}

fn require_ledger_evidence_identity(
    authority: &SpatialGeometryEvidenceTouchAuthority,
    ledger: &CompleteWorkloadEvidenceLedger,
) -> Result<(), SpatialEvidenceLookupDenial> {
    let row = ledger
        .row_for_stage(authority.evidence_stage())
        .ok_or_else(|| {
            SpatialEvidenceLookupDenial::missing_evidence_stage(authority.evidence_stage())
        })?;
    if row.evidence_identity() != authority.evidence_identity() {
        return Err(SpatialEvidenceLookupDenial::wrong_evidence_identity(
            authority.evidence_identity(),
            row.evidence_identity(),
        ));
    }
    Ok(())
}

fn require_admitted_support(
    authority: &SpatialGeometryEvidenceTouchAuthority,
) -> Result<(), SpatialEvidenceLookupDenial> {
    if authority.support() != WorkloadEvidenceSupport::Admitted {
        return Err(SpatialEvidenceLookupDenial::unsupported_support_posture(
            authority.evidence_stage(),
            authority.support(),
        ));
    }
    Ok(())
}
