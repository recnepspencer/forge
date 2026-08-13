from __future__ import annotations

import os
import subprocess
import sys
from dataclasses import dataclass

from worth_ui_ledger_command import (
    ROOT,
    GovernedTest,
    claim_digest,
    source_digest,
    source_revision,
)
from worth_ui_ledger_portfolio_snapshot import DIGEST_ENV, REVISION_ENV, source_state_for_row


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
    if test.requirement in {"P3-PREDECESSOR-01", "P4-PREDECESSOR-01"}:
        supplied = os.environ.get("WORTH_UI_PREDECESSOR_ARTIFACT")
        if supplied is not None:
            if supplied not in test.sources or not (ROOT / supplied).is_file():
                raise RuntimeError("supplied predecessor handoff is not an exact governed source")
            return
        refresh_predecessor_handoff(test)


def refresh_predecessor_handoff(test: GovernedTest) -> None:
    phase = int(test.requirement[1])
    handoff_name = f"p{phase}-predecessor-handoff.json"
    identity = next((source for source in test.sources if source.endswith(handoff_name)), None)
    if identity is None:
        raise ValueError("predecessor proof omits its handoff artifact")
    environment = dict(os.environ)
    for name in (
        "WORTH_UI_MILESTONE_3141_LEDGER",
        "WORTH_UI_SHARED_WORLD_ARTIFACT",
        "WORTH_UI_SUPPORTING_WORLD_ARTIFACT",
        REVISION_ENV,
        DIGEST_ENV,
    ):
        environment.pop(name, None)
    completed = subprocess.run(
        [
            sys.executable,
            "scripts/ci/verify_worth_ui_3141_ledger.py",
            "--through-phase",
            str(phase - 1),
            "--artifact",
            identity,
        ],
        cwd=ROOT,
        env=environment,
        stdout=subprocess.PIPE,
        text=True,
        check=False,
    )
    if completed.returncode != 0:
        sys.stderr.write(completed.stdout)
        raise RuntimeError("fresh predecessor verification failed")


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
