mod current_milestone_nine;
mod declaration_row;

use crate::projection::touched_graph_parity_closeout::{
    TopologyTouchedGraphParityCoverageContributor, TopologyTouchedGraphParityCoverageError,
    TopologyTouchedGraphParityQuerySurfaceKind,
};
use current_milestone_nine::current_projection_validator_invariant_milestone_nine_closeout;

pub use declaration_row::current_topology_validator_invariant_declaration_row;

pub const TOPOLOGY_VALIDATOR_INVARIANT_CLAIM_PATH: &str =
    "crates/worth-topo/src/projection/touched_graph_parity_closeout/validator_invariant_family/mod.rs";
pub const TOPOLOGY_VALIDATOR_INVARIANT_REPLACEMENT_LANE: &str =
    "crates/worth-topo/src/projection/touched_graph_parity_closeout/validator_invariant_family/";

const SOURCE_PATH: &str =
    "crates/worth-topo/src/validator_invariant_catalog/milestone_nine_closeout/current.rs";
const SELECTED_FIELDS: &[&str] = &[
    "public_proof.query_selected_obligation_digests",
    "public_proof.enforcement_receipt_digests",
    "public_proof.execution_proof_digest",
    "milestone_ten_seed.seed_digest",
];

pub fn current_topology_validator_invariant_coverage_contributor(
) -> Result<TopologyTouchedGraphParityCoverageContributor, TopologyTouchedGraphParityCoverageError>
{
    let closeout = current_projection_validator_invariant_milestone_nine_closeout()
        .map_err(|error| TopologyTouchedGraphParityCoverageError::new(format!("{error:?}")))?;
    if closeout
        .public_proof()
        .query_selected_obligation_digests()
        .is_empty()
        || closeout
            .public_proof()
            .enforcement_receipt_digests()
            .is_empty()
        || closeout.public_proof().execution_proof_digest().is_empty()
        || closeout.milestone_ten_seed().seed_digest().is_empty()
    {
        return Err(TopologyTouchedGraphParityCoverageError::new(
            "validator/invariant coverage contributor requires a live milestone-nine routing product with selected obligations, enforcement receipts, execution proof, and milestone-ten seed",
        ));
    }

    Ok(TopologyTouchedGraphParityCoverageContributor::new(
        "current_topology_validator_invariant_milestone_nine_closeout",
        SOURCE_PATH,
        "current_topology_validator_invariant_milestone_nine_closeout",
        "current_topology_validator_invariant_milestone_nine_closeout",
        "family_route_product",
        TOPOLOGY_VALIDATOR_INVARIANT_REPLACEMENT_LANE,
        SELECTED_FIELDS,
        TopologyTouchedGraphParityQuerySurfaceKind::SupportPosture,
        "current_topology_validator_invariant_milestone_nine_closeout",
        SOURCE_PATH,
    ))
}
