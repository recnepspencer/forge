#!/usr/bin/env python3
"""Generate compact content fingerprints for Worth Store topology review rows."""

from __future__ import annotations

import argparse
import csv
import hashlib
import io
import re
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
INVENTORY = REPO_ROOT / "_docs/worth-store/worth-store-topology-inventory.csv"
DEFAULT_OUTPUT = REPO_ROOT / "_docs/worth-store/worth-store-topology-fingerprints.csv"

ITEM_START = re.compile(
    r"^\s*(?:#\[[^]]+\]\s*)*(?:(pub(?:\([^)]*\))?)\s+)?"
    r"(async\s+fn|const|enum|extern\s+crate|fn|impl|macro_rules!|mod|static|struct|trait|type|union|use)\b"
)
CALL_PATTERN = re.compile(r"\b([A-Za-z_][A-Za-z0-9_:]*)\s*(?:::<[^;{}()]*>)?\s*\(")
TYPE_PATTERN = re.compile(r"\b[A-Z][A-Za-z0-9_]*(?:::[A-Z][A-Za-z0-9_]*)*\b")
MILESTONE_PATTERN = re.compile(r"\b(?:S\d+(?:_\d+)?|Phase\d+|Milestone\d+)\w*")
AUTHORITY_MARKERS = (
    "AuthorityWitness",
    "TransitionOutcome",
    "Readiness",
    "Admission",
    "Witness",
    "Capability",
    "Authority",
    "Receipt",
    "Evidence",
    "Handoff",
    "Closeout",
)

FIELDS = (
    "current_path",
    "content_sha256",
    "documentation",
    "imports",
    "module_declarations",
    "public_items",
    "private_items",
    "referenced_types",
    "called_functions",
    "authority_markers",
    "milestone_identifiers",
    "serde_markers",
    "macro_invocations",
    "function_count",
    "branch_tokens",
    "max_brace_depth",
)


def compact(value: str) -> str:
    return " ".join(value.strip().split())


def unique(values: list[str], limit: int = 80) -> str:
    return " | ".join(dict.fromkeys(value for value in values if value))[:12000]


def statement_at(lines: list[str], start: int) -> tuple[str, int]:
    parts: list[str] = []
    index = start
    while index < len(lines) and len(parts) < 16:
        part = compact(lines[index])
        parts.append(part)
        if any(token in part for token in ("{", ";")) and not part.endswith(","):
            break
        index += 1
    return compact(" ".join(parts)), index


def fingerprint(path: Path) -> dict[str, str | int]:
    raw = path.read_bytes()
    text = raw.decode("utf-8", errors="replace")
    lines = text.splitlines()
    docs: list[str] = []
    imports: list[str] = []
    modules: list[str] = []
    public_items: list[str] = []
    private_items: list[str] = []
    macro_invocations: list[str] = []
    brace_depth = 0
    max_depth = 0
    function_count = 0
    index = 0

    while index < len(lines):
        stripped = lines[index].strip()
        if stripped.startswith(("//!", "///")) and len(docs) < 8:
            docs.append(stripped.lstrip("/! "))
        match = ITEM_START.match(lines[index])
        if match and brace_depth <= 1:
            signature, end = statement_at(lines, index)
            visibility, kind = match.groups()
            if kind.endswith("fn") or kind == "fn":
                function_count += 1
            if kind == "use":
                imports.append(signature)
            elif kind == "mod":
                modules.append(signature)
            elif visibility:
                public_items.append(signature)
            else:
                private_items.append(signature)
            index = end
        if "!" in stripped and not stripped.startswith(("//", "#!", "#[")):
            found = re.match(r"\s*([A-Za-z_][A-Za-z0-9_:]*)!", stripped)
            if found:
                macro_invocations.append(found.group(1))
        brace_depth += lines[index].count("{") - lines[index].count("}")
        brace_depth = max(brace_depth, 0)
        max_depth = max(max_depth, brace_depth)
        index += 1

    types = sorted(set(TYPE_PATTERN.findall(text)))
    calls = sorted(set(CALL_PATTERN.findall(text)))
    authority = [marker for marker in AUTHORITY_MARKERS if marker in text]
    milestone = sorted(set(MILESTONE_PATTERN.findall(text)))
    serde = [marker for marker in ("Serialize", "Deserialize", "serde", "serde_json") if marker in text]
    branches = sum(len(re.findall(pattern, text)) for pattern in (r"\bif\b", r"\bmatch\b", r"\bfor\b", r"\bwhile\b", r"\?"))

    return {
        "current_path": path.relative_to(REPO_ROOT).as_posix(),
        "content_sha256": hashlib.sha256(raw).hexdigest(),
        "documentation": unique(docs),
        "imports": unique(imports),
        "module_declarations": unique(modules),
        "public_items": unique(public_items),
        "private_items": unique(private_items),
        "referenced_types": unique(types),
        "called_functions": unique(calls),
        "authority_markers": unique(authority),
        "milestone_identifiers": unique(milestone),
        "serde_markers": unique(serde),
        "macro_invocations": unique(macro_invocations),
        "function_count": function_count,
        "branch_tokens": branches,
        "max_brace_depth": max_depth,
    }


def review_paths() -> list[Path]:
    with INVENTORY.open(newline="", encoding="utf-8") as source:
        rows = csv.DictReader(source)
        return [
            REPO_ROOT / row["current_path"]
            for row in rows
            if row["content_review_required"] == "true" or row["review_batch"]
            and row["file_kind"] == "rs"
            and (REPO_ROOT / row["current_path"]).is_file()
        ]


def render() -> str:
    output = io.StringIO(newline="")
    writer = csv.DictWriter(output, fieldnames=FIELDS, lineterminator="\n")
    writer.writeheader()
    writer.writerows(fingerprint(path) for path in review_paths())
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
    args.output.write_text(generated, encoding="utf-8", newline="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
