from pathlib import Path
from tempfile import TemporaryDirectory
from unittest import TestCase

from run_worth_ui_compile_contracts import (
    Case,
    canonical_diagnostics,
    diagnostic_sources,
    included_sources,
)


class WorthUiCompileContractRunnerTests(TestCase):
    def test_canonical_diagnostics_retain_exact_error_meaning(self) -> None:
        source = Path(
            "workspaces/worth-ui/crates/worth-ui/tests/ui/example/fail.rs"
        ).resolve()
        case = Case("fail", "example", source, source.with_suffix(".stderr"), "product")
        messages = [
            {
                "level": "error",
                "code": {"code": "E0451"},
                "message": "field `sealed` is private",
                "spans": [{"file_name": str(source), "is_primary": True}],
                "children": [
                    {"level": "help", "message": "use the public constructor"},
                    {
                        "level": "note",
                        "message": "private fields `one` and `two` that were not provided",
                    },
                ],
            }
        ]

        rendered = canonical_diagnostics(messages, case)

        self.assertIn("error[E0451]: field `sealed` is private", rendered)
        self.assertIn("$WORKSPACE/crates/worth-ui/tests/ui/example/fail.rs", rendered)
        self.assertIn("help: use the public constructor", rendered)
        self.assertNotIn("private fields `one` and `two`", rendered)

    def test_canonical_diagnostics_retain_primary_expected_found_evidence(self) -> None:
        source = Path(
            "workspaces/worth-ui/crates/worth-ui/tests/ui/example/fail.rs"
        ).resolve()
        case = Case("fail", "example", source, source.with_suffix(".stderr"), "product")
        messages = [
            {
                "level": "error",
                "code": {"code": "E0308"},
                "message": "mismatched types",
                "spans": [
                    {
                        "file_name": str(source),
                        "is_primary": True,
                        "label": "expected `SemanticIntent`, found `HostObservation`",
                    }
                ],
                "children": [],
            }
        ]

        rendered = canonical_diagnostics(messages, case)

        self.assertIn(
            "note: expected `SemanticIntent`, found `HostObservation`", rendered
        )

    def test_canonical_diagnostics_do_not_duplicate_expected_found_children(self) -> None:
        source = Path(
            "workspaces/worth-ui/crates/worth-ui/tests/ui/example/fail.rs"
        ).resolve()
        case = Case("fail", "example", source, source.with_suffix(".stderr"), "product")
        messages = [
            {
                "level": "error",
                "code": {"code": "E0308"},
                "message": "mismatched types",
                "spans": [
                    {
                        "file_name": str(source),
                        "is_primary": True,
                        "label": "expected `Admission`, found `Trace`",
                    }
                ],
                "children": [
                    {
                        "level": "note",
                        "message": "expected struct `Admission`\n   found struct `Trace`",
                    },
                ],
            }
        ]

        rendered = canonical_diagnostics(messages, case)

        self.assertEqual(rendered.count("expected"), 1)
        self.assertEqual(rendered.count("found"), 1)

    def test_included_source_without_a_primary_error_is_observable(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            aggregate = root / "aggregate.rs"
            covered = root / "covered.rs"
            aggregate.write_text(
                'mod covered { include!("covered.rs"); }\nfn main() {}\n',
                encoding="utf-8",
            )
            covered.write_text('compile_error!("covered");\n', encoding="utf-8")
            messages = [
                {
                    "level": "error",
                    "spans": [{"file_name": str(aggregate), "is_primary": True}],
                }
            ]

            missing = included_sources(aggregate) - diagnostic_sources(messages)

            self.assertEqual(missing, {covered.resolve()})
