use crate::catalog::PhysicalArtifactFamily;
use crate::facade::layout_declarations;
use crate::materialization::LayoutCoverageWitness;
use crate::strategy::StrategyRebuildSourceRequirement;
use crate::PhysicalKeyDomainWitness;
use forge_store_contracts::WalRecordFamily;
use forge_store_physical_format::PhysicalRootManifestRebuildWitness;
use forge_store_wal::{BlobWalReplayRebuildWitness, StoreWalRecordIdentity};

use super::basis::DerivedIndexParityBasis;
use super::identity::{DerivedIndexCostEnvelopeParity, DerivedIndexIdentityParity};
use super::outcome::DerivedIndexRebuildDenied;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DerivedIndexRebuildSourceInput {
    PhysicalRootManifest {
        source_witness: PhysicalRootManifestRebuildWitness,
    },
    WalReplayRecord {
        source_witness: BlobWalReplayRebuildWitness,
    },
    DerivedProjectionRows,
    CertificationRows,
    DiagnosticReport,
    JsonProjection,
    TerminalProjection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DerivedIndexAuthoritySource {
    PhysicalSnapshotReplay {
        family: PhysicalArtifactFamily,
        coverage: LayoutCoverageWitness,
        source_witness: PhysicalRootManifestRebuildWitness,
        parity_basis: DerivedIndexParityBasis,
    },
    WalReplay {
        family: PhysicalArtifactFamily,
        coverage: LayoutCoverageWitness,
        source_witness: BlobWalReplayRebuildWitness,
        parity_basis: DerivedIndexParityBasis,
    },
}

impl DerivedIndexAuthoritySource {
    pub(crate) fn declare(
        requirement: StrategyRebuildSourceRequirement,
        family: PhysicalArtifactFamily,
        coverage: LayoutCoverageWitness,
        key_domain: PhysicalKeyDomainWitness,
        source_input: &DerivedIndexRebuildSourceInput,
    ) -> Result<Option<Self>, DerivedIndexRebuildDenied> {
        match (requirement, source_input) {
            (
                StrategyRebuildSourceRequirement::PhysicalSnapshotReplay,
                DerivedIndexRebuildSourceInput::PhysicalRootManifest { source_witness },
            ) => Ok(Some(Self::PhysicalSnapshotReplay {
                family,
                coverage: coverage.clone(),
                parity_basis: physical_root_manifest_parity_basis(
                    source_witness,
                    key_domain,
                    coverage,
                )?,
                source_witness: source_witness.clone(),
            })),
            (
                StrategyRebuildSourceRequirement::WalReplay,
                DerivedIndexRebuildSourceInput::WalReplayRecord { source_witness },
            ) => Ok(Some(Self::WalReplay {
                family,
                coverage: coverage.clone(),
                parity_basis: wal_replay_parity_basis(source_witness, key_domain, coverage)?,
                source_witness: source_witness.clone(),
            })),
            _ => Ok(None),
        }
    }

    pub const fn family(&self) -> PhysicalArtifactFamily {
        match self {
            Self::PhysicalSnapshotReplay { family, .. } | Self::WalReplay { family, .. } => *family,
        }
    }

    pub(crate) const fn parity_basis(&self) -> &DerivedIndexParityBasis {
        match self {
            Self::PhysicalSnapshotReplay { parity_basis, .. }
            | Self::WalReplay { parity_basis, .. } => parity_basis,
        }
    }

    pub(crate) const fn value_identity_parity(&self) -> DerivedIndexIdentityParity {
        match self {
            Self::PhysicalSnapshotReplay { .. } | Self::WalReplay { .. } => {
                DerivedIndexIdentityParity::SourceArtifactDoesNotProveIdentity
            }
        }
    }

    pub(crate) const fn cost_envelope_parity(&self) -> DerivedIndexCostEnvelopeParity {
        match self {
            Self::PhysicalSnapshotReplay { .. } | Self::WalReplay { .. } => {
                DerivedIndexCostEnvelopeParity::SourceArtifactDoesNotProveDeclaredEnvelope
            }
        }
    }

    pub fn authority_row_count(&self) -> usize {
        match self {
            Self::PhysicalSnapshotReplay { source_witness, .. } => source_witness.rows().len(),
            Self::WalReplay { .. } => 1,
        }
    }
}

fn physical_root_manifest_parity_basis(
    source_witness: &PhysicalRootManifestRebuildWitness,
    key_domain: PhysicalKeyDomainWitness,
    coverage: LayoutCoverageWitness,
) -> Result<DerivedIndexParityBasis, DerivedIndexRebuildDenied> {
    let encoding = layout_declarations().require_canonical_key_encoding(key_domain);
    let comparator = layout_declarations().declare_comparator_law(encoding);
    let rows = source_witness
        .rows()
        .iter()
        .map(|row| {
            let key = layout_declarations()
                .admit_page_address_key(key_domain, row.segment_id(), row.page_id())
                .expect("root-manifest rebuild witnesses should use a compatible page key domain");
            let key = layout_declarations()
                .canonical_key_bytes(comparator, key)
                .expect("admitted page address key should encode canonically");

            super::basis::DerivedIndexParityRow::new(key, "")
        })
        .collect::<Vec<_>>();

    DerivedIndexParityBasis::new(
        rows,
        coverage,
        false,
        source_witness.counter_shape().to_vec(),
    )
}

fn wal_replay_parity_basis(
    source_witness: &BlobWalReplayRebuildWitness,
    key_domain: PhysicalKeyDomainWitness,
    coverage: LayoutCoverageWitness,
) -> Result<DerivedIndexParityBasis, DerivedIndexRebuildDenied> {
    let record = source_witness.record();
    let encoding = layout_declarations().require_canonical_key_encoding(key_domain);
    let comparator = layout_declarations().declare_comparator_law(encoding);
    let key = layout_declarations()
        .admit_wal_record_key(
            key_domain,
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(record.identity().sequence()),
        )
        .expect("wal rebuild witnesses should use a compatible wal key domain");
    let key = layout_declarations()
        .canonical_key_bytes(comparator, key)
        .expect("admitted wal record key should encode canonically");

    DerivedIndexParityBasis::new(
        vec![super::basis::DerivedIndexParityRow::new(key, "")],
        coverage,
        false,
        source_witness.counter_shape().to_vec(),
    )
}
