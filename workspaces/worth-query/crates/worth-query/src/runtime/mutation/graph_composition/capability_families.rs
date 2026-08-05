pub(crate) const GRAPH_COMPOSITION_TARGET_COMBINATION_FAMILIES: &[&str] = &[
    "same_batch_entity_relation_identity_edges",
    "mixed_existing_and_symbolic_entity_identity_edges",
];

pub(crate) const GRAPH_COMPOSITION_LIFECYCLE_FAMILIES: &[&str] = &[
    "same_batch_symbolic_entity_followup_mutation",
    "same_batch_symbolic_relation_followup_mutation",
    "same_batch_symbolic_relation_retirement",
    "mixed_existing_target_followup_mutation",
    "mixed_existing_target_retarget",
    "mixed_existing_target_supersession",
    "mixed_existing_target_retirement",
    "mixed_existing_target_verified_followup_mutation",
    "mixed_existing_target_verified_retarget",
    "mixed_existing_target_verified_supersession",
    "mixed_existing_target_verified_retirement",
];

pub(crate) const GRAPH_COMPOSITION_EXTENSION_HOOK_FAMILIES: &[&str] =
    &["domain_lowering_hook", "domain_interpretation_hook"];
