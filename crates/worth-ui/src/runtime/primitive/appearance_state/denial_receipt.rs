use super::digest::{appearance_state_denial_digest, hash_text};
use super::schema::{
    WorthUiAppearanceStatePropSchema, WorthUiAppearanceStateValueDenialCode,
    WorthUiAppearanceStateValueKind,
};
use crate::runtime::WorthUiPrimitiveDenialPresentation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiAppearanceStateTokenDenialReason {
    InvalidTokenSyntax,
    MissingThemeToken,
    MissingDensityToken,
    MissingAppearanceToken,
    WrongAppearanceTokenKind,
    WrongDensityTokenKind,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiAppearanceStateValueDenialReceipt {
    surface_id: String,
    prop_key: String,
    schema_id: String,
    value_kind: WorthUiAppearanceStateValueKind,
    raw_value: String,
    expected_shape: &'static str,
    examples: &'static [&'static str],
    semantic_slice: crate::runtime::WorthUiSemanticSliceId,
    fact_family: crate::runtime::WorthUiRuntimeFactFamily,
    denial_code: WorthUiAppearanceStateValueDenialCode,
    token_denial_reason: Option<WorthUiAppearanceStateTokenDenialReason>,
    source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
    denial_digest: u64,
}

impl WorthUiAppearanceStateValueDenialReceipt {
    pub(super) fn new(
        surface_id: &str,
        schema: &WorthUiAppearanceStatePropSchema,
        raw_value: String,
        source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
    ) -> Self {
        let denial_digest = appearance_state_denial_digest(surface_id, schema, &raw_value);
        Self {
            surface_id: surface_id.to_owned(),
            prop_key: schema.prop_key().to_owned(),
            schema_id: schema.schema_id().to_owned(),
            value_kind: schema.value_kind(),
            raw_value,
            expected_shape: schema.expected_value_syntax(),
            examples: schema.examples(),
            semantic_slice: schema.semantic_slice(),
            fact_family: schema.fact_family(),
            denial_code: schema.denial_code(),
            token_denial_reason: None,
            source_span,
            denial_digest,
        }
    }

    pub(super) fn token_resolution(
        surface_id: &str,
        schema: &WorthUiAppearanceStatePropSchema,
        raw_value: String,
        reason: WorthUiAppearanceStateTokenDenialReason,
        source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
    ) -> Self {
        let denial_digest = hash_text(&format!(
            "appearance-state-denial|surface:{surface_id}|schema:{}|prop:{}|kind:{:?}|value:{}|code:{:?}|token_reason:{reason:?}",
            schema.schema_id(),
            schema.prop_key(),
            schema.value_kind(),
            raw_value,
            schema.denial_code()
        ));
        Self {
            surface_id: surface_id.to_owned(),
            prop_key: schema.prop_key().to_owned(),
            schema_id: schema.schema_id().to_owned(),
            value_kind: schema.value_kind(),
            raw_value,
            expected_shape: schema.expected_value_syntax(),
            examples: schema.examples(),
            semantic_slice: schema.semantic_slice(),
            fact_family: schema.fact_family(),
            denial_code: schema.denial_code(),
            token_denial_reason: Some(reason),
            source_span,
            denial_digest,
        }
    }

    pub(super) fn unknown_prop(
        surface_id: &str,
        prop_key: &str,
        raw_value: String,
        source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
    ) -> Self {
        let schema_id = "worth.primitive.appearance_state.prop.unknown".to_owned();
        let value_kind = WorthUiAppearanceStateValueKind::Unknown;
        let denial_code = WorthUiAppearanceStateValueDenialCode::UnknownAppearanceStateProp;
        let denial_digest = hash_text(&format!(
            "appearance-state-denial|surface:{surface_id}|schema:{schema_id}|prop:{prop_key}|kind:{value_kind:?}|value:{raw_value}|code:{denial_code:?}"
        ));
        Self {
            surface_id: surface_id.to_owned(),
            prop_key: prop_key.to_owned(),
            schema_id,
            value_kind,
            raw_value,
            expected_shape: value_kind.expected_syntax(),
            examples: &[
                "appearance_rest_background",
                "appearance_pressed_border_width",
            ],
            semantic_slice: crate::runtime::WorthUiSemanticSliceId::PrimitiveAppearanceState,
            fact_family: crate::runtime::WorthUiRuntimeFactFamily::PrimitiveAppearanceState,
            denial_code,
            token_denial_reason: None,
            source_span,
            denial_digest,
        }
    }

    pub(super) fn attach_source_span(
        &mut self,
        source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
    ) {
        self.source_span = source_span;
    }

    pub fn prop_key(&self) -> &str {
        &self.prop_key
    }

    pub fn schema_id(&self) -> &str {
        &self.schema_id
    }

    pub fn value_kind(&self) -> WorthUiAppearanceStateValueKind {
        self.value_kind
    }

    pub fn raw_value(&self) -> &str {
        &self.raw_value
    }

    pub fn expected_shape(&self) -> &'static str {
        self.expected_shape
    }

    pub fn examples(&self) -> &'static [&'static str] {
        self.examples
    }

    pub fn fact_family(&self) -> crate::runtime::WorthUiRuntimeFactFamily {
        self.fact_family
    }

    pub fn denial_code(&self) -> WorthUiAppearanceStateValueDenialCode {
        self.denial_code
    }

    pub fn token_denial_reason(&self) -> Option<WorthUiAppearanceStateTokenDenialReason> {
        self.token_denial_reason
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
        let mut rows = vec![
            ("prop", self.prop_key().to_owned()),
            ("value", self.raw_value().to_owned()),
            ("expected", self.expected_shape().to_owned()),
            ("examples", self.examples().join(", ")),
            ("slice", format!("{:?}", self.semantic_slice)),
            ("fact", self.fact_family().token().to_owned()),
            ("source_span", span),
            ("digest", self.denial_digest().to_string()),
        ];
        if let Some(reason) = self.token_denial_reason() {
            rows.push(("token_reason", format!("{reason:?}")));
        }
        WorthUiPrimitiveDenialPresentation::new("Appearance state value rejected", rows)
    }
}
