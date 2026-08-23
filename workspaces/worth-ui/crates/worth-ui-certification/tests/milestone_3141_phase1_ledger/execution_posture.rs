use super::main_for;

pub(crate) fn counter_amount(requirement: &str) -> Option<u64> {
    Some(match requirement {
        "P1-AFFINITY-01" => 3,
        "P1-CLOSE-01" => 20,
        "P1-CONSUMERS-01"
        | "P1-AUTHORITY-01"
        | "P1-DAMAGE-01"
        | "P1-ORDER-01"
        | "P1-ORDER-SOURCE-01"
        | "P1-PLATFORM-AUTHORITY-01"
        | "P1-PRESENTATION-AUTHORITY-01"
        | "P1-PRODUCER-01"
        | "P1-PROFILE-01" => 2,
        "P1-PROTOCOL-01" => 4,
        "P1-HEADLESS-COST-01"
        | "P1-PRODUCER-COST-01"
        | "P1-PREPARATION-LIFECYCLE-01"
        | "P2-CLOSE-01" => 0,
        "P1-TOPOLOGY-01" => 25,
        "P1-WORLDS-01" | "P3-DAMAGE-INDEX-01" | "P3-DAMAGE-REPLAY-01" | "P3-DRAW-LIST-01" => 2_048,
        "P2-PIXELS-01" => 3,
        "P2-PORTS-01" => 4,
        "P3-PREDECESSOR-01" => 30,
        "P3-BASELINE-REPLAY-01"
        | "P3-TRANSACTION-01"
        | "P3-CLIPPED-DELTA-01"
        | "P3-DELTA-SOURCE-01"
        | "P3-PHYSICAL-AMPLIFICATION-01" => 1,
        "P3-CLOSE-01" => 17,
        "P3-HEADLESS-COST-01" | "P3-PRODUCER-SLOPE-01" | "P3-UNCHANGED-01" => 0,
        "P3-HP02-WORLD-01" | "P3-RECONSTRUCTION-01" | "P3-STALE-DELTA-01" | "P3-TOTAL-ORDER-01" => {
            2
        }
        "P4-FONT-COLLECTION-01" => 16,
        "P4-PREDECESSOR-01" => 47,
        "P4-TEXT-PROFILE-01" => 34,
        "P4-COLOR-FONT-ADMISSION-01" => 4,
        "P4-MEASUREMENT-IDENTITY-01"
        | "P4-ACCESSIBILITY-GEOMETRY-01"
        | "P4-TEXT-RECONSTRUCTION-01" => 1,
        "P4-UNICODE-SEGMENTATION-01" => 22_048,
        "P4-EMOJI-SEQUENCE-01" | "P4-FALLBACK-01" => 3_953,
        "P4-BIDI-01" => 582_553,
        "P4-SHAPING-01" => 15,
        "P4-LINE-LAYOUT-01" => 3,
        "P4-TEXT-WIDTH-LOCALITY-01" => 1,
        "P4-CAPACITY-01" => 3,
        "P4-ORIGINAL-RANGE-01" => 8,
        "P4-BIDI-INTERACTION-01" => 29,
        "P4-TEXT-CONTENT-LOCALITY-01" => 13,
        "P4-UNCHANGED-01" | "P4-TEXT-COST-01" => 0,
        "P4-CLOSE-01" => 21,
        "P5-PREDECESSOR-01" => 68,
        "P5-GLYPH-RASTER-01" => 2,
        "P5-COLOR-EMOJI-01" => 3953,
        "P5-ATLAS-01" => 1,
        "P5-ATLAS-PINNING-01" => 3,
        "P5-TEXT-DPI-01" => 1,
        "P5-TEXT-SPAN-PAINT-01" => 2,
        "P5-TEXT-PIXELS-01" => 2,
        "P5-TEXT-RECONSTRUCTION-01" => 7,
        "P5-TEXT-COST-01" => 32,
        "P5-TEXT-ASYNC-PRESENTATION-01" => 10,
        "P5-CLOSE-01" => 12,
        "P6-PREDECESSOR-01" => 80,
        "P6-INPUT-AFFINITY-01" => 2,
        "P6-IME-01" => 3,
        "P6-POINTER-TIME-01" => 1,
        "P6-PROFILE-ORDER-01" => 1,
        "P6-READINESS-01" => 2,
        "P6-SETTLEMENT-01" => 1,
        "P6-PROTOCOL-WORLD-01" => 177,
        "P6-WINDOWS-WORLD-01" => 1,
        "P6-CLOSE-01" => 10,
        _ if main_for(requirement).is_some() => 1,
        _ => return None,
    })
}

pub(crate) fn current_predecessor_counter_amount(requirement: &str) -> Option<u64> {
    match requirement {
        "P1-TOPOLOGY-01" => Some(27),
        _ => counter_amount(requirement),
    }
}

pub(crate) fn fault_boundary(requirement: &str) -> Option<&'static str> {
    Some(match requirement {
        requirement if requirement.starts_with("P1-") => "not-applicable",
        "P2-APPLICATION-01" | "P2-EVENT-LOOP-01" | "P2-GRAPHICS-01" | "P2-READINESS-01"
        | "P2-WINDOW-01" => "before-effects",
        requirement if requirement.starts_with("P2-") => "after-effects-may-have-begun",
        "P3-BASELINE-REPLAY-01"
        | "P3-DAMAGE-REPLAY-01"
        | "P3-HP02-WORLD-01"
        | "P3-PHYSICAL-AMPLIFICATION-01"
        | "P3-TRANSACTION-01" => "after-effects-may-have-begun",
        requirement if requirement.starts_with("P3-") => "not-applicable",
        "P4-TEXT-PROFILE-01"
        | "P4-FONT-COLLECTION-01"
        | "P4-COLOR-FONT-ADMISSION-01"
        | "P4-CAPACITY-01" => "before-effects",
        requirement if requirement.starts_with("P4-") => "not-applicable",
        "P5-GLYPH-RASTER-01"
        | "P5-COLOR-EMOJI-01"
        | "P5-ATLAS-01"
        | "P5-ATLAS-PINNING-01"
        | "P5-TEXT-DPI-01"
        | "P5-TEXT-SPAN-PAINT-01" => "before-effects",
        "P5-TEXT-ASYNC-PRESENTATION-01" => "after-effects-may-have-begun",
        requirement if requirement.starts_with("P5-") => "not-applicable",
        "P6-PREDECESSOR-01" => "predecessor-handoff-source-binding",
        "P6-INPUT-AFFINITY-01" => "input-admission-presentation-affinity",
        "P6-IME-01" => "ime-phase-classification",
        "P6-POINTER-TIME-01" => "pointer-event-time-witness",
        "P6-PROFILE-ORDER-01" => "profile-transition-admission",
        "P6-READINESS-01" => "readiness-commit-signal-consume",
        "P6-SETTLEMENT-01" => "typed-settlement-outcome-mapping",
        "P6-PROTOCOL-WORLD-01" => "protocol-production-oracle-comparison",
        "P6-WINDOWS-WORLD-01" => "windows-message-position-witness",
        "P6-CLOSE-01" => "phase-six-closure-source-prefix",
        _ => return None,
    })
}

pub(crate) fn main_budget_ms(requirement: &str) -> u64 {
    if requirement.starts_with("P2-") {
        30_000
    } else if matches!(
        requirement,
        "P3-DELTA-SOURCE-01" | "P3-HEADLESS-COST-01" | "P3-PRODUCER-SLOPE-01"
    ) {
        120_000
    } else if matches!(requirement, "P4-BIDI-01" | "P4-LINE-LAYOUT-01") {
        if requirement == "P4-BIDI-01" {
            180_000
        } else {
            120_000
        }
    } else if requirement == "P5-COLOR-EMOJI-01" {
        180_000
    } else if requirement == "P5-TEXT-COST-01" {
        570_000
    } else if requirement == "P5-TEXT-RECONSTRUCTION-01" {
        570_000
    } else if requirement == "P6-WINDOWS-WORLD-01" {
        300_000
    } else if matches!(
        requirement,
        "P5-TEXT-PIXELS-01" | "P5-TEXT-ASYNC-PRESENTATION-01"
    ) {
        300_000
    } else {
        60_000
    }
}

pub(crate) fn expected_declared_ignored(requirement: &str) -> bool {
    matches!(
        requirement,
        "P1-CLOSE-01" | "P1-HEADLESS-COST-01" | "P1-WORLDS-01"
    ) || requirement.starts_with("P2-")
        || matches!(
            requirement,
            "P3-BASELINE-REPLAY-01"
                | "P3-CLOSE-01"
                | "P3-DAMAGE-REPLAY-01"
                | "P3-DRAW-LIST-01"
                | "P3-HP02-WORLD-01"
                | "P3-PHYSICAL-AMPLIFICATION-01"
                | "P3-PREDECESSOR-01"
                | "P3-DELTA-SOURCE-01"
                | "P3-HEADLESS-COST-01"
                | "P3-PRODUCER-SLOPE-01"
                | "P3-TRANSACTION-01"
                | "P3-UNCHANGED-01"
        )
        || matches!(
            requirement,
            "P4-PREDECESSOR-01"
                | "P4-TEXT-PROFILE-01"
                | "P4-FONT-COLLECTION-01"
                | "P4-COLOR-FONT-ADMISSION-01"
                | "P4-UNICODE-SEGMENTATION-01"
                | "P4-EMOJI-SEQUENCE-01"
                | "P4-BIDI-01"
                | "P4-FALLBACK-01"
                | "P4-CLOSE-01"
        )
        || matches!(
            requirement,
            "P5-PREDECESSOR-01"
                | "P5-ATLAS-01"
                | "P5-ATLAS-PINNING-01"
                | "P5-TEXT-PIXELS-01"
                | "P5-TEXT-RECONSTRUCTION-01"
                | "P5-TEXT-COST-01"
                | "P5-TEXT-ASYNC-PRESENTATION-01"
                | "P5-CLOSE-01"
        )
        || matches!(
            requirement,
            "P6-PREDECESSOR-01" | "P6-WINDOWS-WORLD-01" | "P6-CLOSE-01"
        )
}

pub(crate) fn is_shared_main(requirement: &str) -> bool {
    requirement == "P1-HEADLESS-COST-01"
        || (requirement.starts_with("P2-") && requirement != "P2-WORLD-01")
        || matches!(
            requirement,
            "P3-BASELINE-REPLAY-01"
                | "P3-DAMAGE-REPLAY-01"
                | "P3-DRAW-LIST-01"
                | "P3-HEADLESS-COST-01"
                | "P3-PHYSICAL-AMPLIFICATION-01"
                | "P3-PRODUCER-SLOPE-01"
                | "P3-TRANSACTION-01"
                | "P3-UNCHANGED-01"
        )
}

pub(crate) fn control_budget_ms(requirement: &str) -> u64 {
    if matches!(
        requirement,
        "P3-DELTA-SOURCE-01" | "P3-HEADLESS-COST-01" | "P3-PRODUCER-SLOPE-01"
    ) {
        return 30_000;
    }
    if matches!(requirement, "P4-BIDI-01" | "P4-LINE-LAYOUT-01") {
        return if requirement == "P4-BIDI-01" {
            60_000
        } else {
            120_000
        };
    }
    if matches!(
        requirement,
        "P4-BIDI-INTERACTION-01"
            | "P4-COLOR-FONT-ADMISSION-01"
            | "P4-TEXT-COST-01"
            | "P4-TEXT-CONTENT-LOCALITY-01"
            | "P4-TEXT-WIDTH-LOCALITY-01"
            | "P5-ATLAS-PINNING-01"
    ) {
        return 30_000;
    }
    if requirement == "P5-COLOR-EMOJI-01" {
        return 60_000;
    }
    if requirement == "P5-TEXT-COST-01" {
        return 120_000;
    }
    if requirement == "P6-WINDOWS-WORLD-01" {
        return 60_000;
    }
    if matches!(
        requirement,
        "P3-PREDECESSOR-01"
            | "P4-PREDECESSOR-01"
            | "P4-FONT-COLLECTION-01"
            | "P4-MEASUREMENT-IDENTITY-01"
            | "P4-ACCESSIBILITY-GEOMETRY-01"
            | "P4-FALLBACK-01"
            | "P4-ORIGINAL-RANGE-01"
            | "P4-SHAPING-01"
            | "P4-TEXT-RECONSTRUCTION-01"
    ) {
        20_000
    } else {
        10_000
    }
}

#[test]
fn every_shared_phase_three_main_retains_its_declared_ignore_posture() {
    for requirement in super::super::schema::EXPECTED_REQUIREMENTS
        .iter()
        .filter(|requirement| requirement.starts_with("P3-"))
    {
        if is_shared_main(requirement) {
            assert!(expected_declared_ignored(requirement), "{requirement}");
        }
    }
}

#[test]
fn phase_five_raster_budgets_match_the_governed_worlds() {
    assert_eq!(main_budget_ms("P5-GLYPH-RASTER-01"), 60_000);
    assert_eq!(main_budget_ms("P5-COLOR-EMOJI-01"), 180_000);
    assert_eq!(control_budget_ms("P5-COLOR-EMOJI-01"), 60_000);
    assert_eq!(main_budget_ms("P5-TEXT-COST-01"), 570_000);
    assert_eq!(control_budget_ms("P5-TEXT-COST-01"), 120_000);
}

#[test]
fn real_dx12_atlas_main_retains_its_declared_ignore_posture() {
    assert!(expected_declared_ignored("P5-ATLAS-01"));
}
