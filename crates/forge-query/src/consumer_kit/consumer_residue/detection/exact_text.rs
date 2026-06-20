use super::super::finding::{
    ForgeQueryConsumerResidueFinding, ForgeQueryConsumerResidueSourceSite,
};
use super::super::registry::{
    forge_query_consumer_residue_registry, ForgeQueryConsumerResidueClass,
    ForgeQueryConsumerResidueDetection,
};
use super::class_filter_allows;
use super::source_text_mask::mask_comments_and_string_literals;

pub(super) fn find_exact_text_residue(
    source_label: &str,
    source_path: &str,
    source: &str,
    class_filter: Option<&[ForgeQueryConsumerResidueClass]>,
) -> Vec<ForgeQueryConsumerResidueFinding> {
    let searchable = mask_comments_and_string_literals(source);
    forge_query_consumer_residue_registry()
        .iter()
        .filter(|row| row.detection() == ForgeQueryConsumerResidueDetection::ExactText)
        .filter(|row| class_filter_allows(class_filter, row.class()))
        .flat_map(|row| {
            source_locations_for_pattern(&searchable, row.detection_key()).map(
                move |(line, column)| {
                    ForgeQueryConsumerResidueFinding::discovered(
                        ForgeQueryConsumerResidueSourceSite::new(
                            source_label,
                            source_path,
                            line,
                            column,
                        ),
                        row.class(),
                        row.detection_key(),
                    )
                },
            )
        })
        .collect()
}

fn source_locations_for_pattern<'a>(
    source: &'a str,
    pattern: &'a str,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    source
        .match_indices(pattern)
        .map(|(byte_index, _)| source_location_for_byte_index(source, byte_index))
}

fn source_location_for_byte_index(source: &str, byte_index: usize) -> (usize, usize) {
    let before = &source[..byte_index];
    let line = before.bytes().filter(|byte| *byte == b'\n').count() + 1;
    let line_start = before.rfind('\n').map_or(0, |index| index + 1);
    (line, byte_index - line_start + 1)
}
