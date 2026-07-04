use crate::facade::evidence_lookup_route::EvidenceLookupRoutePacket;

use super::assembly_input::EvidenceLookupPublicCloseoutAssemblyInput;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SelectedEvidenceLookupPublicCloseoutRouteSupport {
    route_family_identity: String,
    stage_receipt_family_identity: String,
    selected_lookup_plan_digest: String,
    lookup_execution_receipt_digest: String,
    lookup_product_output_digest: String,
    compiled_product_identity_digest: String,
    equivalence_policy_identity_digest: String,
    selected_equivalence_family_identity: String,
    selected_reuse_basis_identity_digest: String,
}

impl SelectedEvidenceLookupPublicCloseoutRouteSupport {
    pub(crate) fn new(
        route_family_identity: String,
        stage_receipt_family_identity: String,
        selected_lookup_plan_digest: String,
        lookup_execution_receipt_digest: String,
        lookup_product_output_digest: String,
        compiled_product_identity_digest: String,
        equivalence_policy_identity_digest: String,
        selected_equivalence_family_identity: String,
        selected_reuse_basis_identity_digest: String,
    ) -> Self {
        Self {
            route_family_identity,
            stage_receipt_family_identity,
            selected_lookup_plan_digest,
            lookup_execution_receipt_digest,
            lookup_product_output_digest,
            compiled_product_identity_digest,
            equivalence_policy_identity_digest,
            selected_equivalence_family_identity,
            selected_reuse_basis_identity_digest,
        }
    }

    pub(crate) fn route_family_identity(&self) -> &str {
        &self.route_family_identity
    }
    pub(crate) fn stage_receipt_family_identity(&self) -> &str {
        &self.stage_receipt_family_identity
    }
    pub(crate) fn selected_lookup_plan_digest(&self) -> &str {
        &self.selected_lookup_plan_digest
    }
    pub(crate) fn lookup_execution_receipt_digest(&self) -> &str {
        &self.lookup_execution_receipt_digest
    }
    pub(crate) fn lookup_product_output_digest(&self) -> &str {
        &self.lookup_product_output_digest
    }
    pub(crate) fn compiled_product_identity_digest(&self) -> &str {
        &self.compiled_product_identity_digest
    }
    pub(crate) fn equivalence_policy_identity_digest(&self) -> &str {
        &self.equivalence_policy_identity_digest
    }
    pub(crate) fn selected_equivalence_family_identity(&self) -> &str {
        &self.selected_equivalence_family_identity
    }
    pub(crate) fn selected_reuse_basis_identity_digest(&self) -> &str {
        &self.selected_reuse_basis_identity_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AdmittedEvidenceLookupPublicCloseoutAssemblyInput {
    input: EvidenceLookupPublicCloseoutAssemblyInput,
}

impl AdmittedEvidenceLookupPublicCloseoutAssemblyInput {
    pub(crate) fn new(input: EvidenceLookupPublicCloseoutAssemblyInput) -> Self {
        Self { input }
    }
    pub(crate) fn assembly_input(&self) -> &EvidenceLookupPublicCloseoutAssemblyInput {
        &self.input
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupPublicCloseoutRouteInput {
    route_packet: EvidenceLookupRoutePacket,
    selected_route_support: SelectedEvidenceLookupPublicCloseoutRouteSupport,
    admitted_assembly_input: AdmittedEvidenceLookupPublicCloseoutAssemblyInput,
}

impl EvidenceLookupPublicCloseoutRouteInput {
    pub(crate) fn new(
        route_packet: EvidenceLookupRoutePacket,
        selected_route_support: SelectedEvidenceLookupPublicCloseoutRouteSupport,
        admitted_assembly_input: AdmittedEvidenceLookupPublicCloseoutAssemblyInput,
    ) -> Self {
        Self {
            route_packet,
            selected_route_support,
            admitted_assembly_input,
        }
    }

    pub fn route_packet(&self) -> &EvidenceLookupRoutePacket {
        &self.route_packet
    }
    pub fn selected_route_family_identity(&self) -> &str {
        self.selected_route_support.route_family_identity()
    }
    pub fn selected_compiled_product_identity_digest(&self) -> &str {
        self.selected_route_support
            .compiled_product_identity_digest()
    }
    pub fn selected_equivalence_family_identity(&self) -> &str {
        self.selected_route_support
            .selected_equivalence_family_identity()
    }
    pub fn selected_reuse_basis_identity_digest(&self) -> &str {
        self.selected_route_support
            .selected_reuse_basis_identity_digest()
    }
    pub(crate) fn selected_route_support(
        &self,
    ) -> &SelectedEvidenceLookupPublicCloseoutRouteSupport {
        &self.selected_route_support
    }
    pub(crate) fn assembly_input(&self) -> &EvidenceLookupPublicCloseoutAssemblyInput {
        self.admitted_assembly_input.assembly_input()
    }
    pub(crate) fn admitted_assembly_input(
        &self,
    ) -> &AdmittedEvidenceLookupPublicCloseoutAssemblyInput {
        &self.admitted_assembly_input
    }
}
