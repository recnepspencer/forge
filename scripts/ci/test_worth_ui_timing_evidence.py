import json
from copy import deepcopy
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest import TestCase

from worth_ui_timing_evidence import REQUIRED_MEASUREMENTS, timing_evidence_violations


class WorthUiTimingEvidenceTests(TestCase):
    def test_malformed_incomparable_and_unreviewed_regressions_are_rejected(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            config = {"timing_evidence": "timing.json"}
            evidence = valid_timing_evidence()
            evidence["closing"] = deepcopy(evidence["opening"])
            for measurement in evidence["closing"]["measurements"].values():
                measurement["samples_seconds"] = [1000.0, 1100.0, 1200.0]
                measurement["median_seconds"] = 1100.0
                measurement["comparison"] = "within_10_percent"
            write_json(root / "timing.json", evidence)
            violations = timing_evidence_violations(root, config)
            self.assertIn("timing-evidence-budget", {item.rule for item in violations})

            evidence["closing"]["reviewed_budget_amendments"] = {
                name: "reviewed fixture regression"
                for name in REQUIRED_MEASUREMENTS
            }
            for measurement in evidence["closing"]["measurements"].values():
                measurement["comparison"] = "reviewed_budget_amendment"
            write_json(root / "timing.json", evidence)
            self.assertEqual(timing_evidence_violations(root, config), [])

            evidence["closing"]["platform"] = "different-platform"
            write_json(root / "timing.json", evidence)
            violations = timing_evidence_violations(root, config)
            self.assertEqual({item.rule for item in violations}, {"timing-evidence-comparability"})

            evidence["opening"]["measurements"]["warm_fast_lane"]["median_seconds"] = 99.0
            write_json(root / "timing.json", evidence)
            violations = timing_evidence_violations(root, config)
            self.assertIn("timing-evidence", {item.rule for item in violations})


def valid_timing_evidence() -> dict[str, object]:
    measurements = {
        name: {
            "command": f"command {name}",
            "classification": classification,
            "target_directories": (
                ["target-root/a", "target-root/b", "target-root/c"]
                if classification == "cold"
                else ["target-root/warm", "target-root/warm", "target-root/warm"]
            ),
            "samples_seconds": [1.0, 2.0, 3.0],
            "median_seconds": 2.0,
            "comparison": "opening_baseline",
        }
        for name, classification in REQUIRED_MEASUREMENTS.items()
    }
    return {
        "schema_version": 1,
        "milestone": "3.9",
        "opening": {
            "captured_at": "2026-07-18T00:00:00Z",
            "git_commit": "fixture",
            "platform": "fixture-platform",
            "cargo": "cargo fixture",
            "rustc": "rustc fixture",
            "cargo_incremental": False,
            "compiler_cache": "disabled",
            "environment": {
                "operating_system": "fixture-os",
                "processor": "fixture-processor",
                "physical_memory_bytes": 1024,
                "worktree_state": "fixture",
                "isolated_target_root": "target-root",
            },
            "measurements": measurements,
        },
        "closing": None,
    }


def write_json(path: Path, value: object) -> None:
    path.write_text(json.dumps(value), encoding="utf-8")
