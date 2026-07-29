#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in super::super) enum AuthoredMeaningDelta {
    None,
    ProvenanceOnly,
    Appearance,
    Layout,
    Structure,
    Reset,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in super::super) enum PixelDelta {
    Equal,
    Different,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in super::super) enum ExpectedChangePosture {
    ObservedNoChange,
    EvidenceOnly,
    Changed,
}

pub(in super::super) fn expected_change(
    meaning: AuthoredMeaningDelta,
    _pixels: PixelDelta,
) -> ExpectedChangePosture {
    match meaning {
        AuthoredMeaningDelta::None => ExpectedChangePosture::ObservedNoChange,
        AuthoredMeaningDelta::ProvenanceOnly => ExpectedChangePosture::EvidenceOnly,
        AuthoredMeaningDelta::Appearance
        | AuthoredMeaningDelta::Layout
        | AuthoredMeaningDelta::Structure
        | AuthoredMeaningDelta::Reset => ExpectedChangePosture::Changed,
    }
}

#[test]
fn semantic_posture_is_independent_of_pixel_difference() {
    for pixels in [
        PixelDelta::Equal,
        PixelDelta::Different,
        PixelDelta::Unavailable,
    ] {
        assert_eq!(
            expected_change(AuthoredMeaningDelta::None, pixels),
            ExpectedChangePosture::ObservedNoChange
        );
        assert_eq!(
            expected_change(AuthoredMeaningDelta::ProvenanceOnly, pixels),
            ExpectedChangePosture::EvidenceOnly
        );
        for meaning in [
            AuthoredMeaningDelta::Appearance,
            AuthoredMeaningDelta::Layout,
            AuthoredMeaningDelta::Structure,
            AuthoredMeaningDelta::Reset,
        ] {
            assert_eq!(
                expected_change(meaning, pixels),
                ExpectedChangePosture::Changed
            );
        }
    }
}

#[test]
fn pixel_only_change_does_not_invent_authored_semantic_change() {
    assert_eq!(
        expected_change(AuthoredMeaningDelta::None, PixelDelta::Different),
        ExpectedChangePosture::ObservedNoChange
    );
    assert_eq!(
        expected_change(AuthoredMeaningDelta::Appearance, PixelDelta::Equal),
        ExpectedChangePosture::Changed
    );
}
