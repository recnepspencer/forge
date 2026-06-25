use super::digest::{hash_text, primitive_content_denial_digest};
use super::schema::{
    WorthUiPrimitiveContentPropSchema, WorthUiPrimitiveContentValueDenialCode,
    WorthUiPrimitiveContentValueKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveContentValueDenialReceipt {
    surface_id: String,
    prop_key: String,
    schema_id: &'static str,
    value_kind: WorthUiPrimitiveContentValueKind,
    raw_value: String,
    expected_shape: &'static str,
    examples: &'static [&'static str],
    semantic_slice: crate::runtime::WorthUiSemanticSliceId,
    fact_family: crate::runtime::WorthUiRuntimeFactFamily,
    denial_code: WorthUiPrimitiveContentValueDenialCode,
    source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
    denial_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveContentDenialPresentation {
    title: &'static str,
    rows: Vec<WorthUiPrimitiveContentDenialPresentationRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveContentDenialPresentationRow {
    label: &'static str,
    value: String,
}

impl WorthUiPrimitiveContentValueDenialReceipt {
    pub(super) fn new(
        surface_id: &str,
        schema: &'static WorthUiPrimitiveContentPropSchema,
        raw_value: String,
        source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
    ) -> Self {
        let denial_digest = primitive_content_denial_digest(surface_id, schema, &raw_value);
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
        source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
    ) -> Self {
        let schema_id = "worth.primitive.content.prop.unknown";
        let value_kind = WorthUiPrimitiveContentValueKind::Unknown;
        let denial_code = WorthUiPrimitiveContentValueDenialCode::UnknownContentProp;
        let denial_digest = hash_text(&format!(
            "content-denial|surface:{surface_id}|schema:{schema_id}|prop:{prop_key}|kind:{:?}|value:{}|code:{:?}",
            value_kind, raw_value, denial_code
        ));
        Self {
            surface_id: surface_id.to_owned(),
            prop_key: prop_key.to_owned(),
            schema_id,
            value_kind,
            raw_value,
            expected_shape: value_kind.expected_syntax(),
            examples: &["content_text", "content_icon", "content_order"],
            semantic_slice: crate::runtime::WorthUiSemanticSliceId::PrimitiveContent,
            fact_family: crate::runtime::WorthUiRuntimeFactFamily::PrimitiveContent,
            denial_code,
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

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn prop_key(&self) -> &str {
        &self.prop_key
    }

    pub fn schema_id(&self) -> &'static str {
        self.schema_id
    }

    pub fn value_kind(&self) -> WorthUiPrimitiveContentValueKind {
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

    pub fn denial_code(&self) -> WorthUiPrimitiveContentValueDenialCode {
        self.denial_code
    }

    pub fn source_span(&self) -> Option<crate::runtime::WorthUiPrimitiveSourceSpan> {
        self.source_span
    }

    pub fn denial_digest(&self) -> u64 {
        self.denial_digest
    }

    pub fn presentation(&self) -> WorthUiPrimitiveContentDenialPresentation {
        let span = self
            .source_span()
            .map(|span| format!("{}..{}", span.start_byte(), span.end_byte()))
            .unwrap_or_else(|| "unavailable".to_owned());
        WorthUiPrimitiveContentDenialPresentation::new(
            "Primitive content value rejected",
            vec![
                ("schema", self.schema_id().to_owned()),
                ("code", format!("{:?}", self.denial_code())),
                ("value_kind", format!("{:?}", self.value_kind())),
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

impl WorthUiPrimitiveContentDenialPresentation {
    fn new(title: &'static str, rows: Vec<(&'static str, String)>) -> Self {
        Self {
            title,
            rows: rows
                .into_iter()
                .map(|(label, value)| WorthUiPrimitiveContentDenialPresentationRow { label, value })
                .collect(),
        }
    }

    pub fn title(&self) -> &'static str {
        self.title
    }

    pub fn rows(&self) -> &[WorthUiPrimitiveContentDenialPresentationRow] {
        &self.rows
    }
}

impl WorthUiPrimitiveContentDenialPresentationRow {
    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
