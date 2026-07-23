use crate::graph::UiGraphNodeIdentity;

/// Sealed UI mapping law joining admitted Query consumption to its graph target.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum UiQueryAllocationPurpose {
    Measurement,
    ScrollContentExtent,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum UiQueryAllocationSourceKey {
    SettledSnapshot(super::super::UiSettledQueryFactKey),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiQueryAllocationTargetMapping {
    source_key: UiQueryAllocationSourceKey,
    target: UiGraphNodeIdentity,
    purposes: Box<[UiQueryAllocationPurpose]>,
    identity_digest: u64,
}

impl UiQueryAllocationTargetMapping {
    pub(super) fn from_admitted_receipt(
        receipt: &crate::evidence::UiSettledQueryFactReceipt,
        target: UiGraphNodeIdentity,
    ) -> Self {
        Self::from_settled_receipt(receipt, target)
    }

    pub(super) fn from_settled_receipt(
        receipt: &crate::evidence::UiSettledQueryFactReceipt,
        target: UiGraphNodeIdentity,
    ) -> Self {
        Self::from_parts(
            UiQueryAllocationSourceKey::SettledSnapshot(receipt.key().clone()),
            receipt.consumed_fact_families(),
            target,
        )
    }

    fn from_parts(
        source_key: UiQueryAllocationSourceKey,
        consumed_families: &[worth_ui_query_binding::WorthUiQueryMeasurementFactFamily],
        target: UiGraphNodeIdentity,
    ) -> Self {
        let mut purposes = vec![UiQueryAllocationPurpose::Measurement];
        if consumed_families.contains(
            &worth_ui_query_binding::WorthUiQueryMeasurementFactFamily::ScrollContentExtent,
        ) {
            purposes.push(UiQueryAllocationPurpose::ScrollContentExtent);
        }
        let identity_digest = source_key.identity_digest()
            ^ target.digest().rotate_left(47)
            ^ purposes.iter().fold(0_u64, |digest, purpose| {
                digest.rotate_left(5)
                    ^ crate::declaration::stable_text_digest(match purpose {
                        UiQueryAllocationPurpose::Measurement => {
                            "worth-ui.query-purpose.measurement"
                        }
                        UiQueryAllocationPurpose::ScrollContentExtent => {
                            "worth-ui.query-purpose.scroll-content-extent"
                        }
                    })
            });
        Self {
            source_key,
            target,
            purposes: purposes.into_boxed_slice(),
            identity_digest,
        }
    }

    pub(crate) fn source_key(&self) -> &UiQueryAllocationSourceKey {
        &self.source_key
    }

    pub(crate) fn target(&self) -> UiGraphNodeIdentity {
        self.target
    }

    pub(crate) fn admits(&self, purpose: UiQueryAllocationPurpose) -> bool {
        self.purposes.binary_search(&purpose).is_ok()
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
}

impl UiQueryAllocationSourceKey {
    pub(crate) fn identity_digest(&self) -> u64 {
        match self {
            Self::SettledSnapshot(key) => {
                crate::declaration::stable_text_digest(key.view_binding_id().as_str())
                    ^ crate::declaration::stable_text_digest(key.query_binding_identity())
                        .rotate_left(29)
            }
        }
    }

    pub(crate) fn from_settled_fact(
        view_binding_id: crate::capability::ViewBindingId,
        fact: &worth_ui_query_binding::WorthUiSettledSnapshotFact,
    ) -> Self {
        Self::SettledSnapshot(super::super::UiSettledQueryFactKey::new(
            view_binding_id,
            fact.query_binding_identity().to_owned(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admitted_query_scroll_purpose_is_closed_and_identity_bearing() {
        let (prerequisites, attempt) = crate::evidence::measurement::projection::fact_test_support::display_field_projection_consumption("query-scroll-purpose");
        let receipt = crate::evidence::consume_declared_measurement_projection_facts(
            crate::evidence::measurement::projection::fact_test_support::synthetic_declaration_identity("query-scroll-purpose"),
            worth_ui_inspection::UiEvidenceAuthorityGeneration::new(17),
            &crate::evidence::measurement::projection::fact_test_support::scroll_viewport_policy(),
            prerequisites,
            &attempt,
        )
        .expect("Query scroll facts admit before allocation mapping");
        let mapping = UiQueryAllocationTargetMapping::from_admitted_receipt(
            &receipt,
            UiGraphNodeIdentity::new(41),
        );
        assert!(mapping.admits(UiQueryAllocationPurpose::Measurement));
        assert!(mapping.admits(UiQueryAllocationPurpose::ScrollContentExtent));
        assert_ne!(mapping.identity_digest(), 0);
    }
}
