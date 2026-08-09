from __future__ import annotations

import argparse
import csv
import hashlib
import json
import os
import re
import secrets
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
MANIFEST = Path("workspaces/worth-ui/Cargo.toml")
LEDGER = Path("_docs/worth-ui/milestone-3.14.1-proof-ledger.csv")
PACKAGES = {
    "worth-ui-certification",
    "worth-ui-host-contract",
    "worth-ui-host-headless",
    "worth-ui-host-native",
    "worth-ui-native-platform",
    "worth-ui-runtime",
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
class GovernedTest:
    requirement: str
    package: str
    target_kind: str
    target_name: str
    test_name: str
    sources: tuple[str, ...]
    artifact: str


def parse_args() -> GovernedTest:
    parser = argparse.ArgumentParser(description="Run one ledger-bound Worth UI test")
    parser.add_argument("--manifest-path", required=True)
    parser.add_argument("--package", required=True, choices=sorted(PACKAGES))
    target = parser.add_mutually_exclusive_group(required=True)
    target.add_argument("--lib", action="store_true")
    target.add_argument("--test")
    parser.add_argument("--test-name", required=True)
    parser.add_argument("--requirement", required=True)
    parser.add_argument("--source", action="append", required=True)
    parser.add_argument("--artifact", required=True)
    arguments = parser.parse_args()
    if Path(arguments.manifest_path).as_posix() != MANIFEST.as_posix():
        parser.error(f"manifest must be {MANIFEST.as_posix()}")
    if "::" not in arguments.test_name:
        parser.error("test name must be fully qualified")
    target_kind = "lib" if arguments.lib else "test"
    target_name = "lib" if arguments.lib else arguments.test
    return GovernedTest(
        arguments.requirement,
        arguments.package,
        target_kind,
        target_name,
        arguments.test_name,
        tuple(arguments.source),
        arguments.artifact,
    )


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
    with repository_path(LEDGER.as_posix()).open(encoding="utf-8", newline="") as source:
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


def git_bytes(arguments: list[str]) -> bytes:
    result = subprocess.run(
        ["git", *arguments], cwd=ROOT, capture_output=True, check=False
    )
    if result.returncode != 0:
        raise RuntimeError(f"git {' '.join(arguments)} failed")
    return result.stdout


def source_state_digest(revision: str) -> str:
    digest = hashlib.sha256()
    digest.update(revision.encode("ascii"))
    digest.update(b"\0tracked-diff\0")
    digest.update(
        git_bytes(
            [
                "diff",
                "--binary",
                "--no-ext-diff",
                "HEAD",
                "--",
                ".",
                f":(exclude){LEDGER.as_posix()}",
            ]
        )
    )
    untracked = git_bytes(["ls-files", "--others", "--exclude-standard", "-z"])
    identities = sorted(item for item in untracked.split(b"\0") if item)
    for encoded_identity in identities:
        identity = encoded_identity.decode("utf-8")
        if Path(identity).as_posix() == LEDGER.as_posix():
            continue
        digest.update(b"\0untracked\0")
        digest.update(encoded_identity)
        digest.update(b"\0")
        digest.update(repository_path(identity).read_bytes())
    return digest.hexdigest()


def cargo_command(test: GovernedTest, list_only: bool) -> list[str]:
    command = [
        "cargo",
        "test",
        "--manifest-path",
        MANIFEST.as_posix(),
        "-p",
        test.package,
    ]
    command.extend(["--lib"] if test.target_kind == "lib" else ["--test", test.target_name])
    if list_only:
        command.extend(["--", "--list", "--format", "terse"])
    else:
        command.extend(
            [test.test_name, "--", "--exact", "--include-ignored", "--nocapture"]
        )
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
        command,
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )


def execution_counts(output: str) -> tuple[int, int, int]:
    summaries = re.findall(
        r"test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;",
        output,
    )
    if len(summaries) != 1:
        return (0, 0, 0)
    passed, failed, ignored = (int(value) for value in summaries[0])
    return (passed + failed, passed, ignored)


def result_payload(test: GovernedTest) -> tuple[dict[str, Any], int]:
    list_command = cargo_command(test, True)
    test_command = cargo_command(test, False)
    revision = source_revision()
    digest = source_digest(test.sources)
    state_digest = source_state_digest(revision)
    run_nonce = secrets.token_hex(16)
    discovery = run(list_command)
    matches = listed_test_names(discovery.stdout).count(test.test_name)
    execution = None
    executed = passed = ignored = 0
    posture = "list-failed" if discovery.returncode else "match-count-rejected"
    if discovery.returncode == 0 and matches == 1:
        execution = run(test_command)
        executed, passed, ignored = execution_counts(execution.stdout)
        posture = (
            "passed"
            if execution.returncode == 0
            and executed == 1
            and passed == 1
            and ignored == 0
            else "test-failed"
        )
    if (
        revision != source_revision()
        or digest != source_digest(test.sources)
        or state_digest != source_state_digest(revision)
    ):
        posture = "source-changed"
    payload = {
        "schema_version": 3,
        "requirement": test.requirement,
        "claim_digest": claim_digest(test.requirement),
        "package": test.package,
        "target_kind": test.target_kind,
        "target_name": test.target_name,
        "test_name": test.test_name,
        "matched_test_count": matches,
        "executed_test_count": executed,
        "passed_test_count": passed,
        "ignored_test_count": ignored,
        "exit_posture": posture,
        "list_exit_code": discovery.returncode,
        "test_exit_code": None if execution is None else execution.returncode,
        "source_revision": revision,
        "source_digest": digest,
        "source_state_digest": state_digest,
        "run_nonce": run_nonce,
        "source_identity": list(test.sources),
        "list_command": list_command,
        "test_command": test_command,
        "list_stdout": discovery.stdout,
        "list_stderr": discovery.stderr,
        "test_stdout": "" if execution is None else execution.stdout,
        "test_stderr": "" if execution is None else execution.stderr,
    }
    return payload, 0 if posture == "passed" else 1


def write_artifact(identity: str, payload: dict[str, Any]) -> str:
    destination = repository_path(identity)
    destination.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary = tempfile.mkstemp(
        prefix=f".{destination.name}.", dir=destination.parent
    )
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8") as output:
            json.dump(payload, output, indent=2)
            output.write("\n")
        os.replace(temporary, destination)
    finally:
        if os.path.exists(temporary):
            os.unlink(temporary)
    return hashlib.sha256(destination.read_bytes()).hexdigest()


def main() -> int:
    try:
        governed_test = parse_args()
        payload, exit_code = result_payload(governed_test)
        artifact_digest = write_artifact(governed_test.artifact, payload)
    except (OSError, RuntimeError, ValueError) as error:
        print(f"ledger evidence runner: {error}", file=sys.stderr)
        return 2
    print(json.dumps({"artifact_sha256": artifact_digest, **payload}, sort_keys=True))
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main())
