from __future__ import annotations

from typing import Any


TEXT_ROOT = "workspaces/worth-ui/crates/worth-ui-text/src"
CERT_ROOT = "workspaces/worth-ui/crates/worth-ui-certification/tests"
RUNTIME_ROOT = "workspaces/worth-ui/crates/worth-ui-runtime/src"
HEADLESS_ROOT = "workspaces/worth-ui/crates/worth-ui-host-headless/src"


def control(
    control_type: Any, package: str, test: str, source: str, target: str = "lib"
) -> Any:
    target_identity = ("lib", "lib") if target == "lib" else ("test", target)
    return control_type(package, target_identity, test, source)


def proof(
    proof_type: Any,
    control_type: Any,
    requirement: str,
    package: str,
    target: tuple[str, str],
    main: str,
    production: str,
    oracle_source: str,
    hostile: str,
    hostile_source: str,
    *extra_sources: str,
) -> Any:
    sources = tuple(
        dict.fromkeys(
            (
                production.rsplit("::", 1)[0],
                oracle_source,
                hostile_source,
                *extra_sources,
            )
        )
    )
    return proof_type(
        package,
        target,
        main,
        production,
        f"{oracle_source}::{main.rsplit('::', 1)[-1]}",
        sources,
        control=control(
            control_type,
            package,
            hostile,
            hostile_source,
            target[1] if target[0] == "test" else "lib",
        ),
    )


def text_proof(
    proof_type: Any,
    control_type: Any,
    requirement: str,
    main: str,
    production: str,
    hostile: str,
    *extra_sources: str,
) -> Any:
    evidence = f"{TEXT_ROOT}/phase4_ledger_evidence.rs"
    return proof(
        proof_type,
        control_type,
        requirement,
        "worth-ui-text",
        ("lib", "lib"),
        f"phase4_ledger_evidence::{main}",
        production,
        evidence,
        f"phase4_ledger_evidence::{hostile}",
        evidence,
        *extra_sources,
    )
