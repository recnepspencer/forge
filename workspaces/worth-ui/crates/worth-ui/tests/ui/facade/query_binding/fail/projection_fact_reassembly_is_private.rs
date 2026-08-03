use worth_ui::facade::query_binding::UiProjectionFactReceipt;

fn requires_affine_authority(_receipt: UiProjectionFactReceipt) {}

fn invalid(receipt: &UiProjectionFactReceipt) {
    let reporting_projections = (
        receipt.query_world_identity_for_reporting(),
        receipt.binding_identity_for_reporting(),
        receipt.source_generation_for_reporting(),
        receipt.result_generation_for_reporting(),
    );
    requires_affine_authority(reporting_projections);
}

fn main() {}
