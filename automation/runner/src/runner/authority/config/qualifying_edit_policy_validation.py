from __future__ import annotations

from typing import Any

SUPPORTED_PROOF_SOURCES = {"git_scoped_diff"}
SUPPORTED_EARLY_DETECTORS = {"filesystem_mtime"}


def validate_qualifying_edit_policy(policy: dict[str, Any], errors: list[str]) -> None:
    include = policy.get("include")
    if include is not None and not _is_string_list(include):
        errors.append("qualifying_edit_policy.include must be a list of strings when present")
    exclude = policy.get("exclude")
    if exclude is not None and not _is_string_list(exclude):
        errors.append("qualifying_edit_policy.exclude must be a list of strings when present")
    proof_source = policy.get("proof_source")
    if proof_source is not None and proof_source not in SUPPORTED_PROOF_SOURCES:
        errors.append(f"qualifying_edit_policy.proof_source must be one of {sorted(SUPPORTED_PROOF_SOURCES)}")
    early_detector = policy.get("early_detector")
    if early_detector is not None and early_detector not in SUPPORTED_EARLY_DETECTORS:
        errors.append(
            f"qualifying_edit_policy.early_detector must be one of {sorted(SUPPORTED_EARLY_DETECTORS)}"
        )


def _is_string_list(value: Any) -> bool:
    return isinstance(value, list) and all(isinstance(item, str) and item for item in value)
