use std::path::PathBuf;

use worth_store_replication::{
    DisasterRecoveryBundleDenial, DisasterRecoveryManifestFormat,
    MaterializedDisasterRecoveryBundle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisasterRecoveryIndependentOpenDenial {
    Manifest(DisasterRecoveryBundleDenial),
}

#[derive(Debug)]
pub struct IndependentlyOpenedDisasterRecoveryBundle {
    pub(super) materialized: MaterializedDisasterRecoveryBundle,
}

pub fn open_disaster_recovery_bundle(
    root: impl Into<PathBuf>,
    maximum_manifest_bytes: usize,
) -> Result<IndependentlyOpenedDisasterRecoveryBundle, DisasterRecoveryIndependentOpenDenial> {
    let materialized =
        DisasterRecoveryManifestFormat::open_materialized(root, maximum_manifest_bytes)
            .map_err(DisasterRecoveryIndependentOpenDenial::Manifest)?;
    Ok(IndependentlyOpenedDisasterRecoveryBundle { materialized })
}
