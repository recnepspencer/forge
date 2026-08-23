from worth_ui_3141_p5_contracts import P5_COUNTERS, P5_MUTATIONS
from worth_ui_3141_p6_contracts import P6_COUNTERS, P6_MUTATIONS


MUTATIONS = {
    "P1-AFFINITY-01": ("affinity", "stale-predecessor"),
    "P1-AUTHORITY-01": ("construction", "public-construction"),
    "P1-BACKEND-FEATURES-01": ("backend-feature", "vulkan-default"),
    "P1-BASELINE-01": ("baseline", "forged-known-empty"),
    "P1-CLOSE-01": ("ledger", "open-requirement"),
    "P1-CONSUMERS-01": ("validated-agreement", "agreement-validation-bypass"),
    "P1-DAMAGE-01": ("damage", "widened-damage"),
    "P1-HEADLESS-01": ("mechanics-substitution", "performed-external-effect"),
    "P1-HEADLESS-COST-01": ("carrier-inflation", "unchanged-carriage"),
    "P1-ORDER-01": ("paint-order", "identity-tie-break"),
    "P1-ORDER-SOURCE-01": ("identity-perturbation", "public-ordering"),
    "P1-PLATFORM-AUTHORITY-01": ("grant-forgery", "downstream-bind"),
    "P1-PREPARATION-LIFECYCLE-01": ("premature-runtime-effect", "host-during-prepare"),
    "P1-PRESENTATION-AUTHORITY-01": ("work-forgery", "external-work-issue"),
    "P1-PRODUCER-01": ("delta-carriage", "dropped-removal"),
    "P1-PRODUCER-COST-01": ("carrier-inflation", "unchanged-payload"),
    "P1-PROFILE-01": ("manifest-field", "qualified-capacity-drift"),
    "P1-PROTOCOL-01": ("protocol-revision", "mixed-revision"),
    "P1-TOPOLOGY-01": ("hidden-edge", "target-dependency-alias"),
    "P1-WORLDS-01": ("oracle-substitution", "damage-and-order-mutants"),
    "P2-APPLICATION-01": ("driver-substitution", "fake-client"),
    "P2-CLOSE-01": ("resource-leak", "held-readback"),
    "P2-EVENT-LOOP-01": ("thread-substitution", "off-thread-run"),
    "P2-GRAPHICS-01": ("backend-substitution", "vulkan-or-small-limit"),
    "P2-PIXELS-01": ("expected-pixel-substitution", "wrong-client-pixel"),
    "P2-PORTS-01": ("scripted-port-substitution", "indeterminate-as-before-effects"),
    "P2-PRESENT-01": ("geometry-color-substitution", "geometry-or-color-drift"),
    "P2-READINESS-01": ("wake-drop-duplicate", "duplicate-generation"),
    "P2-WINDOW-01": ("window-substitution", "dpi-basis-drift"),
    "P2-WORLD-01": ("world-substitution", "os-backend-client-or-close"),
    "P3-PREDECESSOR-01": ("stale-predecessor", "stale-phase-two-source"),
    "P3-BASELINE-REPLAY-01": ("baseline-substitution", "opaque-baseline-clear"),
    "P3-CLIPPED-DELTA-01": ("effect-posture", "zero-paint-as-indeterminate"),
    "P3-CLOSE-01": ("ledger", "open-requirement"),
    "P3-DAMAGE-INDEX-01": ("retained-scan", "full-retained-scan"),
    "P3-DAMAGE-REPLAY-01": ("vacated-replay", "omitted-vacated-replay"),
    "P3-DELTA-SOURCE-01": ("delta-rediscovery", "successor-rediscovery"),
    "P3-DRAW-LIST-01": ("retained-clone", "complete-map-clone"),
    "P3-HEADLESS-COST-01": ("retained-clone", "complete-transcript-clone"),
    "P3-HP02-WORLD-01": ("world-substitution", "synthetic-successor"),
    "P3-PHYSICAL-AMPLIFICATION-01": ("local-present-relabel", "hidden-full-surface-copy"),
    "P3-PRODUCER-SLOPE-01": ("retained-scan", "complete-successor-scan"),
    "P3-RECONSTRUCTION-01": ("derived-state-reuse", "stale-derived-state"),
    "P3-STALE-DELTA-01": ("stale-delta", "stale-affinity-acceptance"),
    "P3-TOTAL-ORDER-01": ("identity-tie-break", "identity-ordering"),
    "P3-TRANSACTION-01": ("premature-commit", "commit-before-handoff"),
    "P3-UNCHANGED-01": ("epoch-mint", "fresh-unchanged-epoch"),
    "P4-FONT-COLLECTION-01": (
        "font-resolution-authority",
        "ambient-or-single-family-or-stale-generation-or-registration-order-substitution",
    ),
    "P4-PREDECESSOR-01": ("stale-predecessor", "stale-phase-three-source"),
    "P4-TEXT-PROFILE-01": ("profile-drift", "font-or-unicode-digest-drift"),
    "P4-COLOR-FONT-ADMISSION-01": ("unsupported-color-table", "unsupported-svg-or-layer-drop"),
    "P4-UNICODE-SEGMENTATION-01": ("emoji-sequence-split", "zwj-or-flag-split"),
    "P4-EMOJI-SEQUENCE-01": ("emoji-sequence-decomposition", "variation-or-zwj-decomposition"),
    "P4-BIDI-01": ("visual-source-order", "logical-order-rendering"),
    "P4-FALLBACK-01": ("cluster-split", "emoji-or-indic-split"),
    "P4-SHAPING-01": ("glyph-substitution", "one-run-latin"),
    "P4-LINE-LAYOUT-01": ("cluster-break", "mid-cluster-wrap"),
    "P4-CAPACITY-01": ("post-admission-overflow", "shape-before-capacity-denial"),
    "P4-MEASUREMENT-IDENTITY-01": ("duplicate-shaper", "independent-measurement-pass"),
    "P4-ORIGINAL-RANGE-01": ("range-normalization", "normalized-offset-substitution"),
    "P4-BIDI-INTERACTION-01": ("caret-affinity-swap", "swapped-bidi-caret-affinity"),
    "P4-ACCESSIBILITY-GEOMETRY-01": ("accessibility-reshape", "accessibility-reshape"),
    "P4-TEXT-CONTENT-LOCALITY-01": ("content-global-rescan", "content-only-global-rescan"),
    "P4-TEXT-WIDTH-LOCALITY-01": ("paragraph-width-global-rescan", "paragraph-width-global-rescan"),
    "P4-TEXT-RECONSTRUCTION-01": ("derived-state-reuse", "stale-layout-reuse"),
    "P4-UNCHANGED-01": ("unchanged-analysis", "unchanged-paragraph-rescan"),
    "P4-TEXT-COST-01": ("paragraph-rescan", "complete-document-rescan"),
    "P4-CLOSE-01": ("ledger", "open-requirement"),
}
MUTATIONS.update(P5_MUTATIONS)
MUTATIONS.update(P6_MUTATIONS)


COUNTERS = {
    "P1-AFFINITY-01": ("work", 3),
    "P1-AUTHORITY-01": ("preparation", 2),
    "P1-BACKEND-FEATURES-01": ("resolved-feature", 1),
    "P1-BASELINE-01": ("baseline", 1),
    "P1-CLOSE-01": ("requirements", 20),
    "P1-CONSUMERS-01": ("consumer", 2),
    "P1-DAMAGE-01": ("damage", 2),
    "P1-HEADLESS-01": ("headless", 1),
    "P1-HEADLESS-COST-01": ("carrier-cost", 0),
    "P1-ORDER-01": ("order", 2),
    "P1-ORDER-SOURCE-01": ("order-source", 2),
    "P1-PLATFORM-AUTHORITY-01": ("grant", 2),
    "P1-PREPARATION-LIFECYCLE-01": ("effect-surface", 0),
    "P1-PRESENTATION-AUTHORITY-01": ("authority", 2),
    "P1-PRODUCER-01": ("producer", 2),
    "P1-PRODUCER-COST-01": ("carrier-cost", 0),
    "P1-PROFILE-01": ("profile", 2),
    "P1-PROTOCOL-01": ("protocol", 4),
    "P1-TOPOLOGY-01": ("inventory", 27),
    "P1-WORLDS-01": ("world", 2048),
    "P2-APPLICATION-01": ("application", 1),
    "P2-CLOSE-01": ("resource-census", 0),
    "P2-EVENT-LOOP-01": ("event-loop", 1),
    "P2-GRAPHICS-01": ("graphics", 1),
    "P2-PIXELS-01": ("pixels", 3),
    "P2-PORTS-01": ("ports", 4),
    "P2-PRESENT-01": ("presentation", 1),
    "P2-READINESS-01": ("readiness", 1),
    "P2-WINDOW-01": ("window", 1),
    "P2-WORLD-01": ("world", 1),
    "P3-PREDECESSOR-01": ("requirements", 30),
    "P3-BASELINE-REPLAY-01": ("baseline-clears", 1),
    "P3-CLIPPED-DELTA-01": ("effect-free-successors", 1),
    "P3-CLOSE-01": ("requirements", 17),
    "P3-DAMAGE-INDEX-01": ("damage-probes", 2048),
    "P3-DAMAGE-REPLAY-01": ("replayed-commands", 2048),
    "P3-DELTA-SOURCE-01": ("source-rows", 1),
    "P3-DRAW-LIST-01": ("draw-list-mutations", 2048),
    "P3-HEADLESS-COST-01": ("retained-scans", 0),
    "P3-HP02-WORLD-01": ("worlds", 2),
    "P3-PHYSICAL-AMPLIFICATION-01": ("amplification-boundary", 1),
    "P3-PRODUCER-SLOPE-01": ("retained-scans", 0),
    "P3-RECONSTRUCTION-01": ("reconstructed-commands", 2),
    "P3-STALE-DELTA-01": ("stale-denials", 2),
    "P3-TOTAL-ORDER-01": ("order-mutations", 2),
    "P3-TRANSACTION-01": ("transactions", 1),
    "P3-UNCHANGED-01": ("unchanged-work", 0),
    "P4-FONT-COLLECTION-01": ("font-resolution-cases", 16),
    "P4-PREDECESSOR-01": ("requirements", 47),
    "P4-TEXT-PROFILE-01": ("qualified-assets", 34),
    "P4-COLOR-FONT-ADMISSION-01": ("color-formats", 4),
    "P4-UNICODE-SEGMENTATION-01": ("conformance-cases", 22048),
    "P4-EMOJI-SEQUENCE-01": ("rgi-sequences", 3953),
    "P4-BIDI-01": ("bidi-runs", 582553),
    "P4-FALLBACK-01": ("fallback-probes", 3953),
    "P4-SHAPING-01": ("shaped-glyphs", 15),
    "P4-LINE-LAYOUT-01": ("lines", 3),
    "P4-CAPACITY-01": ("denied-before-analysis", 3),
    "P4-MEASUREMENT-IDENTITY-01": ("layout-identities", 1),
    "P4-ORIGINAL-RANGE-01": ("original-ranges", 8),
    "P4-BIDI-INTERACTION-01": ("interaction-records", 29),
    "P4-ACCESSIBILITY-GEOMETRY-01": ("layout-identities", 1),
    "P4-TEXT-CONTENT-LOCALITY-01": ("analyzed-bytes", 13),
    "P4-TEXT-WIDTH-LOCALITY-01": ("relayout-paragraphs", 1),
    "P4-TEXT-RECONSTRUCTION-01": ("reconstructed-layouts", 1),
    "P4-UNCHANGED-01": ("unchanged-analysis", 0),
    "P4-TEXT-COST-01": ("retained-scans", 0),
    "P4-CLOSE-01": ("requirements", 21),
}
COUNTERS.update(P5_COUNTERS)
COUNTERS.update(P6_COUNTERS)


EXPECTED_IGNORED = {
    requirement: (
        requirement in {"P1-CLOSE-01", "P1-HEADLESS-COST-01", "P1-WORLDS-01"}
        or requirement.startswith(("P2-", "P3-", "P4-"))
    )
    for requirement in COUNTERS
}
for _requirement in COUNTERS:
    if _requirement.startswith("P3-"):
        EXPECTED_IGNORED[_requirement] = _requirement in {
            "P3-BASELINE-REPLAY-01", "P3-CLOSE-01", "P3-DAMAGE-REPLAY-01",
            "P3-DRAW-LIST-01", "P3-HP02-WORLD-01", "P3-PHYSICAL-AMPLIFICATION-01",
            "P3-PREDECESSOR-01", "P3-TRANSACTION-01", "P3-UNCHANGED-01",
            "P3-DELTA-SOURCE-01", "P3-HEADLESS-COST-01", "P3-PRODUCER-SLOPE-01",
        }
    if _requirement.startswith("P4-"):
        EXPECTED_IGNORED[_requirement] = _requirement in {
            "P4-PREDECESSOR-01", "P4-TEXT-PROFILE-01", "P4-FONT-COLLECTION-01",
            "P4-COLOR-FONT-ADMISSION-01", "P4-UNICODE-SEGMENTATION-01",
            "P4-EMOJI-SEQUENCE-01", "P4-BIDI-01", "P4-FALLBACK-01", "P4-CLOSE-01",
        }
    if _requirement.startswith("P5-"):
        EXPECTED_IGNORED[_requirement] = _requirement in {
            "P5-PREDECESSOR-01", "P5-ATLAS-01", "P5-ATLAS-PINNING-01",
            "P5-TEXT-PIXELS-01", "P5-TEXT-RECONSTRUCTION-01", "P5-TEXT-COST-01",
            "P5-TEXT-ASYNC-PRESENTATION-01", "P5-CLOSE-01",
        }
    if _requirement.startswith("P6-"):
        EXPECTED_IGNORED[_requirement] = _requirement in {
            "P6-PREDECESSOR-01", "P6-WINDOWS-WORLD-01", "P6-CLOSE-01",
        }
