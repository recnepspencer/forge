fn mutate_query_from_ui_admission<I: worth_ui::facade::intent::UiIntent>(
    completion: worth_ui::facade::query_binding::WorthUiScalarProjectionActionPublicationCompletion,
    admission: worth_ui::facade::intent::UiAdmittedIntent<I>,
) {
    let _ = completion.admit_publication(admission);
}
