use std::collections::BTreeSet;

use super::PlanarBoolean7_0CloseoutError;

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub(super) struct PlanarBoolean7_0BoundaryClaims {
    readiness_basis_digests: Vec<String>,
    declaration_digests: Vec<String>,
    operand_pair_identities: Vec<String>,
    blocker_digests: Vec<String>,
    pair_construction_digests: Vec<String>,
}

impl PlanarBoolean7_0BoundaryClaims {
    pub(super) fn record_readiness_basis_digest(&mut self, digest: &str) {
        self.readiness_basis_digests.push(digest.to_string());
    }

    pub(super) fn record_declaration_digest(&mut self, digest: &str) {
        self.declaration_digests.push(digest.to_string());
    }

    pub(super) fn record_operand_pair_identity(&mut self, identity: &str) {
        self.operand_pair_identities.push(identity.to_string());
    }

    pub(super) fn record_blocker_digest(&mut self, digest: &str) {
        self.blocker_digests.push(digest.to_string());
    }

    pub(super) fn record_pair_construction_digest(&mut self, digest: &str) {
        self.pair_construction_digests.push(digest.to_string());
    }

    pub(super) fn validate(&self) -> Result<(), PlanarBoolean7_0CloseoutError> {
        require_single_value("readiness basis boundary", &self.readiness_basis_digests)?;
        require_single_value("declaration boundary", &self.declaration_digests)?;
        require_single_value("operand pair identity", &self.operand_pair_identities)?;
        require_single_value("blocker provenance boundary", &self.blocker_digests)?;
        require_single_value(
            "operand pair construction boundary",
            &self.pair_construction_digests,
        )?;
        Ok(())
    }
}

fn require_single_value(
    boundary: &'static str,
    values: &[String],
) -> Result<(), PlanarBoolean7_0CloseoutError> {
    let distinct: BTreeSet<&str> = values.iter().map(String::as_str).collect();
    if distinct.len() > 1 {
        return Err(PlanarBoolean7_0CloseoutError::MismatchedProofBoundary(
            boundary,
        ));
    }
    Ok(())
}
