use worth_ui::facade::{
    ComponentId, SurfaceId, WorthUiAdmittedCompositionRootSetReceipt,
    WorthUiCompositionRootKind, WorthUiCompositionRootReceipt,
    WorthUiCompositionRootReconciliationOutcome, WorthUiCompositionRootReconciliationReceipt,
    WorthUiCompositionRootSetReceipt, WorthUiExternalCompositionRootMountAuthorityReceipt,
};

fn main() {
    let _root_set = WorthUiAdmittedCompositionRootSetReceipt {
        roots: Vec::new(),
        graphs: Vec::new(),
        consumed_facts: Vec::new(),
        receipt_digest: 0,
    };
    let _root_entry = WorthUiCompositionRootSetReceipt {
        root: root_receipt(),
        graph_digest: 0,
        receipt_digest: 0,
    };
    let _external = WorthUiExternalCompositionRootMountAuthorityReceipt {
        kind: WorthUiCompositionRootKind::PortalEntry,
        authority_identity: String::new(),
        surface_id: SurfaceId::new("validation.surface.external").unwrap(),
        component_id: ComponentId::new("validation.component.external").unwrap(),
        consumed_facts: Vec::new(),
        receipt_digest: 0,
    };
    let _reconciliation = WorthUiCompositionRootReconciliationReceipt {
        prior_root_mount_digest: 0,
        next_root_mount_digest: 0,
        prior_composition_graph_digest: 0,
        next_composition_graph_digest: 0,
        outcome: WorthUiCompositionRootReconciliationOutcome::Moved,
        consumed_facts: Vec::new(),
        receipt_digest: 0,
    };
}

fn root_receipt() -> WorthUiCompositionRootReceipt {
    panic!("this fixture must fail before a root receipt is needed")
}
