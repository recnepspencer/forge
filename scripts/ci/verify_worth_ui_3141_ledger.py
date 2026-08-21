from __future__ import annotations

import csv
import json
import os
from pathlib import Path

from worth_ui_ledger_command import source_digest
from worth_ui_ledger_verifier_invocation import parse_args
from worth_ui_ledger_retained_portfolio import (
    persist_referenced_receipts,
    portfolio_identity,
    validate as validate_retained_portfolio,
)
from worth_ui_predecessor_causal_refresh import refresh_handoff


ROOT = Path(__file__).resolve().parents[2]
LEDGER = ROOT / "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"


def retained_source_binding(through_phase: int) -> tuple[str, str]:
    identity = ROOT / portfolio_identity(through_phase)
    retained = json.loads(identity.read_text(encoding="utf-8"))
    revision = retained.get("source_revision")
    state_digest = retained.get("source_state_digest")
    if not isinstance(revision, str) or not isinstance(state_digest, str):
        raise RuntimeError("retained closure portfolio omits its source binding")
    return revision, state_digest


def ledger_identity() -> Path:
    configured = os.environ.get("WORTH_UI_MILESTONE_3141_LEDGER")
    return Path(configured).resolve() if configured else LEDGER


def rows(through_phase: int) -> list[dict[str, str]]:
    with ledger_identity().open(encoding="utf-8", newline="") as stream:
        complete = list(csv.DictReader(stream))
    result = [row for row in complete if int(row["phase"]) <= through_phase]
    expected = {2: 30, 3: 47, 4: 68, 5: 80}[through_phase]
    if len(result) != expected or any(
        row["result"] != "PROVED" or row["final_source"] != "true" for row in result
    ):
        raise RuntimeError(
            f"operational verification requires {expected} final-source proved rows"
        )
    return result


def main() -> int:
    arguments = parse_args()
    if arguments.artifact is not None:
        refresh_handoff(
            ROOT,
            ledger_identity(),
            arguments.through_phase + 1,
            arguments.artifact,
        )
        print(
            f"Worth UI milestone 3.14.1 predecessor evidence causally refreshed "
            f"through Phase {arguments.through_phase}",
            flush=True,
        )
        return 0
    recorded_revision, recorded_digest = retained_source_binding(arguments.through_phase)
    persist_referenced_receipts(
        ROOT, ledger_identity(), arguments.through_phase, recorded_digest
    )
    validate_retained_portfolio(
        ROOT,
        ledger_identity(),
        arguments.through_phase,
        recorded_revision,
        recorded_digest,
    )
    validate_current_causal_sources(arguments.through_phase)
    print(
        f"Worth UI milestone 3.14.1 retained portfolio causally validated through "
        f"Phase {arguments.through_phase} without execution",
        flush=True,
    )
    return 0


def validate_current_causal_sources(through_phase: int) -> None:
    for row in rows(through_phase):
        identities = tuple(row["source_identity"].split(";"))
        if row.get("source_digest") != source_digest(identities):
            raise RuntimeError(
                f"retained evidence has changed causal sources: {row['requirement']}"
            )


if __name__ == "__main__":
    raise SystemExit(main())
