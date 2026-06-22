use forge_query::facade::ForgeQueryExistingTruthTargetBinding;

fn main() {}

fn removed_existing_truth_binding_collection_alias(binding: &ForgeQueryExistingTruthTargetBinding) {
    let _ = binding.target_collection();
}
