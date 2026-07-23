import csv
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest import TestCase

from worth_ui_query_lifetime_matrix import (
    ARTIFACT_COLUMNS,
    REQUIRED_STATUSES,
    REQUIRED_TRANSITIONS,
    query_lifetime_matrix_violations,
)


class WorthUiQueryLifetimeMatrixTests(TestCase):
    def test_phase_eight_first_path_cannot_be_deferred_to_phase_fourteen(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            config = matrix_fixture(root)
            self.assertEqual(query_lifetime_matrix_violations(root, config), [])
            rows = read_rows(root / "matrix.csv")
            next(row for row in rows if row["transition"] == "rebind")["phase"] = "14"
            write_rows(root / "matrix.csv", rows)

            violations = query_lifetime_matrix_violations(root, config)

            self.assertIn("rebind: phase 14 must be 8", "\n".join(v.detail for v in violations))

    def test_closed_phase_eight_transition_cannot_return_to_assigned(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            config = matrix_fixture(root)
            rows = read_rows(root / "matrix.csv")
            next(row for row in rows if row["transition"] == "rebind")["status"] = "assigned"
            write_rows(root / "matrix.csv", rows)

            violations = query_lifetime_matrix_violations(root, config)

            self.assertIn("rebind: status must be proven", "\n".join(v.detail for v in violations))

    def test_every_artifact_requires_an_explicit_ownership_action(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            config = matrix_fixture(root)
            rows = read_rows(root / "matrix.csv")
            rows[0]["compact_plan_reference"] = "retained"
            write_rows(root / "matrix.csv", rows)

            violations = query_lifetime_matrix_violations(root, config)

            self.assertIn(
                "installation/compact_plan_reference",
                "\n".join(v.detail for v in violations),
            )

    def test_rebind_requires_predecessor_release_and_successor_publication(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            config = matrix_fixture(root)
            rows = read_rows(root / "matrix.csv")
            rebind = next(row for row in rows if row["transition"] == "rebind")
            rebind["query_live_resource"] = "succeeded_once:successor_resource"
            write_rows(root / "matrix.csv", rows)

            violations = query_lifetime_matrix_violations(root, config)

            self.assertIn(
                "rebind/query_live_resource",
                "\n".join(v.detail for v in violations),
            )

    def test_phase_eight_owns_candidate_resource_rollback(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            config = matrix_fixture(root)
            rows = read_rows(root / "matrix.csv")
            denial = next(
                row for row in rows
                if row["transition"] == "candidate_preparation_denial"
            )
            denial["query_live_resource"] = "retained:predecessor_resource"
            write_rows(root / "matrix.csv", rows)

            violations = query_lifetime_matrix_violations(root, config)

            self.assertIn(
                "candidate_preparation_denial/query_live_resource",
                "\n".join(v.detail for v in violations),
            )


def matrix_fixture(root: Path) -> dict[str, str]:
    proof = root / "proof.rs"
    proof.write_text("#[test]\nfn proof() {}\n", encoding="utf-8")
    rows = []
    for transition, phase in REQUIRED_TRANSITIONS.items():
        actions = {
            artifact: "retained:owner" for artifact in ARTIFACT_COLUMNS
        }
        if transition == "rebind":
            actions = {
                artifact: "released_once:predecessor+succeeded_once:successor"
                for artifact in ARTIFACT_COLUMNS
            }
            actions["inspection_reference"] = (
                "released_once:predecessor+observed:successor"
            )
        if transition in {"candidate_preparation_denial", "failed_publication"}:
            actions = {
                "installed_view": (
                    "retained:predecessor+released_once:candidate"
                ),
                "consumed_projection_authority": (
                    "retained:predecessor+rollback_once:candidate"
                ),
                "binding_owned_handle": (
                    "retained:predecessor+released_once:candidate"
                ),
                "query_live_resource": (
                    "retained:predecessor+rollback_once:candidate"
                ),
                "compact_plan_reference": (
                    "retained:predecessor+rollback_once:candidate"
                ),
                "inspection_reference": (
                    "retained:predecessor+released_once:candidate"
                ),
            }
        row = {
            "transition": transition,
            "phase": phase,
            **actions,
            "proof_path": "proof.rs",
            "status": REQUIRED_STATUSES[transition],
        }
        rows.append(row)
    write_rows(root / "matrix.csv", rows)
    return {"query_lifetime_matrix": "matrix.csv"}


def read_rows(path: Path) -> list[dict[str, str]]:
    with path.open(encoding="utf-8", newline="") as source:
        return list(csv.DictReader(source))


def write_rows(path: Path, rows: list[dict[str, str]]) -> None:
    with path.open("w", encoding="utf-8", newline="") as destination:
        writer = csv.DictWriter(destination, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)
