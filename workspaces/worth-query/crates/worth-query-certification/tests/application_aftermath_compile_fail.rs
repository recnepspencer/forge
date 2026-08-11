//! Compile-time boundaries around application aftermath.
//!
//! Arity is not evidence. A case that fails because a function takes two
//! arguments and was given three fails identically for `42u8` as for a forged
//! aftermath contract, and once the parameter is deleted arity is the *only*
//! error obtainable — so an arity case can never be the proof that the deletion
//! mattered. Where a parameter was removed, the evidence here is a passing case
//! showing the sole remaining lane, paired with a failing case showing that what
//! travels down that lane is out of a caller's reach.

#[test]
fn aftermath_compiler_boundaries_hold() {
    let cases = trybuild::TestCases::new();
    declared_aftermath_shape_cases(&cases);
    publication_projection_cases(&cases);
    accepted_and_provisional_facade_cases(&cases);
    recovery_handle_lifecycle_cases(&cases);
    recovery_authority_and_carriage_cases(&cases);
}

/// Publication accepts owner-issued execution terminals but exposes only
/// unforgeable, publication-owned closed projections.
fn publication_projection_cases(cases: &trybuild::TestCases) {
    cases.pass("tests/ui/application_aftermath/publication_owner_derivation_compiles.rs");
    cases.compile_fail(
        "tests/ui/application_aftermath/publication_closed_projection_fields_are_private.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/publication_query_has_no_execution_terminal.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/publication_query_cannot_convert_to_execution_receipt.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/publication_commit_has_no_ambiguous_release_getter.rs",
    );
}

/// Undo/redo remains compiled, but only through the honestly provisional lane.
fn accepted_and_provisional_facade_cases(cases: &trybuild::TestCases) {
    cases.pass("tests/ui/application_aftermath/provisional_aftermath_surface_is_reachable.rs");
    cases.compile_fail(
        "tests/ui/application_aftermath/accepted_primary_graph_excludes_provisional_aftermath.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/accepted_installed_compatibility_excludes_provisional_aftermath.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/application_attempt_work_is_runtime_private.rs",
    );
}

/// What a declaration may say about aftermath, and what it may not.
fn declared_aftermath_shape_cases(cases: &trybuild::TestCases) {
    cases.pass("tests/ui/application_aftermath/preimage_demand_uses_typed_field.rs");
    cases.compile_fail(
        "tests/ui/application_aftermath/preimage_demand_rejects_bare_field_strings.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/preimage_locus_cannot_wrap_raw_identifiers.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/preimage_locus_cannot_substitute_field_ownership.rs",
    );
    cases.pass("tests/ui/application_aftermath/aftermath_contract_accepts_matching_schema.rs");
    cases.compile_fail(
        "tests/ui/application_aftermath/aftermath_contract_rejects_foreign_schema.rs",
    );
    cases.pass("tests/ui/application_aftermath/reversible_exposes_undo_action.rs");
    cases.pass("tests/ui/application_aftermath/published_posture_variants_exist.rs");
    cases.compile_fail("tests/ui/application_aftermath/irreversible_has_no_undo_method.rs");
    cases.compile_fail(
        "tests/ui/application_aftermath/installed_aftermath_constructor_is_private.rs",
    );
    cases.compile_fail("tests/ui/application_aftermath/provisional_discard_unreachable.rs");
    cases.compile_fail("tests/ui/application_aftermath/no_mutation_unreachable.rs");
    cases.compile_fail("tests/ui/application_aftermath/query_has_no_lineage_store.rs");
    // An effect payload that implements only the internal trait must not reach
    // the external boundary: E0277 on `ApplicationExternalEffectPayload`. This
    // case previously imported its macros from the crate root rather than
    // `facade`, so it "passed" on E0432 without ever reaching the bound.
    cases.compile_fail(
        "tests/ui/application_aftermath/external_effect_requires_projected_payload.rs",
    );
    // Q8.25-C2: the outbox record is readable but not authorable. Its four
    // wire-bearing fields are the installed contract's and the admitted
    // emission's; a caller holding the type cannot name any of them.
    cases.compile_fail(
        "tests/ui/application_aftermath/dispatch_outbox_record_is_not_caller_forgeable.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/external_dispatch_request_is_not_caller_forgeable.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/external_effect_causal_link_constructor_is_private.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/external_effect_posture_constructor_is_private.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/external_effect_posture_fields_are_private.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/committed_dispatch_outbox_observation_is_owner_sealed.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/external_dispatch_attempt_authority_is_runtime_owned.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/external_dispatch_runtime_admission_is_private.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/external_dispatch_helper_is_not_host_reachable.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/external_dispatch_completion_wrapper_is_private.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/external_effect_dispatch_fields_are_private.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/commit_created_entity_mapping_is_not_detachable.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/commit_receipt_evidence_axes_are_not_spliceable.rs",
    );
    cases.compile_fail("tests/ui/application_aftermath/commit_terminal_has_no_execution.rs");
    cases.compile_fail("tests/ui/application_aftermath/commit_terminal_has_no_retry_inspection.rs");
    cases.compile_fail(
        "tests/ui/application_aftermath/commit_terminal_cannot_convert_to_cleanup_completion.rs",
    );
    cases.compile_fail("tests/ui/application_aftermath/provider_commit_cannot_use_raw_receipt.rs");
}

/// One handle, one terminal, no second use.
fn recovery_handle_lifecycle_cases(cases: &trybuild::TestCases) {
    cases.compile_fail("tests/ui/application_aftermath/recovery_handle_is_not_clone.rs");
    cases.compile_fail(
        "tests/ui/application_aftermath/recovery_handle_duplicate_transition_unrepresentable.rs",
    );
    cases.compile_fail("tests/ui/application_aftermath/undo_handle_reuse_is_unrepresentable.rs");
    cases.compile_fail("tests/ui/application_aftermath/redo_recovery_reuse_is_unrepresentable.rs");
    cases.compile_fail("tests/ui/application_aftermath/redo_recovery_constructor_is_private.rs");
    cases.compile_fail("tests/ui/application_aftermath/proved_undo_constructor_is_private.rs");
    cases.compile_fail("tests/ui/application_aftermath/redo_intent_derivation_requires_runtime.rs");
    cases.compile_fail(
        "tests/ui/application_aftermath/current_expiry_evidence_cannot_expire_handle.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/recovery_expiry_evidence_constructors_are_private.rs",
    );

    // The one-terminal law is only a law if the terminal is the runtime's to
    // write. No supported feature exports the registry type or a handle route
    // to its slot-addressed controls.
    cases.compile_fail(
        "tests/ui/application_aftermath/recovery_registry_lifecycle_is_not_host_reachable.rs",
    );
}

/// What a caller may present to obtain or exercise recovery authority.
fn recovery_authority_and_carriage_cases(cases: &trybuild::TestCases) {
    cases.compile_fail(
        "tests/ui/application_aftermath/recovery_inspect_rejects_effect_authority.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/recovery_authority_constructor_is_private.rs",
    );
    cases.compile_fail("tests/ui/application_aftermath/foundational_material_cannot_admit_undo.rs");

    // Mint: the sole lane, and the unforgeability of what travels down it.
    // Replaces `recovery_mint_rejects_caller_aftermath`, which was pure E0061.
    cases.pass("tests/ui/application_aftermath/recovery_mint_uses_receipt_only.rs");
    cases.compile_fail(
        "tests/ui/application_aftermath/recovery_mint_receipt_is_not_caller_forgeable.rs",
    );
    cases.compile_fail(
        "tests/ui/application_aftermath/opaque_wire_identity_cannot_mint_recovery_handle.rs",
    );

    // Transitions: the aftermath parameter is gone, so what matters is that the
    // binding a transition reads instead can be neither built nor inspected.
    // Replaces `recovery_transition_rejects_caller_aftermath` (also E0061).
    cases.compile_fail(
        "tests/ui/application_aftermath/recovery_transition_aftermath_is_out_of_caller_reach.rs",
    );

    // Safe-retry and re-dispatch: the proof cannot be constructed at all, and
    // the honest lane still compiles. Replaces the two remaining arity cases,
    // `safe_retry_requires_performed_redispatch` and
    // `redispatch_requires_handle_and_authority`. Whether a proof performed for
    // one handle can be presented for another is a runtime comparison, proved
    // by `recovery_progression::safe_retry_tests`.
    cases.pass("tests/ui/application_aftermath/safe_retry_with_performed_redispatch.rs");
    cases.compile_fail(
        "tests/ui/application_aftermath/performed_external_redispatch_constructor_is_private.rs",
    );
}
