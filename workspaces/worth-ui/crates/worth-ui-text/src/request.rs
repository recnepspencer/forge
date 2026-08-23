use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::UiTextParagraphAdmissionInput;
pub use worth_ui_host_contract::UiQualifiedTextLayoutRequestIdentity;

#[derive(Clone)]
pub struct UiQualifiedTextLayoutRequest {
    identity: UiQualifiedTextLayoutRequestIdentity,
    input: UiTextParagraphAdmissionInput,
    fonts: Arc<crate::UiGlobalFontCollection>,
}

impl UiQualifiedTextLayoutRequest {
    pub fn new(
        input: UiTextParagraphAdmissionInput,
        fonts: Arc<crate::UiGlobalFontCollection>,
    ) -> Self {
        Self {
            identity: identity_for_input_and_collection(&input, fonts.identity_digest()),
            input,
            fonts,
        }
    }

    pub const fn identity(&self) -> UiQualifiedTextLayoutRequestIdentity {
        self.identity
    }

    pub fn qualify(self) -> Result<crate::UiQualifiedTextLayout, crate::UiTextQualificationDenial> {
        crate::qualification::qualify_request(self)
    }

    pub(crate) fn fonts(&self) -> &Arc<crate::UiGlobalFontCollection> {
        &self.fonts
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        UiTextParagraphAdmissionInput,
        Arc<crate::UiGlobalFontCollection>,
    ) {
        (self.input, self.fonts)
    }
}

pub(crate) fn identity_for_input(
    input: &UiTextParagraphAdmissionInput,
) -> UiQualifiedTextLayoutRequestIdentity {
    identity_for_input_and_collection(input, [0; 32])
}

fn identity_for_input_and_collection(
    input: &UiTextParagraphAdmissionInput,
    collection_identity: [u8; 32],
) -> UiQualifiedTextLayoutRequestIdentity {
    let constraints = &input.constraints;
    let mut hash = Sha256::new();
    hash.update(b"worth-ui-qualified-text-layout-request-v1\0");
    hash.update(collection_identity);
    hash_bytes(&mut hash, input.source.as_bytes());
    hash_bytes(&mut hash, constraints.language().as_bytes());
    hash.update([direction_rank(constraints.base_direction())]);
    hash.update([wrap_rank(constraints.wrap())]);
    hash.update([alignment_rank(constraints.alignment())]);
    hash.update([overflow_rank(constraints.overflow())]);
    hash.update(constraints.font_size_millipoints().to_le_bytes());
    hash.update(constraints.width_millipoints().to_le_bytes());
    hash.update(constraints.line_height_millipoints().to_le_bytes());
    hash.update(constraints.letter_spacing_millipoints().to_le_bytes());
    hash.update(constraints.word_spacing_millipoints().to_le_bytes());
    hash.update(constraints.tab_interval_millipoints().to_le_bytes());
    hash.update(constraints.maximum_lines().to_le_bytes());
    hash.update(input.profile_generation.get().to_le_bytes());
    hash.update(input.font_collection_generation.get().to_le_bytes());
    hash.update(input.text_scale_generation.get().to_le_bytes());
    hash.update(
        u64::try_from(input.styles.len())
            .expect("qualified style capacity fits u64")
            .to_le_bytes(),
    );
    for span in &input.styles {
        hash.update(span.original_range().start().to_le_bytes());
        hash.update(span.original_range().end().to_le_bytes());
        hash.update(span.style().identity_digest());
    }
    UiQualifiedTextLayoutRequestIdentity::from_text_mechanics(hash.finalize().into())
}

fn hash_bytes(hash: &mut Sha256, bytes: &[u8]) {
    hash.update(
        u64::try_from(bytes.len())
            .expect("qualified text capacity fits u64")
            .to_le_bytes(),
    );
    hash.update(bytes);
}

const fn direction_rank(value: crate::UiTextBaseDirection) -> u8 {
    match value {
        crate::UiTextBaseDirection::Auto => 0,
        crate::UiTextBaseDirection::LeftToRight => 1,
        crate::UiTextBaseDirection::RightToLeft => 2,
    }
}

const fn wrap_rank(value: crate::UiTextWrap) -> u8 {
    match value {
        crate::UiTextWrap::None => 0,
        crate::UiTextWrap::UnicodeWord => 1,
        crate::UiTextWrap::Grapheme => 2,
    }
}

const fn alignment_rank(value: crate::UiTextAlignment) -> u8 {
    match value {
        crate::UiTextAlignment::Start => 0,
        crate::UiTextAlignment::Center => 1,
        crate::UiTextAlignment::End => 2,
    }
}

const fn overflow_rank(value: crate::UiTextOverflow) -> u8 {
    match value {
        crate::UiTextOverflow::Clip => 0,
        crate::UiTextOverflow::Ellipsis => 1,
    }
}
