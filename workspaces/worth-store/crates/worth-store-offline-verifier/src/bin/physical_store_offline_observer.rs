#[path = "physical_store_offline_observer/bounded_residency_verification.rs"]
mod bounded_residency_verification;
#[path = "physical_store_offline_observer/current_manifest.rs"]
mod current_manifest;
#[path = "physical_store_offline_observer/hostile_physical_truth.rs"]
mod hostile_physical_truth;
#[path = "physical_store_offline_observer/recovery_report.rs"]
mod recovery_report;

fn main() {
    let mut arguments = std::env::args_os().skip(1);
    let Some(first) = arguments.next() else {
        usage();
    };
    if first == "bounded-residency-verify" {
        let Some(root) = arguments.next() else {
            usage();
        };
        let Some(configuration) = arguments.next() else {
            usage();
        };
        if arguments.next().is_some() {
            usage();
        }
        bounded_residency_verification::run(
            std::path::Path::new(&root),
            std::path::Path::new(&configuration),
        );
    } else if first == "hostile-physical-truth" {
        let Some(root) = arguments.next() else {
            usage();
        };
        if arguments.next().is_some() {
            usage();
        }
        hostile_physical_truth::run(std::path::Path::new(&root));
    } else if first == "c8-recovery-observe" {
        let Some(root) = arguments.next() else {
            usage();
        };
        let Some(output) = arguments.next() else {
            usage();
        };
        let Some(maximum_directory_entries) = arguments.next() else {
            usage();
        };
        let Some(maximum_directories) = arguments.next() else {
            usage();
        };
        let Some(maximum_artifacts) = arguments.next() else {
            usage();
        };
        let Some(maximum_bytes) = arguments.next() else {
            usage();
        };
        if arguments.next().is_some() {
            usage();
        }
        recovery_report::run(
            std::path::Path::new(&root),
            std::path::Path::new(&output),
            &maximum_directory_entries.to_string_lossy(),
            &maximum_directories.to_string_lossy(),
            &maximum_artifacts.to_string_lossy(),
            &maximum_bytes.to_string_lossy(),
        );
    } else {
        if arguments.next().is_some() {
            usage();
        }
        current_manifest::run(std::path::Path::new(&first));
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn usage() -> ! {
    eprintln!(
        "usage: physical_store_offline_observer <store-root> | \
         hostile-physical-truth <store-root> | \
         c8-recovery-observe <store-root> <report-output> <maximum-directory-entries> \
         <maximum-directories> <maximum-artifacts> <maximum-bytes> | \
         bounded-residency-verify <store-root> <configuration>"
    );
    std::process::exit(2);
}
