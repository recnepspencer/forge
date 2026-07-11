use super::completeness::{S8PrefixCompletenessWitness, S8RangeCompletenessWitness};
use super::denial::{S8CoverageGapWitness, S8MaterializationDenial};
use super::state::{S8LayoutMaterializationState, S8MaterializationStateClass};
use super::watermark::S8LayoutWatermark;
use crate::catalog::PhysicalArtifactFamily;
use crate::integrity::{layout_corruption, S8LayoutCorruptionInput, S8LayoutCorruptionOutcome};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LayoutCoverageWitness {
    family: PhysicalArtifactFamily,
    state: S8LayoutMaterializationState,
    lower_bound: S8LayoutWatermark,
    upper_bound: S8LayoutWatermark,
    gap: Option<S8CoverageGapWitness>,
}

impl S8LayoutCoverageWitness {
    fn new(
        state: S8LayoutMaterializationState,
        lower_bound: S8LayoutWatermark,
        upper_bound: S8LayoutWatermark,
        gap: Option<S8CoverageGapWitness>,
    ) -> Result<Self, S8MaterializationDenial> {
        let family = state.family();
        let ordered_basis = matches!(
            state.class(),
            S8MaterializationStateClass::Exact
                | S8MaterializationStateClass::ExactThroughPhysicalBasis
                | S8MaterializationStateClass::EmptyInitialized
                | S8MaterializationStateClass::Lagged
                | S8MaterializationStateClass::Stale
                | S8MaterializationStateClass::PartiallyCovered
                | S8MaterializationStateClass::Quarantined
        );

        if ordered_basis && lower_bound.basis_kind() != upper_bound.basis_kind() {
            return Err(
                S8MaterializationDenial::CoverageBasisDoesNotMatchMaterializationState {
                    family,
                    state: state.class(),
                    basis_kind: upper_bound.basis_kind(),
                },
            );
        }

        if lower_bound.value() > upper_bound.value() {
            return Err(S8MaterializationDenial::CoverageIntervalIsReversed {
                family,
                basis_kind: upper_bound.basis_kind(),
                lower_bound: lower_bound.value(),
                upper_bound: upper_bound.value(),
            });
        }

        Ok(Self {
            family,
            state,
            lower_bound,
            upper_bound,
            gap,
        })
    }

    pub(crate) fn exact_through(
        state: S8LayoutMaterializationState,
        watermark: S8LayoutWatermark,
    ) -> Result<Self, S8MaterializationDenial> {
        Self::new(state, watermark, watermark, None)
    }

    pub(crate) fn lagged(
        state: S8LayoutMaterializationState,
        lower_bound: S8LayoutWatermark,
        upper_bound: S8LayoutWatermark,
    ) -> Result<Self, S8MaterializationDenial> {
        Self::new(state, lower_bound, upper_bound, None)
    }

    pub(crate) fn partially_covered(
        state: S8LayoutMaterializationState,
        lower_bound: S8LayoutWatermark,
        upper_bound: S8LayoutWatermark,
        gap: S8CoverageGapWitness,
    ) -> Result<Self, S8MaterializationDenial> {
        Self::new(state, lower_bound, upper_bound, Some(gap))
    }

    pub const fn family(self) -> PhysicalArtifactFamily {
        self.family
    }

    pub const fn state(self) -> S8LayoutMaterializationState {
        self.state
    }

    pub const fn lower_bound(self) -> S8LayoutWatermark {
        self.lower_bound
    }

    pub const fn upper_bound(self) -> S8LayoutWatermark {
        self.upper_bound
    }

    pub const fn gap(self) -> Option<S8CoverageGapWitness> {
        self.gap
    }

    pub const fn is_exact(self) -> bool {
        self.state.supports_exact_access() && self.gap.is_none()
    }

    pub fn require_exact(self) -> Result<Self, S8MaterializationDenial> {
        if let Some(gap) = self.gap {
            if self.state.class() != S8MaterializationStateClass::Quarantined {
                return Err(S8MaterializationDenial::LayoutCoverageIsPartial { gap });
            }
        }

        let outcome = layout_corruption().classify(S8LayoutCorruptionInput::Materialization(self));
        match outcome.view() {
            crate::S8LayoutCorruptionView::Clean(coverage) => Ok(*coverage),
            crate::S8LayoutCorruptionView::StaleBinding(coverage) => {
                Err(S8MaterializationDenial::LayoutCoverageIsStale {
                    family: coverage.family(),
                    basis_kind: coverage.upper_bound().basis_kind(),
                })
            }
            crate::S8LayoutCorruptionView::Quarantined(quarantine) => {
                let coverage = quarantine
                    .coverage()
                    .expect("materialization-backed quarantine retains its coverage");
                let gap = coverage.gap().unwrap_or_else(|| {
                    S8CoverageGapWitness::physical_range(
                        coverage.family(),
                        coverage.upper_bound().basis_kind(),
                        coverage.lower_bound().value(),
                        coverage.upper_bound().value(),
                    )
                });
                Err(S8MaterializationDenial::LayoutRangeIsQuarantined { gap })
            }
            crate::S8LayoutCorruptionView::RebuildRequired(_) => {
                Err(S8MaterializationDenial::LayoutRequiresRebuild {
                    family: self.family,
                })
            }
            crate::S8LayoutCorruptionView::MigrationRequired(family) => {
                Err(S8MaterializationDenial::LayoutIsMigrating { family: *family })
            }
            crate::S8LayoutCorruptionView::Unsupported(unsupported)
                if unsupported.state() == S8MaterializationStateClass::Lagged =>
            {
                Err(S8MaterializationDenial::LayoutCoverageIsLagged {
                    family: unsupported.family(),
                    basis_kind: self.upper_bound.basis_kind(),
                })
            }
            crate::S8LayoutCorruptionView::NotFound(family) => Err(
                S8MaterializationDenial::MaterializationStateDoesNotSupportExactAccess {
                    family: *family,
                    state: S8MaterializationStateClass::Absent,
                },
            ),
            crate::S8LayoutCorruptionView::Unsupported(unsupported) => Err(
                S8MaterializationDenial::MaterializationStateDoesNotSupportExactAccess {
                    family: unsupported.family(),
                    state: unsupported.state(),
                },
            ),
            crate::S8LayoutCorruptionView::QuarantineReadmissionRequired(_)
            | crate::S8LayoutCorruptionView::OfflineReadmissionRequired(_)
            | crate::S8LayoutCorruptionView::ImportReadmissionRequired(_) => {
                unreachable!("materialization classification does not emit readmission outcomes")
            }
        }
    }

    pub fn require_exact_range_completeness(
        self,
    ) -> Result<S8RangeCompletenessWitness, S8MaterializationDenial> {
        let exact = self.require_exact()?;
        Ok(S8RangeCompletenessWitness::new(exact))
    }

    pub fn require_exact_prefix_completeness(
        self,
    ) -> Result<S8PrefixCompletenessWitness, S8MaterializationDenial> {
        let exact = self.require_exact()?;
        Ok(S8PrefixCompletenessWitness::new(exact))
    }
}
