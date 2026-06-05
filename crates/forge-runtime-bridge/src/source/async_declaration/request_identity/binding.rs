use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{AsyncRequestBasisBindingIdentityTag, BridgeIdentity};

use super::super::{
    BridgeAsyncSignalLoweringFamilyKind, BridgeAsyncSourceDeclarationFamilyKind,
    BridgeAsyncSourceDeclarationIdentity, BridgeAsyncSourceLoweringIdentity,
    LoweredBridgeAsyncSourceDeclaration,
};
use super::truth_basis::{BridgeAsyncRequestTruthViewBasis, BridgeAsyncRequestTruthViewBasisKind};

pub type BridgeAsyncRequestBasisBindingIdentity =
    BridgeIdentity<AsyncRequestBasisBindingIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedBridgeAsyncRequestBasisBinding {
    binding_identity: BridgeAsyncRequestBasisBindingIdentity,
    declaration_identity: BridgeAsyncSourceDeclarationIdentity,
    lowering_identity: BridgeAsyncSourceLoweringIdentity,
    family_kind: BridgeAsyncSourceDeclarationFamilyKind,
    lowering_family_kind: BridgeAsyncSignalLoweringFamilyKind,
    truth_view_basis: BridgeAsyncRequestTruthViewBasis,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl ValidatedBridgeAsyncRequestBasisBinding {
    pub fn bind(
        lowered: &LoweredBridgeAsyncSourceDeclaration,
        truth_view_basis: BridgeAsyncRequestTruthViewBasis,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-async-request-basis-binding|declaration={}|lowering={}|family={:?}|lowering-family={:?}|truth-view-basis={}|truth-view-kind={:?}",
            lowered.declaration_identity().as_str(),
            lowered.lowering_identity().as_str(),
            lowered.family_kind(),
            lowered.lowering_family_kind(),
            truth_view_basis.digest(),
            truth_view_basis.kind(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            binding_identity: BridgeAsyncRequestBasisBindingIdentity::new(format!(
                "bridge-async-request-basis-binding-id:sha256:{digest:x}"
            )),
            declaration_identity: lowered.declaration_identity().clone(),
            lowering_identity: lowered.lowering_identity().clone(),
            family_kind: lowered.family_kind(),
            lowering_family_kind: lowered.lowering_family_kind(),
            truth_view_basis,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-async-request-basis-binding:sha256:{digest:x}"
            )),
        }
    }

    pub fn binding_identity(&self) -> &BridgeAsyncRequestBasisBindingIdentity {
        &self.binding_identity
    }

    pub fn declaration_identity(&self) -> &BridgeAsyncSourceDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn lowering_identity(&self) -> &BridgeAsyncSourceLoweringIdentity {
        &self.lowering_identity
    }

    pub fn family_kind(&self) -> BridgeAsyncSourceDeclarationFamilyKind {
        self.family_kind
    }

    pub fn lowering_family_kind(&self) -> BridgeAsyncSignalLoweringFamilyKind {
        self.lowering_family_kind
    }

    pub fn truth_view_basis(&self) -> &BridgeAsyncRequestTruthViewBasis {
        &self.truth_view_basis
    }

    pub fn truth_view_basis_kind(&self) -> BridgeAsyncRequestTruthViewBasisKind {
        self.truth_view_basis.kind()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
