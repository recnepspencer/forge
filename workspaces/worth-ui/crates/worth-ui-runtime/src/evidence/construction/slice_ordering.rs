use crate::evidence::UiEvidenceRef;

pub(crate) fn order_refs(mut refs: Vec<UiEvidenceRef>) -> Box<[UiEvidenceRef]> {
    refs.sort_by_key(|evidence_ref| {
        (
            evidence_ref.family(),
            evidence_ref.authority_generation(),
            evidence_ref.identity().digest(),
            evidence_ref.handle().handle_digest(),
        )
    });
    refs.into_boxed_slice()
}
