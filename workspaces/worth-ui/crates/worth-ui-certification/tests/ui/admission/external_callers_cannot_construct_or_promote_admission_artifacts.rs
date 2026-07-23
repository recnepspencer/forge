use worth_ui::facade::admission::{
    UiAdmissionDecision, UiAdmissionReport, UiAdmissionTarget, UiAdmissionWorld, UiSupportPosture,
    UiSupportSnapshot,
};

fn main() {
    let _ = UiSupportSnapshot::new(todo!(), UiSupportPosture::Supported);
    let support_snapshot: UiSupportSnapshot = todo!();
    let _ = UiAdmissionDecision::new(support_snapshot.clone(), todo!());
    let _ = UiAdmissionReport::from_decision(todo!());
    let _ = UiAdmissionTarget::declaration(todo!(), UiAdmissionWorld::authoritative());
}

// declaration and admission denials share one compiler process.
mod covered_001 { include!("../declaration_aspect/external_callers_cannot_construct_aspect_contracts.rs"); }
mod covered_002 { include!("../declaration/external_callers_cannot_mint_declaration_artifact.rs"); }
mod covered_003 { include!("../declaration/external_callers_cannot_mint_declaration_identity.rs"); }
mod covered_004 { include!("../declaration/external_callers_cannot_construct_semantic_artifact.rs"); }
mod covered_005 { include!("../declaration/external_callers_cannot_mint_dsl_lowering_receipt.rs"); }
mod covered_006 { include!("../declaration/external_callers_cannot_seed_dsl_package_with_semantic_artifact.rs"); }
mod covered_007 { include!("../declaration_boundary/runtime_facade_root_does_not_export_declaration_surface.rs"); }
mod covered_008 { include!("../declaration_posture/external_callers_cannot_construct_declared_posture_contracts.rs"); }
mod covered_009 { include!("../declaration_posture/declared_posture_cannot_promote_to_runtime_receipts.rs"); }
mod covered_010 { include!("../declaration_family/external_callers_cannot_construct_family_wrappers.rs"); }
mod covered_011 { include!("../declaration_graph_handoff/external_callers_cannot_construct_or_substitute_graph_handoff.rs"); }
mod covered_012 { include!("../declaration_structural/external_callers_cannot_construct_structural_semantics_or_handoff.rs"); }
mod covered_013 { include!("../declaration_support/external_callers_cannot_construct_or_promote_declaration_support.rs"); }
