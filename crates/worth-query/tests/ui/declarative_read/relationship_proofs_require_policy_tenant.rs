use worth_query::facade::read::{
    current, WorthQueryReadRelationshipProofs,
};

fn relationship_before_policy(proofs: WorthQueryReadRelationshipProofs) {
    let _context = current().with_relationship_proofs(proofs);
}

fn main() {}
