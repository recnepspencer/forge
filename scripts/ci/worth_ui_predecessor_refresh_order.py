from __future__ import annotations


PROVIDER_PRIORITY = {
    "P1-WORLDS-01": 0,
    "P2-WORLD-01": 0,
    "P3-PREDECESSOR-01": -1,
    "P3-DELTA-SOURCE-01": 0,
    "P3-HP02-WORLD-01": 0,
    "P4-PREDECESSOR-01": -1,
    "P5-PREDECESSOR-01": -1,
    "P6-PREDECESSOR-01": -1,
}


def ordered_rows(rows: list[dict[str, str]]) -> list[dict[str, str]]:
    return sorted(
        rows,
        key=lambda row: (
            int(row["phase"]),
            2 if row["requirement"].endswith("-CLOSE-01")
            else PROVIDER_PRIORITY.get(row["requirement"], 1),
        ),
    )
