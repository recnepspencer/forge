use worth_foundational::facade::{
    CanonicalDigestDerivationDenial, CanonicalDigestId, CanonicalDigestWorkBudget,
};
use worth_query_declaration::facade::application_query::{
    ApplicationQueryOrderingDirection, ApplicationQueryResultTraversalDirection,
};

use super::{digest, prepare_artifact, text, WorthQueryApplicationCanonicalArtifact};
use crate::application_query::WorthQueryInstalledGraphOrdering;

pub(in crate::application_query) struct ContinuationCanonicalInput<'a> {
    pub graph_digest: &'a CanonicalDigestId,
    pub collection_path: &'a str,
    pub slot_type: &'a str,
    pub relation: &'a str,
    pub parent_entity: &'a str,
    pub child_entity: &'a str,
    pub direction: ApplicationQueryResultTraversalDirection,
    pub ordering: &'a [WorthQueryInstalledGraphOrdering],
}

pub(in crate::application_query) fn prepare_continuation_basis(
    input: &ContinuationCanonicalInput<'_>,
    budget: CanonicalDigestWorkBudget,
) -> Result<WorthQueryApplicationCanonicalArtifact, CanonicalDigestDerivationDenial> {
    let mut entries = vec![
        digest("graph", input.graph_digest),
        text("collection-path", input.collection_path),
        text("slot-type", input.slot_type),
        text("relation", input.relation),
        text("parent-entity", input.parent_entity),
        text("child-entity", input.child_entity),
        text("direction", direction_name(input.direction)),
    ];
    for (index, ordering) in input.ordering.iter().enumerate() {
        let path = format!("ordering[{index}]");
        entries.extend([
            text(format!("{path}.result-path"), ordering.result_path()),
            text(format!("{path}.slot-type"), ordering.slot_type()),
            text(
                format!("{path}.direction"),
                ordering_direction_name(ordering.direction()),
            ),
        ]);
    }
    prepare_artifact("continuation", entries, budget)
}

const fn direction_name(direction: ApplicationQueryResultTraversalDirection) -> &'static str {
    match direction {
        ApplicationQueryResultTraversalDirection::Forward => "forward",
        ApplicationQueryResultTraversalDirection::Reverse => "reverse",
    }
}

const fn ordering_direction_name(direction: ApplicationQueryOrderingDirection) -> &'static str {
    match direction {
        ApplicationQueryOrderingDirection::Ascending => "ascending",
        ApplicationQueryOrderingDirection::Descending => "descending",
    }
}
