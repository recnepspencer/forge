use worth_query::facade::policy::{RelationshipProofBudget, RelationshipProofDescriptorSet};

fn main() {
    let _proofs = RelationshipProofDescriptorSet::new(
        vec![true],
        RelationshipProofBudget::bounded(1, 1),
    );
}
