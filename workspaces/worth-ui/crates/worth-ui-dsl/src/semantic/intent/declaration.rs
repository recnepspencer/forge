use crate::source::{WorthUiArtifactInputBodyAtom, WorthUiSemanticArtifactDeclaration};
use crate::{UiDslPostureToken, UiDslSemanticFamily, UiDslSemanticKey};

use super::WorthUiIntentInteractionFamily;
use super::WorthUiIntentPayloadSourceSpec;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiIntentDeclarationSpec {
    identity: UiDslSemanticKey,
    definition_reference: Box<str>,
    interaction: WorthUiIntentInteractionFamily,
    expected_payload_schema: Option<WorthUiIntentSchemaExpectation>,
    expected_outcome_schema: Option<WorthUiIntentSchemaExpectation>,
    payload_sources: Vec<WorthUiIntentPayloadSourceSpec>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiIntentSchemaExpectation {
    identity: Box<str>,
    version: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiIntentDeclarationMeaning {
    definition_reference: Box<str>,
    interaction: WorthUiIntentInteractionFamily,
    expected_payload_schema: Option<WorthUiIntentSchemaExpectation>,
    expected_outcome_schema: Option<WorthUiIntentSchemaExpectation>,
    payload_sources: Box<[WorthUiIntentPayloadSourceSpec]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiIntentDeclarationParseError {
    detail: Box<str>,
}

impl WorthUiIntentDeclarationSpec {
    pub fn new(
        identity: impl Into<String>,
        definition_reference: impl Into<Box<str>>,
        interaction: WorthUiIntentInteractionFamily,
    ) -> Self {
        let definition_reference = definition_reference.into();
        assert!(
            !definition_reference.trim().is_empty(),
            "intent definition reference cannot be empty"
        );
        Self {
            identity: UiDslSemanticKey::new(identity),
            definition_reference,
            interaction,
            expected_payload_schema: None,
            expected_outcome_schema: None,
            payload_sources: Vec::new(),
        }
    }

    pub fn with_expected_schemas(
        mut self,
        payload_identity: impl Into<Box<str>>,
        payload_version: u16,
        outcome_identity: impl Into<Box<str>>,
        outcome_version: u16,
    ) -> Self {
        assert!(
            payload_version > 0 && outcome_version > 0,
            "intent schema expectations require nonzero versions"
        );
        self.expected_payload_schema = Some(WorthUiIntentSchemaExpectation {
            identity: payload_identity.into(),
            version: payload_version,
        });
        self.expected_outcome_schema = Some(WorthUiIntentSchemaExpectation {
            identity: outcome_identity.into(),
            version: outcome_version,
        });
        self
    }

    pub fn with_payload_source(mut self, source: WorthUiIntentPayloadSourceSpec) -> Self {
        self.payload_sources.push(source);
        self
    }

    pub fn identity(&self) -> &str {
        self.identity.as_str()
    }

    pub fn definition_reference(&self) -> &str {
        &self.definition_reference
    }

    pub const fn interaction(&self) -> WorthUiIntentInteractionFamily {
        self.interaction
    }

    pub fn into_semantic_declaration(self) -> WorthUiSemanticArtifactDeclaration {
        let meaning = WorthUiIntentDeclarationMeaning {
            definition_reference: self.definition_reference,
            interaction: self.interaction,
            expected_payload_schema: self.expected_payload_schema,
            expected_outcome_schema: self.expected_outcome_schema,
            payload_sources: self.payload_sources.into_boxed_slice(),
        };
        let mut declaration =
            WorthUiSemanticArtifactDeclaration::new(self.identity, UiDslSemanticFamily::Intent)
                .with_posture_token(UiDslPostureToken::new("intent:standalone"))
                .with_posture_token(UiDslPostureToken::new(format!(
                    "definition:{}",
                    meaning.definition_reference
                )))
                .with_posture_token(UiDslPostureToken::new(format!(
                    "interaction:{}",
                    meaning.interaction.as_str()
                )));
        if let Some(schema) = &meaning.expected_payload_schema {
            declaration = declaration.with_posture_token(UiDslPostureToken::new(format!(
                "payload-schema:{}@{}",
                schema.identity, schema.version
            )));
        }
        if let Some(schema) = &meaning.expected_outcome_schema {
            declaration = declaration.with_posture_token(UiDslPostureToken::new(format!(
                "outcome-schema:{}@{}",
                schema.identity, schema.version
            )));
        }
        for source in &meaning.payload_sources {
            declaration =
                declaration.with_posture_token(UiDslPostureToken::new(source.revision_token()));
        }
        declaration.with_intent_declaration(meaning)
    }

    pub(crate) fn parse_file_authored(
        identity: &str,
        body: &[WorthUiArtifactInputBodyAtom],
    ) -> Result<Self, WorthUiIntentDeclarationParseError> {
        let mut cursor = IntentDeclarationCursor::new(body);
        let mut definition = None;
        let mut interaction = None;
        let mut payload_sources = Vec::new();
        while !cursor.is_eof() {
            match cursor.take_identifier()?.as_str() {
                "definition" => set_once(&mut definition, cursor.take_identifier()?, "definition")?,
                "interaction" => {
                    let authored = cursor.take_identifier()?;
                    let family = WorthUiIntentInteractionFamily::parse(&authored)
                        .ok_or_else(|| error(format!("unknown interaction family `{authored}`")))?;
                    set_once(&mut interaction, family, "interaction")?;
                }
                "payload" => payload_sources.push(parse_payload_source(&mut cursor)?),
                clause => {
                    return Err(error(format!(
                        "unknown intent declaration clause `{clause}`"
                    )));
                }
            }
            cursor.take_optional_semicolon();
        }
        let mut declaration = Self::new(
            identity,
            definition.ok_or_else(|| error("intent declaration requires `definition`"))?,
            interaction.ok_or_else(|| error("intent declaration requires `interaction`"))?,
        );
        declaration.payload_sources = payload_sources;
        Ok(declaration)
    }
}

impl WorthUiIntentDeclarationMeaning {
    pub fn definition_reference(&self) -> &str {
        &self.definition_reference
    }

    pub const fn interaction(&self) -> WorthUiIntentInteractionFamily {
        self.interaction
    }

    pub fn expected_payload_schema(&self) -> Option<&WorthUiIntentSchemaExpectation> {
        self.expected_payload_schema.as_ref()
    }

    pub fn expected_outcome_schema(&self) -> Option<&WorthUiIntentSchemaExpectation> {
        self.expected_outcome_schema.as_ref()
    }

    pub fn payload_sources(&self) -> &[WorthUiIntentPayloadSourceSpec] {
        &self.payload_sources
    }

    pub(crate) fn fold_source_revision(&self, digest: &mut u64) {
        fold_text(digest, self.definition_reference());
        fold_text(digest, self.interaction.as_str());
        fold_schema(digest, self.expected_payload_schema());
        fold_schema(digest, self.expected_outcome_schema());
        fold_u64(digest, self.payload_sources.len() as u64);
        for source in &self.payload_sources {
            fold_text(digest, &source.revision_token());
        }
    }

    pub(crate) fn canonicalize(&mut self) {
        self.payload_sources
            .sort_by_key(WorthUiIntentPayloadSourceSpec::revision_token);
    }
}

impl WorthUiIntentSchemaExpectation {
    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub const fn version(&self) -> u16 {
        self.version
    }
}

impl WorthUiIntentDeclarationParseError {
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

fn set_once<T>(
    slot: &mut Option<T>,
    value: T,
    clause: &str,
) -> Result<(), WorthUiIntentDeclarationParseError> {
    if slot.replace(value).is_some() {
        return Err(error(format!("intent declaration repeats `{clause}`")));
    }
    Ok(())
}

fn error(detail: impl Into<Box<str>>) -> WorthUiIntentDeclarationParseError {
    WorthUiIntentDeclarationParseError {
        detail: detail.into(),
    }
}

fn parse_payload_source(
    cursor: &mut IntentDeclarationCursor<'_>,
) -> Result<WorthUiIntentPayloadSourceSpec, WorthUiIntentDeclarationParseError> {
    let field = cursor.take_identifier()?;
    let kind = cursor.take_identifier()?;
    Ok(match kind.as_str() {
        "projection-text" => {
            WorthUiIntentPayloadSourceSpec::projection_text(field, cursor.take_identifier()?)
        }
        "projection-selection" => {
            WorthUiIntentPayloadSourceSpec::projection_selection(field, cursor.take_identifier()?)
        }
        "committed-draft" => WorthUiIntentPayloadSourceSpec::committed_draft(field),
        "constant-text" => {
            WorthUiIntentPayloadSourceSpec::constant_text(field, cursor.take_string_literal()?)
        }
        "constant-boolean" => {
            let value = match cursor.take_identifier()?.as_str() {
                "true" => true,
                "false" => false,
                value => return Err(error(format!("invalid boolean payload constant `{value}`"))),
            };
            WorthUiIntentPayloadSourceSpec::constant_boolean(field, value)
        }
        "constant-unsigned64" => {
            let authored = cursor.take_string_literal()?;
            let value = authored
                .parse::<u64>()
                .map_err(|_| error(format!("invalid unsigned-64 payload constant `{authored}`")))?;
            WorthUiIntentPayloadSourceSpec::constant_unsigned64(field, value)
        }
        "application-text" => {
            WorthUiIntentPayloadSourceSpec::application_text(field, cursor.take_identifier()?)
        }
        "application-boolean" => {
            WorthUiIntentPayloadSourceSpec::application_boolean(field, cursor.take_identifier()?)
        }
        "application-unsigned64" => {
            WorthUiIntentPayloadSourceSpec::application_unsigned64(field, cursor.take_identifier()?)
        }
        _ => return Err(error(format!("unknown payload source kind `{kind}`"))),
    })
}

fn fold_schema(digest: &mut u64, schema: Option<&WorthUiIntentSchemaExpectation>) {
    match schema {
        Some(schema) => {
            fold_u64(digest, 1);
            fold_text(digest, schema.identity());
            fold_u64(digest, u64::from(schema.version()));
        }
        None => fold_u64(digest, 0),
    }
}

fn fold_text(digest: &mut u64, value: &str) {
    fold_u64(digest, value.len() as u64);
    for byte in value.as_bytes() {
        *digest ^= u64::from(*byte);
        *digest = digest.wrapping_mul(0x100_0000_01b3);
    }
}

fn fold_u64(digest: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        *digest ^= u64::from(byte);
        *digest = digest.wrapping_mul(0x100_0000_01b3);
    }
}

struct IntentDeclarationCursor<'a> {
    atoms: &'a [WorthUiArtifactInputBodyAtom],
    cursor: usize,
}

impl<'a> IntentDeclarationCursor<'a> {
    fn new(atoms: &'a [WorthUiArtifactInputBodyAtom]) -> Self {
        Self { atoms, cursor: 0 }
    }

    fn take_identifier(&mut self) -> Result<String, WorthUiIntentDeclarationParseError> {
        match self.atoms.get(self.cursor) {
            Some(WorthUiArtifactInputBodyAtom::Identifier(value)) => {
                self.cursor += 1;
                Ok(value.clone())
            }
            _ => Err(error("intent declaration expected an identifier")),
        }
    }

    fn take_string_literal(&mut self) -> Result<String, WorthUiIntentDeclarationParseError> {
        match self.atoms.get(self.cursor) {
            Some(WorthUiArtifactInputBodyAtom::StringLiteral(value)) => {
                self.cursor += 1;
                Ok(value.clone())
            }
            _ => Err(error("intent declaration expected a string literal")),
        }
    }

    fn take_optional_semicolon(&mut self) {
        if matches!(
            self.atoms.get(self.cursor),
            Some(WorthUiArtifactInputBodyAtom::Semicolon)
        ) {
            self.cursor += 1;
        }
    }

    fn is_eof(&self) -> bool {
        self.cursor == self.atoms.len()
    }
}
