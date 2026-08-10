from __future__ import annotations

import csv
import hashlib
import io
import json
import subprocess
import sys
import tempfile
from dataclasses import dataclass, replace
from pathlib import Path

from worth_ui_3141_ledger_contracts import (
    COUNTERS,
    FAULT_BOUNDARIES,
    MUTATIONS,
    baseline_path,
    construction_cost,
    execution_cost,
    platform_versions,
)
from worth_ui_3141_p1_proofs import build_p1_proofs
from worth_ui_3141_p2_proofs import build_p2_proofs


ROOT = Path(__file__).resolve().parents[2]
LEDGER = ROOT / "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"
EVIDENCE = "_docs/worth-ui/milestone-3.14.1-evidence"
COMPILE_ARTIFACT = f"{EVIDENCE}/compile-contracts.json"
P1_WORLD_ARTIFACT = f"{EVIDENCE}/p1-worlds-01.json"
P2_WORLD_ARTIFACT = f"{EVIDENCE}/p2-world-01.json"
COMPILE_TEST_SOURCE = (
    "workspaces/worth-ui/crates/worth-ui-certification/tests/"
    "milestone_3141_phase1_topology/compile_contract_artifact.rs"
)
COMPILE_TEST = (
    "milestone_3141_phase1_topology::compile_contract_artifact::"
    "phase_one_compile_contract_artifact_matches_every_executed_case"
)
P2_ORACLE_SOURCE = (
    "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/"
    "courtroom/native_phase2.rs"
)
P2_TEST = (
    "courtroom::native_phase2::"
    "windows_native_boundary_world_presents_quiesces_and_closes_without_residue"
)


@dataclass(frozen=True)
class Control:
    package: str
    target: tuple[str, str]
    test_name: str
    source: str
    features: tuple[str, ...] = ()


@dataclass(frozen=True)
class Proof:
    package: str
    target: tuple[str, str]
    test_name: str
    production_entry: str
    oracle_entry: str
    sources: tuple[str, ...]
    features: tuple[str, ...] = ()
    control: Control | None = None
    shared_main: bool = False


def compile_proof(production_entry: str) -> Proof:
    sources = (
        production_entry.rsplit("::", 1)[0],
        COMPILE_TEST_SOURCE,
        COMPILE_ARTIFACT,
        "scripts/ci/run_worth_ui_compile_contracts.py",
    )
    return Proof(
        "worth-ui-certification",
        ("test", "topology_contracts"),
        COMPILE_TEST,
        production_entry,
        f"{COMPILE_TEST_SOURCE}::phase_one_compile_contract_artifact_matches_every_executed_case",
        sources,
    )


def p2_proof(production_entry: str, control: Control, *extra_sources: str) -> Proof:
    sources = tuple(dict.fromkeys((
        production_entry.rsplit("::", 1)[0], P2_ORACLE_SOURCE,
        control.source, *extra_sources,
    )))
    return Proof(
        "worth-ui-platform-pulse",
        ("test", "executable_world"),
        P2_TEST,
        production_entry,
        f"{P2_ORACLE_SOURCE}::windows_native_boundary_world_presents_quiesces_and_closes_without_residue",
        sources,
        ("executable-world",),
        control,
    )


def proofs() -> dict[str, Proof]:
    result = build_p1_proofs(compile_proof, rust_lib, rust_test)
    result["P1-CONSUMERS-01"] = Proof(
        "worth-ui-host-headless",
        ("lib", "lib"),
        "headless_static_paint_tests::validated_agreement_static_paint_consumes_and_mixed_contract_stops_before_consumer",
        "workspaces/worth-ui/crates/worth-ui-host-headless/src/headless_translation/static_paint.rs::validate_protocol",
        "workspaces/worth-ui/crates/worth-ui-host-headless/src/headless_static_paint_tests.rs::validated_agreement_static_paint_consumes_and_mixed_contract_stops_before_consumer",
        (
            "workspaces/worth-ui/crates/worth-ui-host-headless/src/headless_translation/static_paint.rs",
            "workspaces/worth-ui/crates/worth-ui-host-headless/src/headless_static_paint_tests.rs",
            "workspaces/worth-ui/crates/worth-ui-host-contract/src/mounted_frame/protocol.rs",
            "workspaces/worth-ui/crates/worth-ui-host-egui/src/adapter/native_paint.rs",
            "workspaces/worth-ui/crates/worth-ui-host-egui/src/adapter/semantic_text_tests.rs",
        ),
        control=Control(
            "worth-ui-host-egui",
            ("lib", "lib"),
            "adapter::semantic_text::tests::validated_agreement_semantic_text_consumes_and_mixed_contract_stops_before_consumer",
            "workspaces/worth-ui/crates/worth-ui-host-egui/src/adapter/semantic_text_tests.rs",
        ),
    )
    preparation = result["P1-PREPARATION-LIFECYCLE-01"]
    result["P1-PREPARATION-LIFECYCLE-01"] = Proof(
        preparation.package,
        preparation.target,
        preparation.test_name,
        preparation.production_entry,
        preparation.oracle_entry,
        (
            "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/platform/preparation.rs",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/profile.rs",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/application.rs",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native_profile.rs",
            "workspaces/worth-ui/crates/worth-ui-native-platform/src/lib.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/preparation_call_graph.rs",
        ),
    )
    result["P1-CLOSE-01"] = rust_test(
        "worth-ui-certification",
        "topology_contracts",
        "milestone_3141_phase1_ledger::phase_one_closure_prerequisites_are_final_source",
        "workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_ledger.rs::validate_phase_closure",
        "workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_ledger.rs::phase_one_closure_prerequisites_are_final_source",
    )
    headless_cost = result["P1-HEADLESS-COST-01"]
    result["P1-HEADLESS-COST-01"] = replace(
        headless_cost,
        sources=headless_cost.sources + (
            P1_WORLD_ARTIFACT,
            "scripts/ci/run_worth_ui_shared_ledger_control.py",
        ),
        shared_main=True,
    )
    p2 = build_p2_proofs(Control, p2_proof)
    for requirement, phase_proof in tuple(p2.items()):
        if requirement != "P2-WORLD-01":
            p2[requirement] = replace(
                phase_proof,
                sources=phase_proof.sources + (
                    P2_WORLD_ARTIFACT,
                    "scripts/ci/run_worth_ui_shared_ledger_control.py",
                ),
                shared_main=True,
            )
    result.update(p2)
    return result


def rust_lib(package: str, test: str, production: str, oracle: str) -> Proof:
    return proof(package, ("lib", "lib"), test, production, oracle)


def rust_test(package: str, target: str, test: str, production: str, oracle: str) -> Proof:
    return proof(package, ("test", target), test, production, oracle)


def proof(package: str, target: tuple[str, str], test: str, production: str, oracle: str) -> Proof:
    sources = tuple(dict.fromkeys((production.rsplit("::", 1)[0], oracle.rsplit("::", 1)[0])))
    return Proof(package, target, test, production, oracle, sources)


def command(requirement: str, proof: Proof, artifact: str) -> str:
    runner = (
        "scripts/ci/run_worth_ui_shared_ledger_control.py"
        if proof.shared_main
        else "scripts/ci/run_worth_ui_ledger_test.py"
    )
    words = [
        "python", runner,
        "--manifest-path", "workspaces/worth-ui/Cargo.toml",
        "--package", proof.package,
        f"--{proof.target[0]}",
    ]
    if proof.target[0] == "test":
        words.append(proof.target[1])
    for feature in proof.features:
        words.extend(["--features", feature])
    words.extend(["--test-name", proof.test_name])
    if proof.control is not None:
        words.extend(["--control-package", proof.control.package])
        words.append(f"--control-{proof.control.target[0]}")
        if proof.control.target[0] == "test":
            words.append(proof.control.target[1])
        for feature in proof.control.features:
            words.extend(["--control-features", feature])
        words.extend(["--control-test-name", proof.control.test_name])
    words.extend(["--requirement", requirement])
    for source in proof.sources:
        words.extend(["--source", source])
    words.extend(["--artifact", artifact])
    return " ".join(words)


def prepare_claim(row: dict[str, str], proof: Proof) -> None:
    requirement = row["requirement"]
    family, case = MUTATIONS[requirement]
    counter, amount = COUNTERS[requirement]
    p2 = requirement.startswith("P2-")
    baseline = baseline_path(requirement)
    artifact = f"{EVIDENCE}/{requirement.lower()}.json"
    row.update({
        "baseline_digest": (
            digest(ROOT / baseline)
            if baseline is not None
            else hashlib.sha256(f"not-applicable:{requirement}".encode()).hexdigest()
        ),
        "scenario_delta": case,
        "generated_seed": "not-applicable",
        "production_entry": proof.production_entry,
        "independent_oracle": proof.oracle_entry,
        "mutation_control": f"family={family};case={case}",
        "fault_injection_boundary": FAULT_BOUNDARIES[requirement],
        "retained_failure_artifact": artifact,
        "teardown_result": "terminal" if p2 or "WORLD" in requirement else "not-applicable",
        "construction_cost": construction_cost(requirement),
        "execution_cost": execution_cost(requirement),
        "platform_versions": platform_versions(requirement),
        "exact_command": command(requirement, proof, artifact),
        "retained_result_artifact": artifact,
        "source_identity": ";".join(proof.sources),
        "structural_counters": f"{counter}={amount}",
        "presented_source_readback": (
            "observed:retained-rgba-47-129-247-255"
            if requirement in {"P2-GRAPHICS-01", "P2-PRESENT-01", "P2-PIXELS-01", "P2-WORLD-01"}
            else "not-applicable"
        ),
        "client_area_observation": (
            "observed:three-client-pixels-47-129-247-255"
            if requirement in {"P2-PIXELS-01", "P2-WORLD-01"}
            else "not-applicable"
        ),
        "reopen_lineage": "none",
    })


def close_row(row: dict[str, str], result: dict[str, object]) -> None:
    for field in [
        "matched_test_count", "source_revision", "source_digest",
        "source_state_digest", "run_nonce",
    ]:
        row[field] = str(result[field])
    row["command_result"] = "passed"
    row["result_artifact_digest"] = str(result["artifact_sha256"])
    row["result"] = "PROVED"
    row["final_source"] = "true"


def run(command_text: str) -> dict[str, object]:
    print(command_text, flush=True)
    completed = subprocess.run(
        command_text.split(), cwd=ROOT, capture_output=True, text=True, check=False
    )
    if completed.returncode != 0:
        sys.stderr.write(completed.stdout)
        sys.stderr.write(completed.stderr)
        raise RuntimeError(f"ledger proof failed with {completed.returncode}")
    return json.loads(completed.stdout.splitlines()[-1])


def write_phase(rows: list[dict[str, str]], fields: list[str], phase: int) -> None:
    original = LEDGER.read_text(encoding="utf-8")
    rendered = render_phase_update(original, rows, fields, phase)
    with tempfile.NamedTemporaryFile(
        "w", encoding="utf-8", newline="", dir=LEDGER.parent, delete=False
    ) as stream:
        stream.write(rendered)
        temporary = Path(stream.name)
    temporary.replace(LEDGER)


def render_phase_update(
    original: str,
    rows: list[dict[str, str]],
    fields: list[str],
    phase: int,
) -> str:
    mutable = {row["requirement"]: row for row in rows if int(row["phase"]) == phase}
    lines = original.splitlines(keepends=True)
    requirement_index = fields.index("requirement")
    for index, line in enumerate(lines[1:], 1):
        record = next(csv.reader([line]))
        requirement = record[requirement_index]
        if requirement in mutable:
            lines[index] = serialize_row(mutable[requirement], fields)
    return "".join(lines)


def serialize_row(row: dict[str, str], fields: list[str]) -> str:
    stream = io.StringIO(newline="")
    csv.DictWriter(stream, fieldnames=fields, lineterminator="\n").writerow(row)
    return stream.getvalue()


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> int:
    prepare_only = "--prepare-only" in sys.argv[1:]
    through_phase = requested_phase(sys.argv[1:])
    with LEDGER.open(encoding="utf-8", newline="") as stream:
        reader = csv.DictReader(stream)
        fields = list(reader.fieldnames or ())
        rows = list(reader)
    selected = phase_rows_to_prepare(rows, through_phase)
    configured = {
        requirement: proof
        for requirement, proof in proofs().items()
        if int(requirement[1]) <= through_phase
    }
    if set(configured) != {row["requirement"] for row in selected}:
        raise RuntimeError("proof mapping does not match the selected phase inventory")
    for row in selected:
        prepare_claim(row, configured[row["requirement"]])
        row["result"] = "OPEN"
        row["final_source"] = "false"
    if selected:
        write_phase(rows, fields, through_phase)
    if prepare_only:
        print(
            f"prepared {len(selected)} Worth UI milestone 3.14.1 ledger claims as OPEN",
            flush=True,
        )
        return 0

    if not selected:
        subprocess.run(
            [sys.executable, "scripts/ci/verify_worth_ui_3141_ledger.py"],
            cwd=ROOT,
            check=True,
        )
        return 0

    if through_phase != 2:
        raise RuntimeError("Phase 3 proof mappings are not implemented yet")

    subprocess.run(
        [
            sys.executable,
            "scripts/ci/run_worth_ui_compile_contracts.py",
            "--artifact",
            COMPILE_ARTIFACT,
        ],
        cwd=ROOT,
        check=True,
    )
    phase_one = [
        row for row in rows
        if row["phase"] == "1"
        and row["requirement"] not in {
            "P1-CLOSE-01", "P1-WORLDS-01", "P1-HEADLESS-COST-01"
        }
    ]
    close = next(row for row in rows if row["requirement"] == "P1-CLOSE-01")
    phase_one_world = next(row for row in rows if row["requirement"] == "P1-WORLDS-01")
    headless_cost = next(row for row in rows if row["requirement"] == "P1-HEADLESS-COST-01")
    world = next(row for row in rows if row["requirement"] == "P2-WORLD-01")
    dependent_phase_two = [
        row for row in rows
        if row["phase"] == "2" and row["requirement"] != "P2-WORLD-01"
    ]
    ordered = phase_one + [phase_one_world, headless_cost, close, world] + dependent_phase_two
    for row in ordered:
        close_row(row, run(row["exact_command"]))
        write_phase(rows, fields, through_phase)
    subprocess.run(
        [sys.executable, "scripts/ci/verify_worth_ui_3141_ledger.py"],
        cwd=ROOT,
        check=True,
    )
    return 0


def requested_phase(arguments: list[str]) -> int:
    if "--through-phase" not in arguments:
        return 2
    index = arguments.index("--through-phase")
    try:
        phase = int(arguments[index + 1])
    except (IndexError, ValueError) as error:
        raise RuntimeError("--through-phase requires an integer") from error
    if phase not in {2, 3, 4}:
        raise RuntimeError("--through-phase must be 2, 3, or 4")
    return phase


def phase_rows_to_prepare(
    rows: list[dict[str, str]], through_phase: int
) -> list[dict[str, str]]:
    predecessor = [row for row in rows if int(row["phase"]) < through_phase]
    if any(row["result"] != "PROVED" or row["final_source"] != "true" for row in predecessor):
        raise RuntimeError("cannot prepare a phase before predecessor closure")
    return [
        row
        for row in rows
        if int(row["phase"]) == through_phase
        and row["result"] == "OPEN"
        and row["final_source"] == "false"
    ]


if __name__ == "__main__":
    raise SystemExit(main())
