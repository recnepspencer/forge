from __future__ import annotations

import argparse
from pathlib import Path

from worth_ui_3141_proof_plan import prepare_claim, proofs
from worth_ui_ledger_atomic_closure import (
    AtomicClosurePlan,
    close_atomically,
    verify_closed_prefix,
)
from worth_ui_ledger_command import source_revision
from worth_ui_ledger_source_state import source_state_digest
import worth_ui_ledger_acceptance as closure_acceptance
import worth_ui_ledger_closure_selection as closure_selection
from worth_ui_ledger_closure_selection import phase_proofs, read_ledger, require_complete_phase_mapping
from worth_ui_ledger_phase_two_closure import (
    admit_invalidated_phase_one_prefix,
    phase_two_closure_plan,
)

from worth_ui_ledger_closure_storage import (
    ledger_lock as acquire_ledger_lock,
    write_requirements as write_ledger_requirements,
)


ROOT = Path(__file__).resolve().parents[2]
LEDGER = ROOT / "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"
LEDGER_LOCK = LEDGER.with_suffix(".lock")

def ledger_lock(identity: Path = LEDGER_LOCK):
    return acquire_ledger_lock(identity)


def write_requirements(rows: list[dict[str, str]], fields: list[str], requirements: set[str]) -> None:
    write_ledger_requirements(LEDGER, rows, fields, requirements)


def phase_rows_to_prepare(*arguments: object) -> list[dict[str, str]]:
    return closure_selection.phase_rows_to_prepare(
        *arguments, prepare=prepare_claim
    )


def reopen_claim(row: dict[str, str], proof: object) -> None:
    previous_artifact = row.get("result_artifact_digest")
    prepare_claim(row, proof)
    closure_selection.reopen_prepared_claim(row, previous_artifact)


def reopen_proved_downstream(
    rows: list[dict[str, str]], through_phase: int
) -> list[dict[str, str]]:
    configured = proofs()
    downstream = [
        row
        for row in rows
        if int(row["phase"]) > through_phase
        and (row["result"] == "PROVED" or row["final_source"] == "true")
    ]
    for row in downstream:
        reopen_claim(row, configured[row["requirement"]])
    return downstream


def retain_selected_acceptance(selected: list[dict[str, str]]) -> None:
    closure_acceptance.retain_selected_acceptance(selected, ROOT, LEDGER)


def main() -> int:
    arguments = parse_arguments()
    with ledger_lock():
        return governed_main(arguments)


def governed_main(arguments: argparse.Namespace) -> int:
    prepare_only = arguments.prepare_only
    accept_only = arguments.accept_only
    through_phase = arguments.through_phase
    requirement = arguments.requirement
    fields, rows = read_ledger()
    configured = phase_proofs(through_phase)
    current_state = source_state_digest(source_revision())
    invalidated_prefix = (
        admit_invalidated_phase_one_prefix(rows, ROOT) if through_phase == 2 else []
    )
    if invalidated_prefix:
        if requirement is not None:
            raise RuntimeError("Phase 1 invalidation recovery requires the complete Phase 2 portfolio")
        selected = [
            *invalidated_prefix,
            *[
                row
                for row in rows
                if row["phase"] == "2"
                and row["result"] == "OPEN"
                and row["final_source"] == "false"
            ],
        ]
    else:
        selected = phase_rows_to_prepare(
            rows, through_phase, requirement, configured, current_state
        )
    if requirement is None:
        require_complete_phase_mapping(rows, through_phase, configured)
    configured_prefix = proofs()
    for row in selected:
        reopen_claim(row, configured_prefix[row["requirement"]])
    downstream = reopen_proved_downstream(rows, through_phase) if selected else []
    if prepare_only and (selected or downstream):
        write_requirements(
            rows,
            fields,
            {row["requirement"] for row in [*selected, *downstream]},
        )
    if prepare_only:
        print(f"prepared {len(selected)} Worth UI milestone 3.14.1 ledger claims as OPEN")
        return 0
    if accept_only:
        if not selected:
            print("accepted 0 Worth UI milestone 3.14.1 proof rows")
            return 0
        retain_selected_acceptance(selected)
        return 0
    if not selected:
        verify_closed_prefix(through_phase)
        return 0
    if through_phase == 2:
        close_phase_two(rows, fields)
    elif through_phase == 3:
        close_phase_three(rows, fields, selected)
    elif through_phase == 4:
        close_phase_four(rows, fields, selected)
    elif through_phase == 5:
        close_phase_five(rows, fields, selected)
    elif through_phase == 6:
        close_phase_six(rows, fields, selected)
    else:
        raise RuntimeError("unsupported Worth UI milestone phase")
    return 0

def close_phase_two(rows: list[dict[str, str]], fields: list[str]) -> None:
    close_atomically(rows, fields, phase_two_closure_plan(rows, ROOT), ROOT, LEDGER)


def close_phase_three(
    rows: list[dict[str, str]], fields: list[str], selected: list[dict[str, str]]
) -> None:
    close_phase_with_priority(
        rows, fields, selected, ["P3-PREDECESSOR-01", "P3-DELTA-SOURCE-01", "P3-HP02-WORLD-01"],
        "P3-CLOSE-01", 3,
    )


def close_phase_four(
    rows: list[dict[str, str]], fields: list[str], selected: list[dict[str, str]]
) -> None:
    close_phase_with_priority(
        rows, fields, selected, ["P4-PREDECESSOR-01", "P4-TEXT-PROFILE-01"], "P4-CLOSE-01", 4
    )


def close_phase_five(
    rows: list[dict[str, str]], fields: list[str], selected: list[dict[str, str]]
) -> None:
    requirements = {row["requirement"] for row in selected}
    verify_phase = 5 if "P5-CLOSE-01" in requirements else None
    close_atomically(
        rows, fields, AtomicClosurePlan(tuple(selected), verify_phase), ROOT, LEDGER
    )


def close_phase_six(
    rows: list[dict[str, str]], fields: list[str], selected: list[dict[str, str]]
) -> None:
    requirements = {row["requirement"] for row in selected}
    verify_phase = 6 if "P6-CLOSE-01" in requirements else None
    close_phase_with_priority(
        rows,
        fields,
        selected,
        ["P6-PREDECESSOR-01"],
        "P6-CLOSE-01",
        6,
    ) if verify_phase is not None else close_atomically(
        rows, fields, AtomicClosurePlan(tuple(selected), None), ROOT, LEDGER
    )


def close_phase_with_priority(
    rows: list[dict[str, str]],
    fields: list[str],
    selected: list[dict[str, str]],
    priority: list[str],
    close_requirement: str,
    verify_phase: int,
) -> None:
    by_requirement = {row["requirement"]: row for row in selected}
    ordered = [by_requirement.pop(identity) for identity in priority if identity in by_requirement]
    close = by_requirement.pop(close_requirement, None)
    ordered.extend(by_requirement.values())
    if close is not None:
        ordered.append(close)
    close_atomically(
        rows, fields, AtomicClosurePlan(tuple(ordered), verify_phase), ROOT, LEDGER
    )


def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Atomically close Worth UI milestone 3.14.1 ledger evidence"
    )
    parser.add_argument("--through-phase", type=int, choices=(2, 3, 4, 5, 6), default=2)
    parser.add_argument(
        "--requirement",
        action="append",
        help="repeat to refresh multiple Phase rows in one atomic transaction",
    )
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--prepare-only", action="store_true")
    mode.add_argument(
        "--accept-only",
        action="store_true",
        help="retain authenticated selected-row evidence without publishing ledger closure",
    )
    return parser.parse_args()


if __name__ == "__main__":
    raise SystemExit(main())
