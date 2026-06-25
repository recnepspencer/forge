use crate::graph_read_access_inventory::WorthGraphReadDeclarationCandidate;

use super::lowering_errors::{
    WorthGraphReadTouchedAuthorityLoweringError, WorthGraphReadTouchedAuthorityLoweringErrorKind,
};
use super::operating_world::WorthGraphReadDeclarationOperatingWorld;
use super::query_touch_descriptor::WorthGraphReadQueryTouchDescriptorEvidence;
use super::source_family::WorthGraphReadTouchedAuthoritySourceFamily;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthGraphReadLoweredTouchedAuthority {
    source_family: WorthGraphReadTouchedAuthoritySourceFamily,
    touched_authority_input: String,
    query_touch_descriptor_digest: String,
    query_touch_collection_label: String,
    query_touch_read_verb_digest: String,
    operating_world_label: String,
    operating_world_digest: String,
}

impl WorthGraphReadLoweredTouchedAuthority {
    pub(crate) fn from_candidate(
        candidate: &WorthGraphReadDeclarationCandidate,
    ) -> Result<Self, WorthGraphReadTouchedAuthorityLoweringError> {
        let scope_binding = candidate.inventory_row_context().scope_binding();
        let source_family = WorthGraphReadTouchedAuthoritySourceFamily::from_scope_family(
            scope_binding.scope_family(),
        )
        .ok_or_else(|| {
            WorthGraphReadTouchedAuthorityLoweringError::new(
                WorthGraphReadTouchedAuthorityLoweringErrorKind::UnsupportedTouchedAuthorityScope,
            )
        })?;
        let Some(scope_authority_digest) = scope_binding.authority_digest() else {
            return Err(WorthGraphReadTouchedAuthorityLoweringError::new(
                WorthGraphReadTouchedAuthorityLoweringErrorKind::MissingTouchedAuthorityDigest,
            ));
        };
        if candidate.touched_authority_input().is_empty() {
            return Err(WorthGraphReadTouchedAuthorityLoweringError::new(
                WorthGraphReadTouchedAuthorityLoweringErrorKind::MissingTouchedAuthorityDigest,
            ));
        }
        if candidate.touched_authority_input() != scope_authority_digest {
            return Err(WorthGraphReadTouchedAuthorityLoweringError::new(
                WorthGraphReadTouchedAuthorityLoweringErrorKind::TouchedAuthorityDigestMismatch,
            ));
        }

        let touch_descriptor =
            WorthGraphReadQueryTouchDescriptorEvidence::from_lowered_authority_parts(
                source_family,
                candidate.read_family_target(),
            )?;
        let operating_world =
            WorthGraphReadDeclarationOperatingWorld::from_scope_binding(scope_binding)?;
        Ok(Self {
            source_family,
            touched_authority_input: scope_authority_digest.to_string(),
            query_touch_descriptor_digest: touch_descriptor.descriptor_digest().to_string(),
            query_touch_collection_label: touch_descriptor.collection_label().to_string(),
            query_touch_read_verb_digest: touch_descriptor.read_verb_digest().to_string(),
            operating_world_label: operating_world.selector_label().to_string(),
            operating_world_digest: operating_world.selector_digest().to_string(),
        })
    }

    pub const fn source_family(&self) -> WorthGraphReadTouchedAuthoritySourceFamily {
        self.source_family
    }

    pub fn source_family_label(&self) -> &str {
        self.source_family.as_str()
    }

    pub fn touched_authority_input(&self) -> &str {
        &self.touched_authority_input
    }

    pub fn query_touch_descriptor_digest(&self) -> &str {
        &self.query_touch_descriptor_digest
    }

    pub fn query_touch_collection_label(&self) -> &str {
        &self.query_touch_collection_label
    }

    pub fn query_touch_read_verb_digest(&self) -> &str {
        &self.query_touch_read_verb_digest
    }

    pub fn operating_world_label(&self) -> &str {
        &self.operating_world_label
    }

    pub fn operating_world_digest(&self) -> &str {
        &self.operating_world_digest
    }

    pub const fn claims_read_declaration_authority(&self) -> bool {
        true
    }

    pub const fn claims_selected_obligation_is_declaration_authority(&self) -> bool {
        false
    }

    pub const fn claims_execution_authority(&self) -> bool {
        false
    }
}
