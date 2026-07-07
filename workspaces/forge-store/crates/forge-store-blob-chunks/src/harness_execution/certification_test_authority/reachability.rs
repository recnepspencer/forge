use crate::{
    BlobChunkReachabilityProofSet, BlobChunkReachabilityRegistry, BlobLifecycleDeclaration,
};

use super::super::chunk_sequence::GeneratedBlobSequence;

pub(in crate::harness_execution) fn lifecycle_multichunk_reachability(
    declaration: &BlobLifecycleDeclaration,
    generated: &GeneratedBlobSequence,
) -> BlobChunkReachabilityProofSet {
    let mut registry = BlobChunkReachabilityRegistry::new_store_owned();
    registry
        .admit_lifecycle_multichunk_primary_references(
            declaration,
            generated.sequence.proof_frontier().ordered_leaves(),
        )
        .expect("lifecycle multichunk reachability")
}
