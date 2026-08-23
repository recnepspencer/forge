//! Separately authored wrong-owner algorithms convicted by performed evidence.

mod mutant_execution;
mod performed_basis;

use worth_ui_host_contract::UiMountedPresentationProductionCost;
use worth_ui_host_native::{
    UiNativeClientPresentationSemanticChange as SemanticChange,
    UiNativeClientPresentationSemanticFrontierObservation as Frontier,
    UiNativeClientShutdownObservation, UiNativePhysicalSignalTransitionObservation,
};
use worth_ui_native_platform::UiNativeClientTextPresentationWorkObservation;

use super::case::{Phase5LocalityAxis as Axis, Phase5LocalityCase};

pub const ALL: [&str; 10] = [
    "complete-subscriber-closure",
    "late-aspect-filter",
    "late-scope-filter",
    "global-partition-detail-range-union",
    "every-mounted-presentation-invalidation",
    "paint-to-layout-widening",
    "dpi-to-layout-widening",
    "dropped-immediate-dependency-cause",
    "hidden-retained-document-scan",
    "predicted-counter-substitution",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HostileCostConviction {
    mutant: &'static str,
    performed_work: u64,
    mutant_work: u64,
    performed_trace_digest: [u8; 32],
    mutant_trace_digest: [u8; 32],
    denial: &'static str,
}

pub(super) struct PerformedInputs<'a> {
    shutdown: &'a UiNativeClientShutdownObservation,
    work: &'a UiNativeClientTextPresentationWorkObservation,
    production: UiMountedPresentationProductionCost,
    physical: &'a UiNativePhysicalSignalTransitionObservation,
}

impl HostileCostConviction {
    pub const fn mutant(&self) -> &'static str {
        self.mutant
    }

    pub const fn performed_work(&self) -> u64 {
        self.performed_work
    }

    pub const fn mutant_work(&self) -> u64 {
        self.mutant_work
    }

    pub const fn performed_trace_digest(&self) -> [u8; 32] {
        self.performed_trace_digest
    }

    pub const fn mutant_trace_digest(&self) -> [u8; 32] {
        self.mutant_trace_digest
    }

    pub const fn denial(&self) -> &'static str {
        self.denial
    }
}

impl<'a> PerformedInputs<'a> {
    pub(super) const fn new(
        shutdown: &'a UiNativeClientShutdownObservation,
        work: &'a UiNativeClientTextPresentationWorkObservation,
        production: UiMountedPresentationProductionCost,
        physical: &'a UiNativePhysicalSignalTransitionObservation,
    ) -> Self {
        Self {
            shutdown,
            work,
            production,
            physical,
        }
    }
}

pub(super) fn expected_for(case: Phase5LocalityCase) -> &'static [&'static str] {
    match case.axis() {
        Axis::Content if case.retained_paragraphs() > 1 => {
            &[ALL[0], ALL[1], ALL[2], ALL[3], ALL[4], ALL[8], ALL[9]]
        }
        Axis::Content => &[ALL[0], ALL[1], ALL[2], ALL[3], ALL[8], ALL[9]],
        Axis::PaintValue => &[ALL[0], ALL[1], ALL[2], ALL[3], ALL[5], ALL[8], ALL[9]],
        Axis::Dpi => &[ALL[0], ALL[1], ALL[2], ALL[3], ALL[6], ALL[9]],
        Axis::UploadCompletion => &[ALL[0], ALL[1], ALL[2], ALL[3], ALL[7], ALL[8], ALL[9]],
        _ => &[ALL[0], ALL[1], ALL[2], ALL[3], ALL[8], ALL[9]],
    }
}

pub fn expected_for_label(label: &str, retained: usize) -> Option<&'static [&'static str]> {
    Axis::ALL
        .into_iter()
        .find(|axis| axis.label() == label)
        .map(|axis| expected_for(Phase5LocalityCase::new(retained, axis)))
}

pub(super) fn adjudicate(
    case: Phase5LocalityCase,
    inputs: PerformedInputs<'_>,
) -> Result<Vec<HostileCostConviction>, String> {
    let performed = performed_basis::validate(case, &inputs)?;
    let mut convictions = subscriber_convictions(case, &performed);
    add_axis_conviction(case, &inputs, &performed, &mut convictions)?;
    if case.axis() != Axis::Dpi {
        convictions.push(hidden_retained_scan(
            case,
            inputs.production,
            performed.trace_digest,
        )?);
    }
    convictions.push(predicted_substitution(
        case,
        inputs.physical,
        performed.trace_digest,
    )?);
    validate_placement(case, &convictions)?;
    Ok(convictions)
}

fn subscriber_convictions(
    case: Phase5LocalityCase,
    performed: &performed_basis::ValidatedPerformedBasis<'_>,
) -> Vec<HostileCostConviction> {
    vec![
        conviction(
            ALL[0],
            performed.selected,
            performed.trace_digest,
            mutant_execution::complete_subscriber_closure(case),
            "wrong-aspect-subscriber-enqueued",
        ),
        conviction(
            ALL[1],
            performed.selected,
            performed.trace_digest,
            mutant_execution::late_aspect_filter(case),
            "aspect-filter-ran-after-enqueue",
        ),
        conviction(
            ALL[2],
            performed.selected,
            performed.trace_digest,
            mutant_execution::late_scope_filter(case),
            "scope-filter-ran-after-enqueue",
        ),
        conviction(
            ALL[3],
            performed.selected,
            performed.trace_digest,
            mutant_execution::global_partition_detail_range_union(case),
            "global-scope-union-enqueued",
        ),
    ]
}

fn add_axis_conviction(
    case: Phase5LocalityCase,
    inputs: &PerformedInputs<'_>,
    performed: &performed_basis::ValidatedPerformedBasis<'_>,
    convictions: &mut Vec<HostileCostConviction>,
) -> Result<(), String> {
    match case.axis() {
        Axis::Content if case.retained_paragraphs() > 1 => convictions.push(conviction(
            ALL[4],
            performed.selected,
            performed.trace_digest,
            mutant_execution::every_mounted_presentation(case),
            "unrelated-mounted-presentation-enqueued",
        )),
        Axis::PaintValue => convictions.push(layout_widening(
            ALL[5],
            case,
            inputs.work,
            performed.trace_digest,
        )?),
        Axis::Dpi => convictions.push(layout_widening(
            ALL[6],
            case,
            inputs.work,
            performed.trace_digest,
        )?),
        Axis::UploadCompletion => convictions.push(drop_immediate_cause(
            &performed.frontiers,
            case,
            performed.trace_digest,
        )?),
        _ => {}
    }
    Ok(())
}

fn validate_placement(
    case: Phase5LocalityCase,
    convictions: &[HostileCostConviction],
) -> Result<(), String> {
    let observed = convictions
        .iter()
        .map(HostileCostConviction::mutant)
        .collect::<Vec<_>>();
    if observed != expected_for(case) {
        return Err(format!(
            "hostile owner-twin placement mismatch for {}: expected {:?}, observed {observed:?}",
            case.axis().label(),
            expected_for(case)
        ));
    }
    Ok(())
}

fn conviction(
    mutant: &'static str,
    performed_work: u64,
    performed_trace_digest: [u8; 32],
    execution: mutant_execution::MutantExecution,
    denial: &'static str,
) -> HostileCostConviction {
    HostileCostConviction {
        mutant,
        performed_work,
        mutant_work: execution.work(),
        performed_trace_digest,
        mutant_trace_digest: execution.trace_digest(),
        denial,
    }
}

fn layout_widening(
    mutant: &'static str,
    case: Phase5LocalityCase,
    performed: &UiNativeClientTextPresentationWorkObservation,
    performed_trace_digest: [u8; 32],
) -> Result<HostileCostConviction, String> {
    let lawful = performed.analyzed_bytes()
        + performed.bidi_contexts()
        + performed.fallback_clusters()
        + performed.shaped_runs()
        + performed.emitted_glyphs();
    if lawful != 0 {
        return Err(format!("{mutant} lacks a zero-layout lawful receipt"));
    }
    Ok(conviction(
        mutant,
        lawful,
        performed_trace_digest,
        mutant_execution::layout_widening(case, mutant),
        "paint-or-dpi-reentered-layout-owner",
    ))
}

fn drop_immediate_cause(
    frontiers: &[&Frontier],
    case: Phase5LocalityCase,
    performed_trace_digest: [u8; 32],
) -> Result<HostileCostConviction, String> {
    let lawful = frontiers
        .iter()
        .map(|frontier| frontier.change())
        .collect::<Vec<_>>();
    if lawful != [SemanticChange::Content, SemanticChange::UploadCompletion] {
        return Err(format!(
            "upload-completion lawful causes are not exact: {lawful:?}"
        ));
    }
    Ok(conviction(
        ALL[7],
        frontiers.len() as u64,
        performed_trace_digest,
        mutant_execution::drop_immediate_dependency(case),
        "immediate-dependency-cause-omitted",
    ))
}

fn hidden_retained_scan(
    case: Phase5LocalityCase,
    performed: UiMountedPresentationProductionCost,
    performed_trace_digest: [u8; 32],
) -> Result<HostileCostConviction, String> {
    if performed.retained_command_scans() != 0 {
        return Err("lawful owner performed a retained-command scan".to_owned());
    }
    Ok(conviction(
        ALL[8],
        0,
        performed_trace_digest,
        mutant_execution::hidden_retained_document_scan(case),
        "hidden-retained-command-scan",
    ))
}

fn predicted_substitution(
    case: Phase5LocalityCase,
    physical: &UiNativePhysicalSignalTransitionObservation,
    performed_trace_digest: [u8; 32],
) -> Result<HostileCostConviction, String> {
    let performed = physical.performed_transitions() + physical.performed_nodes();
    if physical.request_sequence() == 0 || performed == 0 {
        return Err("performed physical-Signal provenance is absent".to_owned());
    }
    Ok(conviction(
        ALL[9],
        performed,
        performed_trace_digest,
        mutant_execution::predicted_counter_substitution(case),
        "prediction-has-no-owner-issued-transition",
    ))
}
