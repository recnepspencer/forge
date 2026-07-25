#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiSettledQueryFactReceiptDenial {
    NoQueryMeasurementDependencies,
    SettledFactObservation(worth_ui_query_binding::WorthUiQueryMeasurementFactObservationError),
    MissingRequiredFactFamilies {
        required: Box<[worth_ui_query_binding::WorthUiQueryMeasurementFactFamily]>,
        consumed: Box<[worth_ui_query_binding::WorthUiQueryMeasurementFactFamily]>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiProjectionFactObservation {
    family: worth_ui_query_binding::WorthUiQueryMeasurementFactFamily,
    extent: worth_foundational::CanonicalF32,
    identity_digest: u64,
}

impl UiProjectionFactObservation {
    pub(super) fn from_query_observation(
        observation: worth_ui_query_binding::WorthUiQueryMeasurementFactObservation,
    ) -> Self {
        let identity_digest =
            crate::declaration::stable_text_digest("worth-ui.projection-fact-observation")
                ^ crate::declaration::stable_text_digest(query_measurement_family_name(
                    observation.family(),
                ))
                .rotate_left(7)
                ^ (u64::from(observation.extent().bits())).rotate_left(13);
        Self {
            family: observation.family(),
            extent: observation.extent(),
            identity_digest,
        }
    }

    pub fn family(&self) -> worth_ui_query_binding::WorthUiQueryMeasurementFactFamily {
        self.family
    }

    pub fn extent(&self) -> worth_foundational::CanonicalF32 {
        self.extent
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
}

pub fn consume_declared_measurement_projection_facts(
    declaration_identity: crate::declaration::UiDeclarationIdentity,
    declaration_support_authority_generation: worth_ui_inspection::UiEvidenceAuthorityGeneration,
    measurement_policy: &crate::declaration::UiDeclaredMeasurementPolicyPosture,
    view_binding_id: crate::capability::ViewBindingId,
    fact: &worth_ui_query_binding::WorthUiSettledSnapshotFact,
) -> Result<super::UiSettledQueryFactReceipt, UiSettledQueryFactReceiptDenial> {
    super::consume_settled_query_measurement_fact(
        declaration_identity,
        declaration_support_authority_generation,
        measurement_policy,
        view_binding_id,
        fact,
    )
}

fn query_measurement_family_name(
    family: worth_ui_query_binding::WorthUiQueryMeasurementFactFamily,
) -> &'static str {
    match family {
        worth_ui_query_binding::WorthUiQueryMeasurementFactFamily::ScrollContentExtent => {
            "scroll-content-extent"
        }
    }
}
