use worth_ui::facade::query_binding::UiProjectionFactReceipt;

fn requires_affine_authority(_receipt: UiProjectionFactReceipt) {}

fn invalid(receipt: &UiProjectionFactReceipt) {
    let reporting_projections = (
        receipt.query_world_identity(),
        receipt.binding_identity(),
        receipt.source_generation_identity(),
        receipt.result_generation_identity(),
    );
    requires_affine_authority(reporting_projections);
}

fn main() {}
