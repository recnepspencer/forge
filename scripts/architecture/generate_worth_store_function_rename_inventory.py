#!/usr/bin/env python3
"""Inventory Worth Store functions whose names encode roadmap ordering."""

from __future__ import annotations

import argparse
import csv
import re
from collections import Counter
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
CRATES_ROOT = REPO_ROOT / "workspaces/worth-store/crates"
DEFAULT_OUTPUT = REPO_ROOT / "_docs/worth-store/worth-store-function-rename-inventory.csv"

FUNCTION_PATTERN = re.compile(
    r"^\s*(?P<visibility>pub(?:\([^)]*\))?\s+)?"
    r"(?:(?:const|async|unsafe)\s+|extern\s+\"[^\"]+\"\s+)*"
    r"fn\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)"
)
ROADMAP_TOKEN = re.compile(
    r"(?i)(?:(?<=^)|(?<=_))"
    r"(?P<token>s\d+(?:_\d+)*|phase_?\d+|"
    r"phase_(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|"
    r"thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|"
    r"twenty_one|twenty_two|twenty_three|twenty_four|twenty_five|twenty_six|"
    r"twenty_seven|twenty_eight|twenty_nine|thirty|thirty_one|thirty_two|"
    r"thirty_three|thirty_four|thirty_five|thirty_six|thirty_seven|thirty_eight)|"
    r"milestone_?\d+|roadmap_?\d+)"
    r"(?=_|$)"
)
TEST_PATH = re.compile(
    r"(?:^|/)(?:tests?|fixtures?|compile_fail)(?:/|$)|"
    r"(?:tests?|test_support|fixtures?|compile_fail)\.rs$",
    re.IGNORECASE,
)
WEAK_PROPOSALS = {
    "admit",
    "canonical",
    "certify",
    "closeout",
    "current",
    "fixed",
    "handoff",
    "publish",
    "readiness",
    "required",
    "require",
    "verify",
}

STAGE_DOMAINS = {
    "s0": "foundational_handoff",
    "s1": "physical_format",
    "s2": "physical_substrate",
    "s3": "physical_integrity",
    "s4": "recovery",
    "s4_5": "simulation_harness",
    "s45": "simulation_harness",
    "s5": "physical_isolation",
    "s5_1": "security_scope",
    "s6": "io_qos",
    "s7": "blob_lifecycle",
    "s8": "layout_index",
    "s9": "durability_model",
    "s10": "backup_repair",
    "s11": "operator_authorization",
    "s12": "formal_certification",
}

EXPLICIT_NAMES = {
    ("worth-store-layout-indexes/src/bootstrap/bootstrap_only_path.rs", "s8_fixed"): "fixed_bootstrap_access_path",
    ("worth-store-layout-indexes/src/bootstrap/facade.rs", "s8"): "new",
    ("worth-store-physical-format/src/extent_record/authority.rs", "s1"): "for_canonical_physical_format",
    ("worth-store-physical-format/src/generation/authority.rs", "s1"): "for_canonical_physical_format",
    ("worth-store-physical-format/src/header/authority.rs", "s1"): "for_canonical_physical_format",
    ("worth-store-physical-format/src/manifest/authority.rs", "s1"): "for_canonical_physical_format",
    ("worth-store-physical-format/src/manifest/universe.rs", "s1"): "for_canonical_physical_format",
    ("worth-store-physical-format/src/offline_verifier/verifier.rs", "s1"): "for_canonical_physical_format",
    ("worth-store-physical-format/src/page_record/authority.rs", "s1"): "for_canonical_physical_format",
    ("worth-store-physical-format/src/reference/authority.rs", "s1"): "for_canonical_physical_format",
    ("worth-store-physical-integrity/src/chunk_integrity.rs", "s3"): "new",
    ("worth-store-physical-integrity/src/index_page_integrity.rs", "s3"): "new",
    ("worth-store-physical-integrity/src/manifests/manifest_integrity.rs", "s3"): "new",
    ("worth-store-physical-integrity/src/wal_frame_integrity.rs", "s3"): "new",
    ("worth-store-physical-certification/src/harness/blob/profile.rs", "phase8_required"): "required_qualification_profiles",
    ("worth-store-buffer-pool/src/residency_vocabulary.rs", "s2_phase_one"): "physical_substrate_vocabulary",
    ("worth-store-layout-indexes/src/customization/capability.rs", "phase_eight_capability"): "requested_capability",
    ("worth-store-certification/src/scenario/cross_cutting/harness.rs", "roadmap_2"): "cross_cutting_scenario",
    ("worth-store-certification/src/courtroom/recovery/harness/scenario/crash_matrix.rs", "roadmap_2_s4"): "recovery_crash_matrix",
    ("worth-store-blob-chunks/src/heavy_fixture.rs", "canonical_phase23_patterns"): "canonical_heavy_blob_patterns",
    ("worth-store-blob-chunks/src/harness_execution/chunk_sequence.rs", "phase22_execution_topology"): "blob_harness_execution_topology",
    ("worth-store-physical-backend/src/operation_boundary.rs", "phase4_registered_seams"): "registered_backend_operation_seams",
    ("worth-store-physical-certification/src/oracles/verdict.rs", "phase7_verdict_topology"): "oracle_verdict_topology",
    ("worth-store-physical-certification/src/pressure_harness/tests/io_pressure_harness_tests.rs", "expected_phase10_dimensions"): "expected_pressure_dimensions",
    ("worth-store-physical-certification/src/harness/blob/replay.rs", "phase22_shortcut_rejections"): "blob_replay_shortcut_rejections",
    ("worth-store-physical-certification/src/qualification/scheduling/backends/tests/backend_qualification_cross_backend_tests.rs", "phase_11_capabilities"): "backend_qualification_capabilities",
    ("worth-store-physical-certification/src/qualification/scheduling/backends/tests/backend_qualification_matrix_surface_tests.rs", "phase_11_capabilities"): "backend_qualification_capabilities",
    ("worth-store-layout-indexes/src/keyspace/tests_support.rs", "admit_phase_four_scope"): "admit_key_domain_scope",
    ("worth-store-layout-indexes/src/strategy/tests_support.rs", "admit_phase_five_scope"): "admit_strategy_scope",
    ("worth-store-extensions/src/customization/tests.rs", "admit_phase_five_scope"): "admit_strategy_scope",
    ("worth-store-physical-format/src/format_identity/magic.rs", "s1_store"): "store_format_magic",
    ("worth-store-physical-format/src/format_identity/magic.rs", "s1_initial"): "initial_format_version",
    ("worth-store-physical-format/src/bootstrap/catalog.rs", "s8_minimal"): "minimal_layout_bootstrap_catalog",
    ("worth-store-recovery-physics/src/offline_verifier/persisted_artifacts.rs", "strict_s4"): "strict_offline_recovery_artifacts",
    ("worth-store-physical-isolation/src/epoch/publication.rs", "s7_placeholder"): "blob_placement_placeholder",
}

EXPLICIT_LINE_NAMES = {
    ("worth-store-io-scheduler/src/admission/isolation.rs", "required_from_s5", 111): "required_physical_stability_assumptions",
    ("worth-store-io-scheduler/src/admission/isolation.rs", "required_from_s5", 122): "required_unsupported_qos_non_claims",
    ("worth-store-physical-format/src/binary_format/operation_complexity.rs", "s1_required", 14): "required_physical_operations",
    ("worth-store-physical-format/src/binary_format/operation_complexity.rs", "s1_required", 78): "required_complexity_contract",
    ("worth-store-physical-format/src/facade/reopen.rs", "reopen_s1", 25): "reopen",
    ("worth-store-physical-format/src/facade/reopen.rs", "reopen_s1", 80): "reopen_from_verified_layout",
    ("worth-store-readiness/src/physical_integrity/readiness.rs", "s2_readiness", 30): "physical_substrate_readiness",
    ("worth-store-readiness/src/physical_integrity/readiness.rs", "s2_readiness", 109): "physical_substrate_readiness_fixture",
}

FIELDS = (
    "rename_id",
    "crate",
    "source_set",
    "relative_file",
    "line",
    "visibility",
    "current_name",
    "roadmap_tokens",
    "proposed_name",
    "disposition",
    "confidence",
    "collision_count",
    "boundary_review",
    "rationale",
)


def stage_domain(tokens: list[str]) -> str:
    for token in tokens:
        normalized = token.lower()
        if normalized in STAGE_DOMAINS:
            return STAGE_DOMAINS[normalized]
    return ""


def qualify_weak_proposal(proposed: str, domain: str, relative: str) -> str:
    if proposed == "canonical":
        return f"canonical_{domain}"
    if proposed == "required_for":
        return f"required_for_{domain}"
    if proposed == "locality":
        return "physical_operation_locality"
    if proposed == "bound":
        return "physical_operation_asymptotic_bound"
    if proposed == "recovery":
        return "recovery_vertical_slice"
    if proposed == "minimal":
        return f"minimal_{domain}"
    if proposed == "strict":
        return f"strict_{domain}"
    if proposed == "harness":
        return f"{domain}_harness"
    if proposed == "counters":
        return f"{domain}_counters"
    if proposed in WEAK_PROPOSALS:
        return f"{domain}_{proposed}" if domain else proposed
    if not proposed and domain:
        return f"{domain}_authority"
    return proposed


def collapse_domain_repetition(proposed: str) -> str:
    replacements = {
        "foundational_handoff_handoff": "foundational_handoff",
        "physical_format_physical_format": "physical_format",
        "physical_substrate_physical_substrate": "physical_substrate",
        "physical_integrity_physical_integrity": "physical_integrity",
        "physical_integrity_integrity": "physical_integrity",
        "physical_isolation_physical_isolation": "physical_isolation",
        "simulation_harness_harness": "simulation_harness",
        "security_scope_scope": "security_scope",
        "io_qos_io_qos": "io_qos",
        "io_qos_qos": "io_qos",
        "recovery_recovery": "recovery",
    }
    previous = ""
    while proposed != previous:
        previous = proposed
        for repeated, canonical in replacements.items():
            proposed = proposed.replace(repeated, canonical)
    return proposed


def proposal_for(relative: str, line_number: int, name: str) -> tuple[str, str, str, str, str]:
    tokens = [match.group("token") for match in ROADMAP_TOKEN.finditer(name)]
    proposed = ROADMAP_TOKEN.sub(
        lambda match: STAGE_DOMAINS.get(match.group("token").lower(), ""),
        name,
    )
    proposed = re.sub(r"_+", "_", proposed).strip("_")

    crate_relative = relative.split("/crates/", 1)[-1]
    explicit = EXPLICIT_LINE_NAMES.get((crate_relative, name, line_number))
    explicit = explicit or EXPLICIT_NAMES.get((crate_relative, name))
    if explicit:
        return (
            explicit,
            "rename",
            "high",
            "callsite_validation",
            "The final name describes the permanent constructor or operation rather than roadmap order.",
        )

    domain = stage_domain(tokens)
    proposed = qualify_weak_proposal(proposed, domain, relative)
    proposed = collapse_domain_repetition(proposed)

    if not proposed or proposed in WEAK_PROPOSALS:
        return (
            proposed,
            "semantic_redesign_required",
            "low",
            "required",
            "Removing the roadmap token does not leave a sufficiently specific domain operation name.",
        )

    return (
        proposed,
        "rename",
        "medium",
        "callsite_validation",
        "The final name preserves the domain operation while removing roadmap-order vocabulary.",
    )


def inventory() -> list[dict[str, str | int]]:
    rows: list[dict[str, str | int]] = []
    for path in sorted(CRATES_ROOT.rglob("*.rs")):
        relative = path.relative_to(REPO_ROOT).as_posix()
        crate = path.relative_to(CRATES_ROOT).parts[0]
        source_set = "test_or_support" if TEST_PATH.search(relative) else "production_path"
        for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
            match = FUNCTION_PATTERN.match(line)
            if not match or not ROADMAP_TOKEN.search(match.group("name")):
                continue
            name = match.group("name")
            proposed, disposition, confidence, review, rationale = proposal_for(
                relative, line_number, name
            )
            tokens = ";".join(m.group("token") for m in ROADMAP_TOKEN.finditer(name))
            rows.append(
                {
                    "crate": crate,
                    "source_set": source_set,
                    "relative_file": relative,
                    "line": line_number,
                    "visibility": (match.group("visibility") or "private").strip(),
                    "current_name": name,
                    "roadmap_tokens": tokens,
                    "proposed_name": proposed,
                    "disposition": disposition,
                    "confidence": confidence,
                    "boundary_review": review,
                    "rationale": rationale,
                }
            )

    collisions = Counter(
        (row["relative_file"], row["proposed_name"])
        for row in rows
        if row["proposed_name"]
    )
    for index, row in enumerate(rows, 1):
        row["rename_id"] = f"FS-FN-{index:04d}"
        row["collision_count"] = collisions[(row["relative_file"], row["proposed_name"])]
        if row["collision_count"] > 1:
            row["boundary_review"] = "rust_scope_validation"
            row["rationale"] = (
                "The final name repeats in this file; verify the declarations remain separated by Rust impl or module scope."
            )
    return rows


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    args = parser.parse_args()

    rows = inventory()
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with args.output.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)

    print(f"wrote {len(rows)} rows to {args.output}")


if __name__ == "__main__":
    main()
