use unicode_bidi::{data_source::BidiMatchedOpeningBracket, BidiClass, BidiDataSource};

include!(concat!(env!("OUT_DIR"), "/unicode_17_bidi.rs"));

pub(crate) struct UiUnicode17BidiData;

impl BidiDataSource for UiUnicode17BidiData {
    fn bidi_class(&self, character: char) -> BidiClass {
        let code = u32::from(character);
        range_lookup(EXPLICIT_BIDI, code).unwrap_or_else(|| {
            DEFAULT_BIDI
                .iter()
                .rev()
                .find(|(start, end, _)| *start <= code && code <= *end)
                .map_or(BidiClass::L, |(_, _, class)| *class)
        })
    }

    fn bidi_matched_opening_bracket(&self, character: char) -> Option<BidiMatchedOpeningBracket> {
        let code = u32::from(character);
        BIDI_BRACKETS
            .binary_search_by_key(&code, |(candidate, _, _)| *candidate)
            .ok()
            .and_then(|index| {
                let (_, opening, is_open) = BIDI_BRACKETS[index];
                char::from_u32(opening)
                    .map(|opening| BidiMatchedOpeningBracket { opening, is_open })
            })
    }
}

fn range_lookup(ranges: &[(u32, u32, BidiClass)], code: u32) -> Option<BidiClass> {
    let candidate = ranges.partition_point(|(start, _, _)| *start <= code);
    candidate.checked_sub(1).and_then(|index| {
        let (start, end, class) = ranges[index];
        (start <= code && code <= end).then_some(class)
    })
}
