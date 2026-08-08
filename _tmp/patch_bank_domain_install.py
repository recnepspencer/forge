from pathlib import Path

path = Path("workspaces/worth-query-bank-world/crates/bank-domain/tests/estate_aftermath_contract.rs")
text = path.read_text(encoding="utf-8")
old = """                let installed = install_application_aftermath(
                    &digest(0x41),
                    &digest(0x42),
                    &format!(\"{operation:?}\"),
                    1,
                    &declared,
                    &reads_for(operation),
                )"""
new = """                let installed = install_application_aftermath(
                    &digest(0x41),
                    &digest(0x42),
                    &format!(\"{operation:?}\"),
                    1,
                    &declared,
                    &reads_for(operation),
                    &lowering_catalog_for(operation),
                )"""
if old not in text:
    raise SystemExit("pattern not found")
path.write_text(text.replace(old, new), encoding="utf-8")
print("ok")
