"""Summarize drill_edge_criticality output lines into findings-note tables."""

import re
import sys
import collections

LINE = re.compile(
    r"mutation=(?P<kind>[a-z-]+):(?P<target>\S+) class=\S+ class_size=(?P<class_size>\d+) "
    r"pressure=(?P<pressure>\d+) posture=(?P<posture>\w+) finding=(?P<finding>[A-Z-]+) "
    r"vertices=\d+ edges=\d+ seconds=(?P<seconds>[\d.]+)"
)


def main(path: str) -> None:
    rows = []
    with open(path, encoding="utf-8") as handle:
        for line in handle:
            match = LINE.search(line)
            if match:
                rows.append(match.groupdict())
    by_kind = collections.defaultdict(list)
    for row in rows:
        by_kind[row["kind"]].append(row)
    for kind, kind_rows in by_kind.items():
        findings = collections.Counter(row["finding"] for row in kind_rows)
        print(f"kind={kind} total={len(kind_rows)} findings={dict(findings)}")
        seconds = sorted(float(row["seconds"]) for row in kind_rows)
        mid = seconds[len(seconds) // 2]
        print(f"  seconds min={seconds[0]} median={mid} max={seconds[-1]}")
        slow = [row for row in kind_rows if float(row["seconds"]) >= mid * 4 + 0.2]
        slow.sort(key=lambda row: -float(row["seconds"]))
        for row in slow[:12]:
            print(
                f"  slow target={row['target']} pressure={row['pressure']} "
                f"seconds={row['seconds']} finding={row['finding']}"
            )
        # pressure quartiles vs mean seconds
        ranked = sorted(kind_rows, key=lambda row: int(row["pressure"]))
        quarter = max(1, len(ranked) // 4)
        for index in range(0, len(ranked), quarter):
            chunk = ranked[index : index + quarter]
            mean_seconds = sum(float(row["seconds"]) for row in chunk) / len(chunk)
            pressures = [int(row["pressure"]) for row in chunk]
            print(
                f"  pressure_band {pressures[0]}..{pressures[-1]} n={len(chunk)} "
                f"mean_seconds={mean_seconds:.2f}"
            )
        unusual = [row for row in kind_rows if row["finding"] != "CRITICAL"]
        for row in unusual:
            print(f"  UNUSUAL: {row}")


if __name__ == "__main__":
    main(sys.argv[1])
