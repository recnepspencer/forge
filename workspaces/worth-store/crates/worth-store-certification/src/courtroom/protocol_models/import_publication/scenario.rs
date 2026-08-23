use worth_store_formal_models::{ImportPublicationAction, ImportPublicationModel};

pub(in crate::courtroom::protocol_models) fn execute_ordinary_import_publication(
) -> Vec<ImportPublicationAction> {
    execute_ordinary_import_publication_traces()
        .into_iter()
        .flatten()
        .collect()
}

pub(in crate::courtroom::protocol_models) fn execute_ordinary_import_publication_traces(
) -> Vec<Vec<ImportPublicationAction>> {
    vec![
        model_actions(|model| {
            model.admit_publication_readiness().unwrap();
            model.complete_publication(true).unwrap();
        }),
        model_actions(|model| {
            model.admit_publication_readiness().unwrap();
            model.crash();
        }),
        model_actions(|model| {
            model.admit_publication_readiness().unwrap();
            model.complete_publication(false).unwrap_err();
        }),
    ]
}

pub(in crate::courtroom::protocol_models) fn replay_import_publication_guard(
    _seed: u64,
) -> Vec<ImportPublicationAction> {
    execute_ordinary_import_publication_traces()
        .into_iter()
        .nth(1)
        .expect("import crash trace")
}

fn model_actions(
    terminal: impl FnOnce(&mut ImportPublicationModel),
) -> Vec<ImportPublicationAction> {
    let mut model = ImportPublicationModel::from_raw_declaration();
    model.readmit_current_scope();
    model.admit_recovered_artifact().unwrap();
    model.admit_layout_materialization().unwrap();
    terminal(&mut model);
    model.actions().collect()
}
