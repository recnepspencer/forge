use std::io::Read;

use sha2::{Digest, Sha256};
use worth_store_test_support::structural_preflight::StructuralPreflightEvaluatorIdentity;

pub(super) fn observe() -> Result<StructuralPreflightEvaluatorIdentity, String> {
    let executable = std::env::current_exe()
        .and_then(std::fs::canonicalize)
        .map_err(|error| format!("could not observe structural preflight evaluator: {error}"))?;
    Ok(StructuralPreflightEvaluatorIdentity {
        responsibility: "store-proof-control-structural-preflight".to_owned(),
        executable_path: executable.to_string_lossy().replace('\\', "/"),
        executable_sha256: file_digest(&executable)?,
        version_identity: format!("store-proof-control/{}", env!("CARGO_PKG_VERSION")),
    })
}

fn file_digest(path: &std::path::Path) -> Result<String, String> {
    let mut file = std::fs::File::open(path).map_err(|error| {
        format!(
            "could not read preflight evaluator {}: {error}",
            path.display()
        )
    })?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer).map_err(|error| {
            format!(
                "could not read preflight evaluator {}: {error}",
                path.display()
            )
        })?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    Ok(format!("{:x}", digest.finalize()))
}
