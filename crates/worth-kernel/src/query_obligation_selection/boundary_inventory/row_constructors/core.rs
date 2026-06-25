use super::super::inventory_record::{
    QuerySelectionAuthorityPosture as Posture, QuerySelectionBoundaryInventoryRow,
    QuerySelectionDeletionAction as Action, QuerySelectionProofStrength as Proof,
    QuerySelectionSurfaceClassification as Class, QuerySelectionSurfaceOwner as Owner,
};

#[allow(clippy::too_many_arguments)]
pub(super) fn row(
    source_path: &'static str,
    facade: Option<&'static str>,
    surface: &'static str,
    class: Class,
    posture: Posture,
    proof: Proof,
    caller: &'static str,
    action: Action,
    owner: Owner,
    cap: Option<&'static str>,
    blocker: Option<&'static str>,
    trigger: Option<&'static str>,
) -> QuerySelectionBoundaryInventoryRow {
    QuerySelectionBoundaryInventoryRow::new(
        source_path,
        facade,
        surface,
        class,
        posture,
        proof,
        caller,
        action,
        owner,
        cap,
        blocker,
        trigger,
    )
}
