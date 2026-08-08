from pathlib import Path
import re

roots = [
    Path(r"workspaces/worth-query"),
    Path(r"workspaces/worth-query-bank-world"),
]

reps = [
    ("ApplicationSchemaMember::Currency", "ApplicationSchemaMember::Unit"),
    ("Currency: ApplicationFieldUnit", "Unit: ApplicationFieldUnit"),
    ("Currency = NoApplicationUnit", "Unit = NoApplicationUnit"),
    ("Currency::NAME", "Unit::NAME"),
    ("type Currency =", "type Unit ="),
    ("fn currency(", "fn unit("),
    (".currency(", ".unit("),
    ("currency: Option<", "unit: Option<"),
    ('"currency"', '"unit"'),
    ("currency $Currency:ty", "unit $Unit:ty"),
    ("currency $Currency", "unit $Unit"),
    (", Currency>", ", Unit>"),
    (",\n    Currency,", ",\n    Unit,"),
    (", Currency,", ", Unit,"),
]


def transform(text: str) -> str:
    for old, new in reps:
        text = text.replace(old, new)
    # Remaining type-param Currency> only when preceded by comma/space angle patterns
    text = re.sub(r"(?<![A-Za-z])Currency>", "Unit>", text)
    text = re.sub(r"\bCurrency \{\s*currency:", "Unit { unit:", text)
    text = re.sub(r"\bCurrency \{\s*currency\b", "Unit { unit", text)
    text = re.sub(r"\{ currency \}", "{ unit }", text)
    text = re.sub(r"\{ currency:", "{ unit:", text)
    # Field named currency: on platform structs — avoid Money _currency
    text = re.sub(r"(?<!_)currency:", "unit:", text)
    text = text.replace("currencies", "units")
    return text


updated = []
for root in roots:
    for path in root.rglob("*.rs"):
        if path.name == "money.rs" and "bank-domain" in str(path):
            continue
        text = path.read_text(encoding="utf-8")
        new = transform(text)
        if new != text:
            path.write_text(new, encoding="utf-8")
            updated.append(str(path))

print(f"phase2 files: {len(updated)}")

# Rename UI test files if present
for path in Path(r"workspaces/worth-query").rglob("*currency*"):
    print("rename candidate:", path)
