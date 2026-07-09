use worth_ui::facade::declaration::{
    UiDeclarationGraphHandoff, UiDeclarationSupportRow, UiDeclarationSupportSnapshot,
};
use worth_ui_dsl::{UiDslLoweringReceipt, UiDslSemanticArtifact};

struct WorthGraphReceipt;

fn require_graph_handoff(_: UiDeclarationGraphHandoff) {}

fn main() {
    let fake_handoff =
        unsafe { std::mem::MaybeUninit::<UiDeclarationGraphHandoff>::zeroed().assume_init() };
    let fake_structural = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let fake_posture = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    let _ = UiDeclarationGraphHandoff::new(
        fake_handoff.identity().clone(),
        fake_structural,
        fake_posture,
    );

    let semantic_artifact =
        unsafe { std::mem::MaybeUninit::<UiDslSemanticArtifact>::zeroed().assume_init() };
    let lowering_receipt =
        unsafe { std::mem::MaybeUninit::<UiDslLoweringReceipt>::zeroed().assume_init() };
    let support_snapshot =
        unsafe { std::mem::MaybeUninit::<UiDeclarationSupportSnapshot>::zeroed().assume_init() };
    let support_row =
        unsafe { std::mem::MaybeUninit::<UiDeclarationSupportRow>::zeroed().assume_init() };

    require_graph_handoff(semantic_artifact);
    require_graph_handoff(lowering_receipt);
    require_graph_handoff(support_snapshot);
    require_graph_handoff(support_row);
    require_graph_handoff(WorthGraphReceipt);
}
