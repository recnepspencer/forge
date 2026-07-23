import csv
import re
from pathlib import Path
from typing import Any

from worth_ui_test_topology_config import (
    Violation,
    required_string,
    required_string_list,
)


REQUIRED_FAMILIES = {
    "execution_plan",
    "ordinary",
    "virtualized_data",
    "canvas_spatial",
    "realtime_overlay",
    "activation",
    "equivalence",
}
ALLOWED_DISPOSITIONS = {
    "below_authority_injection",
    "real_boundary_replacement",
    "delete",
}


def test_seam_inventory_violations(
    root: Path, config: dict[str, Any]
) -> list[Violation]:
    path = root / required_string(config, "test_seam_inventory")
    if not path.is_file():
        return [Violation("test-seam-inventory", f"missing {path.relative_to(root).as_posix()}")]
    with path.open(encoding="utf-8", newline="") as source:
        rows = list(csv.DictReader(source))
    required = {"family", "path", "symbol", "disposition", "owner_phase", "reason"}
    if not rows:
        return [Violation("test-seam-inventory", "inventory has no rows")]
    if not required.issubset(rows[0]):
        return [
            Violation(
                "test-seam-inventory",
                f"missing columns: {', '.join(sorted(required - set(rows[0])))}",
            )
        ]
    violations: list[Violation] = []
    closed_phase = config.get("closed_phase")
    if not isinstance(closed_phase, int) or closed_phase < 0:
        violations.append(
            Violation("test-seam-inventory", "closed_phase must be a non-negative integer")
        )
        closed_phase = -1
    families = {row["family"] for row in rows}
    seam_free_families = set(config.get("seam_free_families", []))
    for family in sorted(seam_free_families - REQUIRED_FAMILIES):
        violations.append(
            Violation("test-seam-inventory", f"unknown seam-free family: {family}")
        )
    for family in sorted(seam_free_families & families):
        violations.append(
            Violation(
                "test-seam-inventory",
                f"{family}: cannot be seam-free while inventory rows remain",
            )
        )
    for family in sorted(REQUIRED_FAMILIES - families - seam_free_families):
        violations.append(Violation("test-seam-inventory", f"missing family: {family}"))
    identities: set[tuple[str, str]] = set()
    for row in rows:
        identity = (row["path"], row["symbol"])
        if identity in identities:
            violations.append(
                Violation("test-seam-inventory", f"duplicate seam: {identity}")
            )
        identities.add(identity)
        if row["family"] not in REQUIRED_FAMILIES:
            violations.append(
                Violation(
                    "test-seam-inventory", f"{row['symbol']}: unknown family {row['family']}"
                )
            )
        if row["disposition"] not in ALLOWED_DISPOSITIONS:
            violations.append(
                Violation(
                    "test-seam-inventory",
                    f"{row['symbol']}: invalid disposition {row['disposition']}",
                )
            )
        deletion_due = (
            row["disposition"] == "delete"
            and row["owner_phase"].isdigit()
            and int(row["owner_phase"]) <= closed_phase
        )
        source_path = root / row["path"]
        if not source_path.is_file():
            if not deletion_due:
                violations.append(
                    Violation("test-seam-inventory", f"{row['symbol']}: missing {row['path']}")
                )
        else:
            symbol_present = row["symbol"] in source_path.read_text(encoding="utf-8")
            if deletion_due and symbol_present:
                violations.append(
                    Violation(
                        "test-seam-inventory",
                        f"{row['symbol']}: deletion assigned to closed phase "
                        f"{row['owner_phase']} still exists",
                    )
                )
            elif not deletion_due and not symbol_present:
                violations.append(
                    Violation(
                        "test-seam-inventory",
                        f"{row['symbol']}: symbol is absent from {row['path']}",
                    )
                )
        if not row["owner_phase"].isdigit() or not row["reason"].strip():
            violations.append(
                Violation(
                    "test-seam-inventory", f"{row['symbol']}: owner phase or reason is missing"
                )
            )
    for source_path, symbol in discovered_test_seams(root, config):
        if (source_path, symbol) not in identities:
            violations.append(
                Violation(
                    "test-seam-inventory",
                    f"{symbol}: test-only seam in {source_path} has no disposition",
                )
            )
    return violations


def discovered_test_seams(root: Path, config: dict[str, Any]):
    function_pattern = re.compile(r"fn\s+([A-Za-z0-9_]*for_test[A-Za-z0-9_]*)")
    variant_pattern = re.compile(
        r"(?m)^\s*#\[cfg\(test\)\]\s*\r?\n"
        r"\s*([A-Z][A-Za-z0-9_]*)\s*(?:[,(=])"
    )
    discovered: list[tuple[str, str]] = []
    for configured_root in required_string_list(config, "test_seam_roots"):
        seam_root = root / configured_root
        if not seam_root.is_dir():
            raise ValueError(f"test seam root is missing: {configured_root}")
        for source in seam_root.rglob("*.rs"):
            if "tests" in source.relative_to(seam_root).parts:
                continue
            text = source.read_text(encoding="utf-8")
            symbols = set(function_pattern.findall(text))
            symbols.update(variant_pattern.findall(text))
            discovered.extend(
                (source.relative_to(root).as_posix(), symbol)
                for symbol in sorted(symbols)
            )
    return discovered
