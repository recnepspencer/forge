#!/usr/bin/env python3
"""Capture/compare the whole ignored Signal profile family; capture is not acceptance.
Usage: capture --root ROOT --target-dir EXTERNAL_BUILD --output NEW_EXTERNAL_JSON
       compare --baseline BASE_JSON --candidate CANDIDATE_JSON --output NEW_EXTERNAL_JSON
Runs ordinary then instrumented-peak release serially and never updates goldens.
Run on an idle host; repeat independent A/A before A/B. Exit: 0 pass/capture,
1 relative regression, 2 invalid, 3 benchmark failure, 4 absolute violation.
Peak is measured-group requested object bytes, not RSS or ordinary timing.
"""
import argparse
import json
import math
import os
from pathlib import Path
import platform
import re
import subprocess
import sys
import tomllib

FAMILY = "tests::performance_profiles::"
TEST_SUFFIXES = (
    "chain_bootstrap::perf_chain_10k_bootstrap_serial",
    "dependency_reconciliation_rotating::perf_dependency_reconciliation_rotating_window_serial",
    "dependency_reconciliation_stable_shape::perf_dependency_reconciliation_stable_shape_staged_serial",
    "dependency_reconciliation_staged::perf_dependency_reconciliation_rotating_window_staged_serial",
    "fintech_fanout::perf_fintech_mixed_fanout_profile_matrix",
    "observability_profile::perf_harness_observability_profile_delta",
    "suppression_fanout::perf_suppression_wide_fanout_serial",
    "topology_rewiring::perf_topology_rewiring_churn_serial",
    "topology_rewiring::perf_topology_rewiring_rotating_window_serial",
)
TESTS = [FAMILY + name for name in TEST_SUFFIXES]
CASES = [
    ("chain_10k_bootstrap", "balanced"),
    ("dependency_reconciliation_rotating_window", "balanced"),
    ("dependency_reconciliation_stable_shape_staged", "balanced"),
    ("dependency_reconciliation_rotating_window_staged", "balanced"),
    ("fintech_mixed_fanout", "operational"),
    ("fintech_mixed_fanout", "development"),
    ("fintech_mixed_fanout", "forensic"),
    ("harness_observability_profile", "development"),
    ("harness_observability_profile", "forensic"),
    ("suppression_wide_fanout", "balanced"),
    ("topology_rewiring_churn", "balanced"),
    ("topology_rewiring_rotating_window", "balanced"),
]
CONTRACTS = {
    ("chain_10k_bootstrap", "balanced"): ("median_only", 7, 2,
        ["build_nanos", "bootstrap_plan_nanos", "bootstrap_execute_nanos"],
        [["materialized_entry_reads", 0], ["materialized_entry_writes", 0]]),
    ("dependency_reconciliation_rotating_window", "balanced"): ("structural_only", 3, 0,
        ["reconcile_loop_nanos", "dependency_reconcile_nanos", "snapshot_batch_commit_nanos"], []),
    ("dependency_reconciliation_stable_shape_staged", "balanced"): ("structural_only", 3, 0,
        ["planning_nanos", "report_stage_precompute_nanos", "report_stage_apply_nanos",
         "report_semantic_finalize_nanos", "dependency_reconcile_nanos", "snapshot_batch_commit_nanos"], []),
    ("dependency_reconciliation_rotating_window_staged", "balanced"): ("structural_only", 3, 0,
        ["planning_nanos", "report_stage_precompute_nanos", "report_stage_apply_nanos",
         "report_semantic_finalize_nanos", "dependency_reconcile_nanos", "snapshot_batch_commit_nanos"], []),
    ("fintech_mixed_fanout", "operational"): ("strict_heavy", 5, 0,
        ["read_before_nanos", "mutation_nanos", "read_after_nanos"], []),
    ("fintech_mixed_fanout", "development"): ("median_only", 7, 2,
        ["read_before_nanos", "mutation_nanos", "read_after_nanos"], []),
    ("fintech_mixed_fanout", "forensic"): ("median_only", 7, 2,
        ["read_before_nanos", "mutation_nanos", "read_after_nanos"], []),
    ("harness_observability_profile", "development"): ("structural_only", 3, 0,
        ["observe_loop_nanos"], []),
    ("harness_observability_profile", "forensic"): ("structural_only", 3, 0,
        ["observe_loop_nanos"], []),
    ("suppression_wide_fanout", "balanced"): ("median_only", 7, 2,
        ["leaf_reread_nanos", "stage_execution_nanos"],
        [["materialized_entry_reads", 0], ["materialized_entry_writes", 0]]),
    ("topology_rewiring_churn", "balanced"): ("median_only", 7, 2, ["rewire_loop_nanos"],
        [["materialized_entry_reads", 0], ["materialized_entry_writes", 0],
         ["runtime_artifact_state_reads", 0], ["runtime_artifact_warm_reads", 0]]),
    ("topology_rewiring_rotating_window", "balanced"): ("median_only", 7, 2, ["rewire_loop_nanos"],
        [["materialized_entry_reads", 0], ["materialized_entry_writes", 0],
         ["runtime_artifact_state_reads", 0], ["runtime_artifact_warm_reads", 0]]),
}
WORKLOAD_WARMUPS = {(suite, profile): "none" for suite, profile in CASES}
WORKLOAD_WARMUPS.update({
    ("chain_10k_bootstrap", "balanced"): "once: build-plan-execute prime",
    ("fintech_mixed_fanout", "operational"): "once per profile: read-mutate-read prime",
    ("fintech_mixed_fanout", "development"): "once per profile: read-mutate-read prime",
    ("fintech_mixed_fanout", "forensic"): "once per profile: read-mutate-read prime",
    ("topology_rewiring_churn", "balanced"): "once: churn-rewire prime",
    ("topology_rewiring_rotating_window", "balanced"): "once: window-rewire prime",
})
PROBES = ("ordinary", "peak")
PEAK = "metrics.allocation_metrics.peak_live_bytes"
ACCESS_COUNTERS = (
    "materialized_entry_reads", "materialized_entry_writes",
    "runtime_artifact_warm_reads", "runtime_artifact_state_reads",
    "retained_artifact_reads", "reconstructed_artifact_reads",
)


def require(condition, message):
    if not condition:
        raise ValueError(message)
def unique_object(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result, f"duplicate JSON key: {key}")
        result[key] = value
    return result
def decode(raw):
    return json.loads(raw, object_pairs_hook=unique_object,
                      parse_constant=lambda value: require(False, f"invalid number: {value}"))
def new_output(path, roots):
    require(path.is_absolute(), "output/build paths must be absolute")
    resolved = path.resolve()
    require(not any(resolved.is_relative_to(root.resolve()) for root in roots),
            "output/build paths must be outside source")
    require(path.parent.is_dir(), "output parent must already exist")
    require(not path.exists(), f"refusing existing output: {path}")
    require(path.name != "performance_baseline.json", "cannot write a golden")
    return resolved
def flatten(value, prefix=""):
    if isinstance(value, dict):
        result = {}
        for key, child in value.items():
            require(isinstance(key, str) and "." not in key, "invalid metric key")
            result.update(flatten(child, f"{prefix}.{key}" if prefix else key))
        return result
    return {prefix: value}
def distribution(values):
    require(values and all(type(v) is int and v >= 0 for v in values),
            "metrics must be nonnegative integers")
    ordered = sorted(values)
    return {"min": ordered[0], "median": ordered[math.ceil(len(values) * .5) - 1],
            "p95": ordered[math.ceil(len(values) * .95) - 1],
            "p99": ordered[math.ceil(len(values) * .99) - 1], "max": ordered[-1]}
def summarize(case):
    rows = [flatten(sample) for sample in case["samples"]]
    require(rows and all(row.keys() == rows[0].keys() for row in rows),
            "missing or changing sample metric roster")
    return {key: distribution([row[key] for row in rows])
            for key, value in rows[0].items() if type(value) is int}


def expected_budgets(contract, probe):
    if probe == "peak":
        return {PEAK: {"median": 1.10, "max": 1.10}}
    budgets = {}
    if contract["timing_policy"] == "strict_heavy":
        budgets["elapsed_micros"] = {"median": 1.20, "p95": 1.25, "max": 1.35}
    elif contract["timing_policy"] == "median_only":
        budgets["elapsed_micros"] = {"median": 1.35}
    for metric in ("allocation_calls", "allocated_bytes", "end_live_bytes"):
        budgets[f"metrics.allocation_metrics.{metric}"] = {"median": 1.10, "max": 1.10}
    for counter in ACCESS_COUNTERS:
        budgets[f"metrics.access_counters.{counter}"] = {"max": 1.0}
    if contract["timing_policy"] != "structural_only":
        for phase in contract["phase_metrics"]:
            budgets[f"metrics.{phase}"] = {"median": 1.25}
    return budgets


def validate_cases(cases, probe):
    require(isinstance(cases, list) and len(cases) == len(CASES), "incomplete twelve-case roster")
    for case, (suite, profile) in zip(cases, CASES):
        contract = case["contract"]
        require((contract["suite"], contract["profile"], contract["executor"])
                == (suite, profile, "serial"), "wrong case roster/order/executor")
        require(case["probe"] == probe, "wrong allocation probe posture")
        policy, samples, warmups, phases, maxima = CONTRACTS[(suite, profile)]
        require(case["sample_count"] == samples and len(case["samples"]) == samples,
                "wrong or missing sample count")
        require(case["warmup_count"] == warmups, "wrong warmup count")
        require(case["workload_warmup"] == WORKLOAD_WARMUPS[(suite, profile)],
                "wrong workload Once warmup posture")
        require(contract["timing_policy"] == policy, "wrong timing policy")
        require(contract["phase_metrics"] == phases, "wrong phase metric roster/order")
        require(contract["access_counter_maxima"] == maxima, "wrong absolute contract roster")
        summaries = summarize(case)
        require(summaries["elapsed_micros"]["min"] > 0, "zero elapsed sample")
        for name in ("allocation_calls", "allocated_bytes", "deallocation_calls", "deallocated_bytes",
                     "live_bytes", "end_live_bytes"):
            require(f"metrics.allocation_metrics.{name}" in summaries, f"missing {name}")
        for phase in contract["phase_metrics"]:
            require(f"metrics.{phase}" in summaries, f"missing phase: {phase}")
        require(case["relative_budgets"] == expected_budgets(contract, probe),
                "wrong relative budget authority")
        for metric, budgets in case["relative_budgets"].items():
            require(metric in summaries, f"missing budget metric: {metric}")
            require(budgets and all(stat in ("median", "p95", "max") and type(ratio) in (float, int)
                    and math.isfinite(ratio) and ratio >= 1 for stat, ratio in budgets.items()), "invalid budgets")
        for row in case["samples"]:
            allocation = row["metrics"]["allocation_metrics"]
            require(allocation["end_live_bytes"] == allocation["live_bytes"],
                    "legacy end-live compatibility mismatch")
            if probe == "ordinary":
                require(allocation["peak_live_bytes"] is None and allocation["peak_live_status"].startswith("unavailable:"), "ordinary peak must be explicitly unavailable")
            else:
                require(PEAK in summaries and allocation["peak_live_status"].startswith("measured-group requested object high-water"), "missing genuine peak")
                require(summaries[PEAK]["min"] > 0, "genuine peak tracker recorded zero")
    return cases


def validate_capture(data):
    require(data["version"] == 1 and data["status"] == "captured", "failed/incomplete capture")
    require(data["test_roster"] == TESTS, "wrong test roster")
    require(data["configuration"] == configuration(), "wrong release/configuration posture")
    validate_environment(data["environment"])
    for probe in PROBES:
        require(data["commands"][probe]["returncode"] == 0, f"{probe} benchmark failed")
        validate_command(data["commands"][probe]["argv"], probe)
        validate_cases(data["cases"][probe], probe)
    for ordinary, peak in zip(data["cases"]["ordinary"], data["cases"]["peak"]):
        require(case_posture(ordinary) == case_posture(peak), "ordinary/peak contracts differ")
    return data

def validate_environment(environment):
    required_environment = {"os", "arch", "cpu", "host", "logical_cpus", "variables",
                            "rustc", "cargo", "workspace_profiles", "cargo_config"}
    require(isinstance(environment, dict) and set(environment) == required_environment,
            "missing or extra environment fields")
    for name in ("os", "arch", "cpu", "host", "rustc", "cargo", "cargo_config"):
        require(isinstance(environment[name], str) and environment[name].strip()
                and (name != "cpu" or environment[name].strip().lower() != "unknown"),
                f"invalid environment field: {name}")
    require(type(environment["logical_cpus"]) is int and environment["logical_cpus"] > 0,
            "invalid logical CPU count")
    require(isinstance(environment["variables"], dict)
            and all(isinstance(key, str) and isinstance(value, str)
                    for key, value in environment["variables"].items()), "invalid environment variables")
    require(isinstance(environment["workspace_profiles"], dict) and environment["workspace_profiles"],
            "missing workspace release profiles")


def case_config(case):
    return {key: case[key] for key in ("contract", "sample_count", "warmup_count", "relative_budgets")}


def case_posture(case):
    return {key: case[key] for key in ("contract", "sample_count", "warmup_count", "workload_warmup")}


def validate_command(argv, probe):
    require(isinstance(argv, list) and all(isinstance(value, str) for value in argv),
            "missing benchmark argv")
    feature = "profile-extended" + (",test-peak-allocation" if probe == "peak" else "")
    require(len(argv) == 20, "wrong benchmark argv length")
    manifest, target = Path(argv[5]), Path(argv[7])
    require(manifest.is_absolute() and manifest.name == "Cargo.toml", "invalid manifest path")
    require(target.is_absolute(), "target dir must be absolute")
    expected = ["cargo", "test", "--locked", "--release", "--manifest-path", str(manifest),
                "--target-dir", str(target), "-p", "worth-signal", "--lib", "--no-default-features",
                "--features", feature, FAMILY, "--", "--ignored", "--test-threads=1", "--nocapture",
                "--color=never"]
    require(argv == expected, "wrong exact cargo/libtest benchmark posture")


def compare(baseline, candidate):
    validate_capture(baseline)
    validate_capture(candidate)
    require(baseline["environment"] == candidate["environment"], "environment mismatch; cannot compare")
    report = {"relative_regressions": [], "absolute_contract_violations": [], "distributions": {}}
    for probe in PROBES:
        for before, after in zip(baseline["cases"][probe], candidate["cases"][probe]):
            require(case_config(before) == case_config(after), "sampling/contract/budget mismatch")
            old, new = summarize(before), summarize(after)
            require(old.keys() == new.keys(), "baseline/candidate metric roster mismatch")
            key = "|".join(after["contract"][name] for name in ("suite", "profile", "executor"))
            report["distributions"][f"{probe}|{key}"] = {"baseline": old, "candidate": new}
            for metric, budgets in after["relative_budgets"].items():
                if (probe == "peak") != (metric == PEAK):
                    continue
                for stat, ratio in budgets.items():
                    expected, observed = old[metric][stat], new[metric][stat]
                    allowed = math.ceil(expected * ratio)
                    if observed > allowed:
                        report["relative_regressions"].append(dict(case=key, probe=probe, metric=metric,
                            statistic=stat, baseline=expected, candidate=observed, allowed=allowed, budget=ratio))
            if probe != "ordinary":
                continue
            for side, case, values in (("baseline", before, old), ("candidate", after, new)):
                for counter, maximum in case["contract"]["access_counter_maxima"]:
                    observed = values[f"metrics.access_counters.{counter}"]["max"]
                    if observed > maximum:
                        report["absolute_contract_violations"].append(dict(side=side, probe=probe, case=key,
                            counter=counter, observed=observed, maximum=maximum))
    report["relative_verdict"] = "regression" if report["relative_regressions"] else "within budgets"
    report["absolute_verdict"] = "violation" if report["absolute_contract_violations"] else "within contracts"
    code = 1 if report["relative_regressions"] else 4 if report["absolute_contract_violations"] else 0
    report["status"] = "performance_pass" if code == 0 else "rejected"
    return report, code


def configuration():
    return {"profile": "release", "default_features": False, "features": ["profile-extended"],
            "peak_feature": "test-peak-allocation", "test_threads": 1, "ignored": True,
            "filter": FAMILY, "samples_and_warmups": "unchanged contract defaults",
            "workload_warmups": [[suite, profile, WORKLOAD_WARMUPS[(suite, profile)]]
                                 for suite, profile in CASES]}


def environment(root):
    keys = ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTUP_TOOLCHAIN", "RUST_MIN_STACK", "RUST_LOG", "RAYON_NUM_THREADS")
    variables = {key: value for key, value in os.environ.items()
                 if key in keys or key.startswith(("CARGO_PROFILE_", "CARGO_BUILD_", "CARGO_TARGET_"))}
    require(not any(key.startswith("WORTH_SIGNAL_") for key in os.environ), "remove ambient WORTH_SIGNAL_* overrides before capture")
    cpu = platform.processor() or os.environ.get("PROCESSOR_IDENTIFIER", "")
    context = {"os": platform.platform(), "arch": platform.machine(), "cpu": cpu,
            "host": platform.node(), "logical_cpus": os.cpu_count(), "variables": variables,
            "rustc": subprocess.check_output(["rustc", "-vV"], text=True),
            "cargo": subprocess.check_output(["cargo", "-V"], text=True),
            "workspace_profiles": tomllib.loads((root / "Cargo.toml").read_text())["profile"],
            "cargo_config": (root / ".cargo/config.toml").read_text()}
    validate_environment(context)
    return context


def require_listing(text):
    listed = [line[:-6] for line in text.splitlines() if line.endswith(": test")]
    require(listed == TESTS, "ignored test listing differs from reviewed whole-family roster")


def require_completion(text):
    summaries = re.findall(r"test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored;", text)
    require(summaries == [("ok", str(len(TESTS)), "0", "0")], "benchmark crashed, skipped cases or has no complete success summary")


def capture(args):
    root = args.root.resolve()
    roots = [root, Path(__file__).resolve().parents[2]]
    output = new_output(args.output, roots)
    require(args.target_dir.is_absolute() and not any(args.target_dir.resolve().is_relative_to(r) for r in roots), "target-dir must be outside source")
    context = environment(root)
    paths = {probe: new_output(Path(str(output) + f".{probe}.jsonl"), roots) for probe in PROBES}
    log_path = new_output(Path(str(output) + ".log"), roots)
    data = dict(version=1, status="benchmark_failed", configuration=configuration(), environment=context,
                test_roster=TESTS, cases={}, commands={})
    with output.open("x", encoding="utf-8") as destination, log_path.open("x", encoding="utf-8") as log:
        try:
            for probe in PROBES:
                features = "profile-extended" + (",test-peak-allocation" if probe == "peak" else "")
                command = ["cargo", "test", "--locked", "--release", "--manifest-path", str(root / "Cargo.toml"),
                           "--target-dir", str(args.target_dir), "-p", "worth-signal", "--lib",
                           "--no-default-features", "--features", features, FAMILY, "--"]
                listing = subprocess.run(command + ["--ignored", "--list", "--format=terse"], cwd=root,
                                         text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
                log.write(listing.stdout)
                require(listing.returncode == 0, f"{probe} build/list failed ({listing.returncode})")
                require_listing(listing.stdout)
                run_command = command + ["--ignored", "--test-threads=1", "--nocapture", "--color=never"]
                print(f"[{probe}] {subprocess.list2cmdline(run_command)}", flush=True)
                log.write(json.dumps(run_command) + "\n")
                log.flush()
                env = dict(os.environ, WORTH_SIGNAL_PERF_OUTPUT=str(paths[probe]))
                result = subprocess.run(run_command, cwd=root, env=env, stdout=log, stderr=subprocess.STDOUT)
                log.flush()
                data["commands"][probe] = dict(argv=run_command, returncode=result.returncode)
                require(result.returncode == 0, f"{probe} benchmark failed ({result.returncode}); see {log_path}")
                # Each probe has one completed libtest summary; lists never execute tests.
                text = log_path.read_text(encoding="utf-8")
                require_completion(text[text.rfind(json.dumps(run_command)):])
                data["cases"][probe] = validate_cases([decode(line) for line in paths[probe].read_text().splitlines()], probe)
            data["status"] = "captured"
            validate_capture(data)
        except (ValueError, OSError, KeyError, TypeError, IndexError, AttributeError) as error:
            data["status"] = "benchmark_failed"
            data["error"] = str(error)
        json.dump(data, destination, indent=2)
    print(f"{data['status']}: {output} (capture alone is not a performance pass)")
    return 0 if data["status"] == "captured" else 3


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    commands = parser.add_subparsers(dest="action", required=True)
    record = commands.add_parser("capture")
    record.add_argument("--root", type=Path, required=True)
    record.add_argument("--target-dir", type=Path, required=True)
    record.add_argument("--output", type=Path, required=True)
    comparison = commands.add_parser("compare")
    comparison.add_argument("--baseline", type=Path, required=True)
    comparison.add_argument("--candidate", type=Path, required=True)
    comparison.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    try:
        if args.action == "capture":
            return capture(args)
        require(args.baseline.resolve() != args.candidate.resolve(),
                "baseline and candidate must be independent files")
        output = new_output(args.output, [Path(__file__).resolve().parents[2]])
        report, code = compare(decode(args.baseline.read_text()), decode(args.candidate.read_text()))
        with output.open("x", encoding="utf-8") as destination:
            json.dump(report, destination, indent=2)
        print(f"{report['status']}: relative={report['relative_verdict']}; absolute={report['absolute_verdict']}; {output}")
        return code
    except (ValueError, OSError, KeyError, TypeError, IndexError, AttributeError,
            subprocess.SubprocessError) as error:
        print(f"invalid measurement input/posture: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
