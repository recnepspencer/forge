from __future__ import annotations

import os
from dataclasses import dataclass
from pathlib import Path

from worth_ui_ledger_command import (
    ROOT,
    GovernedTest,
    claim_digest,
    source_digest,
    source_revision,
)
from worth_ui_ledger_artifact_identity import (
    declared_predecessor_handoff,
    requirement_phase,
)
from worth_ui_ledger_portfolio_snapshot import source_state_for_row
from worth_ui_ledger_source_state import source_state_digest
from worth_ui_predecessor_causal_refresh import refresh_handoff
from worth_ui_predecessor_handoff_currentness import (
    PredecessorVerification,
    is_current,
)


@dataclass(frozen=True)
class GovernedSnapshot:
    revision: str
    source_digest: str
    source_state_digest: str
    claim_digest: str


def governed_snapshot(test: GovernedTest) -> GovernedSnapshot:
    revision = source_revision()
    return GovernedSnapshot(
        revision,
        source_digest(test.sources),
        source_state_for_row(revision),
        claim_digest(test.requirement),
    )


def refresh_handoff_when_required(test: GovernedTest) -> None:
    if test.requirement in {
        "P3-PREDECESSOR-01",
        "P4-PREDECESSOR-01",
        "P5-PREDECESSOR-01",
        "P6-PREDECESSOR-01",
    }:
        supplied = os.environ.get("WORTH_UI_PREDECESSOR_ARTIFACT")
        if supplied is not None:
            phase = requirement_phase(test.requirement)
            declared = supplied in test.sources
            temporary = is_temporary_predecessor_handoff(supplied, phase)
            if (not declared and not temporary) or not (ROOT / supplied).is_file():
                raise RuntimeError("supplied predecessor handoff is not an exact governed source")
            configured = os.environ.get("WORTH_UI_MILESTONE_3141_LEDGER")
            ledger = (
                Path(configured).resolve()
                if configured
                else ROOT / "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"
            )
            if current_supplied_handoff(supplied, phase, ledger):
                return
        refresh_predecessor_handoff(test)


def is_temporary_predecessor_handoff(identity: str, phase: int) -> bool:
    try:
        parsed = declared_predecessor_handoff(identity, phase)
    except ValueError:
        return False
    return parsed.relative_path.startswith("workspaces/worth-ui/target/")


def refresh_predecessor_handoff(test: GovernedTest) -> None:
    phase = requirement_phase(test.requirement)
    handoff_name = f"p{phase}-predecessor-handoff.json"
    identity = next((source for source in test.sources if source.endswith(handoff_name)), None)
    if identity is None:
        raise ValueError("predecessor proof omits its handoff artifact")
    configured = os.environ.get("WORTH_UI_MILESTONE_3141_LEDGER")
    ledger = Path(configured).resolve() if configured else ROOT / "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv"
    refresh_handoff(ROOT, ledger, phase, declared_predecessor_handoff(identity, phase))


def current_supplied_handoff(identity: str, phase: int, ledger: Path) -> bool:
    try:
        typed = declared_predecessor_handoff(identity, phase)
    except ValueError:
        return False
    revision = source_revision()
    return is_current(
        typed,
        PredecessorVerification(
            ROOT, ledger, phase, revision, source_state_digest(revision)
        ),
    )


def governed_sources_changed(
    test: GovernedTest,
    revision: str,
    digest: str,
    state_digest: str,
    governed_claim_digest: str,
) -> bool:
    before = (revision, digest, state_digest, governed_claim_digest)
    after = (
        source_revision(),
        source_digest(test.sources),
        source_state_for_row(revision),
        claim_digest(test.requirement),
    )
    return governed_snapshot_changed(before, after)


def governed_snapshot_changed(before: tuple[str, ...], after: tuple[str, ...]) -> bool:
    return before != after
