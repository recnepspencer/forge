#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) enum FinancialLocalityScenario {
    SparseBookFanout,
    PartitionedCurveUniverse,
    ConvergentFactorBatch,
    DenseMarketClose,
    PortfolioDependencyChurn,
    BranchRestoreLocalityReplay,
}

impl FinancialLocalityScenario {
    pub(in crate::tests::domains::fintech) const ALL: [Self; 6] = [
        Self::SparseBookFanout,
        Self::PartitionedCurveUniverse,
        Self::ConvergentFactorBatch,
        Self::DenseMarketClose,
        Self::PortfolioDependencyChurn,
        Self::BranchRestoreLocalityReplay,
    ];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) enum SparseFanoutAxis {
    IndexDisjoint,
    QueriedRejecting,
    RejectedDescendants,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) enum DensityRatio {
    OneInOneHundred,
    OneInFour,
    FourInFive,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) enum RestorePosture {
    Narrow,
    Convergent,
    DenseFourInFive,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::tests::domains::fintech) enum LocalityScaleTuple {
    SparseBookFanout {
        total_outputs: u32,
        axis: SparseFanoutAxis,
    },
    PartitionedCurveUniverse {
        /// Independently owned curve regions. Increasing only this axis adds
        /// index-disjoint subscriptions.
        regions: u16,
        /// Direct dependencies in the queried detail bucket. One admits and
        /// the remainder are valid contract rejections.
        matching_memberships: u16,
        /// Necessary downstream positions behind the one admitted membership.
        instruments_per_matching_region: u16,
    },
    ConvergentFactorBatch {
        producer_permutations: u8,
        duplicate_admissions: u8,
        canonical_seeds: u16,
    },
    DenseMarketClose {
        total_outputs: u32,
        affected_ratio: DensityRatio,
    },
    PortfolioDependencyChurn {
        rounds: u16,
        canonical_seeds: u16,
    },
    BranchRestoreLocalityReplay {
        posture: RestorePosture,
        total_outputs: u32,
        canonical_seeds: u16,
    },
}

impl LocalityScaleTuple {
    pub(in crate::tests::domains::fintech) const fn scenario(self) -> FinancialLocalityScenario {
        match self {
            Self::SparseBookFanout { .. } => FinancialLocalityScenario::SparseBookFanout,
            Self::PartitionedCurveUniverse { .. } => {
                FinancialLocalityScenario::PartitionedCurveUniverse
            }
            Self::ConvergentFactorBatch { .. } => FinancialLocalityScenario::ConvergentFactorBatch,
            Self::DenseMarketClose { .. } => FinancialLocalityScenario::DenseMarketClose,
            Self::PortfolioDependencyChurn { .. } => {
                FinancialLocalityScenario::PortfolioDependencyChurn
            }
            Self::BranchRestoreLocalityReplay { .. } => {
                FinancialLocalityScenario::BranchRestoreLocalityReplay
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) enum LocalityLane {
    OrdinaryChangeGate,
    Scheduled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::tests::domains::fintech) struct LocalityCaseContract {
    pub(in crate::tests::domains::fintech) lane: LocalityLane,
    pub(in crate::tests::domains::fintech) scale: LocalityScaleTuple,
}

impl LocalityCaseContract {
    const fn new(lane: LocalityLane, scale: LocalityScaleTuple) -> Self {
        Self { lane, scale }
    }

    pub(in crate::tests::domains::fintech) const fn scenario(self) -> FinancialLocalityScenario {
        self.scale.scenario()
    }
}

pub(in crate::tests::domains::fintech) fn ordinary_locality_cases() -> Vec<LocalityCaseContract> {
    let mut cases = Vec::new();
    append_sparse_cases(
        &mut cases,
        LocalityLane::OrdinaryChangeGate,
        &[64, 512, 4_096],
    );
    append_ordinary_partition_cases(&mut cases);
    append_ordinary_convergent_cases(&mut cases);
    append_dense_cases(&mut cases, LocalityLane::OrdinaryChangeGate, &[1_000]);
    append_ordinary_churn_case(&mut cases);
    append_ordinary_restore_cases(&mut cases);
    cases
}

fn append_sparse_cases(cases: &mut Vec<LocalityCaseContract>, lane: LocalityLane, totals: &[u32]) {
    for &total_outputs in totals {
        for axis in [
            SparseFanoutAxis::IndexDisjoint,
            SparseFanoutAxis::QueriedRejecting,
            SparseFanoutAxis::RejectedDescendants,
        ] {
            cases.push(LocalityCaseContract::new(
                lane,
                LocalityScaleTuple::SparseBookFanout {
                    total_outputs,
                    axis,
                },
            ));
        }
    }
}

fn append_ordinary_partition_cases(cases: &mut Vec<LocalityCaseContract>) {
    for regions in [16_u16, 256] {
        for matching_memberships in [1, regions / 16, regions / 4]
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>()
        {
            for instruments_per_matching_region in [1, 8] {
                cases.push(LocalityCaseContract::new(
                    LocalityLane::OrdinaryChangeGate,
                    LocalityScaleTuple::PartitionedCurveUniverse {
                        regions,
                        matching_memberships,
                        instruments_per_matching_region,
                    },
                ));
            }
        }
    }
}

fn append_ordinary_convergent_cases(cases: &mut Vec<LocalityCaseContract>) {
    for duplicate_admissions in [0, 1] {
        cases.push(LocalityCaseContract::new(
            LocalityLane::OrdinaryChangeGate,
            LocalityScaleTuple::ConvergentFactorBatch {
                producer_permutations: 24,
                duplicate_admissions,
                canonical_seeds: 1,
            },
        ));
    }
}

fn append_dense_cases(cases: &mut Vec<LocalityCaseContract>, lane: LocalityLane, totals: &[u32]) {
    for &total_outputs in totals {
        for affected_ratio in [
            DensityRatio::OneInOneHundred,
            DensityRatio::OneInFour,
            DensityRatio::FourInFive,
        ] {
            cases.push(LocalityCaseContract::new(
                lane,
                LocalityScaleTuple::DenseMarketClose {
                    total_outputs,
                    affected_ratio,
                },
            ));
        }
    }
}

fn append_ordinary_churn_case(cases: &mut Vec<LocalityCaseContract>) {
    cases.push(LocalityCaseContract::new(
        LocalityLane::OrdinaryChangeGate,
        LocalityScaleTuple::PortfolioDependencyChurn {
            rounds: 8,
            canonical_seeds: 1,
        },
    ));
}

fn append_ordinary_restore_cases(cases: &mut Vec<LocalityCaseContract>) {
    for posture in [
        RestorePosture::Narrow,
        RestorePosture::Convergent,
        RestorePosture::DenseFourInFive,
    ] {
        cases.push(LocalityCaseContract::new(
            LocalityLane::OrdinaryChangeGate,
            LocalityScaleTuple::BranchRestoreLocalityReplay {
                posture,
                total_outputs: if matches!(posture, RestorePosture::DenseFourInFive) {
                    1_000
                } else {
                    0
                },
                canonical_seeds: 1,
            },
        ));
    }
}

pub(in crate::tests::domains::fintech) fn scheduled_locality_cases() -> Vec<LocalityCaseContract> {
    let lane = LocalityLane::Scheduled;
    let mut cases = Vec::new();
    append_sparse_cases(&mut cases, lane, &[1_000, 10_000, 100_000]);
    append_scheduled_partition_cases(&mut cases, lane);
    append_scheduled_singletons(&mut cases, lane);
    append_dense_cases(&mut cases, lane, &[10_000, 100_000]);
    append_scheduled_restore_cases(&mut cases, lane);
    cases
}

fn append_scheduled_partition_cases(cases: &mut Vec<LocalityCaseContract>, lane: LocalityLane) {
    for matching_memberships in [1, 64, 256] {
        for instruments_per_matching_region in [1, 8, 32] {
            cases.push(LocalityCaseContract::new(
                lane,
                LocalityScaleTuple::PartitionedCurveUniverse {
                    regions: 1_024,
                    matching_memberships,
                    instruments_per_matching_region,
                },
            ));
        }
    }
}

fn append_scheduled_singletons(cases: &mut Vec<LocalityCaseContract>, lane: LocalityLane) {
    cases.extend([
        LocalityCaseContract::new(
            lane,
            LocalityScaleTuple::ConvergentFactorBatch {
                producer_permutations: 24,
                duplicate_admissions: 8,
                canonical_seeds: 16,
            },
        ),
        LocalityCaseContract::new(
            lane,
            LocalityScaleTuple::PortfolioDependencyChurn {
                rounds: 256,
                canonical_seeds: 16,
            },
        ),
    ]);
}

fn append_scheduled_restore_cases(cases: &mut Vec<LocalityCaseContract>, lane: LocalityLane) {
    for (posture, total_outputs) in [
        (RestorePosture::Narrow, 0),
        (RestorePosture::Convergent, 0),
        (RestorePosture::DenseFourInFive, 10_000),
    ] {
        cases.push(LocalityCaseContract::new(
            lane,
            LocalityScaleTuple::BranchRestoreLocalityReplay {
                posture,
                total_outputs,
                canonical_seeds: 8,
            },
        ));
    }
}

pub(in crate::tests::domains::fintech) fn retained_locality_benchmark_cases(
) -> Vec<LocalityCaseContract> {
    vec![LocalityCaseContract::new(
        LocalityLane::Scheduled,
        LocalityScaleTuple::BranchRestoreLocalityReplay {
            posture: RestorePosture::DenseFourInFive,
            total_outputs: 100_000,
            canonical_seeds: 8,
        },
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_scale_contract_covers_every_scenario_in_both_lanes() {
        let ordinary = ordinary_locality_cases();
        let scheduled = scheduled_locality_cases();
        for scenario in FinancialLocalityScenario::ALL {
            assert!(ordinary.iter().any(|case| case.scenario() == scenario));
            assert!(scheduled.iter().any(|case| case.scenario() == scenario));
        }
        assert!(ordinary
            .iter()
            .all(|case| case.lane == LocalityLane::OrdinaryChangeGate));
        assert!(scheduled
            .iter()
            .all(|case| case.lane == LocalityLane::Scheduled));
        assert_eq!(
            ordinary
                .iter()
                .map(|case| case.scale)
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            ordinary.len()
        );
        assert_eq!(
            scheduled
                .iter()
                .map(|case| case.scale)
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            scheduled.len()
        );
    }

    #[test]
    fn sparse_and_partition_change_gate_scales_match_the_normative_tuples() {
        let cases = ordinary_locality_cases();
        let sparse_totals = cases
            .iter()
            .filter_map(|case| match case.scale {
                LocalityScaleTuple::SparseBookFanout { total_outputs, .. } => Some(total_outputs),
                _ => None,
            })
            .collect::<std::collections::BTreeSet<_>>();
        let partition_regions = cases
            .iter()
            .filter_map(|case| match case.scale {
                LocalityScaleTuple::PartitionedCurveUniverse { regions, .. } => Some(regions),
                _ => None,
            })
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(sparse_totals, [64, 512, 4_096].into());
        assert_eq!(partition_regions, [16, 256].into());
    }

    #[test]
    fn retained_restore_benchmark_is_not_an_ordinary_merge_gate() {
        let retained = retained_locality_benchmark_cases();
        assert_eq!(retained.len(), 1);
        assert!(!scheduled_locality_cases()
            .iter()
            .any(|case| case.scale == retained[0].scale));
        assert_eq!(
            retained[0].scale,
            LocalityScaleTuple::BranchRestoreLocalityReplay {
                posture: RestorePosture::DenseFourInFive,
                total_outputs: 100_000,
                canonical_seeds: 8,
            }
        );
    }
}
