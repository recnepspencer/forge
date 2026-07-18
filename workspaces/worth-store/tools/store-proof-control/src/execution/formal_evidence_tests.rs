use std::path::{Path, PathBuf};

use serde_json::json;

use super::formal_evidence;
use crate::evidence::{sha256_file, write_json};

#[test]
fn formal_receipt_must_match_schema_toolchain_digest_and_protocol_inventory() {
    let fixture = FormalReceiptFixture::create();
    formal_evidence::collect(&fixture.root, &fixture.receipt, true)
        .unwrap()
        .expect("valid formal receipt is retained");

    fixture.write_receipt(99, "jar", 1);
    let denial = formal_evidence::collect(&fixture.root, &fixture.receipt, true).unwrap_err();
    assert!(denial.contains("schema differs"));
    assert!(denial.contains("jar sha256 differs"));
    assert!(denial.contains("protocol count differs"));

    std::fs::remove_dir_all(&fixture.root).unwrap();
}

struct FormalReceiptFixture {
    root: PathBuf,
    receipt: PathBuf,
    toolchain_sha256: String,
}

impl FormalReceiptFixture {
    fn create() -> Self {
        let root = std::env::temp_dir().join(format!(
            "store-formal-receipt-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let crate_root = root.join("crates/worth-store-formal-models");
        let protocols = crate_root.join("src/protocols");
        std::fs::create_dir_all(&protocols).unwrap();
        std::fs::write(protocols.join("one.tla"), "---- MODULE one ----").unwrap();
        std::fs::write(protocols.join("two.tla"), "---- MODULE two ----").unwrap();
        let toolchain = crate_root.join("formal-toolchain.toml");
        std::fs::write(
            &toolchain,
            concat!(
                "version = \"1.2.3\"\n",
                "sha256 = \"expected-jar\"\n",
                "main_class = \"tlc2.TLC\"\n",
                "model = \"model.tla\"\n",
                "configuration = \"model.cfg\"\n"
            ),
        )
        .unwrap();
        let receipt = root.join("receipt.json");
        let fixture = Self {
            toolchain_sha256: sha256_file(&toolchain).unwrap(),
            root,
            receipt,
        };
        fixture.write_receipt(1, "expected-jar", 2);
        fixture
    }

    fn write_receipt(&self, schema_version: u32, jar_sha256: &str, protocol_count: usize) {
        write_json(
            Path::new(&self.receipt),
            &json!({
                "schema_version": schema_version,
                "tool": "tlc",
                "version": "1.2.3",
                "jar_sha256": jar_sha256,
                "toolchain_sha256": self.toolchain_sha256,
                "main_class": "tlc2.TLC",
                "model": "model.tla",
                "configuration": "model.cfg",
                "java_executable": "/java",
                "protocol_count": protocol_count,
                "verdict": "passed"
            }),
        )
        .unwrap();
    }
}
