from __future__ import annotations

import sys
import tempfile
import unittest
from pathlib import Path


sys.path.insert(0, str(Path(__file__).resolve().parent))

from worth_ui_ledger_artifact_transaction import (
    ArtifactTransaction,
    register_active_identity,
)


ARTIFACT = "_docs/worth-ui/milestone-3.14.1-evidence/p6-transaction-test-01.json"
COMMAND = f"cargo test --requirement P6-TRANSACTION-TEST-01 --artifact {ARTIFACT}"


class ArtifactTransactionTests(unittest.TestCase):
    def test_auxiliary_portfolio_rolls_back_with_row_artifacts(self) -> None:
        directory, root, ledger, _artifact = self.fixture()
        self.addCleanup(directory.cleanup)
        portfolio_identity = (
            "_docs/worth-ui/milestone-3.14.1-evidence/p4-closure-portfolio.json"
        )
        portfolio = root / portfolio_identity
        portfolio.write_bytes(b"old-portfolio")
        transaction = ArtifactTransaction(
            root, ledger, [COMMAND], (portfolio_identity,)
        )
        portfolio.write_bytes(b"candidate-portfolio")
        transaction.rollback()
        self.assertEqual(portfolio.read_bytes(), b"old-portfolio")

    def fixture(self) -> tuple[tempfile.TemporaryDirectory[str], Path, Path, Path]:
        directory = tempfile.TemporaryDirectory()
        root = Path(directory.name)
        ledger = root / "ledger.csv"
        ledger.write_bytes(b"old-ledger")
        artifact = root / ARTIFACT
        artifact.parent.mkdir(parents=True)
        artifact.write_bytes(b"old-artifact")
        return directory, root, ledger, artifact

    def test_explicit_rollback_restores_the_original_artifact(self) -> None:
        directory, root, ledger, artifact = self.fixture()
        with directory:
            transaction = ArtifactTransaction(root, ledger, [COMMAND])
            artifact.write_bytes(b"new-artifact")
            transaction.rollback()
            self.assertEqual(artifact.read_bytes(), b"old-artifact")

    def test_dynamic_observation_is_removed_by_rollback(self) -> None:
        directory, root, ledger, _artifact = self.fixture()
        with directory:
            transaction = ArtifactTransaction(root, ledger, [COMMAND])
            observation = (
                root
                / "_docs/worth-ui/milestone-3.14.1-evidence/execution-observations/aa"
                / f"{'a' * 64}.json"
            )
            register_active_identity(root, observation)
            observation.parent.mkdir(parents=True)
            observation.write_bytes(b"candidate-observation")
            transaction.rollback()
            self.assertFalse(observation.exists())

    def test_next_transaction_recovers_a_crash_before_ledger_commit(self) -> None:
        directory, root, ledger, artifact = self.fixture()
        with directory:
            ArtifactTransaction(root, ledger, [COMMAND])
            artifact.write_bytes(b"partial-artifact")
            recovered = ArtifactTransaction(root, ledger, [])
            self.assertEqual(artifact.read_bytes(), b"old-artifact")
            recovered.rollback()

    def test_recovery_preserves_artifacts_after_exact_ledger_commit(self) -> None:
        directory, root, ledger, artifact = self.fixture()
        with directory:
            transaction = ArtifactTransaction(root, ledger, [COMMAND])
            artifact.write_bytes(b"new-artifact")
            transaction.prepare_commit(b"new-ledger")
            ledger.write_bytes(b"new-ledger")
            recovered = ArtifactTransaction(root, ledger, [])
            self.assertEqual(artifact.read_bytes(), b"new-artifact")
            recovered.rollback()

    def test_recovery_refuses_unrelated_ledger_bytes(self) -> None:
        directory, root, ledger, artifact = self.fixture()
        with directory:
            transaction = ArtifactTransaction(root, ledger, [COMMAND])
            artifact.write_bytes(b"new-artifact")
            transaction.prepare_commit(b"expected-ledger")
            ledger.write_bytes(b"foreign-ledger")
            with self.assertRaisesRegex(RuntimeError, "outside"):
                ArtifactTransaction(root, ledger, [])

    def test_transaction_rejects_cross_requirement_artifact_identity(self) -> None:
        directory, root, ledger, _artifact = self.fixture()
        with directory:
            command = (
                "cargo test --requirement P6-OTHER-01 "
                f"--artifact {ARTIFACT}"
            )
            with self.assertRaisesRegex(ValueError, "artifact must be"):
                ArtifactTransaction(root, ledger, [command])


if __name__ == "__main__":
    unittest.main()
