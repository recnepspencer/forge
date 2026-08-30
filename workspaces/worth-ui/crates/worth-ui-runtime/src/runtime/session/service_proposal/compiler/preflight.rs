pub(in crate::runtime) struct UiPreflightedServiceProposal {
    candidate: super::UiServiceProposalCandidate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum UiServiceProposalPreflightDenial {
    EmptyParticipation,
    DuplicateFamily,
    FamilySetMismatch,
    AggregateDemandMismatch,
    AggregateDemandOverflow,
    UnsupportedFamily(crate::capability::UiRuntimeServiceFamily),
    RequirementBudgetExceeded,
    FactReferenceBudgetExceeded,
    MountedWorkReferenceBudgetExceeded,
    Coherence(super::super::UiServiceRequestCoherenceDrift),
}

#[cfg(test)]
const PARTICIPATING_FAMILY_LIMIT: u8 = 6;
const REQUIREMENT_LIMIT: u8 = 12;
const FACT_REFERENCE_LIMIT: u16 = 64;
const MOUNTED_WORK_REFERENCE_LIMIT: u16 = 64;

pub(super) fn preflight(
    candidate: super::UiServiceProposalCandidate,
    current: &super::super::UiServiceRequestCoherence,
    support: crate::capability::UiRuntimeServiceSupport,
) -> Result<UiPreflightedServiceProposal, UiServiceProposalPreflightDenial> {
    let demand = recompute_and_validate_demand(&candidate)?;
    if demand.participating_families().count() == 0 {
        return Err(UiServiceProposalPreflightDenial::EmptyParticipation);
    }
    if demand.requirements() > REQUIREMENT_LIMIT {
        return Err(UiServiceProposalPreflightDenial::RequirementBudgetExceeded);
    }
    if demand.fact_references() > FACT_REFERENCE_LIMIT {
        return Err(UiServiceProposalPreflightDenial::FactReferenceBudgetExceeded);
    }
    if demand.mounted_work_references() > MOUNTED_WORK_REFERENCE_LIMIT {
        return Err(UiServiceProposalPreflightDenial::MountedWorkReferenceBudgetExceeded);
    }
    candidate
        .coherence()
        .ensure_matches(current)
        .map_err(UiServiceProposalPreflightDenial::Coherence)?;
    validate_family_support(&candidate, support)?;
    Ok(UiPreflightedServiceProposal { candidate })
}

fn recompute_and_validate_demand(
    candidate: &super::UiServiceProposalCandidate,
) -> Result<super::UiServiceProposalDemand, UiServiceProposalPreflightDenial> {
    let recomputed =
        super::UiServiceProposalDemand::from_family_proposals(candidate.family_proposals())
            .map_err(|denial| match denial {
                super::UiServiceProposalDemandConstructionDenial::Participation(
                    super::super::UiServiceFamilyParticipationDenial::DuplicateFamily,
                ) => UiServiceProposalPreflightDenial::DuplicateFamily,
                super::UiServiceProposalDemandConstructionDenial::ArithmeticOverflow => {
                    UiServiceProposalPreflightDenial::AggregateDemandOverflow
                }
            })?;
    let declared = candidate.demand();
    if recomputed.participating_families() != declared.participating_families() {
        return Err(UiServiceProposalPreflightDenial::FamilySetMismatch);
    }
    if recomputed.requirements() != declared.requirements()
        || recomputed.fact_references() != declared.fact_references()
        || recomputed.mounted_work_references() != declared.mounted_work_references()
    {
        return Err(UiServiceProposalPreflightDenial::AggregateDemandMismatch);
    }
    Ok(recomputed)
}

fn validate_family_support(
    candidate: &super::UiServiceProposalCandidate,
    support: crate::capability::UiRuntimeServiceSupport,
) -> Result<(), UiServiceProposalPreflightDenial> {
    for proposal in candidate.family_proposals() {
        let family = proposal.family();
        if support.posture(family) != crate::capability::UiRuntimeServiceSupportPosture::Installed {
            return Err(UiServiceProposalPreflightDenial::UnsupportedFamily(family));
        }
    }
    Ok(())
}

impl UiPreflightedServiceProposal {
    pub(super) fn into_candidate(self) -> super::UiServiceProposalCandidate {
        self.candidate
    }

    pub(super) fn candidate(&self) -> &super::UiServiceProposalCandidate {
        &self.candidate
    }
}

#[cfg(test)]
mod tests {
    use super::{
        UiServiceProposalPreflightDenial, FACT_REFERENCE_LIMIT, MOUNTED_WORK_REFERENCE_LIMIT,
        PARTICIPATING_FAMILY_LIMIT, REQUIREMENT_LIMIT,
    };

    #[test]
    fn preflight_limits_are_explicit_and_cover_the_closed_family_set() {
        assert_eq!(PARTICIPATING_FAMILY_LIMIT, 6);
        assert_eq!(REQUIREMENT_LIMIT, 12);
        assert_eq!(FACT_REFERENCE_LIMIT, 64);
        assert_eq!(MOUNTED_WORK_REFERENCE_LIMIT, 64);
        let coherence = super::super::super::fixture_service_request_coherence(1);
        let empty = super::super::UiServiceProposalCandidate::for_test(
            1,
            super::super::UiServiceProposalDemand::recorded_fixture(
                super::super::super::UiServiceFamilyParticipation::EMPTY,
                0,
                0,
                0,
            ),
            coherence.clone(),
            Vec::new(),
        );
        assert_eq!(
            super::preflight(empty, &coherence, installed_support()).err(),
            Some(UiServiceProposalPreflightDenial::EmptyParticipation)
        );

        for (requirements, fact_references, mounted_work_references, denial) in [
            (
                13,
                1,
                1,
                UiServiceProposalPreflightDenial::RequirementBudgetExceeded,
            ),
            (
                1,
                65,
                1,
                UiServiceProposalPreflightDenial::FactReferenceBudgetExceeded,
            ),
            (
                1,
                1,
                65,
                UiServiceProposalPreflightDenial::MountedWorkReferenceBudgetExceeded,
            ),
        ] {
            let candidate = super::super::UiServiceProposalCandidate::for_test(
                1,
                super::super::UiServiceProposalDemand::recorded_fixture(
                    participation(),
                    requirements,
                    fact_references,
                    mounted_work_references,
                ),
                coherence.clone(),
                vec![super::super::UiServiceFamilyProposal::recorded_fixture(
                    crate::capability::UiRuntimeServiceFamily::Portal,
                    1,
                    requirements,
                    fact_references,
                    mounted_work_references,
                )],
            );
            assert_eq!(
                super::preflight(candidate, &coherence, installed_support()).err(),
                Some(denial)
            );
        }

        let drifted = super::super::UiServiceProposalCandidate::for_test(
            1,
            super::super::UiServiceProposalDemand::recorded_fixture(participation(), 1, 1, 1),
            super::super::super::fixture_service_request_coherence(2),
            vec![family_proposal(
                crate::capability::UiRuntimeServiceFamily::Portal,
            )],
        );
        assert!(matches!(
            super::preflight(drifted, &coherence, installed_support()),
            Err(UiServiceProposalPreflightDenial::Coherence(_))
        ));
    }

    #[test]
    fn preflight_rejects_unsupported_duplicate_and_scope_widened_family_sets() {
        let coherence = super::super::super::fixture_service_request_coherence(3);
        let portal = family_proposal(crate::capability::UiRuntimeServiceFamily::Portal);
        let demand =
            super::super::UiServiceProposalDemand::recorded_fixture(participation(), 1, 1, 1);
        let unsupported = super::super::UiServiceProposalCandidate::for_test(
            3,
            demand,
            coherence.clone(),
            vec![portal],
        );
        assert_eq!(
            super::preflight(
                unsupported,
                &coherence,
                crate::capability::UiRuntimeServiceSupport::none_installed(),
            )
            .err(),
            Some(UiServiceProposalPreflightDenial::UnsupportedFamily(
                crate::capability::UiRuntimeServiceFamily::Portal,
            ))
        );

        let duplicate = super::super::UiServiceProposalCandidate::for_test(
            3,
            super::super::UiServiceProposalDemand::recorded_fixture(
                super::super::super::fixture_service_family_participation(2),
                2,
                2,
                2,
            ),
            coherence.clone(),
            vec![portal, portal],
        );
        assert_eq!(
            super::preflight(duplicate, &coherence, installed_support()).err(),
            Some(UiServiceProposalPreflightDenial::DuplicateFamily)
        );

        let widened = super::super::UiServiceProposalCandidate::for_test(
            3,
            demand,
            coherence.clone(),
            vec![family_proposal(
                crate::capability::UiRuntimeServiceFamily::Focus,
            )],
        );
        assert_eq!(
            super::preflight(widened, &coherence, installed_support()).err(),
            Some(UiServiceProposalPreflightDenial::FamilySetMismatch)
        );
    }

    #[test]
    fn cached_aggregate_cannot_hide_family_budget_and_leaves_no_residue() {
        let mut compiler = super::super::UiServiceProposalCompiler::new();
        let coherence = super::super::super::fixture_service_request_coherence(4);
        let candidate = super::super::UiServiceProposalCandidate::for_test(
            4,
            super::super::UiServiceProposalDemand::recorded_fixture(participation(), 1, 1, 1),
            coherence.clone(),
            vec![super::super::UiServiceFamilyProposal::recorded_fixture(
                crate::capability::UiRuntimeServiceFamily::Portal,
                1,
                1,
                65,
                1,
            )],
        );

        let before = compiler.census();
        assert_eq!(
            compiler
                .preflight(candidate, &coherence, installed_support())
                .err(),
            Some(UiServiceProposalPreflightDenial::AggregateDemandMismatch)
        );
        assert_eq!(compiler.census(), before);
        assert_eq!(compiler.live_occupancy_count(), 0);
        assert_eq!(compiler.live_cancellation_count(), 0);
    }

    #[test]
    fn family_aggregate_overflow_is_typed_and_leaves_no_residue() {
        let mut compiler = super::super::UiServiceProposalCompiler::new();
        let coherence = super::super::super::fixture_service_request_coherence(5);
        let participation = super::super::super::fixture_service_family_participation(2);
        let candidate = super::super::UiServiceProposalCandidate::for_test(
            5,
            super::super::UiServiceProposalDemand::recorded_fixture(participation, 1, 2, 2),
            coherence.clone(),
            vec![
                super::super::UiServiceFamilyProposal::recorded_fixture(
                    crate::capability::UiRuntimeServiceFamily::Portal,
                    1,
                    u8::MAX,
                    1,
                    1,
                ),
                super::super::UiServiceFamilyProposal::recorded_fixture(
                    crate::capability::UiRuntimeServiceFamily::Focus,
                    1,
                    1,
                    1,
                    1,
                ),
            ],
        );

        let before = compiler.census();
        assert_eq!(
            compiler
                .preflight(candidate, &coherence, installed_support())
                .err(),
            Some(UiServiceProposalPreflightDenial::AggregateDemandOverflow)
        );
        assert_eq!(compiler.census(), before);
        assert_eq!(compiler.live_occupancy_count(), 0);
        assert_eq!(compiler.live_cancellation_count(), 0);
    }

    fn participation() -> super::super::super::UiServiceFamilyParticipation {
        super::super::super::fixture_service_family_participation(1)
    }

    fn family_proposal(
        family: crate::capability::UiRuntimeServiceFamily,
    ) -> super::super::UiServiceFamilyProposal {
        super::super::UiServiceFamilyProposal::recorded_fixture(family, 1, 1, 1, 1)
    }

    fn installed_support() -> crate::capability::UiRuntimeServiceSupport {
        crate::capability::UiRuntimeServiceSupport::none_installed()
            .with_installed(crate::capability::UiRuntimeServiceFamily::Portal)
            .with_installed(crate::capability::UiRuntimeServiceFamily::Focus)
    }
}
