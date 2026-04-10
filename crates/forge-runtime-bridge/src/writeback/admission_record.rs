use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, WritebackAdmissionRecordIdentityTag};

use super::AdmittedBridgeWritebackContract;

pub type BridgeWritebackFamilyAdmissionRecordIdentity =
    BridgeIdentity<WritebackAdmissionRecordIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackFamilyAdmissionRecord {
    record_identity: BridgeWritebackFamilyAdmissionRecordIdentity,
    declaration_identity: Arc<str>,
    contract_digest: Arc<str>,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    effect_class: crate::writeback::BridgeWritebackEffectClass,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    strategy_descriptor_digest: Arc<str>,
    family_basis_digest: Arc<str>,
    strategy_basis_digest: Arc<str>,
    lowered_policy_digest: Arc<str>,
    diagnostics_tier: crate::policy::BridgeDiagnosticsTier,
    replay_artifacts_permitted: bool,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackFamilyAdmissionRecord {
    pub(crate) fn new(contract: &AdmittedBridgeWritebackContract) -> Self {
        let validated = contract.validated_declaration();
        let declaration = validated.declaration();
        let family_basis = validated
            .family_basis()
            .expect("writeback family admission record requires family basis");
        let strategy_basis = validated
            .strategy_basis()
            .expect("writeback family admission record requires strategy basis");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-family-admission-record|declaration={}|contract={}|family:{:?}|effect-class:{:?}|strategy-class:{:?}|strategy={}|family-basis={}|strategy-basis={}|lowered-policy={}|diagnostics:{:?}|replay:{}",
            declaration.declaration_identity().as_str(),
            contract.digest(),
            declaration.family_kind().expect("family admission record requires family kind"),
            declaration.effect_class(),
            declaration
                .strategy_class()
                .expect("family admission record requires strategy class"),
            declaration.strategy_descriptor_digest(),
            family_basis.digest(),
            strategy_basis.digest(),
            contract.lowered_policy_digest(),
            contract.authority_inputs().diagnostics_tier(),
            contract.authority_inputs().replay_artifacts_permitted(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            record_identity: BridgeWritebackFamilyAdmissionRecordIdentity::new(format!(
                "bridge-writeback-family-admission-record:sha256:{digest:x}"
            )),
            declaration_identity: Arc::from(
                declaration.declaration_identity().as_str().to_owned(),
            ),
            contract_digest: Arc::from(contract.digest().to_owned()),
            family_kind: declaration
                .family_kind()
                .expect("family admission record requires family kind"),
            effect_class: declaration.effect_class(),
            strategy_class: declaration
                .strategy_class()
                .expect("family admission record requires strategy class"),
            strategy_descriptor_digest: Arc::from(
                declaration.strategy_descriptor_digest().to_owned(),
            ),
            family_basis_digest: Arc::from(family_basis.digest().to_owned()),
            strategy_basis_digest: Arc::from(strategy_basis.digest().to_owned()),
            lowered_policy_digest: Arc::from(contract.lowered_policy_digest().to_owned()),
            diagnostics_tier: contract.authority_inputs().diagnostics_tier(),
            replay_artifacts_permitted: contract.authority_inputs().replay_artifacts_permitted(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-writeback-family-admission-record:sha256:{digest:x}"
            )),
        }
    }

    pub fn record_identity(&self) -> &BridgeWritebackFamilyAdmissionRecordIdentity {
        &self.record_identity
    }

    pub fn declaration_identity(&self) -> &str {
        self.declaration_identity.as_ref()
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_ref()
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn effect_class(&self) -> crate::writeback::BridgeWritebackEffectClass {
        self.effect_class
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.strategy_descriptor_digest.as_ref()
    }

    pub fn family_basis_digest(&self) -> &str {
        self.family_basis_digest.as_ref()
    }

    pub fn strategy_basis_digest(&self) -> &str {
        self.strategy_basis_digest.as_ref()
    }

    pub fn lowered_policy_digest(&self) -> &str {
        self.lowered_policy_digest.as_ref()
    }

    pub fn diagnostics_tier(&self) -> crate::policy::BridgeDiagnosticsTier {
        self.diagnostics_tier
    }

    pub fn replay_artifacts_permitted(&self) -> bool {
        self.replay_artifacts_permitted
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
