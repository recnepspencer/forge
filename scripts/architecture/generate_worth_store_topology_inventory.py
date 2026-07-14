#!/usr/bin/env python3
"""Generate the reviewable Worth Store topology migration inventory."""

from __future__ import annotations

import argparse
import csv
import io
import re
import subprocess
from pathlib import Path, PurePosixPath

from topology_inventory_overrides import apply_override, load_overrides, validate_rows


REPO_ROOT = Path(__file__).resolve().parents[2]
WORKSPACE_PREFIX = PurePosixPath("workspaces/worth-store")
DEFAULT_OUTPUT = REPO_ROOT / "_docs/worth-store/worth-store-topology-inventory.csv"
CLASSIFICATION_DIR = REPO_ROOT / "_docs/worth-store/topology-classifications"

MILESTONE_PATTERN = re.compile(r"(^|[/_.-])(s\d+(?:_\d+)?|phase\d+|milestone\d+)([/_.-]|$)")
TEST_PATTERN = re.compile(r"(^|_)(test|tests|fixture|fixtures|compile_fail|property|scenario)($|_)")

ROOT_FAMILIES: dict[str, tuple[tuple[str, str], ...]] = {
    "worth-store-aspect-native": (
        ("canonical_basis_", "canonical_basis"),
        ("terminal_projection_", "terminal_projection"),
        ("authoritative_", "authority"),
        ("identity_authority", "authority"),
        ("digest_authority", "authority"),
        ("evidence_receipts", "receipts"),
        ("performance_receipts", "receipts"),
    ),
    "worth-store-buffer-pool": (
        ("background_envelope_", "background_work"),
        ("speculative_work_", "speculative_work"),
        ("resident_frame_", "residency"),
        ("record_view_", "record_access"),
        ("dirty_", "dirty_pages"),
        ("eviction_", "eviction"),
        ("pin_", "pinning"),
        ("entry_", "residency"),
        ("allocation_", "allocation"),
    ),
    "worth-store-operations": (
        ("backup_export_", "backup/export"),
        ("backup_import_", "backup/import"),
        ("import_", "backup/import"),
        ("repair_blast_radius_", "repair/blast_radius"),
        ("repair_quarantine_", "repair/quarantine"),
    ),
    "worth-store-physical-integrity": (
        ("integrity_authority_", "authority"),
        ("integrity_evidence_", "evidence"),
        ("index_page_integrity_", "index_pages"),
        ("wal_frame_integrity_", "wal_frames"),
        ("container_integrity_", "containers"),
        ("chunk_integrity_", "blob_chunks"),
        ("manifest_integrity_", "manifests"),
        ("manifest_", "manifests"),
        ("checksum_", "checksums"),
        ("physical_scope_", "admission/physical_scope"),
        ("pre_decode_", "admission/pre_decode"),
        ("entry_", "admission/entry"),
        ("quarantine_", "quarantine"),
        ("scrub_", "scrub"),
    ),
    "worth-store-security": (
        ("physical_security_metadata_", "physical_metadata"),
        ("security_scope_", "scope"),
        ("authenticity_", "authenticity"),
        ("trust_boundary_", "trust_boundary"),
    ),
}

LAYOUT_INDEX_TARGETS = {
    "access_shape": "access/shape",
    "artifact_family": "catalog",
    "budget": "access/budget",
    "corruption": "integrity",
    "customization": "customization",
    "degraded_access": "access/degraded",
    "execution": "access/execution",
    "key_domain": "keyspace",
    "legacy_disposition": "evolution/compatibility",
    "maintenance": "maintenance",
    "materialization": "materialization",
    "migration": "evolution/migration",
    "planning": "access/planning",
    "strategy": "strategy",
    "strategy_registry": "strategy/registry",
}

CERTIFICATION_DOMAINS = (
    (("blob", "chunk", "capsule", "stream"), "blobs"),
    (("layout", "btree", "lsm", "index"), "layout"),
    (("security", "authenticity", "scope", "custody"), "security"),
    (("recovery", "replay", "checkpoint", "redo"), "recovery"),
    (("wal", "durability", "publication"), "durability"),
    (("buffer", "resident", "eviction", "pin", "allocation", "memory"), "memory"),
    (("queue", "foreground", "background", "pacing", "latency", "io_"), "scheduling"),
    (("integrity", "checksum", "manifest", "quarantine", "scrub", "physical"), "physical_integrity"),
    (("aspect", "canonical", "terminal", "json", "digest", "foundational"), "foundational"),
)

FIELDS = (
    "current_path",
    "crate",
    "target_crate",
    "crate_relative_path",
    "current_directory",
    "source_set",
    "file_kind",
    "line_count",
    "current_subsystem",
    "semantic_role",
    "authority_posture",
    "proposed_action",
    "candidate_action",
    "target_subsystem",
    "target_directory",
    "target_path",
    "mechanical_ready",
    "confidence",
    "content_review_required",
    "review_batch",
    "reviewed_content_sha256",
    "rationale",
)

def discover_files() -> list[Path]:
    command = [
        "git",
        "ls-files",
        "--cached",
        "--others",
        "--exclude-standard",
        "--",
        str(WORKSPACE_PREFIX),
    ]
    result = subprocess.run(command, cwd=REPO_ROOT, check=True, capture_output=True, text=True)
    paths = []
    for raw_path in result.stdout.splitlines():
        path = REPO_ROOT / raw_path
        if path.is_file():
            paths.append(path)
    return sorted(paths, key=lambda path: path.as_posix().lower())


def locate_crate(relative: PurePosixPath) -> tuple[str, PurePosixPath]:
    parts = relative.parts
    if len(parts) >= 4 and parts[:3] == ("workspaces", "worth-store", "crates"):
        return parts[3], PurePosixPath(*parts[4:])
    return "worth-store-workspace", PurePosixPath(*parts[2:])


def source_set(crate_relative: PurePosixPath) -> str:
    first = crate_relative.parts[0] if crate_relative.parts else ""
    if first == "src":
        return "production"
    if first == "tests":
        return "integration_test"
    if first == "benches":
        return "benchmark"
    if first == "examples":
        return "example"
    if crate_relative.name == "build.rs":
        return "build"
    if crate_relative.name == "Cargo.toml":
        return "manifest"
    if crate_relative.suffix.lower() == ".md":
        return "documentation"
    return "workspace_support"


def semantic_role(path: PurePosixPath, set_name: str) -> str:
    stem = path.stem.lower()
    joined = path.as_posix().lower()
    if set_name in {"integration_test", "benchmark"} or TEST_PATTERN.search(joined):
        return "test_or_certification"
    if stem in {"lib", "mod", "public_api"} or "facade" in stem or "exports" in stem:
        return "facade_or_aggregation"
    for token, role in (
        ("admission", "admission"),
        ("classif", "classification"),
        ("verif", "verification"),
        ("transition", "transition"),
        ("receipt", "receipt_projection"),
        ("counter", "measurement"),
        ("evidence", "evidence_projection"),
        ("denial", "denial"),
        ("request", "request_declaration"),
        ("plan", "planning"),
        ("execution", "execution"),
        ("model", "model"),
        ("vocabulary", "vocabulary"),
    ):
        if token in stem:
            return role
    return "implementation"


def authority_posture(path: PurePosixPath, role: str, set_name: str) -> str:
    joined = path.as_posix().lower()
    if set_name in {"integration_test", "benchmark"} or "certification" in joined:
        return "test_or_courtroom"
    if role in {"receipt_projection", "measurement", "evidence_projection"}:
        return "derived_or_observational"
    if any(token in joined for token in ("authority", "admission", "witness", "permit", "capability", "transition")):
        return "authority_related_requires_review"
    if role == "facade_or_aggregation":
        return "boundary_surface"
    return "unclassified"


def current_subsystem(crate_relative: PurePosixPath) -> str:
    parts = crate_relative.parts
    if parts and parts[0] in {"src", "tests", "benches", "examples"}:
        return parts[1] if len(parts) > 2 else "crate_root"
    return "crate_support"


def line_count(path: Path) -> int:
    if path.suffix.lower() not in {".rs", ".md", ".toml", ".py", ".ps1"}:
        return 0
    try:
        return len(path.read_text(encoding="utf-8", errors="replace").splitlines())
    except OSError:
        return 0


def family_target(crate: str, filename: str) -> str | None:
    for prefix, target in ROOT_FAMILIES.get(crate, ()):
        if filename.startswith(prefix):
            return target
    return None


def certification_target(filename: str, role: str) -> str:
    domain = "cross_cutting"
    for tokens, candidate in CERTIFICATION_DOMAINS:
        if any(token in filename for token in tokens):
            domain = candidate
            break
    if role == "evidence_projection" or "evidence" in filename:
        return f"evidence/{domain}"
    if any(token in filename for token in ("scenario", "harness", "fixture", "driver", "transcript", "story")):
        return f"scenario/{domain}"
    return f"courtroom/{domain}"


def target_for(crate: str, crate_relative: PurePosixPath, role: str) -> tuple[str, str | None]:
    parts = crate_relative.parts
    if len(parts) < 2 or parts[0] != "src":
        return current_subsystem(crate_relative), None
    subsystem = current_subsystem(crate_relative)
    filename = crate_relative.name.lower()
    if crate == "worth-store-layout-indexes" and subsystem in LAYOUT_INDEX_TARGETS:
        mapped_root = PurePosixPath(LAYOUT_INDEX_TARGETS[subsystem])
        nested_parent = PurePosixPath(*parts[2:-1]) if len(parts) > 3 else PurePosixPath()
        target = mapped_root / nested_parent
        current_parent = PurePosixPath(*parts[1:-1])
        if target == current_parent:
            return target.as_posix(), None
        return target.as_posix(), target.as_posix()
    if crate == "worth-store-certification" and subsystem == "crate_root" and filename not in {"lib.rs", "public_api.rs", "internal_modules.rs"}:
        target = certification_target(filename, role)
        return target, target
    root_target = family_target(crate, filename) if subsystem == "crate_root" else None
    if root_target:
        return root_target, root_target
    return subsystem, None


def classify(path: Path) -> dict[str, str | int]:
    relative = PurePosixPath(path.relative_to(REPO_ROOT).as_posix())
    crate, crate_relative = locate_crate(relative)
    set_name = source_set(crate_relative)
    role = semantic_role(crate_relative, set_name)
    authority = authority_posture(crate_relative, role, set_name)
    lines = line_count(path)
    subsystem, target_directory = target_for(crate, crate_relative, role)
    joined = crate_relative.as_posix().lower()
    milestone_named = bool(MILESTONE_PATTERN.search(joined))
    structural_debt = any(part in {"layout_access", "skeleton", "production_transition"} for part in crate_relative.parts)
    oversized_rust = path.suffix.lower() == ".rs" and lines > 400

    action = "keep"
    candidate = "keep"
    confidence = "medium"
    review = "false"
    rationale = "Current placement names a plausible stable responsibility."

    if milestone_named:
        action = "review"
        candidate = "move_or_delete"
        confidence = "high"
        review = "true"
        rationale = "Roadmap provenance appears in code topology or filename; permanent domain ownership must replace it."
    elif structural_debt:
        action = "review"
        candidate = "move_or_delete"
        confidence = "high"
        review = "true"
        rationale = "Cross-cutting scaffolding obscures the permanent owner and cannot be moved safely by name alone."
    elif oversized_rust:
        action = "review"
        candidate = "split_or_exempt"
        confidence = "high"
        review = "true"
        rationale = "Rust file exceeds the default 400-line cap; cohesion or an explicit exemption must be established."
    elif target_directory:
        action = "move"
        candidate = "move"
        confidence = "high" if crate != "worth-store-certification" else "medium"
        review = "false"
        rationale = "Repeated filename family or established domain mapping supplies a deterministic target directory."
    elif role == "facade_or_aggregation" and lines > 100:
        action = "review"
        candidate = "split"
        confidence = "medium"
        review = "true"
        rationale = "Broad facade or aggregation file is large enough to require a business-logic audit before retention."

    target_path = ""
    mechanical_ready = "false"
    if action == "move" and target_directory:
        target_path = f"workspaces/worth-store/crates/{crate}/src/{target_directory}/{crate_relative.name}"
        mechanical_ready = "true"
    elif action == "keep":
        target_path = relative.as_posix()
        mechanical_ready = "true"

    return {
        "current_path": relative.as_posix(),
        "crate": crate,
        "target_crate": crate,
        "crate_relative_path": crate_relative.as_posix(),
        "current_directory": crate_relative.parent.as_posix(),
        "source_set": set_name,
        "file_kind": path.suffix.lower().lstrip(".") or "none",
        "line_count": lines,
        "current_subsystem": current_subsystem(crate_relative),
        "semantic_role": role,
        "authority_posture": authority,
        "proposed_action": action,
        "candidate_action": candidate,
        "target_subsystem": subsystem,
        "target_directory": target_directory or crate_relative.parent.as_posix(),
        "target_path": target_path,
        "mechanical_ready": mechanical_ready,
        "confidence": confidence,
        "content_review_required": review,
        "review_batch": "",
        "reviewed_content_sha256": "",
        "rationale": rationale,
    }


def render() -> str:
    overrides = load_overrides(CLASSIFICATION_DIR)
    paths = discover_files()
    rows = [classify(path) for path in paths]
    rows_by_path = {str(row["current_path"]): (row, path) for row, path in zip(rows, paths)}
    missing = sorted(set(overrides) - set(rows_by_path))
    if missing:
        raise ValueError(f"classifications reference missing files: {missing}")
    for current_path, override in overrides.items():
        row, path = rows_by_path[current_path]
        apply_override(row, path, override)
    validate_rows(rows)

    output = io.StringIO(newline="")
    writer = csv.DictWriter(output, fieldnames=FIELDS, lineterminator="\n")
    writer.writeheader()
    writer.writerows(rows)
    return output.getvalue()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    generated = render()
    if args.check:
        existing = args.output.read_text(encoding="utf-8") if args.output.exists() else ""
        return 0 if existing == generated else 1
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(generated, encoding="utf-8", newline="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
