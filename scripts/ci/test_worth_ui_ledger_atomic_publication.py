import csv
import hashlib
import json
import sys
import tempfile
import unittest
from contextlib import ExitStack, contextmanager
from pathlib import Path
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parent))

import worth_ui_ledger_atomic_closure as ledger_closer


FIELDS = [
    "requirement", "exact_command", "matched_test_count", "source_revision",
    "source_digest", "source_state_digest", "run_nonce", "command_result",
    "result_artifact_digest", "result", "final_source", "source_identity",
    "production_entry", "independent_oracle",
]


def open_row(requirement: str) -> dict[str, str]:
    return {field: "" for field in FIELDS} | {
        "requirement": requirement,
        "exact_command": (
            "runner --artifact _docs/worth-ui/milestone-3.14.1-evidence/"
            f"{requirement.lower()}.json"
        ),
        "result": "OPEN",
        "final_source": "false",
        "source_identity": "source.rs",
        "production_entry": "source.rs::production",
        "independent_oracle": "source.rs::oracle",
    }


RESULT = {
    "matched_test_count": 1,
    "source_revision": "a" * 40,
    "source_digest": "sources",
    "source_state_digest": "b" * 64,
    "run_nonce": "nonce",
    "artifact_sha256": "artifact",
    "source_identity": ["source.rs"],
}


class AtomicPublicationTests(unittest.TestCase):
    def test_completed_row_is_exposed_only_to_its_successors(self) -> None:
        rows = [open_row(requirement) for requirement in ["P3-FIRST", "P3-SECOND"]]
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            ledger = root / "ledger.csv"
            write_ledger(ledger, rows)
            calls = 0

            def execute(command: str, candidate: Path):
                nonlocal calls
                with candidate.open(encoding="utf-8", newline="") as stream:
                    observed = list(csv.DictReader(stream))
                expected = ["OPEN", "OPEN"] if calls == 0 else ["PROVED", "OPEN"]
                self.assertEqual([row["result"] for row in observed], expected)
                calls += 1
                words = command.split()
                artifact = root / words[words.index("--artifact") + 1]
                artifact.parent.mkdir(parents=True, exist_ok=True)
                artifact.write_text(json.dumps({"case": calls}) + "\n", encoding="utf-8")
                return {
                    **RESULT,
                    "run_nonce": f"nonce-{calls}",
                    "artifact_sha256": hashlib.sha256(artifact.read_bytes()).hexdigest(),
                }

            with atomic_closure_patches(root, ledger, execute) as verify:
                ledger_closer.close_atomically(
                    rows,
                    FIELDS,
                    ledger_closer.AtomicClosurePlan(tuple(rows), 3),
                    root,
                    ledger,
                )
            self.assertEqual(calls, 2)
            verify.assert_called_once()
            self.assertEqual(verify.call_args.args[0], 3)
            self.assertNotEqual(verify.call_args.args[2], ledger)
            with ledger.open(encoding="utf-8", newline="") as stream:
                self.assertEqual(
                    [row["result"] for row in csv.DictReader(stream)],
                    ["PROVED", "PROVED"],
                )

    def test_failed_candidate_does_not_publish(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            ledger = root / "ledger.csv"
            row = open_row("P3-ONLY")
            write_ledger(ledger, [row])
            artifact = root / "_docs/worth-ui/milestone-3.14.1-evidence/p3-only.json"
            artifact.parent.mkdir(parents=True)
            artifact.write_bytes(b"original evidence\n")
            compile_artifact = root / ledger_closer.COMPILE_ARTIFACT
            compile_artifact.write_bytes(b"original compile evidence\n")
            original = ledger.read_bytes()

            def replace_artifact(_command: str, _candidate: Path):
                artifact.write_text("{}\n", encoding="utf-8")
                return {
                    **RESULT,
                    "artifact_sha256": hashlib.sha256(artifact.read_bytes()).hexdigest(),
                }

            with failed_closure_patches(root, ledger, compile_artifact, replace_artifact):
                with self.assertRaisesRegex(RuntimeError, "fresh verification failed"):
                    ledger_closer.close_atomically(
                        [row],
                        FIELDS,
                        ledger_closer.AtomicClosurePlan(
                            (row,),
                            3,
                            ledger_closer.ClosurePreparation.CURRENT_COMPILE_CONTRACTS,
                        ),
                        root,
                        ledger,
                    )
            self.assertEqual(ledger.read_bytes(), original)
            self.assertEqual(artifact.read_bytes(), b"original evidence\n")
            self.assertEqual(compile_artifact.read_bytes(), b"original compile evidence\n")


@contextmanager
def atomic_closure_patches(root: Path, ledger: Path, execute):
    with common_closure_patches(root, ledger) as stack:
        stack.enter_context(patch.object(
            ledger_closer,
            "run_row",
            side_effect=lambda _root, command, candidate: execute(command, candidate),
        ))
        verify = stack.enter_context(patch.object(ledger_closer, "verify_closed_prefix"))
        yield verify


@contextmanager
def failed_closure_patches(
    root: Path, ledger: Path, compile_artifact: Path, execute
):
    with common_closure_patches(root, ledger) as stack:
        stack.enter_context(patch.object(
            ledger_closer,
            "run_row",
            side_effect=lambda _root, command, candidate: execute(command, candidate),
        ))
        stack.enter_context(patch.object(
            ledger_closer,
            "_prepare",
            side_effect=lambda _preparation, _root: compile_artifact.write_bytes(
                b"candidate compile evidence\n"
            ),
        ))
        stack.enter_context(patch.object(
            ledger_closer,
            "verify_closed_prefix",
            side_effect=RuntimeError("fresh verification failed"),
        ))
        yield


def common_closure_patches(root: Path, ledger: Path) -> ExitStack:
    stack = ExitStack()
    for active in [
        patch.object(ledger_closer, "ROOT", root),
        patch.object(ledger_closer, "LEDGER", ledger),
        patch.object(ledger_closer, "claim_digest_for_row", return_value="c" * 64),
        patch.object(ledger_closer, "source_revision", return_value="a" * 40),
        patch.object(ledger_closer, "source_state_digest", return_value="b" * 64),
        patch.object(
            ledger_closer,
            "execute_or_restore",
            side_effect=lambda row, candidate, _cache, _claim, runner, finalize, *, restore=True: finalize(
                runner(row["exact_command"], candidate)
            ),
        ),
        patch.object(ledger_closer, "publish"),
        patch.object(ledger_closer, "synchronize_historical_rows"),
        patch.object(ledger_closer, "validate_ledger_posture"),
    ]:
        stack.enter_context(active)
    return stack


def write_ledger(ledger: Path, rows: list[dict[str, str]]) -> None:
    with ledger.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(rows)


if __name__ == "__main__":
    unittest.main()
