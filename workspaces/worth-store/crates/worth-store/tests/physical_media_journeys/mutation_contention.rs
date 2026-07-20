use std::io::{Read, Write};
use std::path::Path;

use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::MediaShutdownOutcome;
use worth_store_physical_backend::{
    CertificationConfinementEffect, MediaEffectStatus, NamespaceConfinementDenialKind,
};

use super::child_dispatch::{emit, run_role};
use super::{admit_runtime, media_admission};

const EVENT_PREFIX: &str = "C4_EVENT ";
const CONTENDER_COUNT: usize = 8;

#[path = "mutation_contention/counter_evidence.rs"]
mod counter_evidence;
#[path = "mutation_contention/death_boundary.rs"]
mod death_boundary;
#[path = "mutation_contention/independent_observation.rs"]
mod independent_observation;
#[path = "mutation_contention/process_barrier.rs"]
mod process_barrier;
use counter_evidence::{
    assert_loser_counters, campaign_counter_projection, exact_counter_projection,
    inspection_counters,
};
use death_boundary::{contention_admission, reach_death_boundary};
use independent_observation::{
    create_file_link, create_sentinel, expected_store_manifest, manifest_projection, observe_tree,
};
use process_barrier::Contender;
const HOSTILE_COMPONENTS: [(&str, NamespaceConfinementDenialKind); 10] = [
    ("", NamespaceConfinementDenialKind::EmptyPath),
    (".", NamespaceConfinementDenialKind::SpecialComponent),
    ("..", NamespaceConfinementDenialKind::SpecialComponent),
    (
        "../outside",
        NamespaceConfinementDenialKind::EmbeddedSeparator,
    ),
    (
        "namespace/../../outside",
        NamespaceConfinementDenialKind::EmbeddedSeparator,
    ),
    (
        "same/name",
        NamespaceConfinementDenialKind::EmbeddedSeparator,
    ),
    (
        "same\\name",
        NamespaceConfinementDenialKind::EmbeddedSeparator,
    ),
    ("nul", NamespaceConfinementDenialKind::ReservedDeviceName),
    ("name:", NamespaceConfinementDenialKind::AlternateDataStream),
    (
        "name ",
        NamespaceConfinementDenialKind::NonPortableComponent,
    ),
];

#[derive(Debug, PartialEq, Eq)]
struct CampaignProjection {
    owners: usize,
    contentions: usize,
    target_manifest_shape: Vec<String>,
    sentinel_manifest: String,
    winner_counters: String,
    loser_counters: Vec<String>,
    successor_counters: String,
}

#[test]
fn mutation_contention_confinement_and_readmission() {
    let parent = tempfile::tempdir().unwrap();
    let first = run_campaign(&parent.path().join("campaign-one"));
    let second = run_campaign(&parent.path().join("campaign-two"));
    assert_eq!(first, second);
}

fn run_campaign(enclosure: &Path) -> CampaignProjection {
    std::fs::create_dir(enclosure).unwrap();
    let root = enclosure.join("store");
    let sentinel = enclosure.join("outside-sentinel");
    create_sentinel(&sentinel);
    let sentinel_before = observe_tree(&sentinel);

    let mut contenders = (0..CONTENDER_COUNT)
        .map(|index| Contender::spawn(&root, index))
        .collect::<Vec<_>>();
    for contender in &contenders {
        assert_eq!(contender.event(), "ready");
    }
    for contender in &mut contenders {
        contender.send(b'S');
    }

    let mut winner = None;
    let mut stable_identity = None;
    let mut winner_owner = None;
    let mut winner_attempt = None;
    let mut winner_runtime = None;
    let mut loser_counters = Vec::new();
    let mut contender_processes = std::collections::BTreeSet::new();
    let mut contentions = 0;
    for (index, contender) in contenders.iter().enumerate() {
        let event = contender.event();
        if let Some(fields) = event.strip_prefix("owned;") {
            assert!(
                winner.replace(index).is_none(),
                "more than one owner admitted"
            );
            stable_identity = Some(field(fields, "store").to_owned());
            winner_owner = Some(field(fields, "owner").to_owned());
            winner_attempt = Some(field(fields, "attempt").to_owned());
            winner_runtime = Some(field(fields, "runtime").to_owned());
            assert!(contender_processes.insert(field(fields, "process").to_owned()));
            assert_eq!(
                field(fields, "confinement_denials"),
                expected_confinements()
            );
        } else {
            assert!(
                event.starts_with("contended;"),
                "unexpected contender event: {event}"
            );
            contentions += 1;
            assert!(contender_processes
                .insert(field(event.strip_prefix("contended;").unwrap(), "process").to_owned()));
            loser_counters
                .push(field(event.strip_prefix("contended;").unwrap(), "stable_counters").into());
        }
    }
    let winner = winner.expect("exactly one process must own mutation");
    assert_eq!(contentions, CONTENDER_COUNT - 1);
    for (index, contender) in contenders.iter_mut().enumerate() {
        if index != winner {
            assert!(contender.wait().success());
        }
    }

    let escape_probe = root.join("staging/confinement-probe");
    create_file_link(&sentinel.join("families/same-name"), &escape_probe);
    contenders[winner].send(b'E');
    let escape_event = contenders[winner].event();
    assert_eq!(field(&escape_event, "outcome"), "open-denied");
    assert_eq!(field(&escape_event, "status"), "denied-before-effect");
    assert_eq!(observe_tree(&sentinel), sentinel_before);
    std::fs::remove_file(&escape_probe).unwrap();

    contenders[winner].send(b'D');
    let death_boundary = contenders[winner].event();
    assert_eq!(field(&death_boundary, "role"), "release_mutation_lease");
    assert_eq!(field(&death_boundary, "ordinal"), "1");
    assert_eq!(field(&death_boundary, "operation"), "none");
    assert_eq!(field(&death_boundary, "handle"), "none");
    assert_eq!(field(&death_boundary, "release_attempts"), "1");
    assert_eq!(field(&death_boundary, "release_completed"), "0");
    assert_eq!(field(&death_boundary, "ownership_releases"), "0");
    let winner_counters = field(&death_boundary, "stable_counters").to_owned();
    contenders[winner].kill();
    let killed = contenders[winner].wait();
    assert!(
        !killed.success(),
        "OS termination must not look like a normal close"
    );

    let successor = run_role("post-death-successor", &root, &[]);
    assert_eq!(successor.value("store"), stable_identity.unwrap());
    assert_ne!(successor.value("owner"), winner_owner.unwrap());
    assert_ne!(successor.value("attempt"), winner_attempt.unwrap());
    assert_ne!(successor.value("runtime"), winner_runtime.unwrap());
    assert!(contender_processes.insert(successor.value("process").to_owned()));
    assert_eq!(contender_processes.len(), CONTENDER_COUNT + 1);
    assert_eq!(
        successor.value("confinement_denials"),
        expected_confinements()
    );
    assert_eq!(successor.value("conserved"), "true");
    assert_eq!(successor.number("live_files"), 0);
    assert_eq!(successor.number("live_directories"), 0);
    assert_eq!(successor.number("ownership_releases"), 1);
    assert_eq!(observe_tree(&sentinel), sentinel_before);
    loser_counters.sort();

    let target_manifest_shape = manifest_projection(&observe_tree(&root));
    assert_eq!(target_manifest_shape, expected_store_manifest());
    CampaignProjection {
        owners: 1,
        contentions,
        target_manifest_shape,
        sentinel_manifest: sentinel_before,
        winner_counters,
        loser_counters,
        successor_counters: successor.value("stable_counters").into(),
    }
}

pub(super) fn run_child(role: &str, root: &Path) {
    match role {
        "mutation-contender" => contender_child(root),
        "unrelated-inheritance-probe" => contention_probe(root),
        "post-death-successor" => successor_child(root),
        _ => panic!("unsupported mutation role"),
    }
}

fn contender_child(root: &Path) {
    event("ready");
    let mut command = [0_u8; 1];
    std::io::stdin().read_exact(&mut command).unwrap();
    assert_eq!(command[0], b'S');
    let (admission, release_gate) = contention_admission();
    match admit_runtime(root)
        .try_admit_filesystem_media(admission)
        .into_raw()
    {
        TransitionOutcome::Success(media) => {
            let denied = run_confinement_corpus(&media);
            let observation = media.observer().snapshot().unwrap();
            event(&format!(
                "owned;store={};runtime={};process={};owner={};attempt={};confinement_denials={denied};counters={};stable_counters={}",
                hex(&media.store_identity().bytes()),
                media.runtime_identity().get(),
                std::process::id(),
                hex(&observation.mutation_owner().owner().bytes()),
                hex(&observation.mutation_owner().attempt().bytes()),
                exact_counter_projection(observation.media_counters()),
                campaign_counter_projection(observation.media_counters()),
            ));
            std::io::stdin().read_exact(&mut command).unwrap();
            assert_eq!(command[0], b'E');
            let effect = media.certification_staging_effect_probe(
                confinement_probe_authority(),
                "confinement-probe",
            );
            event(&format!(
                "escape-probe;outcome={};status={}",
                confinement_effect_name(effect),
                confinement_status_name(effect),
            ));
            std::io::stdin().read_exact(&mut command).unwrap();
            assert_eq!(command[0], b'D');
            let probe = run_role("unrelated-inheritance-probe", root, &[]);
            assert_eq!(probe.value("outcome"), "contended");
            reach_death_boundary(media, release_gate);
        }
        TransitionOutcome::Deferred(deferred) => {
            let counters = deferred.reason().counters();
            assert_loser_counters(counters, false);
            deferred.into_runtime().abort();
            event(&format!(
                "contended;effectful=false;process={};counters={};stable_counters={}",
                std::process::id(),
                exact_counter_projection(counters),
                campaign_counter_projection(counters),
            ));
        }
        TransitionOutcome::Failed(failure) => {
            let counters = inspection_counters(failure.cause());
            assert_loser_counters(counters, true);
            event(&format!(
                "contended;effectful=true;process={};counters={};stable_counters={}",
                std::process::id(),
                exact_counter_projection(counters),
                campaign_counter_projection(counters),
            ));
        }
        _ => panic!("contention returned a non-contention outcome"),
    }
}

fn contention_probe(root: &Path) {
    let runtime = admit_runtime(root);
    match runtime
        .try_admit_filesystem_media(media_admission())
        .into_raw()
    {
        TransitionOutcome::Deferred(deferred) => {
            assert_loser_counters(deferred.reason().counters(), false);
            deferred.into_runtime().abort();
            emit(&[("outcome", "contended".into())]);
        }
        _ => panic!("unrelated child inherited or acquired mutation authority"),
    }
}

fn successor_child(root: &Path) {
    let media = match admit_runtime(root)
        .try_admit_filesystem_media(media_admission())
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("successor failed to acquire the released OS lease"),
    };
    let denied = run_confinement_corpus(&media);
    let observation = media.observer().snapshot().unwrap();
    let observer = media.observer();
    let store = hex(&media.store_identity().bytes());
    assert!(matches!(media.close(), MediaShutdownOutcome::Released(_)));
    let counters = observer.media_counters();
    let fields = [
        ("store", store),
        ("runtime", observation.runtime_identity().get().to_string()),
        ("process", std::process::id().to_string()),
        ("owner", hex(&observation.mutation_owner().owner().bytes())),
        (
            "attempt",
            hex(&observation.mutation_owner().attempt().bytes()),
        ),
        ("confinement_denials", denied.to_string()),
        ("conserved", counters.is_conserved().to_string()),
        ("live_files", counters.live_file_handles().to_string()),
        (
            "live_directories",
            counters.live_directory_handles().to_string(),
        ),
        (
            "ownership_releases",
            counters.ownership_releases().to_string(),
        ),
        ("counters", exact_counter_projection(counters)),
        ("stable_counters", campaign_counter_projection(counters)),
    ];
    emit(&fields);
}

fn run_confinement_corpus(media: &worth_store::physical_runtime::MediaOwnedPhysicalRuntime) -> u64 {
    let before = media.media_counters().confinement_denials();
    let authority = confinement_probe_authority();
    for (component, expected) in HOSTILE_COMPONENTS {
        let denial = media
            .certification_confinement_probe(authority, component)
            .expect_err("hostile component escaped confinement");
        assert_eq!(denial.kind(), expected);
    }
    media.media_counters().confinement_denials() - before
}

fn confinement_probe_authority(
) -> worth_store::physical_runtime::certification::CertificationMediaFaultAuthority {
    media_admission().fault_schedule_authority()
}

fn expected_confinements() -> &'static str {
    "10"
}

fn confinement_effect_name(effect: CertificationConfinementEffect) -> &'static str {
    match effect {
        CertificationConfinementEffect::ComponentDenied(_) => "component-denied",
        CertificationConfinementEffect::OpenDenied(_) => "open-denied",
        CertificationConfinementEffect::WriteReached(_) => "write-reached",
    }
}

fn confinement_status_name(effect: CertificationConfinementEffect) -> &'static str {
    let status = match effect {
        CertificationConfinementEffect::ComponentDenied(_) => return "component-denial",
        CertificationConfinementEffect::OpenDenied(status)
        | CertificationConfinementEffect::WriteReached(status) => status,
    };
    match status {
        MediaEffectStatus::DeniedBeforeEffect => "denied-before-effect",
        MediaEffectStatus::PartialTransfer => "partial-transfer",
        MediaEffectStatus::CompletedEffect => "completed-effect",
        MediaEffectStatus::IndeterminateEffect => "indeterminate-effect",
        MediaEffectStatus::UnsupportedCapability => "unsupported-capability",
        MediaEffectStatus::StaleHandle => "stale-handle",
    }
}

pub(super) fn event(message: &str) {
    println!("{EVENT_PREFIX}{message}");
    std::io::stdout().flush().unwrap();
}

fn field<'a>(fields: &'a str, name: &str) -> &'a str {
    fields
        .split(';')
        .find_map(|field| field.strip_prefix(&format!("{name}=")))
        .unwrap_or_else(|| panic!("event omitted {name}"))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
