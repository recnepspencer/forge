pub(in crate::mounting::projection) fn current_text_profile_generation(
) -> worth_ui_host_contract::UiTextProfileGeneration {
    worth_ui_host_contract::UiTextProfileGeneration::new(1)
        .expect("the qualified Worth UI text profile generation is nonzero")
}
