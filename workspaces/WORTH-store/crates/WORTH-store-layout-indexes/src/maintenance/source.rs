use crate::artifact_family::PhysicalArtifactFamily;
use crate::facade::layout_declarations;
use crate::materialization::S8LayoutCoverageWitness;
use crate::strategy::S8StrategyRebuildSourceRequirement;
use crate::PhysicalKeyDomainWitness;
use worth_store_contracts::WalRecordFamily;
use worth_store_physical_format::PhysicalRootManifestRebuildWitness;
use worth_store_wal::{BlobWalReplayRebuildWitness, StoreWalRecordIdentity};

use super::basis::S8DerivedIndexParityBasis;
use super::identity::{S8DerivedIndexCostEnvelopeParity, S8DerivedIndexIdentityParity};
use super::outcome::S8DerivedIndexRebuildDenied;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S8DerivedIndexRebuildSourceInput {
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
pub(crate) enum S8DerivedIndexAuthoritySource {
    PhysicalSnapshotReplay {
        family: PhysicalArtifactFamily,
        coverage: S8LayoutCoverageWitness,
        source_witness: PhysicalRootManifestRebuildWitness,
        parity_basis: S8DerivedIndexParityBasis,
    },
    WalReplay {
        family: PhysicalArtifactFamily,
        coverage: S8LayoutCoverageWitness,
        source_witness: BlobWalReplayRebuildWitness,
        parity_basis: S8DerivedIndexParityBasis,
    },
}

impl S8DerivedIndexAuthoritySource {
    pub(crate) fn declare(
        requirement: S8StrategyRebuildSourceRequirement,
        family: PhysicalArtifactFamily,
        coverage: S8LayoutCoverageWitness,
        key_domain: PhysicalKeyDomainWitness,
        source_input: &S8DerivedIndexRebuildSourceInput,
    ) -> Result<Option<Self>, S8DerivedIndexRebuildDenied> {
        match (requirement, source_input) {
            (
                S8StrategyRebuildSourceRequirement::PhysicalSnapshotReplay,
                S8DerivedIndexRebuildSourceInput::PhysicalRootManifest { source_witness },
            ) => Ok(Some(Self::PhysicalSnapshotReplay {
                family,
                coverage,
                parity_basis: physical_root_manifest_parity_basis(
                    source_witness,
                    key_domain,
                    coverage,
                )?,
                source_witness: source_witness.clone(),
            })),
            (
                S8StrategyRebuildSourceRequirement::WalReplay,
                S8DerivedIndexRebuildSourceInput::WalReplayRecord { source_witness },
            ) => Ok(Some(Self::WalReplay {
                family,
                coverage,
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

    pub const fn coverage(&self) -> S8LayoutCoverageWitness {
        match self {
            Self::PhysicalSnapshotReplay { coverage, .. } | Self::WalReplay { coverage, .. } => {
                *coverage
            }
        }
    }

    pub(crate) const fn parity_basis(&self) -> &S8DerivedIndexParityBasis {
        match self {
            Self::PhysicalSnapshotReplay { parity_basis, .. }
            | Self::WalReplay { parity_basis, .. } => parity_basis,
        }
    }

    pub(crate) const fn value_identity_parity(&self) -> S8DerivedIndexIdentityParity {
        match self {
            Self::PhysicalSnapshotReplay { .. } | Self::WalReplay { .. } => {
                S8DerivedIndexIdentityParity::SourceArtifactDoesNotProveIdentity
            }
        }
    }

    pub(crate) const fn cost_envelope_parity(&self) -> S8DerivedIndexCostEnvelopeParity {
        match self {
            Self::PhysicalSnapshotReplay { .. } | Self::WalReplay { .. } => {
                S8DerivedIndexCostEnvelopeParity::SourceArtifactDoesNotProveDeclaredEnvelope
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
    coverage: S8LayoutCoverageWitness,
) -> Result<S8DerivedIndexParityBasis, S8DerivedIndexRebuildDenied> {
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

            super::basis::S8DerivedIndexParityRow::new(key, "")
        })
        .collect::<Vec<_>>();

    S8DerivedIndexParityBasis::new(
        rows,
        coverage,
        false,
        source_witness.counter_shape().to_vec(),
    )
}

fn wal_replay_parity_basis(
    source_witness: &BlobWalReplayRebuildWitness,
    key_domain: PhysicalKeyDomainWitness,
    coverage: S8LayoutCoverageWitness,
) -> Result<S8DerivedIndexParityBasis, S8DerivedIndexRebuildDenied> {
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

    S8DerivedIndexParityBasis::new(
        vec![super::basis::S8DerivedIndexParityRow::new(key, "")],
        coverage,
        false,
        source_witness.counter_shape().to_vec(),
    )
}
