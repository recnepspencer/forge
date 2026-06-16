use std::collections::{BTreeMap, BTreeSet};

use crate::capability::{
    CapabilitySnapshot, FrozenThemeTokenCapabilities, ThemeColorValue,
    ThemeTokenAcceptedRegistrationProof, ThemeTokenDescriptor, ThemeTokenId, ThemeTokenValue,
};
use crate::runtime::{WorthUiRuntimeFactId, WorthUiRuntimeFactSet};

use super::{WorthUiCapabilityReloadStage, WorthUiThemeTokenReloadPackage};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiThemeTokenDelta {
    snapshot: CapabilitySnapshot,
    touched_theme_token_count: usize,
    registry_lookup_count: usize,
    theme_token_family_entry_count: usize,
    changed_facts: WorthUiRuntimeFactSet,
}

impl WorthUiThemeTokenDelta {
    pub(crate) fn derive(
        active_snapshot: &CapabilitySnapshot,
        package: &WorthUiThemeTokenReloadPackage,
    ) -> Result<Self, WorthUiThemeTokenDeltaDenial> {
        let parsed_tokens = parse_theme_token_assignments(package.source_text())?;
        let descriptors = replacement_descriptors(active_snapshot, &parsed_tokens)?;
        let theme_token_family_entry_count = descriptors.len();
        let accepted = ThemeTokenAcceptedRegistrationProof::from_identity_texts(
            descriptors
                .iter()
                .map(|descriptor| descriptor.id().as_str().to_owned())
                .collect(),
        );
        let theme_tokens =
            FrozenThemeTokenCapabilities::from_accepted_descriptors(descriptors, &accepted);
        let mut changed_facts = WorthUiRuntimeFactSet::empty();
        changed_facts.extend(
            parsed_tokens
                .iter()
                .map(|(token_id, _)| WorthUiRuntimeFactId::theme_token(token_id)),
        );
        Ok(Self {
            snapshot: active_snapshot.with_theme_tokens_replaced(theme_tokens),
            touched_theme_token_count: parsed_tokens.len(),
            registry_lookup_count: parsed_tokens.len(),
            theme_token_family_entry_count,
            changed_facts,
        })
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        CapabilitySnapshot,
        usize,
        usize,
        usize,
        WorthUiRuntimeFactSet,
    ) {
        (
            self.snapshot,
            self.touched_theme_token_count,
            self.theme_token_family_entry_count,
            self.registry_lookup_count,
            self.changed_facts,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiThemeTokenDeltaDenial {
    stage: WorthUiCapabilityReloadStage,
    detail: String,
}

impl WorthUiThemeTokenDeltaDenial {
    pub(crate) fn stage(&self) -> WorthUiCapabilityReloadStage {
        self.stage
    }

    pub(crate) fn detail(self) -> String {
        self.detail
    }
}

fn parse_theme_token_assignments(
    source_text: &str,
) -> Result<Vec<(ThemeTokenId, ThemeColorValue)>, WorthUiThemeTokenDeltaDenial> {
    let mut seen = BTreeSet::new();
    let mut assignments = Vec::new();
    for (line_index, line) in source_text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((raw_id, raw_value)) = trimmed.split_once('=') else {
            return Err(parse_denial(format!(
                "line {} is not `token = #rrggbb`",
                line_index + 1
            )));
        };
        let token_id = ThemeTokenId::new(raw_id.trim())
            .map_err(|_| parse_denial(format!("line {} has invalid token id", line_index + 1)))?;
        if !seen.insert(token_id.clone()) {
            return Err(parse_denial(format!("duplicate theme token `{token_id}`")));
        }
        let color = ThemeColorValue::hex(raw_value.trim())
            .map_err(|_| parse_denial(format!("invalid color for theme token `{token_id}`")))?;
        assignments.push((token_id, color));
    }
    Ok(assignments)
}

fn replacement_descriptors(
    active_snapshot: &CapabilitySnapshot,
    parsed_tokens: &[(ThemeTokenId, ThemeColorValue)],
) -> Result<Vec<ThemeTokenDescriptor>, WorthUiThemeTokenDeltaDenial> {
    let mut descriptors = active_snapshot
        .theme_tokens()
        .entries()
        .iter()
        .map(|entry| entry.descriptor().clone())
        .collect::<Vec<_>>();
    let token_indices = descriptors
        .iter()
        .enumerate()
        .map(|(index, descriptor)| (descriptor.id().clone(), index))
        .collect::<BTreeMap<_, _>>();
    for (token_id, color) in parsed_tokens {
        let Some(index) = token_indices.get(token_id).copied() else {
            return Err(admission_denial(format!(
                "unknown theme token `{token_id}`"
            )));
        };
        let previous = &descriptors[index];
        descriptors[index] = ThemeTokenDescriptor::define(
            token_id.clone(),
            previous.family().clone(),
            previous.source().clone(),
            ThemeTokenValue::color(color.clone()),
        );
    }
    Ok(descriptors)
}

fn parse_denial(detail: String) -> WorthUiThemeTokenDeltaDenial {
    WorthUiThemeTokenDeltaDenial {
        stage: WorthUiCapabilityReloadStage::ThemeTokenSourceParse,
        detail,
    }
}

fn admission_denial(detail: String) -> WorthUiThemeTokenDeltaDenial {
    WorthUiThemeTokenDeltaDenial {
        stage: WorthUiCapabilityReloadStage::ThemeTokenAdmission,
        detail,
    }
}
