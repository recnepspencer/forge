use worth_proof::NonEmpty;
use worth_query_installation::facade::{
    WorthQueryInstalledGraphObligationKind, WorthQueryInstalledGraphObligationSet,
    WorthQueryInstalledGraphObligationSubjectKind,
};

use super::{
    WorthQueryGraphObligationSelectionCounters, WorthQueryGraphObligationSelectionDenial,
    WorthQueryGraphObligationSelectionDenialKind as DenialKind, WorthQueryGraphWorkIntent,
    WorthQueryGraphWorkIntentKind, WorthQuerySelectedGraphObligations,
};

pub fn select_installed_graph_obligations(
    installed: &WorthQueryInstalledGraphObligationSet,
    intent: WorthQueryGraphWorkIntent,
) -> Result<WorthQuerySelectedGraphObligations, WorthQueryGraphObligationSelectionDenial> {
    let mut counters = WorthQueryGraphObligationSelectionCounters::default();
    counters.checked_subject();
    validate_subject(installed, intent)?;
    validate_effect_posture(installed, intent)?;
    let mut selected = Vec::with_capacity(installed.rows().len());
    for row in installed.rows() {
        counters.examined_row();
        selected.push(row.clone());
        counters.selected_row();
    }
    let rows = NonEmpty::try_from_vec(selected)
        .expect("installation proves every graph-obligation set is non-empty");
    Ok(WorthQuerySelectedGraphObligations::seal(
        installed.clone(),
        rows,
        intent,
        counters,
    ))
}

fn validate_subject(
    installed: &WorthQueryInstalledGraphObligationSet,
    intent: WorthQueryGraphWorkIntent,
) -> Result<(), WorthQueryGraphObligationSelectionDenial> {
    let expected = match intent.kind() {
        WorthQueryGraphWorkIntentKind::ApplicationQueryRead => {
            WorthQueryInstalledGraphObligationSubjectKind::ApplicationQuery
        }
        WorthQueryGraphWorkIntentKind::ApplicationOperationRead
        | WorthQueryGraphWorkIntentKind::ApplicationOperationMutation => {
            WorthQueryInstalledGraphObligationSubjectKind::ApplicationOperation
        }
    };
    (installed.subject_kind() == expected)
        .then_some(())
        .ok_or_else(|| denial(DenialKind::SubjectKindMismatch, installed))
}

fn validate_effect_posture(
    installed: &WorthQueryInstalledGraphObligationSet,
    intent: WorthQueryGraphWorkIntent,
) -> Result<(), WorthQueryGraphObligationSelectionDenial> {
    let has_mutation = installed.rows().iter().any(|row| {
        matches!(
            row.kind(),
            WorthQueryInstalledGraphObligationKind::MutationTouch
                | WorthQueryInstalledGraphObligationKind::EffectApplication
                | WorthQueryInstalledGraphObligationKind::InvariantExecution
        )
    });
    match intent.kind() {
        WorthQueryGraphWorkIntentKind::ApplicationOperationMutation if !has_mutation => {
            Err(denial(DenialKind::MutationAuthorityRequired, installed))
        }
        WorthQueryGraphWorkIntentKind::ApplicationQueryRead
        | WorthQueryGraphWorkIntentKind::ApplicationOperationRead
            if has_mutation =>
        {
            Err(denial(
                DenialKind::ReadOnlyIntentCannotSelectMutation,
                installed,
            ))
        }
        _ => Ok(()),
    }
}

fn denial(
    kind: DenialKind,
    installed: &WorthQueryInstalledGraphObligationSet,
) -> WorthQueryGraphObligationSelectionDenial {
    WorthQueryGraphObligationSelectionDenial::new(kind, installed.subject_name())
}
