use crate::runtime::{WorthUiPrimitiveDenialPresentation, WorthUiPrimitiveSourceSpan};

use super::digest::{hash_text, primitive_denial_digest};
use crate::runtime::primitive::{
    WorthUiPrimitiveAuthoredPropSchema, WorthUiPrimitiveAuthoredValueKind,
    WorthUiPrimitiveValueDenialCode,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveValueDenialReceipt {
    surface_id: String,
    prop_key: String,
    schema_id: &'static str,
    value_kind: WorthUiPrimitiveAuthoredValueKind,
    raw_value: String,
    expected_shape: &'static str,
    examples: &'static [&'static str],
    semantic_slice: crate::runtime::WorthUiSemanticSliceId,
    fact_family: crate::runtime::WorthUiRuntimeFactFamily,
    denial_code: WorthUiPrimitiveValueDenialCode,
    source_span: Option<WorthUiPrimitiveSourceSpan>,
    denial_digest: u64,
}

impl WorthUiPrimitiveValueDenialReceipt {
    pub(super) fn new(
        surface_id: &str,
        schema: &'static WorthUiPrimitiveAuthoredPropSchema,
        raw_value: String,
        source_span: Option<WorthUiPrimitiveSourceSpan>,
    ) -> Self {
        let denial_digest = primitive_denial_digest(surface_id, schema, &raw_value);
        Self {
            surface_id: surface_id.to_owned(),
            prop_key: schema.prop_key().to_owned(),
            schema_id: schema.schema_id(),
            value_kind: schema.value_kind(),
            raw_value,
            expected_shape: schema.expected_value_syntax(),
            examples: schema.examples(),
            semantic_slice: schema.semantic_slice(),
            fact_family: schema.fact_family(),
            denial_code: schema.denial_code(),
            source_span,
            denial_digest,
        }
    }

    pub(super) fn unknown_prop(
        surface_id: &str,
        prop_key: &str,
        raw_value: String,
        source_span: Option<WorthUiPrimitiveSourceSpan>,
    ) -> Self {
        let schema_id = "worth.primitive.prop.unknown";
        let expected_shape = WorthUiPrimitiveAuthoredValueKind::Unknown.expected_syntax();
        let denial_code = WorthUiPrimitiveValueDenialCode::UnknownPrimitiveProp;
        let denial_digest = hash_text(&format!(
            "primitive-denial|surface:{surface_id}|schema:{schema_id}|prop:{prop_key}|kind:{:?}|value:{}|code:{:?}",
            WorthUiPrimitiveAuthoredValueKind::Unknown,
            raw_value,
            denial_code
        ));
        Self {
            surface_id: surface_id.to_owned(),
            prop_key: prop_key.to_owned(),
            schema_id,
            value_kind: WorthUiPrimitiveAuthoredValueKind::Unknown,
            raw_value,
            expected_shape,
            examples: &["primitive_background", "primitive_padding"],
            semantic_slice: crate::runtime::WorthUiSemanticSliceId::AuthoredSurfaceInstanceProps,
            fact_family: crate::runtime::WorthUiRuntimeFactFamily::AuthoredSurfaceProps,
            denial_code,
            source_span,
            denial_digest,
        }
    }

    pub(super) fn attach_source_span(&mut self, source_span: Option<WorthUiPrimitiveSourceSpan>) {
        self.source_span = source_span;
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn prop_key(&self) -> &str {
        &self.prop_key
    }

    pub fn schema_id(&self) -> &'static str {
        self.schema_id
    }

    pub fn value_kind(&self) -> WorthUiPrimitiveAuthoredValueKind {
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

    pub fn semantic_slice(&self) -> crate::runtime::WorthUiSemanticSliceId {
        self.semantic_slice
    }

    pub fn fact_family(&self) -> crate::runtime::WorthUiRuntimeFactFamily {
        self.fact_family
    }

    pub fn denial_code(&self) -> WorthUiPrimitiveValueDenialCode {
        self.denial_code
    }

    pub fn source_span(&self) -> Option<WorthUiPrimitiveSourceSpan> {
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
            "Primitive value rejected",
            vec![
                ("prop", self.prop_key().to_owned()),
                ("value", self.raw_value().to_owned()),
                ("expected", self.expected_shape().to_owned()),
                ("examples", self.examples().join(", ")),
                ("slice", format!("{:?}", self.semantic_slice())),
                ("fact", self.fact_family().token().to_owned()),
                ("source_span", span),
                ("digest", self.denial_digest().to_string()),
            ],
        )
    }
}
