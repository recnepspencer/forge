import json
from copy import deepcopy
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest import TestCase

from worth_ui_test_cost_evidence import test_cost_evidence_violations


class WorthUiTestCostEvidenceTests(TestCase):
    def test_eight_layers_and_mirrored_metrics_are_accepted(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            write_evidence(root, valid_evidence())
            self.assertEqual(test_cost_evidence_violations(root, config()), [])

    def test_missing_layer_and_decorative_metrics_are_rejected(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            evidence = valid_evidence()
            del evidence["opening"]["proof_cost_layers"]["linking"]
            evidence["opening"]["proof_cost_layers"]["retries"]["metrics"] = {}
            write_evidence(root, evidence)
            violations = test_cost_evidence_violations(root, config())
            details = "\n".join(item.detail for item in violations)
            self.assertIn("missing cost layer linking", details)
            self.assertIn("retries.metrics must be non-empty", details)

    def test_mirrored_metric_drift_is_rejected(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            evidence = valid_evidence()
            evidence["opening"]["proof_cost_layers"]["execution"]["metrics"][
                "application_contract_cases"
            ] = 9
            write_evidence(root, evidence)
            rules = {
                item.rule
                for item in test_cost_evidence_violations(root, config())
            }
            self.assertIn("test-cost-evidence-consistency", rules)

    def test_closing_must_preserve_metric_vocabulary(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            evidence = valid_evidence()
            evidence["closing"] = deepcopy(evidence["opening"])
            evidence["closing"]["proof_cost_layers"]["linking"]["metrics"] = {
                "different_metric": 1
            }
            write_evidence(root, evidence)
            rules = {
                item.rule
                for item in test_cost_evidence_violations(root, config())
            }
            self.assertIn("test-cost-evidence-comparability", rules)


def config() -> dict[str, str]:
    return {"test_cost_evidence": "cost.json"}


def valid_evidence() -> dict[str, object]:
    topology = {
        "workspace_cargo_targets": 20,
        "compile_contract_cargo_sessions": 2,
        "integration_test_targets": 9,
        "flake_retry_budget": 0,
        "test_retries_used": 0,
    }
    measurements = {
        "warm_application_contracts": {"executed_tests_per_sample": 53},
        "warm_filesystem_external_boundary": {"executed_tests_per_sample": 6},
    }
    retained = {
        "valid_measurement_target_files": 20,
        "valid_measurement_target_bytes": 200,
    }
    return {
        "schema_version": 2,
        "milestone": "3.10",
        "opening": {
            "topology": topology,
            "measurements": measurements,
            "retained_artifacts": retained,
            "proof_cost_layers": {
                "compilation": layer(
                    workspace_cargo_targets=20,
                    compile_contract_cargo_sessions=2,
                ),
                "linking": layer(integration_test_targets=9, linked_bytes=100),
                "immutable_world_construction": layer(reusable_blueprints=2),
                "isolated_delta_construction": layer(named_delta_families=0),
                "execution": layer(application_contract_cases=53),
                "external_startup": layer(filesystem_cases=6),
                "retained_artifacts": layer(files=20, bytes=200),
                "retries": layer(budget=0, used=0),
            },
        },
        "closing": None,
    }


def layer(**metrics: int) -> dict[str, object]:
    return {
        "owner": "fixture owner",
        "posture": "fixture posture",
        "evidence": ["fixture evidence"],
        "metrics": metrics,
    }


def write_evidence(root: Path, evidence: dict[str, object]) -> None:
    (root / "cost.json").write_text(json.dumps(evidence), encoding="utf-8")
