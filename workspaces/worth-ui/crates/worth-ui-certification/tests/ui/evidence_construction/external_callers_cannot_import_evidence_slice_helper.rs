use worth_ui_runtime::evidence::evidence_slice;

fn main() {
    let _ = evidence_slice;
}

// inspection and lifecycle denials share one compiler process.
mod covered_001 { include!("../facade_export/runtime_facade_root_does_not_export_runtime_host.rs"); }
mod covered_002 { include!("../inspection/external_callers_cannot_mint_inspection_receipts.rs"); }
mod covered_003 { include!("../inspection/external_callers_cannot_mint_unsupported_posture_witnesses.rs"); }
mod covered_004 { include!("../inspection/facade_callers_cannot_mint_evidence_identity.rs"); }
mod covered_005 { include!("../inspection/exhaustive_matching_over_public_inspection_contract_enums_is_forbidden.rs"); }
mod covered_006 { include!("../inspection/external_callers_cannot_mint_obligation_reason_projection.rs"); }
mod covered_007 { include!("../lifecycle/external_runtime_root_lifecycle_factories_are_not_public.rs"); }
mod covered_008 { include!("../lifecycle/external_runtime_support_inventory_construction_is_private.rs"); }
mod covered_009 { include!("../lifecycle/external_inspection_scope_inventory_construction_is_private.rs"); }
mod covered_010 { include!("../lifecycle/external_inspection_subsystem_bootstrap_is_private.rs"); }
