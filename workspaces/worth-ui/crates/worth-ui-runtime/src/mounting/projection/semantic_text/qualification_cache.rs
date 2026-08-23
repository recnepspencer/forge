use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub(in crate::mounting::projection) struct UiMountedTextQualificationCache {
    layouts: RefCell<HashMap<[u8; 32], Arc<worth_ui_text::UiQualifiedTextLayout>>>,
}

impl UiMountedTextQualificationCache {
    pub(super) fn qualify(
        &self,
        request: worth_ui_text::UiQualifiedTextLayoutRequest,
    ) -> Result<Arc<worth_ui_text::UiQualifiedTextLayout>, worth_ui_text::UiTextQualificationDenial>
    {
        let identity = request.identity().digest();
        if let Some(layout) = self.layouts.borrow().get(&identity) {
            return Ok(Arc::clone(layout));
        }
        let layout = Arc::new(request.qualify()?);
        self.layouts
            .borrow_mut()
            .insert(identity, Arc::clone(&layout));
        Ok(layout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_requests_share_layouts_without_collapsing_distinct_text() {
        let (fonts, _) = worth_ui_text::UiGlobalFontCollection::admit_qualified_profile().unwrap();
        let fonts = Arc::new(fonts);
        let cache = UiMountedTextQualificationCache::default();
        let repeated_request = request("same", Arc::clone(&fonts));

        let first = cache.qualify(repeated_request.clone()).unwrap();
        let repeated = cache.qualify(repeated_request).unwrap();
        let distinct = cache
            .qualify(request("different", Arc::clone(&fonts)))
            .unwrap();

        assert!(Arc::ptr_eq(&first, &repeated));
        assert!(!Arc::ptr_eq(&first, &distinct));
    }

    fn request(
        source: &str,
        fonts: Arc<worth_ui_text::UiGlobalFontCollection>,
    ) -> worth_ui_text::UiQualifiedTextLayoutRequest {
        let source: Arc<str> = Arc::from(source);
        let constraints = worth_ui_text::UiTextParagraphConstraints::new(
            worth_ui_text::UiTextParagraphConstraintsInput {
                language: Arc::from("und"),
                base_direction: worth_ui_text::UiTextBaseDirection::Auto,
                wrap: worth_ui_text::UiTextWrap::UnicodeWord,
                alignment: worth_ui_text::UiTextAlignment::Start,
                overflow: worth_ui_text::UiTextOverflow::Clip,
                font_size_millipoints: 14_000,
                width_millipoints: 160_000,
                line_height_millipoints: 18_000,
                letter_spacing_millipoints: 0,
                word_spacing_millipoints: 0,
                tab_interval_millipoints: 56_000,
                maximum_lines: 1,
            },
        )
        .unwrap();
        let range = worth_ui_host_contract::UiTextOriginalRange::new(
            0,
            u32::try_from(source.len()).unwrap(),
        )
        .unwrap();
        let style = worth_ui_text::UiTextStyleSpan::new(
            range,
            worth_ui_text::UiTextStyle::from_paragraph_constraints(&constraints),
        )
        .unwrap();
        worth_ui_text::UiQualifiedTextLayoutRequest::new(
            worth_ui_text::UiTextParagraphAdmissionInput {
                source,
                constraints,
                profile_generation: worth_ui_host_contract::UiTextProfileGeneration::new(1)
                    .unwrap(),
                font_collection_generation: fonts.generation(),
                text_scale_generation: worth_ui_host_contract::UiTextScaleGeneration::new(1)
                    .unwrap(),
                styles: Box::new([style]),
            },
            fonts,
        )
    }
}
