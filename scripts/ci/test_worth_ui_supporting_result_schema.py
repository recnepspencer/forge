from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from worth_ui_3141_supporting_world import require_result_dependency_schema


class SupportingResultSchemaTests(unittest.TestCase):
    def test_historical_and_current_schemas_are_the_only_admitted_versions(self) -> None:
        for schema in (5, 7):
            require_result_dependency_schema(
                {"schema_version": schema}, "supporting result"
            )
        for schema in (None, 6, 8):
            with self.assertRaisesRegex(ValueError, "wrong schema_version"):
                require_result_dependency_schema(
                    {"schema_version": schema}, "supporting result"
                )


if __name__ == "__main__":
    unittest.main()
