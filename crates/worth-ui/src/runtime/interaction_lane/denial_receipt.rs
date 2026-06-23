use crate::runtime::WorthUiPrimitiveDenialPresentation;

use super::schema::{WorthUiInteractionPropSchema, WorthUiInteractionValueDenialCode};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiInteractionValueDenialReceipt {
    surface_id: String,
    prop_key: String,
    raw_value: String,
    denial_code: WorthUiInteractionValueDenialCode,
    expected: String,
    source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
    denial_digest: u64,
}

impl WorthUiInteractionValueDenialReceipt {
    pub(crate) fn new(
        surface_id: &str,
        schema: &WorthUiInteractionPropSchema,
        raw_value: impl Into<String>,
        source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
    ) -> Self {
        Self::with_code(
            surface_id,
            schema.prop_key(),
            raw_value,
            schema.denial_code(),
            schema.expected_value_syntax(),
            source_span,
        )
    }

    pub(crate) fn missing_required(
        surface_id: &str,
        schema: &WorthUiInteractionPropSchema,
    ) -> Self {
        Self::with_code(
            surface_id,
            schema.prop_key(),
            "<missing>",
            WorthUiInteractionValueDenialCode::MissingRequiredValue,
            schema.expected_value_syntax(),
            None,
        )
    }

    pub(crate) fn unknown_prop(
        surface_id: &str,
        prop_key: &str,
        raw_value: impl Into<String>,
        source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
    ) -> Self {
        Self::with_code(
            surface_id,
            prop_key,
            raw_value,
            WorthUiInteractionValueDenialCode::UnknownInteractionProp,
            "a declared interaction prop",
            source_span,
        )
    }

    pub(crate) fn target_reference(
        surface_id: &str,
        schema: &WorthUiInteractionPropSchema,
        raw_value: impl Into<String>,
        source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
    ) -> Self {
        Self::with_code(
            surface_id,
            schema.prop_key(),
            raw_value,
            WorthUiInteractionValueDenialCode::InvalidTargetReference,
            schema.expected_value_syntax(),
            source_span,
        )
    }

    fn with_code(
        surface_id: &str,
        prop_key: &str,
        raw_value: impl Into<String>,
        denial_code: WorthUiInteractionValueDenialCode,
        expected: &str,
        source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
    ) -> Self {
        let raw_value = raw_value.into();
        let denial_digest = denial_digest(surface_id, prop_key, &raw_value, denial_code);
        Self {
            surface_id: surface_id.to_owned(),
            prop_key: prop_key.to_owned(),
            raw_value,
            denial_code,
            expected: expected.to_owned(),
            source_span,
            denial_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn prop_key(&self) -> &str {
        &self.prop_key
    }

    pub fn raw_value(&self) -> &str {
        &self.raw_value
    }

    pub fn denial_code(&self) -> WorthUiInteractionValueDenialCode {
        self.denial_code
    }

    pub fn expected(&self) -> &str {
        &self.expected
    }

    pub fn source_span(&self) -> Option<crate::runtime::WorthUiPrimitiveSourceSpan> {
        self.source_span
    }

    pub fn denial_digest(&self) -> u64 {
        self.denial_digest
    }

    pub fn presentation(&self) -> WorthUiPrimitiveDenialPresentation {
        let span = self
            .source_span()
            .map(|span| format!("{}..{}", span.start_byte(), span.end_byte()))
            .unwrap_or_else(|| "unavailable".to_owned());
        WorthUiPrimitiveDenialPresentation::new(
            "Interaction value rejected",
            vec![
                ("prop", self.prop_key().to_owned()),
                ("value", self.raw_value().to_owned()),
                ("expected", self.expected().to_owned()),
                ("code", format!("{:?}", self.denial_code())),
                ("source_span", span),
                ("digest", self.denial_digest().to_string()),
            ],
        )
    }
}

fn denial_digest(
    surface_id: &str,
    prop_key: &str,
    raw_value: &str,
    denial_code: WorthUiInteractionValueDenialCode,
) -> u64 {
    format!("interaction-denial|{surface_id}|{prop_key}|{raw_value}|{denial_code:?}")
        .bytes()
        .fold(0xcbf2_9ce4_8422_2325, |mut digest, byte| {
            digest ^= u64::from(byte);
            digest.wrapping_mul(0x0000_0100_0000_01b3)
        })
}
