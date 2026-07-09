use std::sync::Arc;

use worth_foundational::facade::{
    aspects, AspectContract, AspectFieldLocator, AspectLocator, AspectMask, AspectShape,
    LocatorAuthority, ProjectionMask,
};
use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, SnapshotReadTargetIdentityTag};
use crate::snapshot::SnapshotReadContract;

mod native_basis;

pub type SnapshotReadTargetIdentity = BridgeIdentity<SnapshotReadTargetIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotReadTarget {
    target_identity: SnapshotReadTargetIdentity,
    contract: SnapshotReadContract,
    projection_contract: AspectContract,
    aspect_locator: AspectLocator,
    field_locator: Option<AspectFieldLocator>,
    projection_mask: AspectMask<ProjectionMask>,
    native_target_basis: Arc<str>,
}

impl SnapshotReadTarget {
    pub(crate) fn whole_aspect(contract: SnapshotReadContract) -> Self {
        let aspect_locator =
            AspectLocator::new(LocatorAuthority::Planned, contract.aspect_key().clone());
        let projection_mask = AspectMask::whole_aspect();
        let native_target_basis = native_basis::snapshot_read_target_canonical_basis(
            &aspect_locator,
            None,
            &projection_mask,
        );
        let target_identity =
            snapshot_read_target_identity(&contract, native_target_basis.as_str());
        Self {
            target_identity,
            projection_contract: contract.aspect_contract().clone(),
            contract,
            aspect_locator,
            field_locator: None,
            projection_mask,
            native_target_basis: Arc::from(native_target_basis),
        }
    }

    pub(crate) fn native_subscription_slice(
        contract: SnapshotReadContract,
        aspect_locator: AspectLocator,
        field_locator: Option<AspectFieldLocator>,
        projection_mask: AspectMask<ProjectionMask>,
    ) -> Self {
        let projection_contract = projection_contract_for_native_target(&contract, &field_locator);
        let native_target_basis = native_basis::snapshot_read_target_canonical_basis(
            &aspect_locator,
            field_locator.as_ref(),
            &projection_mask,
        );
        let target_identity =
            snapshot_read_target_identity(&contract, native_target_basis.as_str());
        Self {
            target_identity,
            projection_contract,
            contract,
            aspect_locator,
            field_locator,
            projection_mask,
            native_target_basis: Arc::from(native_target_basis),
        }
    }

    pub fn contract(&self) -> &SnapshotReadContract {
        &self.contract
    }

    pub(crate) fn target_identity(&self) -> &SnapshotReadTargetIdentity {
        &self.target_identity
    }

    pub(crate) fn projection_contract(&self) -> &AspectContract {
        &self.projection_contract
    }

    pub fn aspect_locator(&self) -> &AspectLocator {
        &self.aspect_locator
    }

    pub fn aspect_key(&self) -> &worth_foundational::facade::AspectKey {
        self.aspect_locator.aspect_key()
    }

    pub fn field_locator(&self) -> Option<&AspectFieldLocator> {
        self.field_locator.as_ref()
    }

    pub fn projection_mask(&self) -> &AspectMask<ProjectionMask> {
        &self.projection_mask
    }

    pub(crate) fn native_target_basis(&self) -> &str {
        self.native_target_basis.as_ref()
    }
}

fn snapshot_read_target_identity(
    contract: &SnapshotReadContract,
    native_target_basis: &str,
) -> SnapshotReadTargetIdentity {
    let basis = format!(
        "snapshot-read-target|contract={}|native-target={native_target_basis}",
        contract.canonical_basis()
    );
    let digest = Sha256::digest(basis.as_bytes());
    SnapshotReadTargetIdentity::admit_bridge_owned(format!(
        "snapshot-read-target:sha256:{digest:x}"
    ))
}

fn projection_contract_for_native_target(
    read_contract: &SnapshotReadContract,
    field_locator: &Option<AspectFieldLocator>,
) -> AspectContract {
    let Some(field_locator) = field_locator else {
        return read_contract.aspect_contract().clone();
    };
    let [field_key] = field_locator.field_path().fields() else {
        return read_contract.aspect_contract().clone();
    };
    let AspectShape::Scalar(scalar_type) = read_contract.aspect_contract().shape() else {
        return read_contract.aspect_contract().clone();
    };

    let shape = aspects()
        .struct_fields()
        .required(field_key.as_str(), *scalar_type)
        .finish()
        .expect("field locator path already carries a valid foundational field key");
    AspectContract::struct_aspect(
        read_contract.aspect_key().clone(),
        read_contract.aspect_contract().identity(),
        read_contract.aspect_contract().revision(),
        shape,
    )
}
