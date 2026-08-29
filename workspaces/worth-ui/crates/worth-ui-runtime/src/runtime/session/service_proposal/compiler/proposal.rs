#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct UiServiceProposalIdentity(super::super::UiServiceRequestIdentity);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct UiServiceProposalDemand {
    participating_families: super::super::UiServiceFamilyParticipation,
    requirements: u8,
    fact_references: u16,
    mounted_work_references: u16,
}

#[derive(Debug)]
pub(in crate::runtime) struct UiServiceProposalCandidate {
    identity: UiServiceProposalIdentity,
    coherence: super::super::UiServiceRequestCoherence,
    demand: UiServiceProposalDemand,
    family_proposals: Box<[super::UiServiceFamilyProposal]>,
}

impl UiServiceProposalIdentity {
    pub(super) const fn for_request(request: super::super::UiServiceRequestIdentity) -> Self {
        Self(request)
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(in crate::runtime) fn for_test(value: u64) -> Self {
        Self(super::super::UiServiceRequestIdentity::for_test(value))
    }

    pub(in crate::runtime) const fn diagnostic_value(self) -> u64 {
        self.0.diagnostic_value()
    }
}

impl UiServiceProposalDemand {
    #[cfg(any(test, feature = "certification-support"))]
    pub(in crate::runtime) const fn recorded_fixture(
        participating_families: super::super::UiServiceFamilyParticipation,
        requirements: u8,
        fact_references: u16,
        mounted_work_references: u16,
    ) -> Self {
        Self {
            participating_families,
            requirements,
            fact_references,
            mounted_work_references,
        }
    }

    pub(super) fn from_family_proposals(
        family_proposals: &[super::UiServiceFamilyProposal],
    ) -> Result<Self, UiServiceProposalDemandConstructionDenial> {
        let families = family_proposals
            .iter()
            .map(|proposal| proposal.family())
            .collect::<Vec<_>>();
        let participating_families =
            super::super::UiServiceFamilyParticipation::from_families(&families)
                .map_err(UiServiceProposalDemandConstructionDenial::Participation)?;
        let mut requirements = 0_u8;
        let mut fact_references = 0_u16;
        let mut mounted_work_references = 0_u16;
        for proposal in family_proposals {
            requirements = requirements
                .checked_add(proposal.requirements())
                .ok_or(UiServiceProposalDemandConstructionDenial::ArithmeticOverflow)?;
            fact_references = fact_references
                .checked_add(proposal.fact_references())
                .ok_or(UiServiceProposalDemandConstructionDenial::ArithmeticOverflow)?;
            mounted_work_references = mounted_work_references
                .checked_add(proposal.mounted_work_references())
                .ok_or(UiServiceProposalDemandConstructionDenial::ArithmeticOverflow)?;
        }
        Ok(Self {
            participating_families,
            requirements,
            fact_references,
            mounted_work_references,
        })
    }

    pub(in crate::runtime) const fn participating_families(
        self,
    ) -> super::super::UiServiceFamilyParticipation {
        self.participating_families
    }

    pub(in crate::runtime) const fn requirements(self) -> u8 {
        self.requirements
    }

    pub(in crate::runtime) const fn fact_references(self) -> u16 {
        self.fact_references
    }

    pub(in crate::runtime) const fn mounted_work_references(self) -> u16 {
        self.mounted_work_references
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiServiceProposalDemandConstructionDenial {
    Participation(super::super::UiServiceFamilyParticipationDenial),
    ArithmeticOverflow,
}

impl UiServiceProposalCandidate {
    #[cfg(any(test, feature = "certification-support"))]
    pub(super) fn for_test(
        identity: u64,
        demand: UiServiceProposalDemand,
        coherence: super::super::UiServiceRequestCoherence,
        family_proposals: Vec<super::UiServiceFamilyProposal>,
    ) -> Self {
        Self {
            identity: UiServiceProposalIdentity::for_test(identity),
            coherence,
            demand,
            family_proposals: family_proposals.into_boxed_slice(),
        }
    }

    pub(in crate::runtime) fn from_request<Authority>(
        request: &super::super::UiServiceRequestBasis<Authority>,
        family_proposals: Vec<super::UiServiceFamilyProposal>,
    ) -> Result<Self, UiServiceProposalDemandConstructionDenial>
    where
        Authority: super::super::UiServiceRequestOriginAuthority,
    {
        let demand = UiServiceProposalDemand::from_family_proposals(&family_proposals)?;
        Ok(Self {
            identity: UiServiceProposalIdentity::for_request(request.identity()),
            coherence: request.coherence(),
            demand,
            family_proposals: family_proposals.into_boxed_slice(),
        })
    }

    pub(in crate::runtime) const fn identity(&self) -> UiServiceProposalIdentity {
        self.identity
    }

    pub(in crate::runtime) const fn demand(&self) -> UiServiceProposalDemand {
        self.demand
    }

    pub(in crate::runtime) fn coherence(&self) -> &super::super::UiServiceRequestCoherence {
        &self.coherence
    }

    pub(in crate::runtime) fn family_proposals(&self) -> &[super::UiServiceFamilyProposal] {
        &self.family_proposals
    }

    pub(in crate::runtime) fn application(
        &self,
    ) -> &crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity {
        &self.coherence.axes().application
    }

    pub(in crate::runtime) fn surface(&self) -> super::super::UiServiceSurfaceBasis {
        super::super::UiServiceSurfaceBasis::from_coherence(&self.coherence)
    }

    pub(in crate::runtime) fn cancellation(&self) -> super::super::UiServiceCancellationIdentity {
        self.coherence.axes().cancellation
    }
}

#[cfg(test)]
mod tests {
    use super::{UiServiceProposalDemand, UiServiceProposalDemandConstructionDenial};

    #[test]
    fn proposal_demand_is_typed_and_bounded_by_preflight_not_dynamic_payloads() {
        let proposals = vec![
            fixture(crate::capability::UiRuntimeServiceFamily::Portal, 1, 2, 3),
            fixture(crate::capability::UiRuntimeServiceFamily::Focus, 2, 3, 4),
            fixture(crate::capability::UiRuntimeServiceFamily::Motion, 1, 1, 2),
        ];
        let demand = UiServiceProposalDemand::from_family_proposals(&proposals).unwrap();
        assert_eq!(demand.participating_families().count(), 3);
        assert_eq!(demand.requirements(), 4);
        assert_eq!(demand.fact_references(), 6);
        assert_eq!(demand.mounted_work_references(), 9);
    }

    #[test]
    fn duplicate_families_and_aggregate_overflow_deny_during_demand_construction() {
        let duplicate = vec![
            fixture(crate::capability::UiRuntimeServiceFamily::Portal, 1, 1, 1),
            fixture(crate::capability::UiRuntimeServiceFamily::Portal, 1, 1, 1),
        ];
        assert!(matches!(
            UiServiceProposalDemand::from_family_proposals(&duplicate),
            Err(UiServiceProposalDemandConstructionDenial::Participation(_))
        ));

        let overflow = vec![
            fixture(
                crate::capability::UiRuntimeServiceFamily::Portal,
                u8::MAX,
                1,
                1,
            ),
            fixture(crate::capability::UiRuntimeServiceFamily::Focus, 1, 1, 1),
        ];
        assert_eq!(
            UiServiceProposalDemand::from_family_proposals(&overflow),
            Err(UiServiceProposalDemandConstructionDenial::ArithmeticOverflow)
        );
    }

    fn fixture(
        family: crate::capability::UiRuntimeServiceFamily,
        requirements: u8,
        fact_references: u16,
        mounted_work_references: u16,
    ) -> super::super::UiServiceFamilyProposal {
        super::super::UiServiceFamilyProposal::recorded_fixture(
            family,
            1,
            requirements,
            fact_references,
            mounted_work_references,
        )
    }
}
