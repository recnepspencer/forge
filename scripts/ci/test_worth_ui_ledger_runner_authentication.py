from __future__ import annotations

import tempfile
import sys
import unittest
from pathlib import Path
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parent))

import worth_ui_ledger_runner_authentication as authentication


class RunnerAuthenticationTests(unittest.TestCase):
    def test_machine_key_authenticates_exact_content_across_invocations(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            base = Path(directory)
            repository = base / "repository"
            repository.mkdir()
            identity = base / "authority" / authentication.KEY_FILE
            with patch.object(authentication, "machine_key_identity", return_value=identity):
                tag = authentication.authentication_tag({"result": "passed"}, repository)
                self.assertTrue(
                    authentication.authenticates({"result": "passed"}, tag, repository)
                )
                self.assertFalse(
                    authentication.authenticates({"result": "forged"}, tag, repository)
                )
                self.assertEqual(identity.read_bytes(), authentication.machine_key(repository))

    def test_machine_key_cannot_live_in_repository_writable_state(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            repository = Path(directory)
            identity = repository / "target/forged.key"
            with patch.object(authentication, "machine_key_identity", return_value=identity):
                with self.assertRaisesRegex(RuntimeError, "outside the repository"):
                    authentication.authentication_tag({}, repository)


if __name__ == "__main__":
    unittest.main()
