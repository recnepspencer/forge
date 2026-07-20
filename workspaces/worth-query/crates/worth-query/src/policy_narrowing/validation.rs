use crate::authorized_projection::PolicyInfluenceSet;
use crate::canonicalization::CanonicalQueryBundle;
use crate::relationship_proof::RelationshipProofDescriptorSet;

use super::{
    PolicyNarrowingCounters, PolicyNarrowingError, PolicyNarrowingFailureClass,
    PolicyNarrowingWorkBudget,
};

pub(crate) fn validate_narrowing_budget(
    canonical: &CanonicalQueryBundle,
    influence: &PolicyInfluenceSet,
    descriptors: &RelationshipProofDescriptorSet,
    budget: &PolicyNarrowingWorkBudget,
) -> Result<(), PolicyNarrowingError> {
    let field_references = canonical.query().projection().len()
        + canonical.query().predicates().len()
        + canonical.query().ordering().len()
        + canonical.result_shape().fields().len()
        + influence.entries().len();
    if field_references > budget.max_field_references() {
        return Err(PolicyNarrowingError::new(
            PolicyNarrowingFailureClass::UnboundedDerivedInfluence,
            "canonical query field reference count exceeds narrowing budget",
            PolicyNarrowingCounters::denied_unknown_cost(),
        ));
    }

    if descriptors.descriptors().len() > budget.max_relationship_descriptors() {
        return Err(PolicyNarrowingError::new(
            PolicyNarrowingFailureClass::UnboundedProofTopology,
            "relationship proof descriptor count exceeds narrowing budget",
            PolicyNarrowingCounters::denied_unknown_cost(),
        ));
    }

    let mut topology_width = 0usize;
    for descriptor in descriptors.descriptors() {
        let Some(width) = descriptor.topology_width() else {
            continue;
        };
        topology_width += width;
    }
    if topology_width > budget.max_relationship_topology_width() {
        return Err(PolicyNarrowingError::new(
            PolicyNarrowingFailureClass::UnboundedProofTopology,
            "relationship proof topology width exceeds narrowing budget",
            PolicyNarrowingCounters::denied_unknown_cost(),
        ));
    }

    let digest_part_count = 8
        + field_references
        + descriptors.descriptors().len()
        + canonical.query().predicates().len()
        + canonical.query().ordering().len()
        + influence.digest_parts().len();
    if digest_part_count > budget.max_digest_part_count() {
        return Err(PolicyNarrowingError::new(
            PolicyNarrowingFailureClass::DigestPartBudgetExceeded,
            "policy narrowing digest part count exceeds declared budget",
            PolicyNarrowingCounters::denied_unknown_cost(),
        ));
    }

    Ok(())
}
