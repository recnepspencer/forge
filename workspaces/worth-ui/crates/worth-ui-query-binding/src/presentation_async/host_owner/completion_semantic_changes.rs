use crate::presentation_async::{
    WorthUiPresentationRequestBasis, WorthUiPresentationSemanticChange,
};

pub(super) fn completion_semantic_changes(
    basis: &WorthUiPresentationRequestBasis,
) -> Vec<WorthUiPresentationSemanticChange> {
    [
        (!basis.mechanics().is_empty() || !basis.pin_additions().is_empty())
            .then_some(WorthUiPresentationSemanticChange::UploadCompletion),
        (!basis.pin_releases().is_empty()).then_some(WorthUiPresentationSemanticChange::PinRelease),
    ]
    .into_iter()
    .flatten()
    .collect()
}
