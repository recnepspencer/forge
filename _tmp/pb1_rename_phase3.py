from pathlib import Path
import re

roots = [
    Path(r"workspaces/worth-query"),
    Path(r"workspaces/worth-query-bank-world"),
]


def is_protected_file(path: Path) -> bool:
    return path.name == "money.rs" and "bank-domain" in str(path)


def transform(text: str) -> str:
    text = text.replace("pub fn currency<Unit>", "pub fn unit<Unit>")
    text = text.replace("unit: currency.name()", "unit: unit.name()")
    text = text.replace("type Currency: 'static;", "type Unit: 'static;")
    text = text.replace(
        "TypedUnitApplicationValue>::Currency",
        "TypedUnitApplicationValue>::Unit",
    )
    text = text.replace(
        "named_reference!(ApplicationUnitRef, Schema, Currency);",
        "named_reference!(ApplicationUnitRef, Schema, Unit);",
    )
    # Standalone type-param lines
    text = re.sub(r"^(\s+)Currency,(\s*)$", r"\1Unit,\2", text, flags=re.M)
    # Generic list Currency tokens (not UsdCurrency)
    text = re.sub(r"([,<]\s*)Currency(\s*[,>])", r"\1Unit\2", text)
    # PhantomData trailing Currency
    text = re.sub(
        r"(fn\(\) -> \([^)]*?)\bCurrency\b(\s*\))",
        r"\1Unit\2",
        text,
        flags=re.S,
    )
    # Protect money bounds if mangled
    text = re.sub(r"\bC: Unit\b", "C: Currency", text)
    text = text.replace("trait Unit {", "trait Currency {")
    text = text.replace("impl Unit for", "impl Currency for")
    return text


updated = []
for root in roots:
    for path in root.rglob("*.rs"):
        if is_protected_file(path):
            continue
        text = path.read_text(encoding="utf-8")
        new = transform(text)
        if new != text:
            path.write_text(new, encoding="utf-8")
            updated.append(str(path))

print(f"updated {len(updated)}")

# Rename UI test files
ui_dir = Path(
    r"workspaces/worth-query/crates/worth-query-certification/tests/ui/application_schema"
)
for old_name, new_name in [
    ("wrong_currency_value.rs", "wrong_unit_value.rs"),
    ("wrong_currency_value.stderr", "wrong_unit_value.stderr"),
]:
    old = ui_dir / old_name
    new = ui_dir / new_name
    if old.exists():
        old.rename(new)
        print("renamed", old_name)

# Update compile_certification / trybuild lists referencing old name
for path in Path(r"workspaces/worth-query/crates/worth-query-certification").rglob("*"):
    if path.suffix not in {".rs", ".toml", ".md"}:
        continue
    text = path.read_text(encoding="utf-8")
    if "wrong_currency_value" in text:
        path.write_text(text.replace("wrong_currency_value", "wrong_unit_value"), encoding="utf-8")
        print("rewrote ref in", path)
