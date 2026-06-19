use crate::application::{
    ForgeQueryDeclarationAdmissionError, ForgeQueryDeclarationAdmissionOrLegalityError,
    ForgeQueryDeclarationEntryProgressionError, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityDenial,
    ForgeQueryDeclarationProgressionTerminalError, ForgeQueryDomainEntryMarker,
    ForgeQueryGraphObligationOrchestrationDispatch,
    ForgeQueryGraphObligationOrchestrationDispatchError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGroupedDeclarationStopKind {
    Deferred,
    Unsupported,
    InvalidContext,
    Canonicalization,
    Denied,
    Stale,
    RebindRequired,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGroupedDeclarationStop {
    member_index: usize,
    declaration_family_key: &'static str,
    stop_kind: ForgeQueryGroupedDeclarationStopKind,
    reason: String,
    graph_obligation_dispatch: Option<ForgeQueryGraphObligationOrchestrationDispatch>,
    graph_obligation_dispatch_error: Option<ForgeQueryGraphObligationOrchestrationDispatchError>,
}

impl ForgeQueryGroupedDeclarationStop {
    pub(super) fn new(
        member_index: usize,
        declaration_family_key: &'static str,
        stop_kind: ForgeQueryGroupedDeclarationStopKind,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            member_index,
            declaration_family_key,
            stop_kind,
            reason: reason.into(),
            graph_obligation_dispatch: None,
            graph_obligation_dispatch_error: None,
        }
    }

    pub(super) fn graph_obligation_dispatch_failed(
        member_index: usize,
        declaration_family_key: &'static str,
        error: ForgeQueryGraphObligationOrchestrationDispatchError,
    ) -> Self {
        Self {
            member_index,
            declaration_family_key,
            stop_kind: ForgeQueryGroupedDeclarationStopKind::Failed,
            reason: format!("graph obligation orchestration dispatch failed: {error:?}"),
            graph_obligation_dispatch: None,
            graph_obligation_dispatch_error: Some(error),
        }
    }

    pub(super) fn graph_obligation_denied(
        member_index: usize,
        declaration_family_key: &'static str,
        dispatch: ForgeQueryGraphObligationOrchestrationDispatch,
    ) -> Self {
        Self {
            member_index,
            declaration_family_key,
            stop_kind: ForgeQueryGroupedDeclarationStopKind::Denied,
            reason: "grouped declaration denied by graph obligation orchestration dispatch"
                .to_string(),
            graph_obligation_dispatch: Some(dispatch),
            graph_obligation_dispatch_error: None,
        }
    }

    pub fn member_index(&self) -> usize {
        self.member_index
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub fn stop_kind(&self) -> ForgeQueryGroupedDeclarationStopKind {
        self.stop_kind
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn graph_obligation_dispatch(
        &self,
    ) -> Option<&ForgeQueryGraphObligationOrchestrationDispatch> {
        self.graph_obligation_dispatch.as_ref()
    }

    pub fn graph_obligation_dispatch_error(
        &self,
    ) -> Option<&ForgeQueryGraphObligationOrchestrationDispatchError> {
        self.graph_obligation_dispatch_error.as_ref()
    }
}

pub(super) fn grouped_declaration_stop<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    member_index: usize,
    error: &ForgeQueryDeclarationEntryProgressionError<D, I>,
) -> ForgeQueryGroupedDeclarationStop {
    match error {
        ForgeQueryDeclarationEntryProgressionError::Entry(entry) => {
            grouped_declaration_entry_stop(member_index, entry)
        }
        ForgeQueryDeclarationEntryProgressionError::Progression(progression) => {
            grouped_progression_stop(member_index, progression)
        }
    }
}

fn grouped_declaration_entry_stop<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    member_index: usize,
    error: &ForgeQueryDeclarationAdmissionOrLegalityError<D, I>,
) -> ForgeQueryGroupedDeclarationStop {
    match error {
        ForgeQueryDeclarationAdmissionOrLegalityError::Admission(admission) => {
            grouped_admission_stop(member_index, admission)
        }
        ForgeQueryDeclarationAdmissionOrLegalityError::Legality(legality) => {
            grouped_legality_stop(member_index, legality)
        }
    }
}

fn grouped_admission_stop<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    member_index: usize,
    error: &ForgeQueryDeclarationAdmissionError<D, I>,
) -> ForgeQueryGroupedDeclarationStop {
    match error {
        ForgeQueryDeclarationAdmissionError::Deferred(denial) => ForgeQueryGroupedDeclarationStop::new(
            member_index,
            I::Family::semantic_family_key(),
            ForgeQueryGroupedDeclarationStopKind::Deferred,
            format!(
                "member {member_index} declaration deferred with capability status {}",
                denial.capability_status().as_str()
            ),
        ),
        ForgeQueryDeclarationAdmissionError::AsyncDeferred(denial) => ForgeQueryGroupedDeclarationStop::new(
            member_index,
            I::Family::semantic_family_key(),
            ForgeQueryGroupedDeclarationStopKind::Deferred,
            format!(
                "member {member_index} declaration deferred because async support is {}",
                denial.async_support().as_str()
            ),
        ),
        ForgeQueryDeclarationAdmissionError::TemporalDeferred(denial) => ForgeQueryGroupedDeclarationStop::new(
            member_index,
            I::Family::semantic_family_key(),
            ForgeQueryGroupedDeclarationStopKind::Deferred,
            format!(
                "member {member_index} declaration deferred because temporal support is {}",
                denial.temporal_support().as_str()
            ),
        ),
        ForgeQueryDeclarationAdmissionError::Unsupported(denial) => ForgeQueryGroupedDeclarationStop::new(
            member_index,
            I::Family::semantic_family_key(),
            ForgeQueryGroupedDeclarationStopKind::Unsupported,
            format!(
                "member {member_index} declaration unsupported with capability status {}",
                denial.capability_status().as_str()
            ),
        ),
        ForgeQueryDeclarationAdmissionError::AsyncUnsupported(denial) => ForgeQueryGroupedDeclarationStop::new(
            member_index,
            I::Family::semantic_family_key(),
            ForgeQueryGroupedDeclarationStopKind::Unsupported,
            format!(
                "member {member_index} declaration unsupported because async support is {}",
                denial.async_support().as_str()
            ),
        ),
        ForgeQueryDeclarationAdmissionError::TemporalUnsupported(denial) => ForgeQueryGroupedDeclarationStop::new(
            member_index,
            I::Family::semantic_family_key(),
            ForgeQueryGroupedDeclarationStopKind::Unsupported,
            format!(
                "member {member_index} declaration unsupported because temporal support is {}",
                denial.temporal_support().as_str()
            ),
        ),
        ForgeQueryDeclarationAdmissionError::InvalidContext(denial) => ForgeQueryGroupedDeclarationStop::new(
            member_index,
            I::Family::semantic_family_key(),
            ForgeQueryGroupedDeclarationStopKind::InvalidContext,
            format!(
                "member {member_index} declaration invalid in the admitted context with capability status {}",
                denial.capability_status().as_str()
            ),
        ),
        ForgeQueryDeclarationAdmissionError::Canonicalization(error) => ForgeQueryGroupedDeclarationStop::new(
            member_index,
            I::Family::semantic_family_key(),
            ForgeQueryGroupedDeclarationStopKind::Canonicalization,
            format!("member {member_index} canonicalization failed: {error:?}"),
        ),
    }
}

fn grouped_legality_stop<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    member_index: usize,
    error: &ForgeQueryDeclarationLegalityDenial<D, I>,
) -> ForgeQueryGroupedDeclarationStop {
    match error {
        ForgeQueryDeclarationLegalityDenial::DeferredByLegalityBoundary { .. } => {
            ForgeQueryGroupedDeclarationStop::new(
                member_index,
                I::Family::semantic_family_key(),
                ForgeQueryGroupedDeclarationStopKind::Deferred,
                format!("member {member_index} declaration deferred by legality boundary"),
            )
        }
        ForgeQueryDeclarationLegalityDenial::UnsupportedLegalityClass { .. } => {
            ForgeQueryGroupedDeclarationStop::new(
                member_index,
                I::Family::semantic_family_key(),
                ForgeQueryGroupedDeclarationStopKind::Unsupported,
                format!("member {member_index} declaration uses an unsupported legality class"),
            )
        }
        ForgeQueryDeclarationLegalityDenial::TemporalProjectionUnsupported { kind, .. }
            if kind.is_deferred() =>
        {
            ForgeQueryGroupedDeclarationStop::new(
                member_index,
                I::Family::semantic_family_key(),
                ForgeQueryGroupedDeclarationStopKind::Deferred,
                format!("member {member_index} declaration deferred by temporal legality boundary"),
            )
        }
        ForgeQueryDeclarationLegalityDenial::AsyncProjectionUnsupported { kind, .. }
            if kind.is_deferred() =>
        {
            ForgeQueryGroupedDeclarationStop::new(
                member_index,
                I::Family::semantic_family_key(),
                ForgeQueryGroupedDeclarationStopKind::Deferred,
                format!("member {member_index} declaration deferred by async legality boundary"),
            )
        }
        ForgeQueryDeclarationLegalityDenial::WrongAdmittedWorld { .. }
        | ForgeQueryDeclarationLegalityDenial::IllegalRoleClaim { .. }
        | ForgeQueryDeclarationLegalityDenial::IllegalSurfaceDisposition { .. }
        | ForgeQueryDeclarationLegalityDenial::TemporalProjectionUnsupported { .. }
        | ForgeQueryDeclarationLegalityDenial::AsyncProjectionUnsupported { .. } => {
            ForgeQueryGroupedDeclarationStop::new(
                member_index,
                I::Family::semantic_family_key(),
                ForgeQueryGroupedDeclarationStopKind::InvalidContext,
                format!("member {member_index} declaration failed legality review"),
            )
        }
    }
}

fn grouped_progression_stop<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
    member_index: usize,
    error: &ForgeQueryDeclarationProgressionTerminalError<D, I>,
) -> ForgeQueryGroupedDeclarationStop {
    let (kind, reason) = match error {
        ForgeQueryDeclarationProgressionTerminalError::Deferred(_) => (
            ForgeQueryGroupedDeclarationStopKind::Deferred,
            "declaration progression deferred",
        ),
        ForgeQueryDeclarationProgressionTerminalError::Denied(_) => (
            ForgeQueryGroupedDeclarationStopKind::Denied,
            "declaration progression denied",
        ),
        ForgeQueryDeclarationProgressionTerminalError::Stale(_) => (
            ForgeQueryGroupedDeclarationStopKind::Stale,
            "declaration progression went stale",
        ),
        ForgeQueryDeclarationProgressionTerminalError::RebindRequired(_) => (
            ForgeQueryGroupedDeclarationStopKind::RebindRequired,
            "declaration progression requires rebind",
        ),
        ForgeQueryDeclarationProgressionTerminalError::Failed(_) => (
            ForgeQueryGroupedDeclarationStopKind::Failed,
            "declaration progression failed",
        ),
    };
    ForgeQueryGroupedDeclarationStop::new(
        member_index,
        I::Family::semantic_family_key(),
        kind,
        format!("member {member_index} {reason}"),
    )
}
