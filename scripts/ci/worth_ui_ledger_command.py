from __future__ import annotations

import argparse
import csv
import hashlib
import os
import re
import subprocess
import time
from decimal import Decimal, ROUND_CEILING
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
MANIFEST = Path("workspaces/worth-ui/Cargo.toml")
LEDGER = Path("_docs/worth-ui/milestone-3.14.1-proof-ledger.csv")
PACKAGES = {
    "worth-ui-certification",
    "worth-ui-host-contract",
    "worth-ui-host-egui",
    "worth-ui-host-headless",
    "worth-ui-host-native",
    "worth-ui-native-platform",
    "worth-ui-platform-pulse",
    "worth-ui-runtime",
    "worth-ui-text",
}
CLAIM_FIELDS = (
    "phase", "requirement", "owner", "production_boundary", "world_identity",
    "world_version", "proof_kind", "evidence_schema", "baseline_digest",
    "scenario_delta", "generated_seed", "authority_provenance", "production_entry",
    "independent_oracle", "mutation_control", "fault_injection_boundary",
    "retained_failure_artifact", "teardown_result", "construction_cost",
    "execution_cost", "source_identity", "font_profile_identity", "font_profile_digest",
    "native_profile_identity", "native_profile_digest", "platform_versions",
    "structural_counters", "presented_source_readback", "client_area_observation",
)


@dataclass(frozen=True)
class ControlTest:
    package: str
    target_kind: str
    target_name: str
    test_name: str
    features: tuple[str, ...]


@dataclass(frozen=True)
class GovernedTest:
    requirement: str
    package: str
    target_kind: str
    target_name: str
    features: tuple[str, ...]
    test_name: str
    sources: tuple[str, ...]
    artifact: str
    control: ControlTest | None


def parse_args() -> GovernedTest:
    parser = argparse.ArgumentParser(description="Run one ledger-bound Worth UI test")
    parser.add_argument("--manifest-path", required=True)
    parser.add_argument("--package", required=True, choices=sorted(PACKAGES))
    target = parser.add_mutually_exclusive_group(required=True)
    target.add_argument("--lib", action="store_true")
    target.add_argument("--test")
    parser.add_argument("--features", action="append", default=[])
    parser.add_argument("--test-name", required=True)
    parser.add_argument("--control-package", choices=sorted(PACKAGES))
    control_target = parser.add_mutually_exclusive_group()
    control_target.add_argument("--control-lib", action="store_true")
    control_target.add_argument("--control-test")
    parser.add_argument("--control-features", action="append", default=[])
    parser.add_argument("--control-test-name")
    parser.add_argument("--requirement", required=True)
    parser.add_argument("--source", action="append", required=True)
    parser.add_argument("--artifact", required=True)
    arguments = parser.parse_args()
    validate_manifest_and_test_name(parser, arguments)
    target_kind = "lib" if arguments.lib else "test"
    target_name = "lib" if arguments.lib else arguments.test
    features = tuple(arguments.features)
    validate_features(parser, arguments.package, features, "main")
    control = parse_control(parser, arguments)
    requires_control = (
        arguments.requirement.startswith(("P2-", "P3-", "P4-"))
        or arguments.requirement == "P1-CONSUMERS-01"
    )
    if requires_control != (control is not None):
        parser.error("this requirement has the wrong hostile-control posture")
    return GovernedTest(
        arguments.requirement,
        arguments.package,
        target_kind,
        target_name,
        features,
        arguments.test_name,
        tuple(arguments.source),
        arguments.artifact,
        control,
    )


def validate_manifest_and_test_name(parser: argparse.ArgumentParser, arguments: argparse.Namespace) -> None:
    if Path(arguments.manifest_path).as_posix() != MANIFEST.as_posix():
        parser.error(f"manifest must be {MANIFEST.as_posix()}")
    if "::" not in arguments.test_name:
        parser.error("test name must be fully qualified")


def parse_control(
    parser: argparse.ArgumentParser, arguments: argparse.Namespace
) -> ControlTest | None:
    values = (
        arguments.control_package,
        arguments.control_lib or bool(arguments.control_test),
        arguments.control_test_name,
    )
    if any(values) != all(values):
        parser.error("control package, target, and exact test name must be supplied together")
    if not all(values):
        return None
    features = tuple(arguments.control_features)
    validate_features(parser, arguments.control_package, features, "control")
    return ControlTest(
        arguments.control_package,
        "lib" if arguments.control_lib else "test",
        "lib" if arguments.control_lib else arguments.control_test,
        arguments.control_test_name,
        features,
    )


def validate_features(
    parser: argparse.ArgumentParser,
    package: str,
    features: tuple[str, ...],
    owner: str,
) -> None:
    if features and (package, features) != (
        "worth-ui-platform-pulse", ("executable-world",)
    ):
        parser.error(f"only the governed {owner} executable-world feature is allowed")


def repository_path(value: str) -> Path:
    candidate = (ROOT / value).resolve()
    try:
        candidate.relative_to(ROOT)
    except ValueError as error:
        raise ValueError(f"path escapes repository: {value}") from error
    return candidate


def source_digest(sources: tuple[str, ...]) -> str:
    if len(sources) != len(set(sources)):
        raise ValueError("source identities must be unique")
    digest = hashlib.sha256()
    for identity in sorted(sources):
        source = repository_path(identity)
        if not source.is_file():
            raise ValueError(f"source does not exist: {identity}")
        digest.update(identity.encode("utf-8"))
        digest.update(b"\0")
        digest.update(source.read_bytes())
        digest.update(b"\0")
    return digest.hexdigest()


def source_revision() -> str:
    result = subprocess.run(
        ["git", "rev-parse", "--verify", "HEAD"],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    revision = result.stdout.strip()
    if result.returncode != 0 or len(revision) != 40:
        raise RuntimeError("cannot resolve the governed source revision")
    return revision


def claim_digest(requirement: str) -> str:
    configured = os.environ.get("WORTH_UI_MILESTONE_3141_LEDGER")
    ledger = Path(configured).resolve() if configured else repository_path(LEDGER.as_posix())
    with ledger.open(encoding="utf-8", newline="") as source:
        matches = [row for row in csv.DictReader(source) if row["requirement"] == requirement]
    if len(matches) != 1:
        raise ValueError(f"ledger requirement must occur exactly once: {requirement}")
    digest = hashlib.sha256()
    for field in CLAIM_FIELDS:
        digest.update(field.encode("utf-8"))
        digest.update(b"\0")
        digest.update(matches[0][field].encode("utf-8"))
        digest.update(b"\0")
    return digest.hexdigest()


def cargo_command(test: GovernedTest, list_only: bool) -> list[str]:
    command = [
        "cargo", "test", "--manifest-path", MANIFEST.as_posix(), "-p", test.package,
    ]
    command.extend(["--lib"] if test.target_kind == "lib" else ["--test", test.target_name])
    for feature in test.features:
        command.extend(["--features", feature])
    if list_only:
        command.extend(["--", "--list", "--format", "terse"])
    else:
        command.extend([test.test_name, "--", "--exact", "--include-ignored", "--nocapture"])
    return command


def ignored_list_command(test: GovernedTest) -> list[str]:
    command = cargo_command(test, True)
    command[-4:] = ["--", "--ignored", "--list", "--format", "terse"]
    return command


def control_cargo_command(test: ControlTest, list_only: bool) -> list[str]:
    command = [
        "cargo", "test", "--manifest-path", MANIFEST.as_posix(), "-p", test.package,
    ]
    command.extend(["--lib"] if test.target_kind == "lib" else ["--test", test.target_name])
    for feature in test.features:
        command.extend(["--features", feature])
    if list_only:
        command.extend(["--", "--list", "--format", "terse"])
    else:
        command.extend([test.test_name, "--", "--exact", "--nocapture"])
    return command


def listed_test_names(output: str) -> list[str]:
    names = []
    for line in output.splitlines():
        name, separator, kind = line.rpartition(": ")
        if separator and kind == "test":
            names.append(name)
    return names


def run(command: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        command, cwd=ROOT, capture_output=True, text=True, check=False, timeout=300,
    )


def timed_run(command: list[str]) -> tuple[subprocess.CompletedProcess[str], int]:
    started = time.perf_counter_ns()
    result = run(command)
    elapsed = time.perf_counter_ns() - started
    return result, max(1, (elapsed + 999_999) // 1_000_000)


def execution_budget_ms(requirement: str) -> int:
    if requirement.startswith("P2-"):
        return 30_000
    if requirement in {
        "P3-DELTA-SOURCE-01",
        "P3-HEADLESS-COST-01",
        "P3-PRODUCER-SLOPE-01",
    }:
        return 90_000
    if requirement == "P4-BIDI-01":
        return 180_000
    if requirement == "P4-LINE-LAYOUT-01":
        return 120_000
    return 60_000


def exact_test_duration_ms(output: str, command_duration_ms: int) -> int:
    matches = re.findall(
        r"^test result: .*; finished in ([0-9]+(?:\.[0-9]+)?)s$",
        output,
        flags=re.MULTILINE,
    )
    if not matches:
        return command_duration_ms
    milliseconds = Decimal(matches[-1]) * Decimal(1_000)
    return max(1, int(milliseconds.to_integral_value(rounding=ROUND_CEILING)))


def control_budget_ms(requirement: str) -> int:
    if requirement in {"P4-BIDI-01", "P4-LINE-LAYOUT-01"}:
        return 60_000 if requirement == "P4-BIDI-01" else 120_000
    if requirement in {
        "P4-BIDI-INTERACTION-01",
        "P4-TEXT-COST-01",
        "P4-TEXT-CONTENT-LOCALITY-01",
        "P4-TEXT-WIDTH-LOCALITY-01",
    }:
        return 30_000
    return (
        20_000
        if requirement in {
            "P3-PREDECESSOR-01",
            "P4-PREDECESSOR-01",
            "P4-FONT-COLLECTION-01",
            "P4-MEASUREMENT-IDENTITY-01",
            "P4-ACCESSIBILITY-GEOMETRY-01",
            "P4-FALLBACK-01",
            "P4-ORIGINAL-RANGE-01",
            "P4-SHAPING-01",
            "P4-TEXT-RECONSTRUCTION-01",
        }
        else 10_000
    )


def execution_counts(output: str) -> tuple[int, int, int]:
    import re

    summaries = re.findall(
        r"test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;",
        output,
    )
    if len(summaries) != 1:
        return (0, 0, 0)
    passed, failed, ignored = (int(value) for value in summaries[0])
    return (passed + failed, passed, ignored)
