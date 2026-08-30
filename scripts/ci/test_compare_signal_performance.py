import copy
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import types
import unittest
from unittest import mock


SCRIPT = Path(__file__).with_name("compare_signal_performance.py")
SPEC = importlib.util.spec_from_file_location("signal_perf_compare", SCRIPT)
PERF = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PERF)


def sample(contract, probe, value=100, access_value=0):
    allocation = {
        "allocation_calls": value,
        "deallocation_calls": value,
        "allocated_bytes": value,
        "deallocated_bytes": value,
        "live_bytes": value,
        "end_live_bytes": value,
        "peak_live_bytes": None if probe == "ordinary" else value,
        "peak_live_status": (
            "unavailable: ordinary timing uses the unwrapped stats allocator"
            if probe == "ordinary"
            else "measured-group requested object high-water; instrumented realloc allocates/copies/frees"
        ),
    }
    metrics = {
        "allocation_metrics": allocation,
        "access_counters": {name: access_value for name in PERF.ACCESS_COUNTERS},
    }
    metrics.update({phase: value for phase in contract["phase_metrics"]})
    return {"elapsed_micros": value, "metrics": metrics}


def capture(value=100, access_value=0):
    cases = {}
    for probe in PERF.PROBES:
        records = []
        for suite, profile in PERF.CASES:
            policy, sample_count, warmups, phases, maxima = PERF.CONTRACTS[(suite, profile)]
            contract = {
                "suite": suite,
                "profile": profile,
                "executor": "serial",
                "timing_policy": policy,
                "phase_metrics": phases,
                "access_counter_maxima": maxima,
            }
            records.append({
                "contract": contract,
                "probe": probe,
                "sample_count": sample_count,
                "warmup_count": warmups,
                "workload_warmup": PERF.WORKLOAD_WARMUPS[(suite, profile)],
                "relative_budgets": PERF.expected_budgets(contract, probe),
                "samples": [sample(contract, probe, value, access_value)
                            for _ in range(sample_count)],
            })
        cases[probe] = records
    environment = {
        "os": "test-os", "arch": "test-arch", "cpu": "test-cpu", "host": "controlled-test-host",
        "logical_cpus": 1, "variables": {}, "rustc": "rustc test", "cargo": "cargo test",
        "workspace_profiles": {"release": {"opt-level": 3}}, "cargo_config": "[build]",
    }
    commands = {}
    for probe in PERF.PROBES:
        feature = "profile-extended" + (",test-peak-allocation" if probe == "peak" else "")
        commands[probe] = {"returncode": 0, "argv": [
            "cargo", "test", "--locked", "--release", "--manifest-path", str(Path("C:/baseline/Cargo.toml")),
            "--target-dir", str(Path("C:/evidence/target")), "-p", "worth-signal", "--lib",
            "--no-default-features", "--features", feature, PERF.FAMILY, "--", "--ignored",
            "--test-threads=1", "--nocapture", "--color=never",
        ]}
    return {
        "version": 1,
        "status": "captured",
        "configuration": PERF.configuration(),
        "environment": environment,
        "test_roster": PERF.TESTS,
        "commands": commands,
        "cases": cases,
    }


class ComparisonTests(unittest.TestCase):
    def test_complete_same_source_capture_passes_both_verdicts(self):
        report, code = PERF.compare(capture(), capture())
        self.assertEqual(code, 0)
        self.assertEqual(report["status"], "performance_pass")
        self.assertEqual(report["relative_verdict"], "within budgets")
        self.assertEqual(report["absolute_verdict"], "within contracts")

    def test_relative_boundary_and_regression_use_unchanged_allocation_budget(self):
        baseline = capture(1_000)
        at_boundary = capture(1_100)
        report, code = PERF.compare(baseline, at_boundary)
        self.assertEqual(code, 0, report)

        over_budget = capture(1_101)
        report, code = PERF.compare(baseline, over_budget)
        self.assertEqual(code, 1)
        self.assertTrue(any(item["metric"].endswith("allocated_bytes")
                            for item in report["relative_regressions"]))

    def test_absolute_baseline_debt_is_separate_from_relative_verdict(self):
        baseline = capture(access_value=1)
        report, code = PERF.compare(baseline, copy.deepcopy(baseline))
        self.assertEqual(code, 4)
        self.assertEqual(report["relative_verdict"], "within budgets")
        self.assertEqual(report["absolute_verdict"], "violation")
        self.assertEqual(report["status"], "rejected")

    def test_missing_case_metric_and_wrong_budget_are_invalid(self):
        missing_case = capture()
        missing_case["cases"]["ordinary"].pop()
        with self.assertRaisesRegex(ValueError, "twelve-case"):
            PERF.validate_capture(missing_case)

        missing_metric = capture()
        del missing_metric["cases"]["ordinary"][0]["samples"][0]["metrics"]["allocation_metrics"]["allocation_calls"]
        with self.assertRaisesRegex(ValueError, "metric roster"):
            PERF.validate_capture(missing_metric)

        wrong_budget = capture()
        wrong_budget["cases"]["ordinary"][0]["relative_budgets"]["elapsed_micros"]["median"] = 99
        with self.assertRaisesRegex(ValueError, "budget authority"):
            PERF.validate_capture(wrong_budget)

    def test_wrong_environment_and_corrupt_json_are_invalid(self):
        baseline, candidate = capture(), capture()
        candidate["environment"]["host"] = "different"
        with self.assertRaisesRegex(ValueError, "environment mismatch"):
            PERF.compare(baseline, candidate)
        with self.assertRaisesRegex(ValueError, "duplicate JSON key"):
            PERF.decode('{"status":"captured","status":"forged"}')

    def test_extra_command_flags_zero_peak_and_warmup_drift_are_invalid(self):
        wrong_command = capture()
        wrong_command["commands"]["ordinary"]["argv"][2:2] = ["--config", "profile.release.opt-level=0"]
        with self.assertRaisesRegex(ValueError, "argv length"):
            PERF.validate_capture(wrong_command)

        zero_peak = capture()
        for row in zero_peak["cases"]["peak"][0]["samples"]:
            row["metrics"]["allocation_metrics"]["peak_live_bytes"] = 0
        with self.assertRaisesRegex(ValueError, "tracker recorded zero"):
            PERF.validate_capture(zero_peak)

        warmup_drift = capture()
        warmup_drift["cases"]["ordinary"][0]["workload_warmup"] = "none"
        with self.assertRaisesRegex(ValueError, "Once warmup posture"):
            PERF.validate_capture(warmup_drift)

        malformed_environment = capture()
        malformed_environment["environment"]["logical_cpus"] = None
        with self.assertRaisesRegex(ValueError, "logical CPU"):
            PERF.validate_capture(malformed_environment)
        missing_cpu = capture()
        missing_cpu["environment"]["cpu"] = "unknown"
        with self.assertRaisesRegex(ValueError, "environment field: cpu"):
            PERF.validate_capture(missing_cpu)

    def test_capture_preserves_build_failure_as_noncomparison_evidence(self):
        with tempfile.TemporaryDirectory() as temporary:
            temporary = Path(temporary)
            args = types.SimpleNamespace(
                root=SCRIPT.parents[2],
                target_dir=temporary / "target",
                output=temporary / "capture.json",
            )
            failed = subprocess.CompletedProcess([], 23, stdout="compiler failed\n")
            with mock.patch.object(PERF, "environment", return_value={"host": "test"}), \
                    mock.patch.object(PERF.subprocess, "run", return_value=failed):
                self.assertEqual(PERF.capture(args), 3)
            evidence = json.loads(args.output.read_text())
            self.assertEqual(evidence["status"], "benchmark_failed")
            self.assertIn("build/list failed", evidence["error"])
            self.assertNotIn("cases", evidence["commands"])


if __name__ == "__main__":
    unittest.main()
