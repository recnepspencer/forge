use crate::catalog::PhysicalArtifactFamily;
use crate::facade::layout_declarations;
use crate::materialization::LayoutCoverageWitness;
use crate::strategy::StrategyRebuildSourceRequirement;
use crate::PhysicalKeyDomainWitness;
use worth_store_contracts::WalRecordFamily;
use worth_store_physical_format::PhysicalRootManifestRebuildSource;
use worth_store_wal::{BlobWalReplayRebuildWitness, StoreWalRecordIdentity};

use super::super::basis::DerivedIndexParityBasis;
use super::super::{
    DerivedIndexCostEnvelopeParity, DerivedIndexIdentityParity, DerivedIndexResultIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DerivedIndexRebuildSourceInput {
    PhysicalRootManifest {
        source: PhysicalRootManifestRebuildSource,
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
        source: PhysicalRootManifestRebuildSource,
    },
    WalReplay {
        family: PhysicalArtifactFamily,
        coverage: LayoutCoverageWitness,
        source_witness: BlobWalReplayRebuildWitness,
    },
}

impl DerivedIndexAuthoritySource {
    pub(crate) fn declare(
        requirement: StrategyRebuildSourceRequirement,
        family: PhysicalArtifactFamily,
        coverage: LayoutCoverageWitness,
        source_input: &DerivedIndexRebuildSourceInput,
    ) -> Option<Self> {
        match (requirement, source_input) {
            (
                StrategyRebuildSourceRequirement::PhysicalSnapshotReplay,
                DerivedIndexRebuildSourceInput::PhysicalRootManifest { source },
            ) => Some(Self::PhysicalSnapshotReplay {
                family,
                coverage,
                source: source.clone(),
            }),
            (
                StrategyRebuildSourceRequirement::WalReplay,
                DerivedIndexRebuildSourceInput::WalReplayRecord { source_witness },
            ) => Some(Self::WalReplay {
                family,
                coverage,
                source_witness: source_witness.clone(),
            }),
            _ => None,
        }
    }

    pub(crate) fn rebuild_candidate(
        &self,
        key_domain: PhysicalKeyDomainWitness,
    ) -> DerivedIndexParityBasis {
        match self {
            Self::PhysicalSnapshotReplay {
                source, coverage, ..
            } => physical_root_manifest_parity_basis(source, key_domain, coverage.clone()),
            Self::WalReplay {
                source_witness,
                coverage,
                ..
            } => wal_replay_parity_basis(source_witness, key_domain, coverage.clone()),
        }
    }

    pub(crate) fn authoritative_parity_basis(
        &self,
        key_domain: PhysicalKeyDomainWitness,
    ) -> DerivedIndexParityBasis {
        self.rebuild_candidate(key_domain)
    }

    pub(crate) const fn source_artifact_count(&self) -> u64 {
        1
    }

    pub(crate) fn result_identity(&self) -> DerivedIndexResultIdentity {
        match self {
            Self::PhysicalSnapshotReplay { source, .. } => {
                DerivedIndexResultIdentity::PhysicalRoot {
                    reference: source.witness().root_reference(),
                    authority: source.store_authority_identity(),
                }
            }
            Self::WalReplay { source_witness, .. } => DerivedIndexResultIdentity::WalReplay {
                record: source_witness.record().identity(),
                security: source_witness.security_metadata().security_identity(),
                authority: source_witness.security_metadata().authority_identity(),
            },
        }
    }

    pub(crate) const fn value_identity_parity(&self) -> DerivedIndexIdentityParity {
        match self {
            Self::PhysicalSnapshotReplay { .. } | Self::WalReplay { .. } => {
                DerivedIndexIdentityParity::Exact
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
}

fn physical_root_manifest_parity_basis(
    source: &PhysicalRootManifestRebuildSource,
    key_domain: PhysicalKeyDomainWitness,
    coverage: LayoutCoverageWitness,
) -> DerivedIndexParityBasis {
    let source_witness = source.witness();
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

            super::super::basis::DerivedIndexParityRow::new(key, row.value_fingerprint())
        })
        .collect::<Vec<_>>();

    DerivedIndexParityBasis::from_admitted_source(
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
) -> DerivedIndexParityBasis {
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

    DerivedIndexParityBasis::from_admitted_source(
        vec![super::super::basis::DerivedIndexParityRow::new(
            key,
            record.payload_digest(),
        )],
        coverage,
        false,
        source_witness.counter_shape().to_vec(),
    )
}
