use super::super::{
    WorthQueryApplicationQueryAdmissionDenial, WorthQueryApplicationQueryAdmissionDenialKind,
};

pub(super) fn denial(
    kind: WorthQueryApplicationQueryAdmissionDenialKind,
    subject: impl Into<String>,
) -> WorthQueryApplicationQueryAdmissionDenial {
    WorthQueryApplicationQueryAdmissionDenial::new(kind, subject)
}

pub(super) fn graph_work_denial(
    subject: impl Into<String>,
) -> WorthQueryApplicationQueryAdmissionDenial {
    denial(
        WorthQueryApplicationQueryAdmissionDenialKind::GraphWorkAdmissionUnavailable,
        subject,
    )
}
