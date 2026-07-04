#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KernelCompiledProductProofBasis {
    source_authority_basis: &'static str,
    locality_footprint_basis: &'static str,
    prior_proof_basis: &'static str,
    evidence_support_basis: &'static str,
    equivalence_policy_identity: &'static str,
}

impl KernelCompiledProductProofBasis {
    pub const fn new(
        source_authority_basis: &'static str,
        locality_footprint_basis: &'static str,
        prior_proof_basis: &'static str,
        evidence_support_basis: &'static str,
        equivalence_policy_identity: &'static str,
    ) -> Self {
        Self {
            source_authority_basis,
            locality_footprint_basis,
            prior_proof_basis,
            evidence_support_basis,
            equivalence_policy_identity,
        }
    }

    pub const fn source_authority_basis(&self) -> &'static str {
        self.source_authority_basis
    }

    pub const fn locality_footprint_basis(&self) -> &'static str {
        self.locality_footprint_basis
    }

    pub const fn prior_proof_basis(&self) -> &'static str {
        self.prior_proof_basis
    }

    pub const fn evidence_support_basis(&self) -> &'static str {
        self.evidence_support_basis
    }

    pub const fn equivalence_policy_identity(&self) -> &'static str {
        self.equivalence_policy_identity
    }
}
