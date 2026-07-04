use super::closeout::{TopologyQueryBackedConsumerCutover, TopologyQueryBackedConsumerFamilyRow};
#[cfg(any(test, feature = "test-support-lowering"))]
use super::selected_route_authority::{
    require_selected_route_authority_matches, TopologyQueryBackedReadFamilySelectedRouteAuthority,
};
use super::{
    require_optional_match, require_string_match, TopologyQueryBackedReadFamilyAdmissionError,
    TopologyQueryBackedReadFamilyRouteInput,
};
use crate::derived_topology::compiled_product_consumer_cutover::DerivedEquivalenceContractReport;
use crate::projection::read_views::domain::TopologyCurrentHeadReadSession;
use crate::query_domain::TopologyCurrentHeadQueryBasisEvidence;
use crate::selected_equivalence_family::TopologySelectedEquivalenceFamilyIdentity;
use schema::facade::platform::authority::compiled_product_semantic_graph::{
    admit_compiled_product_rebuild_denial_identity, CompiledProductEquivalencePolicyIdentity,
    CompiledProductIdentity, CompiledProductRebuildDenialIdentity,
    CompiledProductReuseDecisionIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TopologyQueryBackedReadFamilyAdmissionAuthority {
    handle_identity_digest: String,
    support_snapshot_digest: String,
    operating_context_identity_digest: String,
    parity_verified_count: usize,
    compiled_product_identity: Option<CompiledProductIdentity>,
    equivalence_policy_identity: Option<CompiledProductEquivalencePolicyIdentity>,
    selected_equivalence_family_identity: Option<TopologySelectedEquivalenceFamilyIdentity>,
    selected_equivalence_basis_identity_digest: Option<String>,
    selected_compatibility_basis_identity_digest: Option<String>,
    selected_reuse_basis_identity_digest: Option<String>,
    reuse_decision_identity: Option<CompiledProductReuseDecisionIdentity>,
    rebuild_denial_identity: Option<CompiledProductRebuildDenialIdentity>,
}

pub fn admit_topology_query_backed_consumer_cutover(
    session: &TopologyCurrentHeadReadSession<'_>,
    basis_evidence: &TopologyCurrentHeadQueryBasisEvidence,
    equivalence_contract: &DerivedEquivalenceContractReport,
) -> TopologyQueryBackedConsumerCutover {
    let route_input =
        TopologyQueryBackedReadFamilyRouteInput::new(session, basis_evidence, equivalence_contract);
    admit_topology_query_backed_read_family_route(&route_input).expect(
        "query-backed route admission built mismatched authority from the same admitted input",
    )
}

pub(crate) fn admit_topology_query_backed_read_family_route(
    route_input: &TopologyQueryBackedReadFamilyRouteInput<'_>,
) -> Result<TopologyQueryBackedConsumerCutover, TopologyQueryBackedReadFamilyAdmissionError> {
    let admitted_authority =
        TopologyQueryBackedReadFamilyAdmissionAuthority::from_route_input(route_input);
    admitted_authority.require_matches(route_input)?;
    let family_rows = route_input
        .observed_family_rows()
        .iter()
        .map(|row| {
            TopologyQueryBackedConsumerFamilyRow::from_observed_route_row(row, &admitted_authority)
        })
        .collect::<Vec<_>>();
    Ok(TopologyQueryBackedConsumerCutover::new(
        admitted_authority.handle_identity_digest.clone(),
        admitted_authority.operating_context_identity_digest.clone(),
        admitted_authority.support_snapshot_digest.clone(),
        route_input.query_executed_debt_free_family_count(),
        route_input.debt_family_count(),
        admitted_authority.parity_verified_count,
        family_rows,
    ))
}

#[cfg(any(test, feature = "test-support-lowering"))]
pub(crate) fn admit_topology_query_backed_read_family_route_with_selected_route_authority<
    A: TopologyQueryBackedReadFamilySelectedRouteAuthority,
>(
    route_input: &TopologyQueryBackedReadFamilyRouteInput<'_>,
    authority: &A,
) -> Result<TopologyQueryBackedConsumerCutover, TopologyQueryBackedReadFamilyAdmissionError> {
    require_selected_route_authority_matches(route_input, authority)?;
    let admitted_authority =
        TopologyQueryBackedReadFamilyAdmissionAuthority::from_route_input(route_input);
    let family_rows = route_input
        .observed_family_rows()
        .iter()
        .map(|row| {
            TopologyQueryBackedConsumerFamilyRow::from_observed_route_row(row, &admitted_authority)
        })
        .collect::<Vec<_>>();
    Ok(TopologyQueryBackedConsumerCutover::new(
        admitted_authority.handle_identity_digest.clone(),
        admitted_authority.operating_context_identity_digest.clone(),
        admitted_authority.support_snapshot_digest.clone(),
        route_input.query_executed_debt_free_family_count(),
        route_input.debt_family_count(),
        admitted_authority.parity_verified_count,
        family_rows,
    ))
}

impl TopologyQueryBackedReadFamilyAdmissionAuthority {
    pub(crate) fn from_route_input(
        route_input: &TopologyQueryBackedReadFamilyRouteInput<'_>,
    ) -> Self {
        let equivalence_contract = route_input.equivalence_contract();
        Self {
            handle_identity_digest: route_input.handle_identity_digest().to_string(),
            support_snapshot_digest: route_input.support_snapshot_digest().to_string(),
            operating_context_identity_digest: route_input
                .operating_context_identity_digest()
                .to_string(),
            parity_verified_count: route_input.parity_verified_count(),
            compiled_product_identity: equivalence_contract
                .compiled_product_identity_ref()
                .cloned(),
            equivalence_policy_identity: equivalence_contract
                .equivalence_policy_identity_ref()
                .cloned(),
            selected_equivalence_family_identity: equivalence_contract
                .selected_equivalence_family_identity(),
            selected_equivalence_basis_identity_digest: equivalence_contract
                .selected_equivalence_basis_identity_digest()
                .map(str::to_string),
            selected_compatibility_basis_identity_digest: equivalence_contract
                .selected_compatibility_basis_identity_digest()
                .map(str::to_string),
            selected_reuse_basis_identity_digest: equivalence_contract
                .selected_reuse_basis_identity_digest()
                .map(str::to_string),
            reuse_decision_identity: equivalence_contract.reuse_decision_identity_ref().cloned(),
            rebuild_denial_identity: None,
        }
    }

    pub(super) fn compiled_product_identity_for_admission(
        &self,
    ) -> Option<&CompiledProductIdentity> {
        self.compiled_product_identity.as_ref()
    }

    pub(super) fn equivalence_policy_identity_for_admission(
        &self,
    ) -> Option<&CompiledProductEquivalencePolicyIdentity> {
        self.equivalence_policy_identity.as_ref()
    }

    pub(super) fn compiled_product_digest_for_admission(&self) -> Option<&str> {
        self.compiled_product_identity
            .as_ref()
            .map(|identity| identity.identity_digest())
    }

    pub(super) fn equivalence_policy_digest_for_admission(&self) -> Option<&str> {
        self.equivalence_policy_identity
            .as_ref()
            .map(|identity| identity.identity_digest())
    }

    pub(super) fn selected_equivalence_family_for_admission(
        &self,
    ) -> Option<TopologySelectedEquivalenceFamilyIdentity> {
        self.selected_equivalence_family_identity
    }

    pub(super) fn selected_equivalence_basis_digest_for_admission(&self) -> Option<&str> {
        self.selected_equivalence_basis_identity_digest.as_deref()
    }

    pub(super) fn selected_compatibility_basis_digest_for_admission(&self) -> Option<&str> {
        self.selected_compatibility_basis_identity_digest.as_deref()
    }

    pub(super) fn selected_reuse_basis_digest_for_admission(&self) -> Option<&str> {
        self.selected_reuse_basis_identity_digest.as_deref()
    }

    pub(super) fn reuse_decision_identity_for_admission(
        &self,
    ) -> Option<&CompiledProductReuseDecisionIdentity> {
        self.reuse_decision_identity.as_ref()
    }

    pub(super) fn rebuild_required_identity(
        &self,
        compiled_product_identity: &CompiledProductIdentity,
        denial_reason: &str,
    ) -> CompiledProductRebuildDenialIdentity {
        admit_compiled_product_rebuild_denial_identity(compiled_product_identity, denial_reason)
            .expect("static query-backed rebuild denial reason should admit")
    }

    fn require_matches(
        &self,
        route_input: &TopologyQueryBackedReadFamilyRouteInput<'_>,
    ) -> Result<(), TopologyQueryBackedReadFamilyAdmissionError> {
        require_string_match(
            "query handle identity",
            route_input.handle_identity_digest(),
            &self.handle_identity_digest,
        )?;
        require_string_match(
            "query support snapshot",
            route_input.support_snapshot_digest(),
            &self.support_snapshot_digest,
        )?;
        require_string_match(
            "query operating context",
            route_input.operating_context_identity_digest(),
            &self.operating_context_identity_digest,
        )?;
        require_optional_match(
            "compiled product identity",
            route_input
                .equivalence_contract()
                .compiled_product_identity_digest(),
            self.compiled_product_digest_for_admission(),
        )?;
        require_optional_match(
            "equivalence policy identity",
            route_input
                .equivalence_contract()
                .equivalence_policy_identity_digest(),
            self.equivalence_policy_digest_for_admission(),
        )?;
        require_optional_match(
            "selected equivalence family identity",
            route_input
                .equivalence_contract()
                .selected_equivalence_family_identity()
                .map(TopologySelectedEquivalenceFamilyIdentity::as_str),
            self.selected_equivalence_family_identity
                .map(|identity| identity.as_str()),
        )?;
        require_optional_match(
            "selected equivalence basis identity",
            route_input
                .equivalence_contract()
                .selected_equivalence_basis_identity_digest(),
            self.selected_equivalence_basis_identity_digest.as_deref(),
        )?;
        require_optional_match(
            "selected compatibility basis identity",
            route_input
                .equivalence_contract()
                .selected_compatibility_basis_identity_digest(),
            self.selected_compatibility_basis_identity_digest.as_deref(),
        )?;
        require_optional_match(
            "selected reuse basis identity",
            route_input
                .equivalence_contract()
                .selected_reuse_basis_identity_digest(),
            self.selected_reuse_basis_identity_digest.as_deref(),
        )?;
        require_optional_match(
            "reuse decision identity",
            route_input
                .equivalence_contract()
                .reuse_decision_identity_ref()
                .map(CompiledProductReuseDecisionIdentity::identity_digest),
            self.reuse_decision_identity
                .as_ref()
                .map(|identity| identity.identity_digest()),
        )?;
        require_optional_match(
            "rebuild denial identity",
            None,
            self.rebuild_denial_identity
                .as_ref()
                .map(|identity| identity.identity_digest()),
        )?;
        if route_input.parity_verified_count() != self.parity_verified_count {
            return Err(TopologyQueryBackedReadFamilyAdmissionError::new(format!(
                "query-backed route admission rejected mismatched parity verification count: expected {}, observed {}",
                self.parity_verified_count,
                route_input.parity_verified_count()
            )));
        }
        Ok(())
    }

    #[cfg(any(test, feature = "test-support-lowering"))]
    pub(crate) fn with_support_snapshot_digest(mut self, support_snapshot_digest: &str) -> Self {
        self.support_snapshot_digest = support_snapshot_digest.to_string();
        self
    }
}

#[cfg(any(test, feature = "test-support-lowering"))]
impl TopologyQueryBackedReadFamilySelectedRouteAuthority
    for TopologyQueryBackedReadFamilyAdmissionAuthority
{
    fn topology_query_handle_identity_digest(&self) -> &str {
        &self.handle_identity_digest
    }

    fn topology_query_support_snapshot_digest(&self) -> &str {
        &self.support_snapshot_digest
    }

    fn topology_query_operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
    }

    fn topology_query_parity_verified_count(&self) -> usize {
        self.parity_verified_count
    }

    fn topology_query_compiled_product_identity_digest(&self) -> Option<&str> {
        self.compiled_product_digest_for_admission()
    }

    fn topology_query_equivalence_policy_identity_digest(&self) -> Option<&str> {
        self.equivalence_policy_digest_for_admission()
    }

    fn topology_query_selected_equivalence_family_identity(&self) -> Option<&str> {
        self.selected_equivalence_family_identity
            .map(|identity| identity.as_str())
    }

    fn topology_query_selected_equivalence_basis_identity_digest(&self) -> Option<&str> {
        self.selected_equivalence_basis_identity_digest.as_deref()
    }

    fn topology_query_selected_compatibility_basis_identity_digest(&self) -> Option<&str> {
        self.selected_compatibility_basis_identity_digest.as_deref()
    }

    fn topology_query_selected_reuse_basis_identity_digest(&self) -> Option<&str> {
        self.selected_reuse_basis_identity_digest.as_deref()
    }

    fn topology_query_reuse_decision_identity_digest(&self) -> Option<&str> {
        self.reuse_decision_identity
            .as_ref()
            .map(|identity| identity.identity_digest())
    }

    fn topology_query_rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.rebuild_denial_identity
            .as_ref()
            .map(|identity| identity.identity_digest())
    }
}
