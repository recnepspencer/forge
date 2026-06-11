use worth_ui::facade::{
    MosaicSizingContractDescriptor, MosaicSizingContractId, MosaicSizingKind,
};

fn main() {
    let _descriptor = MosaicSizingContractDescriptor {
        id: MosaicSizingContractId::new("workspace.sizing.sidebar")
            .expect("valid mosaic sizing contract id"),
        kind: MosaicSizingKind::bounded(),
    };
}
