use crate::access::shape::{S8AccessLaneClassification, S8AccessShape, S8AccessShapeDetail};
use crate::facade::layout_declarations;
use crate::integrity::{layout_corruption, S8LayoutCorruptionInput, S8LayoutCorruptionOutcome};
use crate::strategy::{
    admit_strategy, S8AdmittedLayoutStrategy, S8StrategyRebuildSourceRequirement,
};
use crate::{CanonicalKeyBytes, PhysicalKeyDomainWitness};
use forge_store_contracts::WalRecordFamily;
use forge_store_wal::{record_kind_admits_recovery_replay, StoreWalRecordIdentity};

use super::basis::S8DerivedIndexParityBasis;
use super::outcome::{
    S8DerivedIndexParityOutcome, S8DerivedIndexRebuildDenied, S8DerivedIndexRebuildOutcome,
};
use super::parity::verify_parity;
use super::plan::{S8DerivedIndexRebuildPlan, S8DerivedIndexRebuildRequest};
use super::scope::S8DerivedIndexRebuildScope;
use super::source::{S8DerivedIndexAuthoritySource, S8DerivedIndexRebuildSourceInput};

#[derive(Debug, PartialEq, Eq)]
pub struct S8DerivedIndexRebuildReceipt {
    plan: S8DerivedIndexRebuildPlan,
    admitted_strategy: S8AdmittedLayoutStrategy,
    rebuilt_basis: S8DerivedIndexParityBasis,
}

impl S8DerivedIndexRebuildReceipt {
    pub(crate) fn new(
        plan: S8DerivedIndexRebuildPlan,
        admitted_strategy: S8AdmittedLayoutStrategy,
        rebuilt_basis: S8DerivedIndexParityBasis,
    ) -> Self {
        Self {
            plan,
            admitted_strategy,
            rebuilt_basis,
        }
    }

    pub const fn plan(&self) -> &S8DerivedIndexRebuildPlan {
        &self.plan
    }

    pub const fn admitted_strategy(&self) -> S8AdmittedLayoutStrategy {
        self.admitted_strategy
    }

    pub const fn rebuilt_basis(&self) -> &S8DerivedIndexParityBasis {
        &self.rebuilt_basis
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LayoutRebuildFacade;

impl S8LayoutRebuildFacade {
    pub fn admit_plan(
        &self,
        request: S8DerivedIndexRebuildRequest,
    ) -> Result<S8DerivedIndexRebuildPlan, S8DerivedIndexRebuildDenied> {
        let admitted_strategy = admit_strategy(
            request.lifecycle(),
            request.key_domain(),
            request.strategy_family(),
        )
        .map_err(|denial| S8DerivedIndexRebuildDenied::StrategyDenied { denial })?;
        if request.rebuild_shape().shape() != S8AccessShape::RebuildRead
            || !matches!(
                request.rebuild_shape().detail(),
                S8AccessShapeDetail::RebuildRead(_)
            )
            || request.rebuild_shape().lane() != S8AccessLaneClassification::Maintenance
        {
            return Err(S8DerivedIndexRebuildDenied::RebuildShapeRequired {
                family: request.strategy_family(),
            });
        }
        let shape_coverage = request
            .rebuild_shape()
            .coverage()
            .expect("rebuild-read shape carries exact coverage")
            .require_exact()
            .map_err(|denial| S8DerivedIndexRebuildDenied::CoverageDenied { denial })?;
        let source_authority = admit_source_authority(&request, admitted_strategy, shape_coverage)?;
        let corruption = classify_corruption(&source_authority);

        Ok(S8DerivedIndexRebuildPlan::new(
            request,
            source_authority,
            S8DerivedIndexRebuildScope::from_coverage(shape_coverage),
            corruption,
        ))
    }

    pub fn rebuild(
        &self,
        plan: S8DerivedIndexRebuildPlan,
        rebuilt_basis: S8DerivedIndexParityBasis,
    ) -> S8DerivedIndexRebuildOutcome {
        if let crate::S8LayoutCorruptionView::Quarantined(quarantine) = plan.corruption().view() {
            return S8DerivedIndexRebuildOutcome::quarantined(quarantine.clone());
        }
        if rebuilt_basis.coverage() != plan.rebuild_scope().authority_coverage() {
            return S8DerivedIndexRebuildOutcome::denied(
                S8DerivedIndexRebuildDenied::ParityCoverageMismatch {
                    expected: plan.rebuild_scope().authority_coverage(),
                    actual: rebuilt_basis.coverage(),
                },
            );
        }

        match admit_strategy(
            plan.request().lifecycle(),
            plan.request().key_domain(),
            plan.request().strategy_family(),
        ) {
            Ok(admitted_strategy) => S8DerivedIndexRebuildOutcome::rebuilt(
                S8DerivedIndexRebuildReceipt::new(plan, admitted_strategy, rebuilt_basis),
            ),
            Err(denial) => {
                S8DerivedIndexRebuildOutcome::denied(S8DerivedIndexRebuildDenied::StrategyDenied {
                    denial,
                })
            }
        }
    }

    pub fn verify_parity(
        &self,
        receipt: S8DerivedIndexRebuildReceipt,
    ) -> S8DerivedIndexParityOutcome {
        match verify_parity(receipt) {
            Ok(witness) => S8DerivedIndexParityOutcome::verified(witness),
            Err(denial) => S8DerivedIndexParityOutcome::denied(denial),
        }
    }
}

fn admit_source_authority(
    request: &S8DerivedIndexRebuildRequest,
    admitted_strategy: S8AdmittedLayoutStrategy,
    shape_coverage: crate::materialization::S8LayoutCoverageWitness,
) -> Result<S8DerivedIndexAuthoritySource, S8DerivedIndexRebuildDenied> {
    let family = request.lifecycle().declaration().family();
    let requirement = admitted_strategy.rebuild_source_requirement();
    let source_authority = S8DerivedIndexAuthoritySource::declare(
        requirement,
        family,
        shape_coverage,
        request.key_domain(),
        request.source_input(),
    )?
    .ok_or_else(|| source_strategy_denial(requirement, request.source_input()))?;

    if source_authority.family() != family {
        return Err(S8DerivedIndexRebuildDenied::SourceFamilyMismatch {
            expected: family,
            actual: source_authority.family(),
        });
    }
    if source_authority.parity_basis().coverage() != shape_coverage {
        return Err(
            S8DerivedIndexRebuildDenied::SourceCoverageDoesNotMatchRebuildShape {
                expected: shape_coverage,
                actual: source_authority.parity_basis().coverage(),
            },
        );
    }
    let expected_rows = source_authority.authority_row_count();
    let actual_rows = source_authority.parity_basis().row_count();
    if expected_rows != actual_rows {
        return Err(
            S8DerivedIndexRebuildDenied::SourceParityBasisDoesNotMatchAuthorityArtifact {
                expected_rows,
                actual_rows,
            },
        );
    }
    validate_source_parity_keys(&source_authority, request.key_domain())?;

    Ok(source_authority)
}

fn classify_corruption(
    source_authority: &S8DerivedIndexAuthoritySource,
) -> S8LayoutCorruptionOutcome {
    let family = source_authority.family();
    let classification = match source_authority {
        S8DerivedIndexAuthoritySource::PhysicalSnapshotReplay { source_witness, .. } => {
            if source_witness.manifest().page_slots().is_empty() {
                super::corruption::LayoutCorruptionClassification::AuthoritativeSourceQuarantineRequired { family }
            } else {
                super::corruption::LayoutCorruptionClassification::DerivedProjectionRebuildToParity
            }
        }
        S8DerivedIndexAuthoritySource::WalReplay { source_witness, .. } => {
            if !record_kind_admits_recovery_replay(source_witness.record().identity().kind()) {
                super::corruption::LayoutCorruptionClassification::AuthoritativeSourceQuarantineRequired { family }
            } else {
                super::corruption::LayoutCorruptionClassification::DerivedProjectionRebuildToParity
            }
        }
    };

    layout_corruption().classify(S8LayoutCorruptionInput::RebuildClassification(
        classification,
    ))
}

fn source_strategy_denial(
    requirement: S8StrategyRebuildSourceRequirement,
    source_input: &S8DerivedIndexRebuildSourceInput,
) -> S8DerivedIndexRebuildDenied {
    match source_input {
        S8DerivedIndexRebuildSourceInput::PhysicalRootManifest { .. } => {
            S8DerivedIndexRebuildDenied::SourceArtifactDoesNotMatchStrategy {
                required: requirement,
                source: "physical_root_manifest",
            }
        }
        S8DerivedIndexRebuildSourceInput::WalReplayRecord { .. } => {
            S8DerivedIndexRebuildDenied::SourceArtifactDoesNotMatchStrategy {
                required: requirement,
                source: "wal_replay_record",
            }
        }
        source => S8DerivedIndexRebuildDenied::SourceInputIsNotAuthority {
            source: source.clone(),
        },
    }
}

fn validate_source_parity_keys(
    source_authority: &S8DerivedIndexAuthoritySource,
    key_domain: PhysicalKeyDomainWitness,
) -> Result<(), S8DerivedIndexRebuildDenied> {
    let expected_keys = expected_authority_keys(source_authority, key_domain);
    let parity_basis = source_authority.parity_basis();

    if parity_basis.unique_keys() != expected_keys.as_slice()
        || parity_basis
            .ordered_rows()
            .iter()
            .map(|row| row.key())
            .ne(expected_keys.iter())
    {
        return Err(S8DerivedIndexRebuildDenied::SourceParityBasisKeysDoNotMatchAuthorityArtifact);
    }

    Ok(())
}

fn expected_authority_keys(
    source_authority: &S8DerivedIndexAuthoritySource,
    key_domain: PhysicalKeyDomainWitness,
) -> Vec<CanonicalKeyBytes> {
    let encoding = layout_declarations().require_canonical_key_encoding(key_domain);
    let comparator = layout_declarations().declare_comparator_law(encoding);

    let mut keys = match source_authority {
        S8DerivedIndexAuthoritySource::PhysicalSnapshotReplay { source_witness, .. } => {
            source_witness
                .manifest()
                .page_slots()
                .iter()
                .map(|entry| {
                    let slot = entry.page_slot();
                    let key = layout_declarations()
                        .admit_page_address_key(key_domain, slot.segment_id(), slot.page_id())
                        .expect("admitted rebuild plan should carry a compatible page key domain");
                    layout_declarations()
                        .canonical_key_bytes(comparator, key)
                        .expect("admitted page address key should encode canonically")
                })
                .collect()
        }
        S8DerivedIndexAuthoritySource::WalReplay { source_witness, .. } => {
            let key = layout_declarations()
                .admit_wal_record_key(
                    key_domain,
                    WalRecordFamily::DurableMutationIntent,
                    StoreWalRecordIdentity::new(source_witness.record().identity().sequence()),
                )
                .expect("admitted rebuild plan should carry a compatible wal key domain");
            vec![layout_declarations()
                .canonical_key_bytes(comparator, key)
                .expect("admitted wal record key should encode canonically")]
        }
    };
    keys.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    keys
}

pub const fn layout_rebuild() -> S8LayoutRebuildFacade {
    S8LayoutRebuildFacade
}
