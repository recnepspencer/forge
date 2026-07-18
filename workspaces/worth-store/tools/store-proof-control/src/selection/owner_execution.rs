use super::{ProofExecutionUnit, ProofProductUnavailable, StoreProofMode, StoreProofRequest};

pub(super) fn validate_owner_execution_locality(
    request: &StoreProofRequest,
    units: &[ProofExecutionUnit],
) -> Result<(), ProofProductUnavailable> {
    if request.mode() != StoreProofMode::Owner {
        return Ok(());
    }
    let owner = request
        .package()
        .expect("owner mode was validated before execution-unit lowering");
    for unit in units {
        if unit.package != owner {
            return Err(ProofProductUnavailable::OwnerBoundaryViolation {
                owner: owner.to_owned(),
                reached_target: format!("{}::{}", unit.package, unit.target_name),
            });
        }
    }
    Ok(())
}
