use std::collections::BTreeMap;

use crate::capability::{
    AppearanceTokenId, CapabilitySnapshot, FrozenAppearanceCapabilities, ThemeColorValue,
    WorthUiAppearanceAcceptedRegistrationProof, WorthUiAppearanceTokenDescriptor,
    WorthUiAppearanceValue, WorthUiBorderWidthValue, WorthUiCornerRadiusValue,
    WorthUiFontSizeValue, WorthUiLengthValue, WorthUiPaddingValue, WorthUiShadowValue,
    WorthUiSpacingValue,
};
use crate::runtime::{WorthUiRuntimeFactId, WorthUiRuntimeFactSet};

use super::{
    WorthUiAppearanceReloadPackage, WorthUiAppearanceShadowParseDenialCode,
    WorthUiCapabilityFamilyDelta, WorthUiCapabilityReloadDenialCode,
    WorthUiCapabilityReloadFamilyCounters, WorthUiCapabilityReloadFamilyKind,
    WorthUiCapabilityReloadStage,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiAppearanceDelta {
    snapshot: CapabilitySnapshot,
    touched_appearance_count: usize,
    changed_appearance_count: usize,
    canonicalization_count: usize,
    descriptor_lookup_count: usize,
    appearance_family_entry_count: usize,
    changed_facts: WorthUiRuntimeFactSet,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiAppearanceDeltaDenial {
    stage: WorthUiCapabilityReloadStage,
    detail: String,
    denial_code: Option<WorthUiCapabilityReloadDenialCode>,
    counters: WorthUiCapabilityReloadFamilyCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AppearanceSourceAssignment {
    token_id: AppearanceTokenId,
    raw_value: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WorthUiCanonicalAppearanceReload {
    descriptors: Vec<WorthUiAppearanceTokenDescriptor>,
    canonical_digest: u64,
    changed_token_ids: Vec<AppearanceTokenId>,
    touched_descriptor_count: usize,
    canonicalization_count: usize,
    descriptor_lookup_count: usize,
}

impl WorthUiAppearanceDelta {
    pub(crate) fn derive(
        active_snapshot: &CapabilitySnapshot,
        package: &WorthUiAppearanceReloadPackage,
    ) -> Result<Self, WorthUiAppearanceDeltaDenial> {
        let canonical = WorthUiCanonicalAppearanceReload::from_package(active_snapshot, package)?;
        let appearance_family_entry_count = canonical.descriptors.len();
        let accepted = WorthUiAppearanceAcceptedRegistrationProof::from_identity_texts(
            canonical
                .descriptors
                .iter()
                .map(|descriptor| descriptor.id().as_str().to_owned())
                .collect(),
        );
        let appearance_tokens = FrozenAppearanceCapabilities::from_accepted_descriptors(
            canonical.descriptors,
            &accepted,
        );
        let mut changed_facts = WorthUiRuntimeFactSet::empty();
        changed_facts.extend(
            canonical
                .changed_token_ids
                .iter()
                .map(WorthUiRuntimeFactId::appearance_token),
        );

        Ok(Self {
            snapshot: active_snapshot.with_appearance_tokens_replaced(appearance_tokens),
            touched_appearance_count: canonical.touched_descriptor_count,
            changed_appearance_count: canonical.changed_token_ids.len(),
            canonicalization_count: canonical.canonicalization_count,
            descriptor_lookup_count: canonical.descriptor_lookup_count,
            appearance_family_entry_count,
            changed_facts,
        })
    }

    pub(crate) fn into_family_delta(self) -> WorthUiCapabilityFamilyDelta {
        WorthUiCapabilityFamilyDelta::new(
            WorthUiCapabilityReloadFamilyKind::Appearance,
            self.snapshot,
            WorthUiCapabilityReloadFamilyCounters::new(
                1,
                self.canonicalization_count,
                self.touched_appearance_count,
                self.changed_appearance_count,
                self.appearance_family_entry_count,
                self.descriptor_lookup_count,
            ),
            self.changed_facts,
        )
    }
}

impl WorthUiAppearanceDeltaDenial {
    pub(crate) fn stage(&self) -> WorthUiCapabilityReloadStage {
        self.stage
    }

    pub(crate) fn detail(&self) -> String {
        self.detail.clone()
    }

    pub(crate) fn counters(&self) -> WorthUiCapabilityReloadFamilyCounters {
        self.counters
    }

    pub(crate) fn denial_code(&self) -> Option<WorthUiCapabilityReloadDenialCode> {
        self.denial_code
    }
}

impl WorthUiCanonicalAppearanceReload {
    fn from_package(
        active_snapshot: &CapabilitySnapshot,
        package: &WorthUiAppearanceReloadPackage,
    ) -> Result<Self, WorthUiAppearanceDeltaDenial> {
        let assignments = parse_assignments(package.source_text())?;
        let mut descriptors = active_snapshot.appearance_tokens().entries().to_vec();
        let indices = descriptors
            .iter()
            .enumerate()
            .map(|(index, descriptor)| (descriptor.id().clone(), index))
            .collect::<BTreeMap<_, _>>();
        let mut changed_token_ids = Vec::new();
        let mut digest = package.source_digest();

        for (assignment_index, assignment) in assignments.iter().enumerate() {
            let Some(index) = indices.get(&assignment.token_id).copied() else {
                return Err(admission_denial(
                    format!("unknown appearance token `{}`", assignment.token_id),
                    assignments.len(),
                    assignment_index,
                    assignment_index + 1,
                    descriptors.len(),
                ));
            };
            let previous = &descriptors[index];
            let value = parse_value(
                previous.value(),
                &assignment.raw_value,
                &assignment.token_id,
            )
            .map_err(|denial| {
                let mut parse_denial = source_parse_denial(
                    denial.detail,
                    assignments.len(),
                    assignment_index,
                    assignment_index + 1,
                    descriptors.len(),
                );
                parse_denial.denial_code = denial.denial_code;
                parse_denial
            })?;
            digest = fold_bytes(digest, value.digest_basis().as_bytes());
            if previous.value() != &value {
                changed_token_ids.push(assignment.token_id.clone());
            }
            descriptors[index] = WorthUiAppearanceTokenDescriptor::define(
                assignment.token_id.clone(),
                previous.family().clone(),
                previous.source().clone(),
                value,
            );
        }

        Ok(Self {
            descriptors,
            canonical_digest: digest,
            changed_token_ids,
            touched_descriptor_count: assignments.len(),
            canonicalization_count: assignments.len(),
            descriptor_lookup_count: assignments.len(),
        })
    }
}

fn parse_assignments(
    source_text: &str,
) -> Result<Vec<AppearanceSourceAssignment>, WorthUiAppearanceDeltaDenial> {
    let mut seen = BTreeMap::<AppearanceTokenId, String>::new();
    let mut assignments = Vec::new();
    for (line_index, line) in source_text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((raw_id, raw_value)) = trimmed.split_once('=') else {
            return Err(parse_denial(format!(
                "line {} is not `appearance.token = value`",
                line_index + 1
            )));
        };
        let token_id = AppearanceTokenId::new(raw_id.trim()).map_err(|_| {
            parse_denial(format!(
                "line {} has invalid appearance token id",
                line_index + 1
            ))
        })?;
        let value = raw_value.trim().trim_matches('"').to_owned();
        if value.is_empty() {
            return Err(parse_denial(format!(
                "appearance token `{token_id}` cannot have an empty value"
            )));
        }
        if let Some(previous) = seen.get(&token_id) {
            let detail = if previous == &value {
                format!("duplicate appearance token `{token_id}`")
            } else {
                format!("conflicting appearance token edits for `{token_id}`")
            };
            return Err(parse_denial(detail));
        }
        seen.insert(token_id.clone(), value.clone());
        assignments.push(AppearanceSourceAssignment {
            token_id,
            raw_value: value,
        });
    }
    Ok(assignments)
}

struct AppearanceValueParseDenial {
    detail: String,
    denial_code: Option<WorthUiCapabilityReloadDenialCode>,
}

fn parse_value(
    previous: &WorthUiAppearanceValue,
    raw_value: &str,
    token_id: &AppearanceTokenId,
) -> Result<WorthUiAppearanceValue, AppearanceValueParseDenial> {
    match previous {
        WorthUiAppearanceValue::Color(_) => ThemeColorValue::hex(raw_value)
            .map(WorthUiAppearanceValue::color)
            .map_err(|_| {
                plain_parse_denial(format!("appearance token `{token_id}` expects a color"))
            }),
        WorthUiAppearanceValue::Length(_) => WorthUiLengthValue::from_px(raw_value)
            .map(WorthUiAppearanceValue::length)
            .map_err(|_| {
                plain_parse_denial(format!("appearance token `{token_id}` expects a length"))
            }),
        WorthUiAppearanceValue::FontSize(_) => WorthUiFontSizeValue::from_px(raw_value)
            .map(WorthUiAppearanceValue::font_size)
            .map_err(|_| {
                plain_parse_denial(format!("appearance token `{token_id}` expects a font size"))
            }),
        WorthUiAppearanceValue::Padding(_) => WorthUiPaddingValue::from_shorthand_px(raw_value)
            .map(WorthUiAppearanceValue::padding)
            .map_err(|_| {
                plain_parse_denial(format!("appearance token `{token_id}` expects padding"))
            }),
        WorthUiAppearanceValue::Spacing(_) => WorthUiSpacingValue::from_px(raw_value)
            .map(WorthUiAppearanceValue::spacing)
            .map_err(|_| {
                plain_parse_denial(format!("appearance token `{token_id}` expects spacing"))
            }),
        WorthUiAppearanceValue::BorderWidth(_) => WorthUiBorderWidthValue::from_px(raw_value)
            .map(WorthUiAppearanceValue::border_width)
            .map_err(|_| {
                plain_parse_denial(format!(
                    "appearance token `{token_id}` expects a border width"
                ))
            }),
        WorthUiAppearanceValue::CornerRadius(_) => WorthUiCornerRadiusValue::from_px(raw_value)
            .map(WorthUiAppearanceValue::corner_radius)
            .map_err(|_| {
                plain_parse_denial(format!(
                    "appearance token `{token_id}` expects a corner radius"
                ))
            }),
        WorthUiAppearanceValue::Shadow(_) => parse_shadow(raw_value, token_id),
    }
}

fn parse_shadow(
    raw_value: &str,
    token_id: &AppearanceTokenId,
) -> Result<WorthUiAppearanceValue, AppearanceValueParseDenial> {
    let parts = raw_value.split_whitespace().collect::<Vec<_>>();
    if parts.len() != 5 {
        return Err(shadow_parse_denial(
            format!(
                "appearance token `{token_id}` expects `#rrggbbaa offset_x offset_y blur spread`"
            ),
            WorthUiAppearanceShadowParseDenialCode::InvalidArity,
        ));
    }
    let color = ThemeColorValue::hex(parts[0]).map_err(|_| {
        shadow_parse_denial(
            format!("appearance token `{token_id}` expects a shadow"),
            WorthUiAppearanceShadowParseDenialCode::InvalidColor,
        )
    })?;
    WorthUiShadowValue::from_authored_parts(color.clone(), parts[1], "0px", "0px", "0px").map_err(
        |_| {
            shadow_parse_denial(
                format!("appearance token `{token_id}` expects a shadow"),
                WorthUiAppearanceShadowParseDenialCode::InvalidOffsetX,
            )
        },
    )?;
    WorthUiShadowValue::from_authored_parts(color.clone(), "0px", parts[2], "0px", "0px").map_err(
        |_| {
            shadow_parse_denial(
                format!("appearance token `{token_id}` expects a shadow"),
                WorthUiAppearanceShadowParseDenialCode::InvalidOffsetY,
            )
        },
    )?;
    WorthUiShadowValue::from_authored_parts(color.clone(), "0px", "0px", parts[3], "0px").map_err(
        |_| {
            shadow_parse_denial(
                format!("appearance token `{token_id}` expects a shadow"),
                WorthUiAppearanceShadowParseDenialCode::InvalidBlur,
            )
        },
    )?;
    WorthUiShadowValue::from_authored_parts(color.clone(), "0px", "0px", "0px", parts[4]).map_err(
        |_| {
            shadow_parse_denial(
                format!("appearance token `{token_id}` expects a shadow"),
                WorthUiAppearanceShadowParseDenialCode::InvalidSpread,
            )
        },
    )?;
    WorthUiShadowValue::from_authored_parts(color, parts[1], parts[2], parts[3], parts[4])
        .map(WorthUiAppearanceValue::shadow)
        .map_err(|_| plain_parse_denial(format!("appearance token `{token_id}` expects a shadow")))
}

fn plain_parse_denial(detail: String) -> AppearanceValueParseDenial {
    AppearanceValueParseDenial {
        detail,
        denial_code: None,
    }
}

fn shadow_parse_denial(
    detail: String,
    code: WorthUiAppearanceShadowParseDenialCode,
) -> AppearanceValueParseDenial {
    AppearanceValueParseDenial {
        detail,
        denial_code: Some(WorthUiCapabilityReloadDenialCode::AppearanceShadow(code)),
    }
}

fn parse_denial(detail: String) -> WorthUiAppearanceDeltaDenial {
    WorthUiAppearanceDeltaDenial {
        stage: WorthUiCapabilityReloadStage::AppearanceSourceParse,
        detail,
        denial_code: None,
        counters: WorthUiCapabilityReloadFamilyCounters::new(1, 0, 0, 0, 0, 0),
    }
}

fn source_parse_denial(
    detail: String,
    touched_descriptor_count: usize,
    canonicalization_count: usize,
    descriptor_lookup_count: usize,
    appearance_family_entry_count: usize,
) -> WorthUiAppearanceDeltaDenial {
    WorthUiAppearanceDeltaDenial {
        stage: WorthUiCapabilityReloadStage::AppearanceSourceParse,
        detail,
        denial_code: None,
        counters: WorthUiCapabilityReloadFamilyCounters::new(
            1,
            canonicalization_count,
            touched_descriptor_count,
            0,
            appearance_family_entry_count,
            descriptor_lookup_count,
        ),
    }
}

fn admission_denial(
    detail: String,
    touched_descriptor_count: usize,
    canonicalization_count: usize,
    descriptor_lookup_count: usize,
    appearance_family_entry_count: usize,
) -> WorthUiAppearanceDeltaDenial {
    WorthUiAppearanceDeltaDenial {
        stage: WorthUiCapabilityReloadStage::AppearanceAdmission,
        detail,
        denial_code: None,
        counters: WorthUiCapabilityReloadFamilyCounters::new(
            1,
            canonicalization_count,
            touched_descriptor_count,
            0,
            appearance_family_entry_count,
            descriptor_lookup_count,
        ),
    }
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
