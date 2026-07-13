use crate::access::shape::{AccessLaneClassification, AccessShape, AccessShapeDetail};
use crate::facade::layout_declarations;
use crate::integrity::{layout_corruption, LayoutCorruptionInput, LayoutCorruptionOutcome};
use crate::strategy::{
    admit_strategy_from_basis, AdmittedLayoutStrategy, StrategyAuthorityBasis,
    StrategyRebuildSourceRequirement,
};
use crate::{CanonicalKeyBytes, PhysicalKeyDomainWitness};
use forge_store_contracts::WalRecordFamily;
use forge_store_wal::{record_kind_admits_recovery_replay, StoreWalRecordIdentity};

use super::basis::DerivedIndexParityBasis;
use super::outcome::{
    DerivedIndexParityOutcome, DerivedIndexRebuildDenied, DerivedIndexRebuildOutcome,
};
use super::parity::verify_parity;
use super::plan::{DerivedIndexRebuildPlan, DerivedIndexRebuildRequest};
use super::scope::DerivedIndexRebuildScope;
use super::source::{DerivedIndexAuthoritySource, DerivedIndexRebuildSourceInput};

#[derive(Debug, PartialEq, Eq)]
pub struct DerivedIndexRebuildReceipt {
    plan: DerivedIndexRebuildPlan,
    admitted_strategy: AdmittedLayoutStrategy,
    rebuilt_basis: DerivedIndexParityBasis,
}

impl DerivedIndexRebuildReceipt {
    pub(crate) fn new(
        plan: DerivedIndexRebuildPlan,
        admitted_strategy: AdmittedLayoutStrategy,
        rebuilt_basis: DerivedIndexParityBasis,
    ) -> Self {
        Self {
            plan,
            admitted_strategy,
            rebuilt_basis,
        }
    }

    pub const fn plan(&self) -> &DerivedIndexRebuildPlan {
        &self.plan
    }

    pub const fn admitted_strategy(&self) -> AdmittedLayoutStrategy {
        self.admitted_strategy
    }

    pub const fn rebuilt_basis(&self) -> &DerivedIndexParityBasis {
        &self.rebuilt_basis
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutRebuildFacade;

impl LayoutRebuildFacade {
    pub fn admit_plan(
        &self,
        request: DerivedIndexRebuildRequest,
    ) -> Result<DerivedIndexRebuildPlan, DerivedIndexRebuildDenied> {
        let admitted_strategy = admit_strategy_from_basis(
            StrategyAuthorityBasis::admitted(
                request.admitted_family(),
                request.admitted_key_domain(),
            ),
            request.strategy_family(),
        )
        .map_err(|denial| DerivedIndexRebuildDenied::StrategyDenied { denial })?;
        if request.rebuild_shape().shape() != AccessShape::RebuildRead
            || !matches!(
                request.rebuild_shape().detail(),
                AccessShapeDetail::RebuildRead(_)
            )
            || request.rebuild_shape().lane() != AccessLaneClassification::Maintenance
        {
            return Err(DerivedIndexRebuildDenied::RebuildShapeRequired {
                family: request.strategy_family(),
            });
        }
        let shape_coverage = request
            .materialization()
            .coverage()
            .require_exact()
            .map_err(|denial| DerivedIndexRebuildDenied::CoverageDenied { denial })?;
        let source_authority =
            admit_source_authority(&request, admitted_strategy, shape_coverage.clone())?;
        let corruption = classify_corruption(&source_authority);

        Ok(DerivedIndexRebuildPlan::new(
            request,
            source_authority,
            DerivedIndexRebuildScope::from_coverage(shape_coverage),
            corruption,
        ))
    }

    pub fn rebuild(
        &self,
        plan: DerivedIndexRebuildPlan,
        rebuilt_basis: DerivedIndexParityBasis,
    ) -> DerivedIndexRebuildOutcome {
        if let crate::LayoutCorruptionView::Quarantined(quarantine) = plan.corruption().view() {
            return DerivedIndexRebuildOutcome::quarantined(quarantine.clone());
        }
        if rebuilt_basis.coverage() != plan.rebuild_scope().authority_coverage() {
            return DerivedIndexRebuildOutcome::denied(
                DerivedIndexRebuildDenied::ParityCoverageMismatch {
                    expected: plan.rebuild_scope().authority_coverage().clone(),
                    actual: rebuilt_basis.coverage().clone(),
                },
            );
        }

        match admit_strategy_from_basis(
            StrategyAuthorityBasis::admitted(
                plan.request().admitted_family(),
                plan.request().admitted_key_domain(),
            ),
            plan.request().strategy_family(),
        ) {
            Ok(admitted_strategy) => DerivedIndexRebuildOutcome::rebuilt(
                DerivedIndexRebuildReceipt::new(plan, admitted_strategy, rebuilt_basis),
            ),
            Err(denial) => {
                DerivedIndexRebuildOutcome::denied(DerivedIndexRebuildDenied::StrategyDenied {
                    denial,
                })
            }
        }
    }

    pub fn verify_parity(&self, receipt: DerivedIndexRebuildReceipt) -> DerivedIndexParityOutcome {
        match verify_parity(receipt) {
            Ok(witness) => DerivedIndexParityOutcome::verified(witness),
            Err(denial) => DerivedIndexParityOutcome::denied(denial),
        }
    }
}

fn admit_source_authority(
    request: &DerivedIndexRebuildRequest,
    admitted_strategy: AdmittedLayoutStrategy,
    shape_coverage: crate::materialization::LayoutCoverageWitness,
) -> Result<DerivedIndexAuthoritySource, DerivedIndexRebuildDenied> {
    let family = request.lifecycle().declaration().family();
    let requirement = admitted_strategy.rebuild_source_requirement();
    let source_authority = DerivedIndexAuthoritySource::declare(
        requirement,
        family,
        shape_coverage.clone(),
        request.key_domain(),
        request.source_input(),
    )?
    .ok_or_else(|| source_strategy_denial(requirement, request.source_input()))?;

    if source_authority.family() != family {
        return Err(DerivedIndexRebuildDenied::SourceFamilyMismatch {
            expected: family,
            actual: source_authority.family(),
        });
    }
    if source_authority.parity_basis().coverage() != &shape_coverage {
        return Err(
            DerivedIndexRebuildDenied::SourceCoverageDoesNotMatchRebuildShape {
                expected: shape_coverage,
                actual: source_authority.parity_basis().coverage().clone(),
            },
        );
    }
    let expected_rows = source_authority.authority_row_count();
    let actual_rows = source_authority.parity_basis().row_count();
    if expected_rows != actual_rows {
        return Err(
            DerivedIndexRebuildDenied::SourceParityBasisDoesNotMatchAuthorityArtifact {
                expected_rows,
                actual_rows,
            },
        );
    }
    validate_source_parity_keys(&source_authority, request.key_domain())?;

    Ok(source_authority)
}

fn classify_corruption(source_authority: &DerivedIndexAuthoritySource) -> LayoutCorruptionOutcome {
    let family = source_authority.family();
    let classification = match source_authority {
        DerivedIndexAuthoritySource::PhysicalSnapshotReplay { source_witness, .. } => {
            if source_witness.manifest().page_slots().is_empty() {
                super::corruption::LayoutCorruptionClassification::AuthoritativeSourceQuarantineRequired { family }
            } else {
                super::corruption::LayoutCorruptionClassification::DerivedProjectionRebuildToParity
            }
        }
        DerivedIndexAuthoritySource::WalReplay { source_witness, .. } => {
            if !record_kind_admits_recovery_replay(source_witness.record().identity().kind()) {
                super::corruption::LayoutCorruptionClassification::AuthoritativeSourceQuarantineRequired { family }
            } else {
                super::corruption::LayoutCorruptionClassification::DerivedProjectionRebuildToParity
            }
        }
    };

    layout_corruption().classify(LayoutCorruptionInput::RebuildClassification(classification))
}

fn source_strategy_denial(
    requirement: StrategyRebuildSourceRequirement,
    source_input: &DerivedIndexRebuildSourceInput,
) -> DerivedIndexRebuildDenied {
    match source_input {
        DerivedIndexRebuildSourceInput::PhysicalRootManifest { .. } => {
            DerivedIndexRebuildDenied::SourceArtifactDoesNotMatchStrategy {
                required: requirement,
                source: "physical_root_manifest",
            }
        }
        DerivedIndexRebuildSourceInput::WalReplayRecord { .. } => {
            DerivedIndexRebuildDenied::SourceArtifactDoesNotMatchStrategy {
                required: requirement,
                source: "wal_replay_record",
            }
        }
        source => DerivedIndexRebuildDenied::SourceInputIsNotAuthority {
            source: source.clone(),
        },
    }
}

fn validate_source_parity_keys(
    source_authority: &DerivedIndexAuthoritySource,
    key_domain: PhysicalKeyDomainWitness,
) -> Result<(), DerivedIndexRebuildDenied> {
    let expected_keys = expected_authority_keys(source_authority, key_domain);
    let parity_basis = source_authority.parity_basis();

    if parity_basis.unique_keys() != expected_keys.as_slice()
        || parity_basis
            .ordered_rows()
            .iter()
            .map(|row| row.key())
            .ne(expected_keys.iter())
    {
        return Err(DerivedIndexRebuildDenied::SourceParityBasisKeysDoNotMatchAuthorityArtifact);
    }

    Ok(())
}

fn expected_authority_keys(
    source_authority: &DerivedIndexAuthoritySource,
    key_domain: PhysicalKeyDomainWitness,
) -> Vec<CanonicalKeyBytes> {
    let encoding = layout_declarations().require_canonical_key_encoding(key_domain);
    let comparator = layout_declarations().declare_comparator_law(encoding);

    let mut keys = match source_authority {
        DerivedIndexAuthoritySource::PhysicalSnapshotReplay { source_witness, .. } => {
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
        DerivedIndexAuthoritySource::WalReplay { source_witness, .. } => {
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

pub const fn layout_rebuild() -> LayoutRebuildFacade {
    LayoutRebuildFacade
}
