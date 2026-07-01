use worth_ui::facade::declaration::{
    UiAspectContract, UiAspectCoverageReport, UiAspectName, UiConsumedAspectContract,
    UiPublishedAspectContract,
};
use worth_ui_dsl::{UiDslAspectName, UiDslLoweringReceipt};

fn main() {
    let _name = UiAspectName::admit(&UiDslAspectName::new("content.text"));
    let _published =
        UiPublishedAspectContract::admit(&[UiDslAspectName::new("content.text")]);
    let _consumed =
        UiConsumedAspectContract::admit(&[UiDslAspectName::new("interaction.operability")]);
    let fake_receipt = unsafe { std::mem::MaybeUninit::<UiDslLoweringReceipt>::zeroed().assume_init() };
    let _contract = UiAspectContract::admit(&fake_receipt);
    let _coverage = UiAspectCoverageReport::new(&[], &[]);
}
