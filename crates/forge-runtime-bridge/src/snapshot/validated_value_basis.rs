use forge_foundational::facade::{
    admit_authoritative_record_aspect_state, canonicalization, CanonicalizationRuleVersion,
    ContractValidatedAspectArtifact,
};
use forge_proof::TransitionOutcome;

use crate::canonical_basis::canonical_basis_ready_text;

const SNAPSHOT_READ_VALUE_CANONICAL_VERSION: &str = "bridge.snapshot-read-value.v1";

pub(crate) fn validated_snapshot_read_value_canonical_basis(
    validated_value: &ContractValidatedAspectArtifact,
) -> String {
    let TransitionOutcome::Success(admitted_state) =
        admit_authoritative_record_aspect_state([validated_value.clone()])
    else {
        unreachable!("single contract-validated snapshot read value admits into state");
    };

    let TransitionOutcome::Success(ready_basis) = canonicalization()
        .basis()
        .at(snapshot_read_value_canonical_version())
        .from_state(admitted_state)
    else {
        unreachable!("single admitted snapshot read value has a canonical state basis");
    };

    canonical_basis_ready_text(&ready_basis)
        .expect("bridge canonical basis renderer supports foundational state value basis")
}

fn snapshot_read_value_canonical_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new(SNAPSHOT_READ_VALUE_CANONICAL_VERSION)
        .expect("snapshot read value canonical version is a valid foundational rule version")
}
