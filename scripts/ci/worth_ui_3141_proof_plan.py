from __future__ import annotations

import hashlib
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
from worth_ui_3141_p3_proofs import build_p3_proofs
from worth_ui_3141_p4_proofs import build_p4_proofs
from worth_ui_3141_p5_proofs import build_p5_proofs
from worth_ui_3141_p6_proofs import build_p6_proofs


ROOT = Path(__file__).resolve().parents[2]
EVIDENCE = "_docs/worth-ui/milestone-3.14.1-evidence"
COMPILE_ARTIFACT = f"{EVIDENCE}/compile-contracts.json"
P1_WORLD_ARTIFACT = f"{EVIDENCE}/p1-worlds-01.json"
P2_WORLD_ARTIFACT = f"{EVIDENCE}/p2-world-01.json"
P3_WORLD_ARTIFACT = f"{EVIDENCE}/p3-hp02-world-01.json"
P3_MIXED_WORLD_ARTIFACT = f"{EVIDENCE}/p3-delta-source-01.json"
P3_PREDECESSOR_HANDOFF = f"{EVIDENCE}/p3-predecessor-handoff.json"
P4_PREDECESSOR_HANDOFF = f"{EVIDENCE}/p4-predecessor-handoff.json"
P5_PREDECESSOR_HANDOFF = f"{EVIDENCE}/p5-predecessor-handoff.json"
P6_PREDECESSOR_HANDOFF = f"{EVIDENCE}/p6-predecessor-handoff.json"
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

    def __post_init__(self) -> None:
        object.__setattr__(self, "sources", tuple(dict.fromkeys(self.sources)))


def proofs() -> dict[str, Proof]:
    result = build_p1_proofs(compile_proof, rust_lib, rust_test)
    replace_consumer_proof(result)
    replace_preparation_proof(result)
    result["P1-CLOSE-01"] = rust_test(
        "worth-ui-certification",
        "topology_contracts",
        "milestone_3141_phase1_ledger::phase_one_closure_prerequisites_are_final_source",
        "workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_ledger.rs::validate_phase_closure",
        "workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_ledger.rs::phase_one_closure_prerequisites_are_final_source",
    )
    result["P1-HEADLESS-COST-01"] = replace(
        result["P1-HEADLESS-COST-01"],
        sources=result["P1-HEADLESS-COST-01"].sources
        + (P1_WORLD_ARTIFACT, "scripts/ci/run_worth_ui_shared_ledger_control.py"),
        shared_main=True,
    )
    result.update(shared_phase_two_proofs())
    result.update(shared_phase_three_proofs())
    result.update(build_p4_proofs(Proof, Control, P4_PREDECESSOR_HANDOFF))
    result.update(build_p5_proofs(Proof, Control, P5_PREDECESSOR_HANDOFF))
    result.update(build_p6_proofs(Proof, Control, P6_PREDECESSOR_HANDOFF))
    return result


def replace_consumer_proof(result: dict[str, Proof]) -> None:
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


def replace_preparation_proof(result: dict[str, Proof]) -> None:
    preparation = result["P1-PREPARATION-LIFECYCLE-01"]
    result["P1-PREPARATION-LIFECYCLE-01"] = replace(
        preparation,
        sources=(
            "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/platform/preparation.rs",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/profile.rs",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/application.rs",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native_profile.rs",
            "workspaces/worth-ui/crates/worth-ui-native-platform/src/lib.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/preparation_call_graph.rs",
        ),
    )


def shared_phase_two_proofs() -> dict[str, Proof]:
    result = build_p2_proofs(Control, p2_proof)
    for requirement, phase_proof in tuple(result.items()):
        if requirement != "P2-WORLD-01":
            result[requirement] = replace(
                phase_proof,
                sources=phase_proof.sources
                + (P2_WORLD_ARTIFACT, "scripts/ci/run_worth_ui_shared_ledger_control.py"),
                shared_main=True,
            )
    return result


def shared_phase_three_proofs() -> dict[str, Proof]:
    result = build_p3_proofs(Proof, Control, P3_PREDECESSOR_HANDOFF)
    native = {
        "P3-BASELINE-REPLAY-01", "P3-DAMAGE-REPLAY-01", "P3-DRAW-LIST-01",
        "P3-PHYSICAL-AMPLIFICATION-01",
        "P3-TRANSACTION-01", "P3-UNCHANGED-01",
    }
    for requirement in native:
        phase_proof = result[requirement]
        result[requirement] = replace(
            phase_proof,
            sources=phase_proof.sources
            + (P3_WORLD_ARTIFACT, "scripts/ci/run_worth_ui_shared_ledger_control.py"),
            shared_main=True,
        )
    for requirement in {"P3-HEADLESS-COST-01", "P3-PRODUCER-SLOPE-01"}:
        phase_proof = result[requirement]
        result[requirement] = replace(
            phase_proof,
            sources=phase_proof.sources
            + (P3_MIXED_WORLD_ARTIFACT, "scripts/ci/run_worth_ui_shared_ledger_control.py"),
            shared_main=True,
        )
    return result


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


def rust_lib(package: str, test: str, production: str, oracle: str) -> Proof:
    return proof(package, ("lib", "lib"), test, production, oracle)


def rust_test(package: str, target: str, test: str, production: str, oracle: str) -> Proof:
    return proof(package, ("test", target), test, production, oracle)


def proof(
    package: str, target: tuple[str, str], test: str, production: str, oracle: str
) -> Proof:
    sources = tuple(dict.fromkeys((production.rsplit("::", 1)[0], oracle.rsplit("::", 1)[0])))
    return Proof(package, target, test, production, oracle, sources)


def command(requirement: str, proof: Proof, artifact: str) -> str:
    runner = (
        "scripts/ci/run_worth_ui_shared_ledger_control.py"
        if proof.shared_main
        else "scripts/ci/run_worth_ui_ledger_test.py"
    )
    words = [
        "python", runner, "--manifest-path", "workspaces/worth-ui/Cargo.toml",
        "--package", proof.package, f"--{proof.target[0]}",
    ]
    if proof.target[0] == "test":
        words.append(proof.target[1])
    for feature in proof.features:
        words.extend(["--features", feature])
    words.extend(["--test-name", proof.test_name])
    add_control(words, proof.control)
    words.extend(["--requirement", requirement])
    for source in source_inventory(proof):
        words.extend(["--source", source])
    words.extend(["--artifact", artifact])
    return " ".join(words)


def add_control(words: list[str], control: Control | None) -> None:
    if control is None:
        return
    words.extend(["--control-package", control.package, f"--control-{control.target[0]}"])
    if control.target[0] == "test":
        words.append(control.target[1])
    for feature in control.features:
        words.extend(["--control-features", feature])
    words.extend(["--control-test-name", control.test_name])


def prepare_claim(row: dict[str, str], proof: Proof) -> None:
    requirement = row["requirement"]
    family, case = MUTATIONS[requirement]
    counter, amount = COUNTERS[requirement]
    p2 = requirement.startswith("P2-")
    baseline = baseline_path(requirement)
    artifact = f"{EVIDENCE}/{requirement.lower()}.json"
    row.update({
        "baseline_digest": digest(ROOT / baseline) if baseline else not_applicable_digest(requirement),
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
        "source_identity": ";".join(source_inventory(proof)),
        "structural_counters": f"{counter}={amount}",
        "presented_source_readback": presented_source_posture(requirement),
        "client_area_observation": client_area_posture(requirement),
    })
    if requirement.startswith(("P4-", "P5-")):
        text_profile = (
            ROOT / "workspaces/worth-ui/profiles/worth-ui-global-text-v2/manifest.toml"
        )
        row["font_profile_identity"] = "worth-ui-global-text-v2"
        row["font_profile_digest"] = digest(text_profile)
    if requirement.startswith("P6-"):
        native_profile = (
            ROOT
            / "workspaces/worth-ui/crates/worth-ui-host-native/profiles/"
            "worth-ui-windows-dx12-v1.toml"
        )
        row["native_profile_identity"] = "worth-ui-windows-dx12-v1"
        row["native_profile_digest"] = digest(native_profile)


def source_inventory(proof: Proof) -> tuple[str, ...]:
    sources = list(proof.sources)
    if proof.control is not None:
        sources.append(proof.control.source)
    producer = (
        "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/"
        "work_producer.rs"
    )
    if producer in sources:
        sources.append(
            "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/"
            "work_producer/successor_issue.rs"
        )
    return tuple(dict.fromkeys(sources))


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def not_applicable_digest(requirement: str) -> str:
    return hashlib.sha256(f"not-applicable:{requirement}".encode()).hexdigest()


def presented_source_posture(requirement: str) -> str:
    if requirement in {"P2-GRAPHICS-01", "P2-PRESENT-01", "P2-PIXELS-01", "P2-WORLD-01"}:
        return "observed:retained-rgba-47-129-247-255"
    if requirement == "P3-DAMAGE-REPLAY-01":
        return "observed:native-vacated-region-transparent-readback"
    if requirement == "P3-PHYSICAL-AMPLIFICATION-01":
        return "observed:native-retained-to-surface-full-area-readback"
    if requirement == "P3-HP02-WORLD-01":
        return "observed:runtime-issued-native-phase3-frame-sequence"
    return "not-applicable"


def client_area_posture(requirement: str) -> str:
    if requirement in {"P2-PIXELS-01", "P2-WORLD-01"}:
        return "observed:three-client-pixels-47-129-247-255"
    return "not-applicable"
