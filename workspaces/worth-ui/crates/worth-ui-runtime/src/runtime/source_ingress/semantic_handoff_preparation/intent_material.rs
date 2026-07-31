use worth_ui_dsl::{
    UiDslSemanticFamily, WorthUiArtifactInputProvenance, WorthUiIntentInteractionFamily,
    WorthUiIntentInteractionRoute, WorthUiSealedSemanticPackage, WorthUiSemanticDeclaration,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthUiAuthoredIntentMaterial {
    declarations: Box<[WorthUiAuthoredIntentDeclaration]>,
    routes: Box<[WorthUiAuthoredIntentRoute]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiAuthoredIntentDeclaration {
    identity: Box<str>,
    definition_reference: Box<str>,
    interaction: WorthUiIntentInteractionFamily,
    expected_payload_schema: Option<WorthUiAuthoredIntentSchemaExpectation>,
    expected_outcome_schema: Option<WorthUiAuthoredIntentSchemaExpectation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiAuthoredIntentSchemaExpectation {
    identity: Box<str>,
    version: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiAuthoredIntentRoute {
    target_provenance_digest: u64,
    route: WorthUiIntentInteractionRoute,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiAuthoredIntentMaterialDenial {
    InvalidDeclaration {
        identity: Box<str>,
        reason: WorthUiAuthoredIntentDeclarationDenial,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiAuthoredIntentDeclarationDenial {
    UnexpectedSemanticSurface,
    MissingDefinition,
    DuplicateDefinition,
    MissingInteraction,
    DuplicateInteraction,
    InvalidInteraction,
    DuplicatePayloadSchema,
    InvalidPayloadSchema,
    DuplicateOutcomeSchema,
    InvalidOutcomeSchema,
    UnsupportedPostureToken(Box<str>),
}

struct WorthUiAuthoredIntentDeclarationDraft {
    identity: Box<str>,
    definition: Option<Box<str>>,
    interaction: Option<WorthUiIntentInteractionFamily>,
    payload_schema: Option<WorthUiAuthoredIntentSchemaExpectation>,
    outcome_schema: Option<WorthUiAuthoredIntentSchemaExpectation>,
}

pub(crate) fn prepare_authored_intent_material(
    package: &WorthUiSealedSemanticPackage,
) -> Result<WorthUiAuthoredIntentMaterial, WorthUiAuthoredIntentMaterialDenial> {
    let mut declarations = Vec::new();
    let mut routes = Vec::new();
    for module_id in package.module_ids() {
        let views = package
            .declaration_views(module_id)
            .expect("sealed package contains every canonical module");
        for view in views {
            match view.declaration() {
                WorthUiSemanticDeclaration::SemanticArtifact(artifact)
                    if artifact.declaration().family() == UiDslSemanticFamily::Intent =>
                {
                    declarations.push(parse_declaration(artifact.declaration())?);
                }
                WorthUiSemanticDeclaration::Component(component) => {
                    let target_provenance_digest = provenance_digest(view.provenance());
                    routes.extend(
                        component
                            .structure()
                            .interaction_routes()
                            .iter()
                            .cloned()
                            .map(|route| WorthUiAuthoredIntentRoute {
                                target_provenance_digest,
                                route,
                            }),
                    );
                }
                _ => {}
            }
        }
    }
    Ok(WorthUiAuthoredIntentMaterial {
        declarations: declarations.into_boxed_slice(),
        routes: routes.into_boxed_slice(),
    })
}

fn parse_declaration(
    declaration: &worth_ui_dsl::WorthUiSemanticArtifactDeclaration,
) -> Result<WorthUiAuthoredIntentDeclaration, WorthUiAuthoredIntentMaterialDenial> {
    let identity: Box<str> = declaration.key().as_str().into();
    validate_declaration_surface(declaration, &identity)?;
    let mut draft = WorthUiAuthoredIntentDeclarationDraft::new(identity);
    for token in declaration.posture_tokens() {
        draft.admit_posture_token(token.as_str())?;
    }
    draft.finish()
}

fn validate_declaration_surface(
    declaration: &worth_ui_dsl::WorthUiSemanticArtifactDeclaration,
    identity: &Box<str>,
) -> Result<(), WorthUiAuthoredIntentMaterialDenial> {
    if !declaration.published_aspects().is_empty()
        || !declaration.consumed_aspects().is_empty()
        || !declaration.structural_tokens().is_empty()
        || !declaration.support_tokens().is_empty()
    {
        return Err(invalid(
            identity.clone(),
            WorthUiAuthoredIntentDeclarationDenial::UnexpectedSemanticSurface,
        ));
    }
    Ok(())
}

impl WorthUiAuthoredIntentDeclarationDraft {
    fn new(identity: Box<str>) -> Self {
        Self {
            identity,
            definition: None,
            interaction: None,
            payload_schema: None,
            outcome_schema: None,
        }
    }

    fn admit_posture_token(
        &mut self,
        token: &str,
    ) -> Result<(), WorthUiAuthoredIntentMaterialDenial> {
        if token == "intent:standalone" {
            return Ok(());
        }
        if let Some(value) = token.strip_prefix("definition:") {
            return set_once(
                &self.identity,
                &mut self.definition,
                value.into(),
                WorthUiAuthoredIntentDeclarationDenial::DuplicateDefinition,
            );
        }
        if let Some(value) = token.strip_prefix("interaction:") {
            return self.admit_interaction(value);
        }
        if let Some(value) = token.strip_prefix("payload-schema:") {
            return self.admit_payload_schema(value);
        }
        if let Some(value) = token.strip_prefix("outcome-schema:") {
            return self.admit_outcome_schema(value);
        }
        Err(invalid(
            self.identity.clone(),
            WorthUiAuthoredIntentDeclarationDenial::UnsupportedPostureToken(token.into()),
        ))
    }

    fn admit_interaction(
        &mut self,
        authored: &str,
    ) -> Result<(), WorthUiAuthoredIntentMaterialDenial> {
        let family = WorthUiIntentInteractionFamily::parse(authored).ok_or_else(|| {
            invalid(
                self.identity.clone(),
                WorthUiAuthoredIntentDeclarationDenial::InvalidInteraction,
            )
        })?;
        set_once(
            &self.identity,
            &mut self.interaction,
            family,
            WorthUiAuthoredIntentDeclarationDenial::DuplicateInteraction,
        )
    }

    fn admit_payload_schema(
        &mut self,
        authored: &str,
    ) -> Result<(), WorthUiAuthoredIntentMaterialDenial> {
        let schema = self.parse_schema(
            authored,
            WorthUiAuthoredIntentDeclarationDenial::InvalidPayloadSchema,
        )?;
        set_once(
            &self.identity,
            &mut self.payload_schema,
            schema,
            WorthUiAuthoredIntentDeclarationDenial::DuplicatePayloadSchema,
        )
    }

    fn admit_outcome_schema(
        &mut self,
        authored: &str,
    ) -> Result<(), WorthUiAuthoredIntentMaterialDenial> {
        let schema = self.parse_schema(
            authored,
            WorthUiAuthoredIntentDeclarationDenial::InvalidOutcomeSchema,
        )?;
        set_once(
            &self.identity,
            &mut self.outcome_schema,
            schema,
            WorthUiAuthoredIntentDeclarationDenial::DuplicateOutcomeSchema,
        )
    }

    fn parse_schema(
        &self,
        authored: &str,
        denial: WorthUiAuthoredIntentDeclarationDenial,
    ) -> Result<WorthUiAuthoredIntentSchemaExpectation, WorthUiAuthoredIntentMaterialDenial> {
        parse_schema(authored).ok_or_else(|| invalid(self.identity.clone(), denial))
    }

    fn finish(
        self,
    ) -> Result<WorthUiAuthoredIntentDeclaration, WorthUiAuthoredIntentMaterialDenial> {
        let definition_reference = self.definition.ok_or_else(|| {
            invalid(
                self.identity.clone(),
                WorthUiAuthoredIntentDeclarationDenial::MissingDefinition,
            )
        })?;
        let interaction = self.interaction.ok_or_else(|| {
            invalid(
                self.identity.clone(),
                WorthUiAuthoredIntentDeclarationDenial::MissingInteraction,
            )
        })?;
        Ok(WorthUiAuthoredIntentDeclaration {
            identity: self.identity,
            definition_reference,
            interaction,
            expected_payload_schema: self.payload_schema,
            expected_outcome_schema: self.outcome_schema,
        })
    }
}

fn set_once<T>(
    identity: &Box<str>,
    slot: &mut Option<T>,
    value: T,
    denial: WorthUiAuthoredIntentDeclarationDenial,
) -> Result<(), WorthUiAuthoredIntentMaterialDenial> {
    if slot.replace(value).is_some() {
        return Err(invalid(identity.clone(), denial));
    }
    Ok(())
}

fn parse_schema(value: &str) -> Option<WorthUiAuthoredIntentSchemaExpectation> {
    let (identity, version) = value.rsplit_once('@')?;
    let version = version.parse::<u16>().ok()?;
    (!identity.is_empty() && version > 0).then(|| WorthUiAuthoredIntentSchemaExpectation {
        identity: identity.into(),
        version,
    })
}

fn invalid(
    identity: Box<str>,
    reason: WorthUiAuthoredIntentDeclarationDenial,
) -> WorthUiAuthoredIntentMaterialDenial {
    WorthUiAuthoredIntentMaterialDenial::InvalidDeclaration { identity, reason }
}

fn provenance_digest(provenance: &WorthUiArtifactInputProvenance) -> u64 {
    match provenance {
        WorthUiArtifactInputProvenance::ParsedSourceDeclaration {
            declaration_span,
            declaration_index,
            ..
        } => crate::declaration::authored_source_provenance_digest(
            declaration_span.module_id().as_str(),
            *declaration_index,
        ),
        WorthUiArtifactInputProvenance::RustAuthoredDeclaration {
            authored_module_path,
            declaration_index,
        } => crate::declaration::authored_source_provenance_digest(
            authored_module_path,
            *declaration_index,
        ),
    }
}

impl WorthUiAuthoredIntentMaterial {
    pub(crate) fn declarations(&self) -> &[WorthUiAuthoredIntentDeclaration] {
        &self.declarations
    }

    pub(crate) fn routes(&self) -> &[WorthUiAuthoredIntentRoute] {
        &self.routes
    }
}

impl WorthUiAuthoredIntentDeclaration {
    pub(crate) fn identity(&self) -> &str {
        &self.identity
    }

    pub(crate) fn definition_reference(&self) -> &str {
        &self.definition_reference
    }

    pub(crate) const fn interaction(&self) -> WorthUiIntentInteractionFamily {
        self.interaction
    }

    pub(crate) fn expected_payload_schema(
        &self,
    ) -> Option<&WorthUiAuthoredIntentSchemaExpectation> {
        self.expected_payload_schema.as_ref()
    }

    pub(crate) fn expected_outcome_schema(
        &self,
    ) -> Option<&WorthUiAuthoredIntentSchemaExpectation> {
        self.expected_outcome_schema.as_ref()
    }
}

impl WorthUiAuthoredIntentSchemaExpectation {
    pub(crate) fn identity(&self) -> &str {
        &self.identity
    }

    pub(crate) const fn version(&self) -> u16 {
        self.version
    }
}

impl WorthUiAuthoredIntentRoute {
    pub(crate) const fn target_provenance_digest(&self) -> u64 {
        self.target_provenance_digest
    }

    pub(crate) const fn route(&self) -> &WorthUiIntentInteractionRoute {
        &self.route
    }
}
