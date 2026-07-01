use worth_ui::facade::declaration::{
    UiDeclaredHostCapabilityPosture, UiDeclaredPostureApplicability, UiDeclaredPostureContract,
    UiDeclaredPostureLane, UiDeclaredQueryBindingPosture,
};
use worth_ui::facade::{UiInspectionPosture, WorthUiHostCapability, WorthUiHostContract};

fn main() {
    let query_binding = UiDeclaredPostureLane::new(
        UiDeclaredPostureApplicability::Optional,
        Some(UiDeclaredQueryBindingPosture::AttachedViewBinding),
    );
    let _contract = UiDeclaredPostureContract::new(
        query_binding,
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
    );
    let _host_contract: WorthUiHostContract =
        unsafe { std::mem::MaybeUninit::<UiDeclaredHostCapabilityPosture>::zeroed().assume_init() };
    let _inspection: UiInspectionPosture = WorthUiHostCapability::Ime;
}
