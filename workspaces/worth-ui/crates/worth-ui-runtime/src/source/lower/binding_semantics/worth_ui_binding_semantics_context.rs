use crate::capability::{
    AdmittedCapability, CapabilitySnapshot, CapabilitySupportId, CapabilitySupportPosture,
    CommandDescriptor, CommandId, CommandProjectionDescriptor, CommandProjectionId,
    FrozenThemeTokenEntry, FrozenViewBindingEntry, IconDescriptor, IconId, SupportRequirement,
    ThemeTokenId, ViewBindingId,
};
use crate::source::{
    WorthUiArtifactInputProvenance, WorthUiBindingDiagnostic, WorthUiBindingDiagnosticCode,
    WorthUiBindingSemanticsMetrics, WorthUiSourceModuleId,
};

pub(crate) type CommandResolution = (AdmittedCapability<CommandId>, CommandDescriptor);
pub(crate) type CommandProjectionResolution = (
    AdmittedCapability<CommandProjectionId>,
    CommandProjectionDescriptor,
);
pub(crate) type IconResolution = (AdmittedCapability<IconId>, IconDescriptor);

const COMMAND_REFERENCE_CODES: [WorthUiBindingDiagnosticCode; 4] = [
    WorthUiBindingDiagnosticCode::MissingSemanticCommandReference,
    WorthUiBindingDiagnosticCode::DeferredSemanticCommandReference,
    WorthUiBindingDiagnosticCode::UnsupportedSemanticCommandReference,
    WorthUiBindingDiagnosticCode::PlatformInternalSemanticCommandReference,
];
const COMMAND_PROJECTION_REFERENCE_CODES: [WorthUiBindingDiagnosticCode; 4] = [
    WorthUiBindingDiagnosticCode::MissingSemanticCommandProjectionReference,
    WorthUiBindingDiagnosticCode::DeferredSemanticCommandProjectionReference,
    WorthUiBindingDiagnosticCode::UnsupportedSemanticCommandProjectionReference,
    WorthUiBindingDiagnosticCode::PlatformInternalSemanticCommandProjectionReference,
];
const VIEW_BINDING_REFERENCE_CODES: [WorthUiBindingDiagnosticCode; 4] = [
    WorthUiBindingDiagnosticCode::MissingSemanticViewBindingReference,
    WorthUiBindingDiagnosticCode::DeferredSemanticViewBindingReference,
    WorthUiBindingDiagnosticCode::UnsupportedSemanticViewBindingReference,
    WorthUiBindingDiagnosticCode::PlatformInternalSemanticViewBindingReference,
];
const THEME_TOKEN_REFERENCE_CODES: [WorthUiBindingDiagnosticCode; 4] = [
    WorthUiBindingDiagnosticCode::MissingSemanticThemeTokenReference,
    WorthUiBindingDiagnosticCode::DeferredSemanticThemeTokenReference,
    WorthUiBindingDiagnosticCode::UnsupportedSemanticThemeTokenReference,
    WorthUiBindingDiagnosticCode::PlatformInternalSemanticThemeTokenReference,
];
pub(crate) type ViewBindingResolution = (AdmittedCapability<ViewBindingId>, FrozenViewBindingEntry);
pub(crate) type ThemeTokenResolution = (AdmittedCapability<ThemeTokenId>, FrozenThemeTokenEntry);

pub(crate) struct WorthUiBindingSemanticsContext<'snapshot> {
    snapshot: &'snapshot CapabilitySnapshot,
    metrics: WorthUiBindingSemanticsMetrics,
}

impl<'snapshot> WorthUiBindingSemanticsContext<'snapshot> {
    pub(crate) fn new(snapshot: &'snapshot CapabilitySnapshot) -> Self {
        Self {
            snapshot,
            metrics: WorthUiBindingSemanticsMetrics::default(),
        }
    }

    pub(crate) fn finish_metrics(self) -> WorthUiBindingSemanticsMetrics {
        self.metrics
    }

    pub(crate) fn record_query_owned_semantic_check(&mut self) {
        self.metrics.record_query_owned_semantic_check();
    }

    pub(crate) fn resolve_command(
        &mut self,
        module_id: &WorthUiSourceModuleId,
        authored_text: &str,
        semantic_locus: &str,
        provenance: &WorthUiArtifactInputProvenance,
    ) -> Result<CommandResolution, WorthUiBindingDiagnostic> {
        let command_id = CommandId::new(authored_text).map_err(|_| {
            diagnostic(
                WorthUiBindingDiagnosticCode::InvalidSemanticCommandReferenceId,
                module_id,
                authored_text,
                semantic_locus,
                provenance,
            )
        })?;
        let lookup = self.snapshot.index().commands().lookup(&command_id);
        self.metrics.record_lookup(lookup.counters());
        if let Some(descriptor) = lookup.into_value() {
            let posture = self
                .snapshot
                .support_catalog()
                .command_posture(&command_id)
                .expect("support catalog should contain command ids");
            if posture.is_admitted() {
                let admitted = SupportRequirement::admitted()
                    .check(posture)
                    .expect("admitted command should satisfy admitted support requirement");
                return Ok((admitted, descriptor.clone()));
            }
            return Err(postured_diagnostic(
                Some(posture),
                module_id,
                authored_text,
                semantic_locus,
                provenance,
                COMMAND_REFERENCE_CODES,
            ));
        }
        Err(postured_diagnostic(
            self.snapshot.support_catalog().command_posture(&command_id),
            module_id,
            authored_text,
            semantic_locus,
            provenance,
            COMMAND_REFERENCE_CODES,
        ))
    }

    pub(crate) fn resolve_command_projection(
        &mut self,
        module_id: &WorthUiSourceModuleId,
        command_projection_id: &CommandProjectionId,
        semantic_locus: &str,
        provenance: &WorthUiArtifactInputProvenance,
    ) -> Result<CommandProjectionResolution, WorthUiBindingDiagnostic> {
        let lookup = self
            .snapshot
            .index()
            .command_projections()
            .lookup(command_projection_id);
        self.metrics.record_lookup(lookup.counters());
        if let Some(descriptor) = lookup.into_value() {
            let posture = self
                .snapshot
                .support_catalog()
                .command_projection_posture(command_projection_id)
                .expect("support catalog should contain command projection ids");
            if posture.is_admitted() {
                let admitted = SupportRequirement::admitted().check(posture).expect(
                    "admitted command projection should satisfy admitted support requirement",
                );
                return Ok((admitted, descriptor.clone()));
            }
            return Err(postured_diagnostic(
                Some(posture),
                module_id,
                command_projection_id.as_str(),
                semantic_locus,
                provenance,
                COMMAND_PROJECTION_REFERENCE_CODES,
            ));
        }
        Err(postured_diagnostic(
            self.snapshot
                .support_catalog()
                .command_projection_posture(command_projection_id),
            module_id,
            command_projection_id.as_str(),
            semantic_locus,
            provenance,
            COMMAND_PROJECTION_REFERENCE_CODES,
        ))
    }

    pub(crate) fn resolve_icon(
        &mut self,
        module_id: &WorthUiSourceModuleId,
        icon_id: &IconId,
        semantic_locus: &str,
        provenance: &WorthUiArtifactInputProvenance,
        diagnostic_codes: [WorthUiBindingDiagnosticCode; 4],
    ) -> Result<IconResolution, WorthUiBindingDiagnostic> {
        let lookup = self.snapshot.index().icons().lookup(icon_id);
        self.metrics.record_lookup(lookup.counters());
        if let Some(descriptor) = lookup.into_value() {
            let posture = self
                .snapshot
                .support_catalog()
                .icon_posture(icon_id)
                .expect("support catalog should contain icon ids");
            if posture.is_admitted() {
                let admitted = SupportRequirement::admitted()
                    .check(posture)
                    .expect("admitted icon should satisfy admitted support requirement");
                return Ok((admitted, descriptor.clone()));
            }
            return Err(postured_diagnostic(
                Some(posture),
                module_id,
                icon_id.as_str(),
                semantic_locus,
                provenance,
                diagnostic_codes,
            ));
        }
        Err(postured_diagnostic(
            self.snapshot.support_catalog().icon_posture(icon_id),
            module_id,
            icon_id.as_str(),
            semantic_locus,
            provenance,
            diagnostic_codes,
        ))
    }

    pub(crate) fn resolve_view_binding(
        &mut self,
        module_id: &WorthUiSourceModuleId,
        authored_text: &str,
        semantic_locus: &str,
        provenance: &WorthUiArtifactInputProvenance,
    ) -> Result<ViewBindingResolution, WorthUiBindingDiagnostic> {
        let view_binding_id = ViewBindingId::new(authored_text).map_err(|_| {
            diagnostic(
                WorthUiBindingDiagnosticCode::InvalidSemanticViewBindingReferenceId,
                module_id,
                authored_text,
                semantic_locus,
                provenance,
            )
        })?;
        let lookup = self
            .snapshot
            .index()
            .view_bindings()
            .lookup(&view_binding_id);
        self.metrics.record_lookup(lookup.counters());
        if lookup.into_value().is_some() {
            let posture = self
                .snapshot
                .support_catalog()
                .view_binding_posture(&view_binding_id)
                .expect("support catalog should contain view binding ids");
            if posture.is_admitted() {
                let admitted = SupportRequirement::admitted()
                    .check(posture)
                    .expect("admitted view binding should satisfy admitted support requirement");
                let entry = self
                    .snapshot
                    .view_bindings()
                    .get_entry(&view_binding_id)
                    .expect("frozen view binding entry should exist for descriptor");
                return Ok((admitted, entry.clone()));
            }
            return Err(postured_diagnostic(
                Some(posture),
                module_id,
                authored_text,
                semantic_locus,
                provenance,
                VIEW_BINDING_REFERENCE_CODES,
            ));
        }
        Err(postured_diagnostic(
            self.snapshot
                .support_catalog()
                .view_binding_posture(&view_binding_id),
            module_id,
            authored_text,
            semantic_locus,
            provenance,
            VIEW_BINDING_REFERENCE_CODES,
        ))
    }

    pub(crate) fn resolve_theme_token(
        &mut self,
        module_id: &WorthUiSourceModuleId,
        authored_text: &str,
        semantic_locus: &str,
        provenance: &WorthUiArtifactInputProvenance,
    ) -> Result<ThemeTokenResolution, WorthUiBindingDiagnostic> {
        let theme_token_id = ThemeTokenId::new(authored_text).map_err(|_| {
            diagnostic(
                WorthUiBindingDiagnosticCode::InvalidSemanticThemeTokenReferenceId,
                module_id,
                authored_text,
                semantic_locus,
                provenance,
            )
        })?;
        let lookup = self.snapshot.index().theme_tokens().lookup(&theme_token_id);
        self.metrics.record_lookup(lookup.counters());
        if lookup.into_value().is_some() {
            let posture = self
                .snapshot
                .support_catalog()
                .theme_token_posture(&theme_token_id)
                .expect("support catalog should contain theme token ids");
            if posture.is_admitted() {
                let admitted = SupportRequirement::admitted()
                    .check(posture)
                    .expect("admitted theme token should satisfy admitted support requirement");
                let entry = self
                    .snapshot
                    .theme_tokens()
                    .get_entry(&theme_token_id)
                    .expect("frozen theme token entry should exist for descriptor");
                return Ok((admitted, entry.clone()));
            }
            return Err(postured_diagnostic(
                Some(posture),
                module_id,
                authored_text,
                semantic_locus,
                provenance,
                THEME_TOKEN_REFERENCE_CODES,
            ));
        }
        Err(postured_diagnostic(
            self.snapshot
                .support_catalog()
                .theme_token_posture(&theme_token_id),
            module_id,
            authored_text,
            semantic_locus,
            provenance,
            THEME_TOKEN_REFERENCE_CODES,
        ))
    }
}

fn postured_diagnostic<T: CapabilitySupportId>(
    posture: Option<CapabilitySupportPosture<T>>,
    module_id: &WorthUiSourceModuleId,
    authored_text: &str,
    semantic_locus: &str,
    provenance: &WorthUiArtifactInputProvenance,
    diagnostic_codes: [WorthUiBindingDiagnosticCode; 4],
) -> WorthUiBindingDiagnostic {
    let [missing_code, deferred_code, unsupported_code, platform_internal_code] = diagnostic_codes;
    diagnostic(
        match posture {
            Some(posture) if posture.is_deferred() => deferred_code,
            Some(posture) if posture.is_unsupported() => unsupported_code,
            Some(posture) if posture.is_platform_internal() => platform_internal_code,
            _ => missing_code,
        },
        module_id,
        authored_text,
        semantic_locus,
        provenance,
    )
}

fn diagnostic(
    code: WorthUiBindingDiagnosticCode,
    module_id: &WorthUiSourceModuleId,
    authored_text: &str,
    semantic_locus: &str,
    provenance: &WorthUiArtifactInputProvenance,
) -> WorthUiBindingDiagnostic {
    WorthUiBindingDiagnostic::new(
        code,
        module_id.clone(),
        authored_text,
        semantic_locus,
        provenance.clone(),
    )
}
