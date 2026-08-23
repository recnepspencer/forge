from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path

from worth_ui_ledger_artifact_transaction import ArtifactTransaction, replace_bytes
from worth_ui_ledger_command import claim_digest_for_row, source_revision
from worth_ui_ledger_causal_revalidation import (
    encoded_payload,
    revalidate_row_payload,
)
from worth_ui_ledger_execution_observation_store import CACHE_ENV
from worth_ui_ledger_portfolio_snapshot import DIGEST_ENV, REVISION_ENV
from worth_ui_ledger_row_cache import RowEvidenceCache
from worth_ui_ledger_row_execution import (
    CachedEvidenceRejected,
    execute_or_restore,
    run_row,
)
from worth_ui_ledger_shared_execution_lineage import (
    SharedExecutionLineageRequest,
    inherit_shared_receipt_lineage,
)
from worth_ui_ledger_runner_authentication import authentication_tag
from worth_ui_ledger_source_state import source_state_digest


ROOT = Path(__file__).resolve().parents[2]
LEDGER = ROOT / "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"


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


def run(command_text: str, candidate_ledger: Path | None = None) -> dict[str, object]:
    return run_row(ROOT, command_text, candidate_ledger)


def retain_selected_acceptance(
    selected: list[dict[str, str]], root: Path = ROOT, ledger: Path = LEDGER
) -> None:
    """Retain current authenticated row evidence without publishing ledger closure."""
    original = ledger.read_bytes()
    revision = source_revision()
    state_digest = source_state_digest(revision)
    cache_root = (
        root / "workspaces/worth-ui/target/milestone-3141-execution-cache" / state_digest
    )
    row_cache = RowEvidenceCache(root, cache_root, original, revision, state_digest)
    previous_cache = os.environ.get(CACHE_ENV)
    previous_revision = os.environ.get(REVISION_ENV)
    previous_digest = os.environ.get(DIGEST_ENV)
    os.environ[CACHE_ENV] = str(cache_root)
    os.environ[REVISION_ENV] = revision
    os.environ[DIGEST_ENV] = state_digest
    artifacts = ArtifactTransaction(
        root, ledger, [row["exact_command"] for row in selected]
    )
    try:
        for row in selected:
            result = execute_or_restore(
                row,
                ledger,
                row_cache,
                claim_digest_for_row(row),
                lambda command, candidate=None: run_row(root, command, candidate),
                lambda payload, selected_row=row: bind_current_result_mapping(
                    selected_row, payload, root
                ),
            )
            retain_portfolio_artifact(row, result, {}, root)
        if source_revision() != revision or source_state_digest(revision) != state_digest:
            raise RuntimeError("governed source changed during batch acceptance")
        artifacts.prepare_commit(original)
        replace_bytes(ledger, original)
        artifacts.commit()
        print(
            "accepted "
            f"{len(selected)} Worth UI milestone 3.14.1 proof rows without ledger publication"
        )
    except BaseException:
        artifacts.rollback()
        raise
    finally:
        if previous_cache is None:
            os.environ.pop(CACHE_ENV, None)
        else:
            os.environ[CACHE_ENV] = previous_cache
        restore_environment(REVISION_ENV, previous_revision)
        restore_environment(DIGEST_ENV, previous_digest)


def retain_portfolio_artifact(
    row: dict[str, str],
    result: dict[str, object],
    retained: dict[str, bytes],
    root: Path = ROOT,
) -> None:
    words = row["exact_command"].split()
    identity = words[words.index("--artifact") + 1]
    content = (root / identity).read_bytes()
    observed = hashlib.sha256(content).hexdigest()
    if observed != result["artifact_sha256"]:
        raise RuntimeError(f"authenticated artifact drifted for {row['requirement']}")
    retained[identity] = content


def bind_current_result_mapping(
    row: dict[str, str],
    result: dict[str, object],
    root: Path = ROOT,
    current_claim_digest: str | None = None,
    current_revision: str | None = None,
    current_state_digest: str | None = None,
) -> dict[str, object]:
    canonical = row["source_identity"].split(";")
    if result.get("source_identity") != canonical:
        raise CachedEvidenceRejected(
            f"atomic closer substituted a source for {row['requirement']}"
        )
    words = row["exact_command"].split()
    identity = words[words.index("--artifact") + 1]
    artifact_payload = json.loads((root / identity).read_text(encoding="utf-8"))
    payload = {**result, **artifact_payload}
    revision = current_revision or source_revision()
    state_digest = current_state_digest or source_state_digest(revision)
    if (
        payload.get("source_revision") != revision
        or payload.get("source_state_digest") != state_digest
    ):
        rebound = revalidate_row_payload(
            root,
            row,
            payload,
            str(result["artifact_sha256"]),
            current_claim_digest or claim_digest_for_row(row),
            revision,
            state_digest,
        )
        if rebound is None:
            raise CachedEvidenceRejected(
                f"atomic closer cannot causally reuse {row['requirement']}"
            )
        content = encoded_payload(rebound)
        replace_bytes(root / identity, content)
        rebound["artifact_sha256"] = hashlib.sha256(content).hexdigest()
        return rebound
    payload.update(
        {
            "production_entry": row["production_entry"],
            "independent_oracle": row["independent_oracle"],
            "mapping_source_identity": canonical,
            "source_rebindings": [],
            "executed_exact_command": row["exact_command"],
        }
    )
    inherit_shared_receipt_lineage(SharedExecutionLineageRequest(
        row=row,
        payload=payload,
        root=root,
        revision=revision,
        state_digest=state_digest,
        current_claim=current_claim_digest or claim_digest_for_row(row),
    ))
    payload.pop("artifact_sha256", None)
    payload.pop("runner_authentication", None)
    payload["runner_authentication"] = authentication_tag(payload, root)
    content = (json.dumps(payload, indent=2) + "\n").encode()
    replace_bytes(root / identity, content)
    payload["artifact_sha256"] = hashlib.sha256(content).hexdigest()
    return payload


def restore_environment(name: str, value: str | None) -> None:
    if value is None:
        os.environ.pop(name, None)
    else:
        os.environ[name] = value
