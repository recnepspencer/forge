import re
from pathlib import Path
from typing import Any

from worth_ui_test_topology_config import (
    Violation,
    load_toml,
    required_int,
    required_string,
)


def normalized(path: Path) -> str:
    return path.as_posix()


def rust_test_sources(root: Path, config: dict[str, Any]) -> list[Path]:
    workspace_manifest = root / required_string(config, "workspace_manifest")
    workspace = load_toml(workspace_manifest)
    members = workspace.get("workspace", {}).get("members")
    if not isinstance(members, list) or not all(isinstance(member, str) for member in members):
        raise ValueError("workspace.members must be a list of paths")
    sources: set[Path] = set()
    for member in members:
        member_root = workspace_manifest.parent / member
        for source_root in (member_root / "src", member_root / "tests"):
            if source_root.is_dir():
                sources.update(
                    source
                    for source in source_root.rglob("*.rs")
                    if is_ordinary_test_source(member_root, source)
                )
    return sorted(sources)


def is_ordinary_test_source(member_root: Path, source: Path) -> bool:
    parts = source.relative_to(member_root).parts
    if "target" in parts:
        return False
    if len(parts) > 2 and parts[0] == "tests" and parts[1] == "ui":
        return False
    return not (
        len(parts) > 3
        and parts[0] == "tests"
        and parts[1] == "fixtures"
        and parts[2].startswith("topology_")
    )


def source_violations(root: Path, config: dict[str, Any]) -> list[Violation]:
    violations: list[Violation] = []
    allowed_sessions = {
        normalized(Path(path)) for path in config.get("allowed_trybuild_sessions", [])
    }
    actual_sessions: set[str] = set()
    constructor_count = 0
    for source in rust_test_sources(root, config):
        text = source.read_text(encoding="utf-8")
        relative = normalized(source.relative_to(root))
        source_constructor_count = text.count("trybuild::TestCases::new")
        constructor_count += source_constructor_count
        if source_constructor_count:
            actual_sessions.add(relative)
        if source_constructor_count > 1:
            violations.append(
                Violation(
                    "trybuild-session",
                    f"{relative}: {source_constructor_count} TestCases sessions share one binary",
                )
            )
        if "RUSTFLAGS" in text:
            violations.append(
                Violation("warning-fingerprint", f"{relative}: runtime RUSTFLAGS mutation is forbidden")
            )
        generated_kind = generated_compilation_kind(text)
        if generated_kind is not None:
            violations.append(
                Violation(
                    "generated-compilation",
                    f"{relative}: ordinary test contains {generated_kind}",
                )
            )
    violations.extend(session_owner_violations(actual_sessions, allowed_sessions))
    maximum_sessions = required_int(config, "max_trybuild_sessions")
    if constructor_count > maximum_sessions:
        violations.append(
            Violation(
                "trybuild-session-budget",
                f"{constructor_count} TestCases sessions exceeds {maximum_sessions}",
            )
        )
    violations.extend(real_filesystem_proof_violations(root, config))
    return violations


def real_filesystem_proof_violations(
    root: Path, config: dict[str, Any]
) -> list[Violation]:
    violations: list[Violation] = []
    provider_path = config.get("filesystem_provider_source")
    if isinstance(provider_path, str):
        provider = root / provider_path
        if not provider.is_file():
            violations.append(
                Violation("filesystem-proof-substitution", f"missing {provider_path}")
            )
        else:
            text = provider.read_text(encoding="utf-8")
            for marker in ("with_file", "with_module", "source_text", "source_modules"):
                if marker in text:
                    violations.append(
                        Violation(
                            "filesystem-proof-substitution",
                            f"{provider_path}: filesystem provider contains injection marker {marker}",
                        )
                    )

    prohibited = (
        "WorthUiWatcherEvent",
        "WorthUiSourceEventIngress",
        "WorthUiSourceProvider::in_memory",
        ".with_file(",
    )
    for configured_path in config.get("real_filesystem_proof_sources", []):
        source = root / configured_path
        if not source.is_file():
            violations.append(
                Violation("filesystem-proof-substitution", f"missing {configured_path}")
            )
            continue
        text = source.read_text(encoding="utf-8")
        for marker in prohibited:
            if marker in text:
                violations.append(
                    Violation(
                        "filesystem-proof-substitution",
                        f"{configured_path}: real-boundary proof contains synthetic marker {marker}",
                    )
                )
    return violations


def generated_compilation_kind(text: str) -> str | None:
    command_patterns = (
        (
            "nested Cargo invocation",
            r"(?i)(?:std::process::|process::)?Command::new\([\s\S]{0,120}?cargo",
        ),
        (
            "direct rustc invocation",
            r"(?i)(?:std::process::|process::)?Command::new\([\s\S]{0,120}?rustc",
        ),
        ("nested Cargo manifest invocation", r'\.arg\(\s*"--manifest-path"\s*\)'),
    )
    for kind, pattern in command_patterns:
        if re.search(pattern, text):
            return kind
    writes_files = any(
        primitive in text
        for primitive in ("fs::write", "File::create", "OpenOptions::new", ".write_all(")
    )
    if '"Cargo.toml"' in text and writes_files:
        return "generated Cargo manifest"
    return None


def session_owner_violations(actual, allowed) -> list[Violation]:
    violations: list[Violation] = []
    for path in sorted(actual - allowed):
        violations.append(Violation("trybuild-session", f"unexpected session owner: {path}"))
    for path in sorted(allowed - actual):
        violations.append(Violation("trybuild-session", f"configured session owner missing: {path}"))
    return violations
