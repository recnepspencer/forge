use worth_query::facade::WorthQueryExistingTruthTargetBinding;

fn main() {}

fn removed_existing_truth_binding_collection_alias(binding: &WorthQueryExistingTruthTargetBinding) {
    let _ = binding.target_collection();
}
