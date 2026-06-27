use worth_ui::facade::{
    MosaicSizingContractDescriptor, MosaicSizingContractId, MosaicSizingKind,
};

fn main() {
    let _descriptor = MosaicSizingContractDescriptor::new(
        MosaicSizingContractId::new("workspace.sizing.sidebar")
            .expect("valid mosaic sizing contract id"),
        MosaicSizingKind::bounded(),
    )
    .with_named_measurement(320);
}
