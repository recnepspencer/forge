use crate::source::{WorthUiArtifactInputBodyAtom, WorthUiSemanticArtifactDeclaration};
use crate::{UiDslPostureToken, UiDslSemanticFamily, UiDslSemanticKey};

use super::WorthUiIntentInteractionFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiIntentDeclarationSpec {
    identity: UiDslSemanticKey,
    definition_reference: Box<str>,
    interaction: WorthUiIntentInteractionFamily,
    expected_payload_schema: Option<WorthUiIntentSchemaExpectation>,
    expected_outcome_schema: Option<WorthUiIntentSchemaExpectation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WorthUiIntentSchemaExpectation {
    identity: Box<str>,
    version: u16,
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
        let mut declaration =
            WorthUiSemanticArtifactDeclaration::new(self.identity, UiDslSemanticFamily::Intent)
                .with_posture_token(UiDslPostureToken::new("intent:standalone"))
                .with_posture_token(UiDslPostureToken::new(format!(
                    "definition:{}",
                    self.definition_reference
                )))
                .with_posture_token(UiDslPostureToken::new(format!(
                    "interaction:{}",
                    self.interaction.as_str()
                )));
        if let Some(schema) = self.expected_payload_schema {
            declaration = declaration.with_posture_token(UiDslPostureToken::new(format!(
                "payload-schema:{}@{}",
                schema.identity, schema.version
            )));
        }
        if let Some(schema) = self.expected_outcome_schema {
            declaration = declaration.with_posture_token(UiDslPostureToken::new(format!(
                "outcome-schema:{}@{}",
                schema.identity, schema.version
            )));
        }
        declaration
    }

    pub(crate) fn parse_file_authored(
        identity: &str,
        body: &[WorthUiArtifactInputBodyAtom],
    ) -> Result<Self, WorthUiIntentDeclarationParseError> {
        let mut cursor = IntentDeclarationCursor::new(body);
        let mut definition = None;
        let mut interaction = None;
        while !cursor.is_eof() {
            match cursor.take_identifier()?.as_str() {
                "definition" => set_once(&mut definition, cursor.take_identifier()?, "definition")?,
                "interaction" => {
                    let authored = cursor.take_identifier()?;
                    let family = WorthUiIntentInteractionFamily::parse(&authored)
                        .ok_or_else(|| error(format!("unknown interaction family `{authored}`")))?;
                    set_once(&mut interaction, family, "interaction")?;
                }
                clause => {
                    return Err(error(format!(
                        "unknown intent declaration clause `{clause}`"
                    )));
                }
            }
            cursor.take_optional_semicolon();
        }
        Ok(Self::new(
            identity,
            definition.ok_or_else(|| error("intent declaration requires `definition`"))?,
            interaction.ok_or_else(|| error("intent declaration requires `interaction`"))?,
        ))
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
