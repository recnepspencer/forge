use worth_ui_native_platform::UiNativeApplicationPreparation;

fn extract(mut preparation: UiNativeApplicationPreparation) {
    let _: worth_ui::facade::app::WorthUiApplicationBuilder<
        worth_ui::facade::app::UiChangeProfileInstalled,
        worth_ui::facade::app::UiIntentWiringSatisfied,
        worth_ui::facade::app::UiApplicationHostBound,
    > = preparation.builder();
}

fn main() {}
