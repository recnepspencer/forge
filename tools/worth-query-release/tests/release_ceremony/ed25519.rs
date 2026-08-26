use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use worth_query_package_archive::facade::{
    decode_package_release_envelope, WorthQueryPackageEnvelopeLimits,
};

use super::support::ReleaseWorld;

#[test]
fn openssl_ed25519_output_crosses_the_real_release_ceremony_boundary() {
    let world = ReleaseWorld::new();
    let private_key = world.output_path("private.pem");
    let public_key = world.output_path("public.pem");
    let signature = world.output_path("release.signature");

    require_success(
        Command::new("openssl")
            .args(["genpkey", "-algorithm", "Ed25519", "-out"])
            .arg(&private_key)
            .output()
            .unwrap(),
    );
    require_success(
        Command::new("openssl")
            .args(["pkey", "-in"])
            .arg(&private_key)
            .args(["-pubout", "-out"])
            .arg(&public_key)
            .output()
            .unwrap(),
    );
    require_success(sign(&private_key, world.signing_payload_path(), &signature));
    assert_eq!(fs::metadata(&signature).unwrap().len(), 64);
    require_success(verify(
        &public_key,
        world.signing_payload_path(),
        &signature,
    ));

    let signature_bytes = fs::read(&signature).unwrap();
    world.replace_signature(&signature_bytes);
    let envelope_path = world.output_path("release.envelope");
    let report_path = world.output_path("release.report.json");
    require_success(world.run(&[], &envelope_path, &report_path));

    let decoded = decode_package_release_envelope(
        &fs::read(envelope_path).unwrap(),
        WorthQueryPackageEnvelopeLimits::DEFAULT,
    )
    .unwrap();
    assert_eq!(decoded.signature(), signature_bytes);
}

fn sign(private_key: &Path, payload: &Path, signature: &Path) -> Output {
    Command::new("openssl")
        .args(["pkeyutl", "-sign", "-rawin", "-inkey"])
        .arg(private_key)
        .args(["-in"])
        .arg(payload)
        .args(["-out"])
        .arg(signature)
        .output()
        .unwrap()
}

fn verify(public_key: &Path, payload: &Path, signature: &Path) -> Output {
    Command::new("openssl")
        .args(["pkeyutl", "-verify", "-rawin", "-pubin", "-inkey"])
        .arg(public_key)
        .args(["-in"])
        .arg(payload)
        .args(["-sigfile"])
        .arg(signature)
        .output()
        .unwrap()
}

fn require_success(output: Output) {
    assert!(
        output.status.success(),
        "openssl/release command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
