use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::Path;
#[cfg(not(test))]
use std::process::{Child, Command, Stdio};
use std::thread;
#[cfg(test)]
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use sysinfo::{Pid, System};

use crate::discovery::{observe_artifact_footprint, ObservedArtifactFootprint};
use crate::evidence::{read_json, write_new_json};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalObservationRequest {
    pub schema_version: u32,
    pub request_identity: String,
    pub root_process_path: String,
    pub stop_path: String,
    pub output_path: String,
    pub target_root: String,
    pub poll_interval_millis: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalObservedProcess {
    pub process_id: u32,
    pub parent_process_id: Option<u32>,
    pub executable_name: String,
    pub classifications: BTreeSet<String>,
    pub sample_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalObservationReceipt {
    pub schema_version: u32,
    pub request_identity: String,
    pub observer_process_id: u32,
    pub observer_authority: String,
    pub root_process_id: Option<u32>,
    pub observed_processes: Vec<ExternalObservedProcess>,
    pub peak_observed_descendants: usize,
    pub sample_count: u64,
    pub before: ObservedArtifactFootprint,
    pub after: ObservedArtifactFootprint,
}

pub(crate) struct ExternalObserverGuard {
    request: ExternalObservationRequest,
    worker: ObserverWorker,
}

enum ObserverWorker {
    #[cfg(not(test))]
    Process(Child),
    #[cfg(test)]
    TestThread(JoinHandle<Result<(), String>>),
}

pub(crate) fn start(
    attempt_root: &Path,
    request_identity: String,
    target_root: String,
) -> Result<ExternalObserverGuard, String> {
    let observer_root = attempt_root.join("observer").join(&request_identity[..16]);
    std::fs::create_dir_all(&observer_root)
        .map_err(|error| format!("could not create {}: {error}", observer_root.display()))?;
    let request = ExternalObservationRequest {
        schema_version: 1,
        request_identity,
        root_process_path: normalized(&observer_root.join("root-process")),
        stop_path: normalized(&observer_root.join("stop")),
        output_path: normalized(&observer_root.join("receipt.json")),
        target_root,
        poll_interval_millis: 10,
    };
    let request_path = observer_root.join("request.json");
    write_new_json(&request_path, &request)?;
    #[cfg(test)]
    let worker = {
        let request_path = request_path.clone();
        ObserverWorker::TestThread(thread::spawn(move || observe_request(&request_path)))
    };
    #[cfg(not(test))]
    let worker = {
        let child = Command::new(
            std::env::current_exe()
                .map_err(|error| format!("could not locate proof-control executable: {error}"))?,
        )
        .args(["internal-observe", "--request"])
        .arg(&request_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| format!("could not launch external proof observer: {error}"))?;
        ObserverWorker::Process(child)
    };
    Ok(ExternalObserverGuard { request, worker })
}

impl ExternalObserverGuard {
    pub(crate) fn bind_root_process(&self, process_id: u32) -> Result<(), String> {
        write_once(
            Path::new(&self.request.root_process_path),
            &process_id.to_string(),
        )
    }

    pub(crate) fn finish(self) -> Result<ExternalObservationReceipt, String> {
        write_once(Path::new(&self.request.stop_path), "stop")?;
        #[cfg(not(test))]
        {
            let ObserverWorker::Process(mut child) = self.worker;
            let status = child
                .wait()
                .map_err(|error| format!("could not reap external proof observer: {error}"))?;
            if !status.success() {
                return Err(format!("external proof observer exited with {status}"));
            }
        }
        #[cfg(test)]
        {
            let ObserverWorker::TestThread(worker) = self.worker;
            worker
                .join()
                .map_err(|_| "test observer thread panicked".to_owned())??;
        }
        let receipt: ExternalObservationReceipt = read_json(Path::new(&self.request.output_path))?;
        receipt.validate(&self.request)?;
        Ok(receipt)
    }
}

pub fn observe_request(request_path: &Path) -> Result<(), String> {
    let request: ExternalObservationRequest = read_json(request_path)?;
    if request.schema_version != 1 || request.poll_interval_millis == 0 {
        return Err("external observer request is not admitted".to_owned());
    }
    let before = observe_artifact_footprint(Path::new(&request.target_root))?;
    let started = Instant::now();
    let root_process_id = wait_for_root_process(&request, started)?;
    let mut system = System::new();
    let mut observed = BTreeMap::new();
    let mut peak_observed_descendants = 0;
    let mut sample_count = 0;
    while !Path::new(&request.stop_path).exists() {
        system.refresh_processes();
        let descendants = descendants_of(&system, root_process_id);
        peak_observed_descendants = peak_observed_descendants.max(descendants.len());
        record_processes(&system, &descendants, &mut observed);
        sample_count += 1;
        thread::sleep(Duration::from_millis(request.poll_interval_millis));
    }
    system.refresh_processes();
    let descendants = descendants_of(&system, root_process_id);
    peak_observed_descendants = peak_observed_descendants.max(descendants.len());
    record_processes(&system, &descendants, &mut observed);
    sample_count += 1;
    let receipt = ExternalObservationReceipt {
        schema_version: 1,
        request_identity: request.request_identity.clone(),
        observer_process_id: std::process::id(),
        observer_authority: if cfg!(test) {
            "in-process-test-double"
        } else {
            "independent-observer-process"
        }
        .to_owned(),
        root_process_id,
        observed_processes: observed.into_values().collect(),
        peak_observed_descendants,
        sample_count,
        before,
        after: observe_artifact_footprint(Path::new(&request.target_root))?,
    };
    write_new_json(Path::new(&request.output_path), &receipt)
}

impl ExternalObservationReceipt {
    fn validate(&self, request: &ExternalObservationRequest) -> Result<(), String> {
        if self.schema_version != 1
            || self.request_identity != request.request_identity
            || self.before.target_root != request.target_root
            || self.after.target_root != request.target_root
            || self.sample_count == 0
        {
            return Err("external observer receipt does not match its request".to_owned());
        }
        Ok(())
    }
}

fn wait_for_root_process(
    request: &ExternalObservationRequest,
    started: Instant,
) -> Result<Option<u32>, String> {
    let root_path = Path::new(&request.root_process_path);
    while !root_path.exists() {
        if Path::new(&request.stop_path).exists() {
            return Ok(None);
        }
        if started.elapsed() > Duration::from_secs(30) {
            return Err("external observer timed out waiting for root process identity".to_owned());
        }
        thread::sleep(Duration::from_millis(request.poll_interval_millis));
    }
    let value = std::fs::read_to_string(root_path)
        .map_err(|error| format!("could not read {}: {error}", root_path.display()))?;
    value
        .trim()
        .parse::<u32>()
        .map(Some)
        .map_err(|error| format!("observer root process is invalid: {error}"))
}

fn descendants_of(system: &System, root: Option<u32>) -> BTreeSet<Pid> {
    let Some(root) = root else {
        return BTreeSet::new();
    };
    let root = Pid::from_u32(root);
    let mut descendants = BTreeSet::from([root]);
    loop {
        let before = descendants.len();
        for (pid, process) in system.processes() {
            if process
                .parent()
                .is_some_and(|parent| descendants.contains(&parent))
            {
                descendants.insert(*pid);
            }
        }
        if descendants.len() == before {
            return descendants;
        }
    }
}

fn record_processes(
    system: &System,
    process_ids: &BTreeSet<Pid>,
    observed: &mut BTreeMap<u32, ExternalObservedProcess>,
) {
    for process_id in process_ids {
        let Some(process) = system.process(*process_id) else {
            continue;
        };
        let process_id = process_id.as_u32();
        let name = process.name().to_owned();
        let row = observed
            .entry(process_id)
            .or_insert_with(|| ExternalObservedProcess {
                process_id,
                parent_process_id: process.parent().map(Pid::as_u32),
                executable_name: name.clone(),
                classifications: classify_process(&name),
                sample_count: 0,
            });
        row.sample_count += 1;
    }
}

fn classify_process(name: &str) -> BTreeSet<String> {
    let lower = name.to_ascii_lowercase();
    let mut classifications = BTreeSet::new();
    if lower.contains("cargo") {
        classifications.insert("cargo".to_owned());
    }
    if lower.contains("rustc") || lower.contains("rustdoc") {
        classifications.insert("compiler".to_owned());
    }
    if lower.contains("link") || lower.contains("lld") {
        classifications.insert("linker".to_owned());
    }
    if classifications.is_empty() {
        classifications.insert("test-or-build-child".to_owned());
    }
    classifications
}

fn write_once(path: &Path, contents: &str) -> Result<(), String> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| format!("could not create {}: {error}", path.display()))?;
    file.write_all(contents.as_bytes())
        .and_then(|()| file.sync_all())
        .map_err(|error| format!("could not finish {}: {error}", path.display()))
}

fn normalized(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
