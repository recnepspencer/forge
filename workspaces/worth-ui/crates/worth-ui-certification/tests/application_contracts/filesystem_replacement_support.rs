use worth_ui::facade::app::{
    WorthUiActiveApplicationSession, WorthUiApplicationCutoverDenial,
    WorthUiApplicationReplacementOutcome,
};
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::application_authority_closure::candidate_catalog::admit_candidate_catalog;

use super::filesystem_contract_workspace::FilesystemContractWorkspace;

pub(super) fn activate_current_filesystem_candidate(
    workspace: &FilesystemContractWorkspace,
    session: &mut WorthUiActiveApplicationSession,
) -> Result<WorthUiApplicationReplacementOutcome, WorthUiApplicationCutoverDenial> {
    let snapshot = WorthUiFilesystemSourceProvider::new(workspace.root())
        .read()
        .expect("replacement source should settle through the production filesystem provider");
    let submission = snapshot
        .lower_to_candidate_submission(session.capabilities())
        .expect("replacement source should lower through production semantics");
    let mut prepared = session
        .prepare_replacement(submission)
        .expect("filesystem candidate should prepare through the public session");
    let catalog = admit_candidate_catalog(&mut prepared);
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("prepared filesystem candidate should lower completely");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("lowered filesystem candidate should stage completely");
    let boundary = session
        .execute_framework_turn(|_| {})
        .into_completion()
        .into_execution()
        .unwrap_or_else(|_| panic!("replacement boundary turn should complete"))
        .into_activation_boundary();
    session.activate_prepared_replacement(pending, catalog, boundary, None)
}
