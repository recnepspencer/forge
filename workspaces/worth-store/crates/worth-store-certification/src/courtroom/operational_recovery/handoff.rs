use worth_store_operations::{OperationalComplexityContract, OperationalSessionKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S11UnimplementedSecurityStrengthening {
    ProviderAuthentication,
    ProofOfPossession,
    Encryption,
    TamperEvidence,
    SecureDeletion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S11StructuredAuditHardeningHandoff {
    closeout_identity: [u8; 32],
    structured_audit_schema: &'static str,
    scenario_evidence_identities: [[u8; 32]; 6],
    unimplemented_strengthening: [S11UnimplementedSecurityStrengthening; 5],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S12UnqualifiedDimension {
    HardwareLatency,
    HardwareThroughput,
    OperatingSystemPageCache,
    AllocatorArenaBehavior,
    BackendSpecificDurability,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S12PhysicalQualificationHandoff {
    closeout_identity: [u8; 32],
    scenario_evidence_identities: [[u8; 32]; 6],
    complexity_contracts: Vec<OperationalComplexityContract>,
    unqualified_dimensions: [S12UnqualifiedDimension; 5],
}

impl S11StructuredAuditHardeningHandoff {
    pub(super) const fn from_closeout(
        closeout_identity: [u8; 32],
        scenario_evidence_identities: [[u8; 32]; 6],
    ) -> Self {
        Self {
            closeout_identity,
            structured_audit_schema: "worth-store-operational-audit-v1",
            scenario_evidence_identities,
            unimplemented_strengthening: [
                S11UnimplementedSecurityStrengthening::ProviderAuthentication,
                S11UnimplementedSecurityStrengthening::ProofOfPossession,
                S11UnimplementedSecurityStrengthening::Encryption,
                S11UnimplementedSecurityStrengthening::TamperEvidence,
                S11UnimplementedSecurityStrengthening::SecureDeletion,
            ],
        }
    }

    pub const fn closeout_identity(&self) -> [u8; 32] {
        self.closeout_identity
    }
    pub const fn structured_audit_schema(&self) -> &'static str {
        self.structured_audit_schema
    }
    pub const fn scenario_evidence_identities(&self) -> &[[u8; 32]; 6] {
        &self.scenario_evidence_identities
    }
    pub const fn unimplemented_strengthening(&self) -> &[S11UnimplementedSecurityStrengthening; 5] {
        &self.unimplemented_strengthening
    }
}

impl S12PhysicalQualificationHandoff {
    pub(super) fn from_closeout(
        closeout_identity: [u8; 32],
        scenario_evidence_identities: [[u8; 32]; 6],
    ) -> Self {
        let kinds = [
            OperationalSessionKind::Backup,
            OperationalSessionKind::Restore,
            OperationalSessionKind::PointInTimeRecovery,
            OperationalSessionKind::Rollback,
            OperationalSessionKind::Repair,
            OperationalSessionKind::ReplicaBootstrap,
            OperationalSessionKind::ReplicaPromotion,
            OperationalSessionKind::ForensicAcquisition,
            OperationalSessionKind::OfflineVerification,
        ];
        Self {
            closeout_identity,
            scenario_evidence_identities,
            complexity_contracts: kinds
                .into_iter()
                .map(OperationalComplexityContract::for_kind)
                .collect(),
            unqualified_dimensions: [
                S12UnqualifiedDimension::HardwareLatency,
                S12UnqualifiedDimension::HardwareThroughput,
                S12UnqualifiedDimension::OperatingSystemPageCache,
                S12UnqualifiedDimension::AllocatorArenaBehavior,
                S12UnqualifiedDimension::BackendSpecificDurability,
            ],
        }
    }

    pub const fn closeout_identity(&self) -> [u8; 32] {
        self.closeout_identity
    }
    pub const fn scenario_evidence_identities(&self) -> &[[u8; 32]; 6] {
        &self.scenario_evidence_identities
    }
    pub fn complexity_contracts(&self) -> &[OperationalComplexityContract] {
        &self.complexity_contracts
    }
    pub const fn unqualified_dimensions(&self) -> &[S12UnqualifiedDimension; 5] {
        &self.unqualified_dimensions
    }
}
