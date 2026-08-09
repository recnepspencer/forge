#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiBodyDefaultTextProfileIdentity(&'static str);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiBodyDefaultAtlasCapacities {
    pub pages: u8,
    pub page_width: u16,
    pub page_height: u16,
    pub entries: u16,
    pub texel_bytes: u32,
    pub glyph_width: u16,
    pub glyph_height: u16,
    pub staged_upload_bytes: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiUnsupportedBodyDefaultCodePoint {
    scalar: char,
    utf8_start: u32,
    utf8_end: u32,
}

pub const WORTH_UI_BODY_DEFAULT_FONT: &[u8] =
    include_bytes!("../assets/fonts/NotoSans-Regular.ttf");
pub const WORTH_UI_BODY_DEFAULT_LICENSE: &str = include_str!("../assets/fonts/OFL.txt");
pub const WORTH_UI_TEXT_PROFILE_MANIFEST: &str =
    include_str!("../profiles/worth-ui-body-default-v1.toml");

impl UiBodyDefaultTextProfileIdentity {
    pub const WORTH_UI_BODY_DEFAULT_V1: Self = Self("worth-ui-body-default-v1");

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl UiBodyDefaultAtlasCapacities {
    pub const QUALIFIED: Self = Self {
        pages: 4,
        page_width: 1_024,
        page_height: 1_024,
        entries: 4_096,
        texel_bytes: 4_194_304,
        glyph_width: 256,
        glyph_height: 256,
        staged_upload_bytes: 1_048_576,
    };
}

impl UiUnsupportedBodyDefaultCodePoint {
    pub fn first_in(text: &str) -> Option<Self> {
        text.char_indices().find_map(|(start, scalar)| {
            if (' '..='~').contains(&scalar) {
                return None;
            }
            let end = start.checked_add(scalar.len_utf8())?;
            Some(Self {
                scalar,
                utf8_start: u32::try_from(start).ok()?,
                utf8_end: u32::try_from(end).ok()?,
            })
        })
    }

    pub const fn scalar(self) -> char {
        self.scalar
    }

    pub const fn utf8_range(self) -> (u32, u32) {
        (self.utf8_start, self.utf8_end)
    }
}

#[cfg(test)]
mod tests {
    use super::UiUnsupportedBodyDefaultCodePoint;

    #[test]
    fn printable_basic_latin_is_the_exact_support_set() {
        assert_eq!(UiUnsupportedBodyDefaultCodePoint::first_in(" ~"), None);
        let denial = UiUnsupportedBodyDefaultCodePoint::first_in("Aé")
            .expect("non-Basic-Latin code point should stop");
        assert_eq!(denial.scalar(), 'é');
        assert_eq!(denial.utf8_range(), (1, 3));
    }
}
