use crate::data::authority::{
    PlannerPublicProofIdentity, PlannerSelectedFamilyIdentity, PlannerSelectedProductIdentity,
    PlannerSelectedRouteIdentity, PlannerWitnessIdentity,
};

use super::family_kind::TouchedGraphParityFamilyKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TouchedGraphParityClaimKind {
    DeclareOnceFamilyParity,
    SelectedRouteParity,
    PublicProjectionParity,
    ReadinessParity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TouchedGraphParityArchitectureClaim {
    kind: TouchedGraphParityClaimKind,
    family_kind: TouchedGraphParityFamilyKind,
    selected_route_identity: PlannerSelectedRouteIdentity,
    selected_family_identity: PlannerSelectedFamilyIdentity,
    selected_product_identity: Option<PlannerSelectedProductIdentity>,
    witness_identity: Option<PlannerWitnessIdentity>,
    public_proof_identity: Option<PlannerPublicProofIdentity>,
}

impl TouchedGraphParityArchitectureClaim {
    pub fn declare_once_family_parity(
        family_kind: TouchedGraphParityFamilyKind,
        selected_route_identity: PlannerSelectedRouteIdentity,
        selected_family_identity: PlannerSelectedFamilyIdentity,
        selected_product_identity: Option<PlannerSelectedProductIdentity>,
        witness_identity: Option<PlannerWitnessIdentity>,
    ) -> Self {
        Self {
            kind: TouchedGraphParityClaimKind::DeclareOnceFamilyParity,
            family_kind,
            selected_route_identity,
            selected_family_identity,
            selected_product_identity,
            witness_identity,
            public_proof_identity: None,
        }
    }

    pub fn selected_route_parity(
        family_kind: TouchedGraphParityFamilyKind,
        selected_route_identity: PlannerSelectedRouteIdentity,
        selected_family_identity: PlannerSelectedFamilyIdentity,
        selected_product_identity: Option<PlannerSelectedProductIdentity>,
        witness_identity: Option<PlannerWitnessIdentity>,
    ) -> Self {
        Self {
            kind: TouchedGraphParityClaimKind::SelectedRouteParity,
            family_kind,
            selected_route_identity,
            selected_family_identity,
            selected_product_identity,
            witness_identity,
            public_proof_identity: None,
        }
    }

    pub fn public_projection_parity(
        family_kind: TouchedGraphParityFamilyKind,
        selected_route_identity: PlannerSelectedRouteIdentity,
        selected_family_identity: PlannerSelectedFamilyIdentity,
        selected_product_identity: PlannerSelectedProductIdentity,
        public_proof_identity: PlannerPublicProofIdentity,
    ) -> Self {
        Self {
            kind: TouchedGraphParityClaimKind::PublicProjectionParity,
            family_kind,
            selected_route_identity,
            selected_family_identity,
            selected_product_identity: Some(selected_product_identity),
            witness_identity: None,
            public_proof_identity: Some(public_proof_identity),
        }
    }

    /// ```compile_fail
    /// use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityArchitectureClaim;
    ///
    /// let _constructor = TouchedGraphParityArchitectureClaim::readiness_parity;
    /// ```
    #[cfg(any(test, feature = "touched-graph-parity-internal-authority"))]
    pub(crate) fn readiness_parity(
        family_kind: TouchedGraphParityFamilyKind,
        selected_route_identity: PlannerSelectedRouteIdentity,
        selected_family_identity: PlannerSelectedFamilyIdentity,
        selected_product_identity: PlannerSelectedProductIdentity,
        witness_identity: PlannerWitnessIdentity,
        public_proof_identity: PlannerPublicProofIdentity,
    ) -> Self {
        Self {
            kind: TouchedGraphParityClaimKind::ReadinessParity,
            family_kind,
            selected_route_identity,
            selected_family_identity,
            selected_product_identity: Some(selected_product_identity),
            witness_identity: Some(witness_identity),
            public_proof_identity: Some(public_proof_identity),
        }
    }

    pub const fn kind(&self) -> TouchedGraphParityClaimKind {
        self.kind
    }

    pub const fn family_kind(&self) -> TouchedGraphParityFamilyKind {
        self.family_kind
    }

    pub fn selected_route_identity(&self) -> &PlannerSelectedRouteIdentity {
        &self.selected_route_identity
    }

    pub fn selected_family_identity(&self) -> &PlannerSelectedFamilyIdentity {
        &self.selected_family_identity
    }

    pub fn selected_product_identity(&self) -> Option<&PlannerSelectedProductIdentity> {
        self.selected_product_identity.as_ref()
    }

    pub fn witness_identity(&self) -> Option<&PlannerWitnessIdentity> {
        self.witness_identity.as_ref()
    }

    pub fn public_proof_identity(&self) -> Option<&PlannerPublicProofIdentity> {
        self.public_proof_identity.as_ref()
    }
}

#[cfg(any(test, feature = "touched-graph-parity-internal-authority"))]
pub(crate) fn admit_touched_graph_parity_readiness_claim(
    family_kind: TouchedGraphParityFamilyKind,
    selected_route_identity: PlannerSelectedRouteIdentity,
    selected_family_identity: PlannerSelectedFamilyIdentity,
    selected_product_identity: PlannerSelectedProductIdentity,
    witness_identity: PlannerWitnessIdentity,
    public_proof_identity: PlannerPublicProofIdentity,
) -> TouchedGraphParityArchitectureClaim {
    TouchedGraphParityArchitectureClaim::readiness_parity(
        family_kind,
        selected_route_identity,
        selected_family_identity,
        selected_product_identity,
        witness_identity,
        public_proof_identity,
    )
}
