//! Validation and trace of immutable owner-issued performed evidence.

use sha2::{Digest, Sha256};
use worth_ui_host_native::{
    UiNativeClientPresentationSemanticChange as SemanticChange,
    UiNativeClientPresentationSemanticFrontierObservation as Frontier,
};

use super::{Axis, PerformedInputs, Phase5LocalityCase};

pub(super) struct ValidatedPerformedBasis<'a> {
    pub(super) frontiers: Vec<&'a Frontier>,
    pub(super) selected: u64,
    pub(super) trace_digest: [u8; 32],
}

pub(super) fn validate<'a>(
    case: Phase5LocalityCase,
    inputs: &PerformedInputs<'a>,
) -> Result<ValidatedPerformedBasis<'a>, String> {
    let frontiers = relevant_frontiers(case, inputs.shutdown)?;
    let representative = frontiers[0];
    let selected = representative.subscribers().len() as u64;
    if selected != modeled_selected(case, representative.change()) {
        return Err(
            "owner-issued selected work disagrees with hostile-model precondition".to_owned(),
        );
    }
    if representative.scope_rejections() != modeled_rejections(case, representative.change()) {
        return Err(
            "owner-issued scope rejections disagree with hostile-model precondition".to_owned(),
        );
    }
    Ok(ValidatedPerformedBasis {
        trace_digest: performed_trace(&frontiers, inputs),
        frontiers,
        selected,
    })
}

fn modeled_selected(case: Phase5LocalityCase, change: SemanticChange) -> u64 {
    let paragraphs = case.retained_paragraphs() as u64;
    match (case.axis(), change) {
        (Axis::Dpi, SemanticChange::Dpi)
        | (Axis::UploadCompletion, SemanticChange::Content)
        | (Axis::UploadCompletion, SemanticChange::UploadCompletion) => 2 * paragraphs,
        (Axis::PaintBoundary, SemanticChange::PaintBoundary) => 1,
        _ => 2,
    }
}

fn modeled_rejections(case: Phase5LocalityCase, change: SemanticChange) -> [u64; 4] {
    let paragraphs = case.retained_paragraphs() as u64;
    match (case.axis(), change) {
        (Axis::PaintBoundary, SemanticChange::PaintBoundary) => {
            [28 * paragraphs, 4 * paragraphs - 2, 0, 1]
        }
        (Axis::Dpi, SemanticChange::Dpi) => [42 * paragraphs, 2 * paragraphs, 2 * paragraphs, 0],
        (Axis::UploadCompletion, SemanticChange::Content) => [
            56 * paragraphs * paragraphs,
            8 * paragraphs * paragraphs - 4 * paragraphs,
            2 * paragraphs,
            0,
        ],
        (Axis::UploadCompletion, SemanticChange::UploadCompletion) => {
            [56 * paragraphs, 0, 6 * paragraphs, 0]
        }
        (Axis::PinRelease, SemanticChange::PinRelease) if paragraphs == 1 => [70, 0, 8, 0],
        (Axis::PinRelease, SemanticChange::PinRelease) => {
            [56 * paragraphs, 0, 8 * paragraphs - 2, 0]
        }
        _ => [56 * paragraphs, 8 * paragraphs - 4, 2, 0],
    }
}

fn performed_trace(frontiers: &[&Frontier], inputs: &PerformedInputs<'_>) -> [u8; 32] {
    let mut trace = Sha256::new();
    trace.update(b"worth-ui-phase5-owner-performed-v1");
    for frontier in frontiers {
        trace.update([frontier.change() as u8]);
        trace.update(frontier.source_deliveries().to_le_bytes());
        for rejected in frontier.scope_rejections() {
            trace.update(rejected.to_le_bytes());
        }
        for subscriber in frontier.subscribers() {
            trace.update(subscriber.source_digest());
            trace.update(subscriber.immediate_dependency_digest());
            trace.update(subscriber.content_digest());
            trace.update(subscriber.layout_digest());
            trace.update(subscriber.raster_key_set_digest());
        }
    }
    for value in [
        inputs.work.analyzed_bytes(),
        inputs.work.shaped_runs(),
        inputs.work.emitted_glyphs(),
        inputs.production.retained_command_scans(),
        inputs.production.retained_command_clones(),
        inputs.physical.request_sequence(),
        inputs.physical.performed_transitions(),
        inputs.physical.performed_nodes(),
    ] {
        trace.update(value.to_le_bytes());
    }
    trace.finalize().into()
}

fn relevant_frontiers<'a>(
    case: Phase5LocalityCase,
    shutdown: &'a worth_ui_host_native::UiNativeClientShutdownObservation,
) -> Result<Vec<&'a Frontier>, String> {
    let changes: &[SemanticChange] = match case.axis() {
        Axis::Content | Axis::AtlasMiss => &[SemanticChange::Content],
        Axis::Width => &[SemanticChange::Width],
        Axis::PaintValue => &[SemanticChange::PaintValue],
        Axis::PaintBoundary => &[SemanticChange::PaintBoundary],
        Axis::Dpi => &[SemanticChange::Dpi],
        Axis::UploadCompletion => &[SemanticChange::Content, SemanticChange::UploadCompletion],
        Axis::PinRelease => &[SemanticChange::PinRelease],
    };
    changes
        .iter()
        .map(|change| {
            shutdown
                .presentation_semantic_frontiers()
                .iter()
                .rev()
                .find(|frontier| frontier.change() == *change)
                .ok_or_else(|| format!("hostile model omitted {change:?} performed frontier"))
        })
        .collect()
}
