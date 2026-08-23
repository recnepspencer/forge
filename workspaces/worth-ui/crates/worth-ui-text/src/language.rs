const GRANDFATHERED_TAGS: &[&str] = &[
    "art-lojban",
    "cel-gaulish",
    "en-GB-oed",
    "i-ami",
    "i-bnn",
    "i-default",
    "i-enochian",
    "i-hak",
    "i-klingon",
    "i-lux",
    "i-mingo",
    "i-navajo",
    "i-pwn",
    "i-tao",
    "i-tay",
    "i-tsu",
    "no-bok",
    "no-nyn",
    "sgn-BE-FR",
    "sgn-BE-NL",
    "sgn-CH-DE",
    "zh-guoyu",
    "zh-hakka",
    "zh-min",
    "zh-min-nan",
    "zh-xiang",
];

pub(crate) fn admit_language(language: &str) -> Option<std::sync::Arc<str>> {
    if let Some(canonical) = GRANDFATHERED_TAGS
        .iter()
        .find(|tag| tag.eq_ignore_ascii_case(language))
    {
        return Some(std::sync::Arc::from(*canonical));
    }
    LanguageTag::parse(language)?;
    Some(std::sync::Arc::from(canonicalize(language)))
}

struct LanguageTag<'tag> {
    subtags: Vec<&'tag str>,
    cursor: usize,
}

impl<'tag> LanguageTag<'tag> {
    fn parse(source: &'tag str) -> Option<Self> {
        let subtags = source.split('-').collect::<Vec<_>>();
        if subtags.is_empty() || subtags.iter().any(|subtag| subtag.is_empty()) {
            return None;
        }
        let mut tag = Self { subtags, cursor: 0 };
        if tag.consume_private_use() || tag.consume_language_tag() {
            (tag.cursor == tag.subtags.len()).then_some(tag)
        } else {
            None
        }
    }

    fn consume_language_tag(&mut self) -> bool {
        let Some(primary) = self.peek() else {
            return false;
        };
        if !canonical_lower_alpha(primary) || !(2..=8).contains(&primary.len()) {
            return false;
        }
        self.cursor += 1;
        if primary.len() <= 3 {
            self.consume_extlangs();
        }
        self.consume_script();
        self.consume_region();
        if !self.consume_variants() || !self.consume_extensions() {
            return false;
        }
        self.consume_private_use() || self.cursor == self.subtags.len()
    }

    fn consume_extlangs(&mut self) {
        for _ in 0..3 {
            if self
                .peek()
                .is_some_and(|part| part.len() == 3 && canonical_lower_alpha(part))
            {
                self.cursor += 1;
            } else {
                break;
            }
        }
    }

    fn consume_script(&mut self) {
        if self.peek().is_some_and(canonical_script) {
            self.cursor += 1;
        }
    }

    fn consume_region(&mut self) {
        if self.peek().is_some_and(canonical_region) {
            self.cursor += 1;
        }
    }

    fn consume_variants(&mut self) -> bool {
        let mut variants = Vec::new();
        while self.peek().is_some_and(canonical_variant) {
            let variant = self.subtags[self.cursor];
            if variants
                .iter()
                .any(|prior: &&str| prior.eq_ignore_ascii_case(variant))
            {
                return false;
            }
            variants.push(variant);
            self.cursor += 1;
        }
        true
    }

    fn consume_extensions(&mut self) -> bool {
        let mut singletons = Vec::new();
        while self.peek().is_some_and(extension_singleton) {
            let singleton = self.subtags[self.cursor];
            if singletons
                .iter()
                .any(|prior: &&str| prior.eq_ignore_ascii_case(singleton))
            {
                return false;
            }
            singletons.push(singleton);
            self.cursor += 1;
            let start = self.cursor;
            while self.peek().is_some_and(extension_part) {
                self.cursor += 1;
            }
            if self.cursor == start {
                return false;
            }
        }
        true
    }

    fn consume_private_use(&mut self) -> bool {
        if !self
            .peek()
            .is_some_and(|part| part.eq_ignore_ascii_case("x"))
        {
            return false;
        }
        let private_use_start = self.cursor;
        self.cursor += 1;
        let start = self.cursor;
        while self.peek().is_some_and(private_use_part) {
            self.cursor += 1;
        }
        if self.cursor > start {
            true
        } else {
            self.cursor = private_use_start;
            false
        }
    }

    fn peek(&self) -> Option<&'tag str> {
        self.subtags.get(self.cursor).copied()
    }
}

fn canonical_lower_alpha(part: &str) -> bool {
    part.bytes().all(|byte| byte.is_ascii_alphabetic())
}

fn canonical_lower_alphanumeric(part: &str) -> bool {
    part.bytes().all(|byte| byte.is_ascii_alphanumeric())
}

fn canonical_script(part: &str) -> bool {
    part.len() == 4 && part.bytes().all(|byte| byte.is_ascii_alphabetic())
}

fn canonical_region(part: &str) -> bool {
    (part.len() == 2 && part.bytes().all(|byte| byte.is_ascii_alphabetic()))
        || (part.len() == 3 && part.bytes().all(|byte| byte.is_ascii_digit()))
}

fn canonical_variant(part: &str) -> bool {
    canonical_lower_alphanumeric(part)
        && ((5..=8).contains(&part.len())
            || (part.len() == 4 && part.as_bytes()[0].is_ascii_digit()))
}

fn extension_singleton(part: &str) -> bool {
    part.len() == 1 && !part.eq_ignore_ascii_case("x") && canonical_lower_alphanumeric(part)
}

fn extension_part(part: &str) -> bool {
    (2..=8).contains(&part.len()) && canonical_lower_alphanumeric(part)
}

fn private_use_part(part: &str) -> bool {
    (1..=8).contains(&part.len()) && canonical_lower_alphanumeric(part)
}

fn canonicalize(source: &str) -> String {
    let mut parts = source.split('-').map(str::to_owned).collect::<Vec<_>>();
    if parts[0].eq_ignore_ascii_case("x") {
        parts
            .iter_mut()
            .for_each(|part| part.make_ascii_lowercase());
        return parts.join("-");
    }
    parts[0].make_ascii_lowercase();
    let mut cursor = 1;
    if parts[0].len() <= 3 {
        for _ in 0..3 {
            if parts
                .get(cursor)
                .is_some_and(|part| part.len() == 3 && canonical_lower_alpha(part))
            {
                parts[cursor].make_ascii_lowercase();
                cursor += 1;
            }
        }
    }
    if parts.get(cursor).is_some_and(|part| canonical_script(part)) {
        parts[cursor].make_ascii_lowercase();
        parts[cursor][..1].make_ascii_uppercase();
        cursor += 1;
    }
    if parts.get(cursor).is_some_and(|part| canonical_region(part)) {
        parts[cursor].make_ascii_uppercase();
        cursor += 1;
    }
    parts[cursor..]
        .iter_mut()
        .for_each(|part| part.make_ascii_lowercase());
    parts.join("-")
}

#[cfg(test)]
pub(crate) mod tests {
    use std::sync::Arc;

    use crate::{
        UiFontFamilyStack, UiTextAlignment, UiTextBaseDirection, UiTextFaceRequest, UiTextOverflow,
        UiTextParagraphConstraints, UiTextParagraphConstraintsInput, UiTextStyle, UiTextStyleInput,
        UiTextWrap,
    };

    #[test]
    pub(crate) fn paragraph_and_style_language_admission_share_strict_pinned_bcp47_contract() {
        for language in [
            "und",
            "en",
            "fr-CA",
            "zh-Hant-TW",
            "zh-cmn-Hans-CN",
            "es-419",
            "sl-rozaj-biske-1994",
            "en-US-u-ca-gregory",
            "de-CH-x-phonebk",
            "x-private",
            "en-GB-oed",
            "EN",
            "en-us",
            "en-latn-us",
        ] {
            assert!(
                constraints(language).is_some(),
                "paragraph rejected {language}"
            );
            assert!(style(language).is_some(), "style rejected {language}");
            assert!(
                harfrust::Language::new(language).is_some(),
                "shaping language rejected {language}"
            );
        }
        assert_eq!(constraints("EN").unwrap().language(), "en");
        assert_eq!(style("en-us").unwrap().language(), "en-US");
        assert_eq!(constraints("EN-latn-us").unwrap().language(), "en-Latn-US");
        for language in super::GRANDFATHERED_TAGS {
            assert!(
                constraints(language).is_some(),
                "paragraph rejected {language}"
            );
            assert!(style(language).is_some(), "style rejected {language}");
            assert!(harfrust::Language::new(language).is_some());
        }
        for language in [
            "",
            "-",
            "a--b",
            "-en",
            "en-",
            "e",
            "en-US-ABCD",
            "en-1234-1234",
            "en-u",
            "en-u-ca-u-nu-latn",
            "x",
            "en-x",
            "en_Us",
        ] {
            assert!(
                constraints(language).is_none(),
                "paragraph admitted {language}"
            );
            assert!(style(language).is_none(), "style admitted {language}");
        }
    }

    fn constraints(language: &str) -> Option<UiTextParagraphConstraints> {
        UiTextParagraphConstraints::new(UiTextParagraphConstraintsInput {
            language: Arc::from(language),
            base_direction: UiTextBaseDirection::Auto,
            wrap: UiTextWrap::UnicodeWord,
            alignment: UiTextAlignment::Start,
            overflow: UiTextOverflow::Clip,
            font_size_millipoints: 14_000,
            width_millipoints: 80_000,
            line_height_millipoints: 18_000,
            letter_spacing_millipoints: 0,
            word_spacing_millipoints: 0,
            tab_interval_millipoints: 56_000,
            maximum_lines: 4,
        })
    }

    fn style(language: &str) -> Option<UiTextStyle> {
        UiTextStyle::new(UiTextStyleInput {
            language: Arc::from(language),
            font_size_millipoints: 14_000,
            letter_spacing_millipoints: 0,
            word_spacing_millipoints: 0,
            family_stack: UiFontFamilyStack::profile_sans(),
            face_request: UiTextFaceRequest::regular(),
            features: Box::new([]),
            variations: Box::new([]),
        })
    }
}
