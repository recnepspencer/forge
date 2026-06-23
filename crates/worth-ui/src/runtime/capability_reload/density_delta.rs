use std::collections::BTreeMap;

use crate::capability::{
    CapabilitySnapshot, DensityTokenId, FrozenDensityCapabilities,
    WorthUiDensityAcceptedRegistrationProof, WorthUiDensityPostureValue,
    WorthUiDensityTokenDescriptor, WorthUiDensityValue, WorthUiLengthValue, WorthUiPaddingValue,
    WorthUiSpacingValue,
};
use crate::runtime::{WorthUiRuntimeFactId, WorthUiRuntimeFactSet};

use super::{
    WorthUiCapabilityFamilyDelta, WorthUiCapabilityReloadFamilyCounters,
    WorthUiCapabilityReloadFamilyKind, WorthUiCapabilityReloadStage, WorthUiDensityReloadPackage,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiDensityDelta {
    snapshot: CapabilitySnapshot,
    touched_density_count: usize,
    changed_density_count: usize,
    canonicalization_count: usize,
    descriptor_lookup_count: usize,
    density_family_entry_count: usize,
    changed_facts: WorthUiRuntimeFactSet,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiDensityDeltaDenial {
    stage: WorthUiCapabilityReloadStage,
    detail: String,
    counters: WorthUiCapabilityReloadFamilyCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DensitySourceAssignment {
    token_id: DensityTokenId,
    raw_value: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WorthUiCanonicalDensityReload {
    descriptors: Vec<WorthUiDensityTokenDescriptor>,
    canonical_digest: u64,
    changed_token_ids: Vec<DensityTokenId>,
    touched_descriptor_count: usize,
    canonicalization_count: usize,
    descriptor_lookup_count: usize,
}

impl WorthUiDensityDelta {
    pub(crate) fn derive(
        active_snapshot: &CapabilitySnapshot,
        package: &WorthUiDensityReloadPackage,
    ) -> Result<Self, WorthUiDensityDeltaDenial> {
        let canonical = WorthUiCanonicalDensityReload::from_package(active_snapshot, package)?;
        let density_family_entry_count = canonical.descriptors.len();
        let accepted = WorthUiDensityAcceptedRegistrationProof::from_identity_texts(
            canonical
                .descriptors
                .iter()
                .map(|descriptor| descriptor.id().as_str().to_owned())
                .collect(),
        );
        let density_tokens =
            FrozenDensityCapabilities::from_accepted_descriptors(canonical.descriptors, &accepted);
        let mut changed_facts = WorthUiRuntimeFactSet::empty();
        changed_facts.extend(
            canonical
                .changed_token_ids
                .iter()
                .map(WorthUiRuntimeFactId::density_token),
        );

        Ok(Self {
            snapshot: active_snapshot.with_density_tokens_replaced(density_tokens),
            touched_density_count: canonical.touched_descriptor_count,
            changed_density_count: canonical.changed_token_ids.len(),
            canonicalization_count: canonical.canonicalization_count,
            descriptor_lookup_count: canonical.descriptor_lookup_count,
            density_family_entry_count,
            changed_facts,
        })
    }

    pub(crate) fn into_family_delta(self) -> WorthUiCapabilityFamilyDelta {
        WorthUiCapabilityFamilyDelta::new(
            WorthUiCapabilityReloadFamilyKind::Density,
            self.snapshot,
            WorthUiCapabilityReloadFamilyCounters::new(
                1,
                self.canonicalization_count,
                self.touched_density_count,
                self.changed_density_count,
                self.density_family_entry_count,
                self.descriptor_lookup_count,
            ),
            self.changed_facts,
        )
    }
}

impl WorthUiDensityDeltaDenial {
    pub(crate) fn stage(&self) -> WorthUiCapabilityReloadStage {
        self.stage
    }

    pub(crate) fn detail(&self) -> String {
        self.detail.clone()
    }

    pub(crate) fn counters(&self) -> WorthUiCapabilityReloadFamilyCounters {
        self.counters
    }
}

impl WorthUiCanonicalDensityReload {
    fn from_package(
        active_snapshot: &CapabilitySnapshot,
        package: &WorthUiDensityReloadPackage,
    ) -> Result<Self, WorthUiDensityDeltaDenial> {
        let assignments = parse_assignments(package.source_text())?;
        let mut descriptors = active_snapshot.density_tokens().entries().to_vec();
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
                    format!("unknown density token `{}`", assignment.token_id),
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
            .map_err(|detail| {
                source_parse_denial(
                    detail,
                    assignments.len(),
                    assignment_index,
                    assignment_index + 1,
                    descriptors.len(),
                )
            })?;
            digest = fold_bytes(digest, value.digest_basis().as_bytes());
            if previous.value() != &value {
                changed_token_ids.push(assignment.token_id.clone());
            }
            descriptors[index] = WorthUiDensityTokenDescriptor::define(
                assignment.token_id.clone(),
                previous.family().clone(),
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
) -> Result<Vec<DensitySourceAssignment>, WorthUiDensityDeltaDenial> {
    let mut seen = BTreeMap::<DensityTokenId, String>::new();
    let mut assignments = Vec::new();
    for (line_index, line) in source_text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((raw_id, raw_value)) = trimmed.split_once('=') else {
            return Err(parse_denial(format!(
                "line {} is not `density.token = value`",
                line_index + 1
            )));
        };
        let token_id = DensityTokenId::new(raw_id.trim()).map_err(|_| {
            parse_denial(format!(
                "line {} has invalid density token id",
                line_index + 1
            ))
        })?;
        let value = raw_value.trim().trim_matches('"').to_owned();
        if value.is_empty() {
            return Err(parse_denial(format!(
                "density token `{token_id}` cannot have an empty value"
            )));
        }
        if let Some(previous) = seen.get(&token_id) {
            let detail = if previous == &value {
                format!("duplicate density token `{token_id}`")
            } else {
                format!("conflicting density token edits for `{token_id}`")
            };
            return Err(parse_denial(detail));
        }
        seen.insert(token_id.clone(), value.clone());
        assignments.push(DensitySourceAssignment {
            token_id,
            raw_value: value,
        });
    }
    Ok(assignments)
}

fn parse_value(
    previous: &WorthUiDensityValue,
    raw_value: &str,
    token_id: &DensityTokenId,
) -> Result<WorthUiDensityValue, String> {
    match previous {
        WorthUiDensityValue::Padding(_) => WorthUiPaddingValue::from_shorthand_px(raw_value)
            .map(WorthUiDensityValue::padding)
            .map_err(|_| format!("density token `{token_id}` expects padding")),
        WorthUiDensityValue::Spacing(_) => WorthUiSpacingValue::from_px(raw_value)
            .map(WorthUiDensityValue::spacing)
            .map_err(|_| format!("density token `{token_id}` expects spacing")),
        WorthUiDensityValue::HitTargetMinimum(_) => WorthUiLengthValue::from_px(raw_value)
            .map(WorthUiDensityValue::hit_target_minimum)
            .map_err(|_| format!("density token `{token_id}` expects a length")),
        WorthUiDensityValue::Posture(_) => parse_posture(raw_value, token_id),
    }
}

fn parse_posture(
    raw_value: &str,
    token_id: &DensityTokenId,
) -> Result<WorthUiDensityValue, String> {
    let posture = match raw_value {
        "compact" => WorthUiDensityPostureValue::compact(),
        "comfortable" => WorthUiDensityPostureValue::comfortable(),
        "dense" => WorthUiDensityPostureValue::dense(),
        _ => {
            return Err(format!(
                "density token `{token_id}` expects posture `compact|comfortable|dense`"
            ));
        }
    };
    Ok(WorthUiDensityValue::posture(posture))
}

fn parse_denial(detail: String) -> WorthUiDensityDeltaDenial {
    WorthUiDensityDeltaDenial {
        stage: WorthUiCapabilityReloadStage::DensitySourceParse,
        detail,
        counters: WorthUiCapabilityReloadFamilyCounters::new(1, 0, 0, 0, 0, 0),
    }
}

fn source_parse_denial(
    detail: String,
    touched_descriptor_count: usize,
    canonicalization_count: usize,
    descriptor_lookup_count: usize,
    density_family_entry_count: usize,
) -> WorthUiDensityDeltaDenial {
    WorthUiDensityDeltaDenial {
        stage: WorthUiCapabilityReloadStage::DensitySourceParse,
        detail,
        counters: WorthUiCapabilityReloadFamilyCounters::new(
            1,
            canonicalization_count,
            touched_descriptor_count,
            0,
            density_family_entry_count,
            descriptor_lookup_count,
        ),
    }
}

fn admission_denial(
    detail: String,
    touched_descriptor_count: usize,
    canonicalization_count: usize,
    descriptor_lookup_count: usize,
    density_family_entry_count: usize,
) -> WorthUiDensityDeltaDenial {
    WorthUiDensityDeltaDenial {
        stage: WorthUiCapabilityReloadStage::DensityAdmission,
        detail,
        counters: WorthUiCapabilityReloadFamilyCounters::new(
            1,
            canonicalization_count,
            touched_descriptor_count,
            0,
            density_family_entry_count,
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
