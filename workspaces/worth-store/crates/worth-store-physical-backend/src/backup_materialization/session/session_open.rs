use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use super::{
    allocate_buffer, collect_sources, io_denial, open_read, reject_symbolic_link,
    reject_symbolic_link_if_present, validate_resume_prefix, validate_source_identity_and_length,
    validate_source_set, PhysicalBackupMaterializationCounters,
    PhysicalBackupMaterializationDenial, PhysicalBackupMaterializationSession,
    PhysicalBackupSessionIdentityGuard, SourceProgress,
};

const MAX_SESSION_IDENTITY_BYTES: usize = 128;

impl PhysicalBackupMaterializationSession {
    pub fn open_or_resume(
        target_parent: impl Into<PathBuf>,
        session_identity: &str,
        sources: impl IntoIterator<Item = crate::PhysicalBackupSource>,
        buffer_bytes: usize,
    ) -> Result<Self, PhysicalBackupMaterializationDenial> {
        validate_request(session_identity, buffer_bytes)?;
        let target_parent = target_parent.into();
        let staging_root = target_parent.join(format!(".incomplete-{session_identity}"));
        let final_root = target_parent.join(format!("backup-{session_identity}"));
        let staging_preexisting = staging_root.exists();
        let sources = collect_sources(sources)?;
        validate_source_set(&sources)?;
        if !final_root.exists() {
            validate_sources(&sources)?;
        }
        let guard = PhysicalBackupSessionIdentityGuard::acquire(&target_parent, session_identity)?;
        if final_root.exists() {
            return open_published_recovery(staging_root, final_root, sources, buffer_bytes, guard);
        }
        reject_sources_inside_session_output(&sources, &target_parent, &staging_root, &final_root)?;
        validate_sources(&sources)?;
        std::fs::create_dir_all(&staging_root)
            .map_err(|source| io_denial(&staging_root, source))?;
        reject_symbolic_link(&staging_root)?;
        reject_unexpected_staging_entries(&staging_root, &sources)?;
        reject_preexisting_output_aliases(&staging_root, &sources)?;
        let descriptor_syncs = guard.bind_source_set(&staging_root, session_identity, &sources)?;
        let counters = PhysicalBackupMaterializationCounters {
            peak_buffer_bytes: buffer_bytes as u64,
            sync_operations: descriptor_syncs,
            resumed_sessions: u64::from(staging_preexisting),
            ..PhysicalBackupMaterializationCounters::default()
        };
        if publication_started(&staging_root) {
            return open_publication_recovery(
                staging_root,
                final_root,
                sources,
                buffer_bytes,
                counters,
                guard,
            );
        }
        open_copy_session(
            staging_root,
            final_root,
            sources,
            buffer_bytes,
            counters,
            guard,
        )
    }
}

fn validate_request(
    session_identity: &str,
    buffer_bytes: usize,
) -> Result<(), PhysicalBackupMaterializationDenial> {
    if session_identity.is_empty()
        || session_identity.len() > MAX_SESSION_IDENTITY_BYTES
        || !session_identity
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
    {
        return Err(PhysicalBackupMaterializationDenial::InvalidSessionIdentity);
    }
    if buffer_bytes == 0 {
        return Err(PhysicalBackupMaterializationDenial::InvalidBufferBudget);
    }
    Ok(())
}

fn validate_sources(
    sources: &[crate::PhysicalBackupSource],
) -> Result<(), PhysicalBackupMaterializationDenial> {
    for source in sources {
        validate_source_identity_and_length(source)?;
    }
    Ok(())
}

fn open_published_recovery(
    staging_root: PathBuf,
    final_root: PathBuf,
    sources: Vec<crate::PhysicalBackupSource>,
    buffer_bytes: usize,
    guard: PhysicalBackupSessionIdentityGuard,
) -> Result<PhysicalBackupMaterializationSession, PhysicalBackupMaterializationDenial> {
    reject_symbolic_link(&final_root)?;
    if staging_root.exists() {
        return Err(
            PhysicalBackupMaterializationDenial::ConflictingPublicationState {
                staging_root,
                final_root,
            },
        );
    }
    Ok(recovered_session(
        staging_root,
        final_root,
        sources,
        allocate_buffer(buffer_bytes)?,
        PhysicalBackupMaterializationCounters {
            peak_buffer_bytes: buffer_bytes as u64,
            resumed_sessions: 1,
            ..PhysicalBackupMaterializationCounters::default()
        },
        guard,
    ))
}

fn open_publication_recovery(
    staging_root: PathBuf,
    final_root: PathBuf,
    sources: Vec<crate::PhysicalBackupSource>,
    buffer_bytes: usize,
    counters: PhysicalBackupMaterializationCounters,
    guard: PhysicalBackupSessionIdentityGuard,
) -> Result<PhysicalBackupMaterializationSession, PhysicalBackupMaterializationDenial> {
    Ok(recovered_session(
        staging_root,
        final_root,
        sources,
        allocate_buffer(buffer_bytes)?,
        PhysicalBackupMaterializationCounters {
            resumed_sessions: 1,
            ..counters
        },
        guard,
    ))
}

fn recovered_session(
    staging_root: PathBuf,
    final_root: PathBuf,
    sources: Vec<crate::PhysicalBackupSource>,
    buffer: Vec<u8>,
    counters: PhysicalBackupMaterializationCounters,
    session_identity_guard: PhysicalBackupSessionIdentityGuard,
) -> PhysicalBackupMaterializationSession {
    PhysicalBackupMaterializationSession {
        staging_root,
        final_root,
        sources: Vec::new(),
        source_index: 0,
        buffer,
        counters,
        published_recovery_sources: Some(sources),
        session_identity_guard,
    }
}

fn open_copy_session(
    staging_root: PathBuf,
    final_root: PathBuf,
    sources: Vec<crate::PhysicalBackupSource>,
    buffer_bytes: usize,
    mut counters: PhysicalBackupMaterializationCounters,
    session_identity_guard: PhysicalBackupSessionIdentityGuard,
) -> Result<PhysicalBackupMaterializationSession, PhysicalBackupMaterializationDenial> {
    let mut buffer = allocate_buffer(buffer_bytes)?;
    let sources = prepare_source_progress(&staging_root, sources, &mut buffer, &mut counters)?;
    Ok(PhysicalBackupMaterializationSession {
        staging_root,
        final_root,
        sources,
        source_index: 0,
        buffer,
        counters,
        published_recovery_sources: None,
        session_identity_guard,
    })
}

fn prepare_source_progress(
    staging_root: &Path,
    sources: Vec<crate::PhysicalBackupSource>,
    buffer: &mut [u8],
    counters: &mut PhysicalBackupMaterializationCounters,
) -> Result<Vec<SourceProgress>, PhysicalBackupMaterializationDenial> {
    let mut progress = Vec::new();
    progress
        .try_reserve_exact(sources.len())
        .map_err(|_| PhysicalBackupMaterializationDenial::SourceCollectionAllocationFailed)?;
    for source in sources {
        progress.push(prepare_one_source(staging_root, source, buffer, counters)?);
    }
    Ok(progress)
}

fn reject_sources_inside_session_output(
    sources: &[crate::PhysicalBackupSource],
    target_parent: &Path,
    staging_root: &Path,
    final_root: &Path,
) -> Result<(), PhysicalBackupMaterializationDenial> {
    let canonical_parent =
        std::fs::canonicalize(target_parent).map_err(|source| io_denial(target_parent, source))?;
    let staging_name = staging_root
        .file_name()
        .ok_or(PhysicalBackupMaterializationDenial::InvalidSessionIdentity)?;
    let final_name = final_root
        .file_name()
        .ok_or(PhysicalBackupMaterializationDenial::InvalidSessionIdentity)?;
    let canonical_staging = canonical_parent.join(staging_name);
    let canonical_final = canonical_parent.join(final_name);
    for source in sources {
        let canonical_source = std::fs::canonicalize(source.source_path())
            .map_err(|error| io_denial(source.source_path(), error))?;
        for reserved_root in [&canonical_staging, &canonical_final] {
            if canonical_source.starts_with(reserved_root) {
                return Err(
                    PhysicalBackupMaterializationDenial::SourceInsideSessionOutput {
                        source: source.source_path().to_path_buf(),
                        reserved_root: reserved_root.clone(),
                    },
                );
            }
        }
    }
    Ok(())
}

fn reject_preexisting_output_aliases(
    staging_root: &Path,
    sources: &[crate::PhysicalBackupSource],
) -> Result<(), PhysicalBackupMaterializationDenial> {
    let mut output_identities = Vec::new();
    output_identities
        .try_reserve_exact(sources.len())
        .map_err(|_| PhysicalBackupMaterializationDenial::SourceCollectionAllocationFailed)?;
    for source in sources {
        let output = staging_root.join(source.output_name());
        reject_symbolic_link_if_present(&output)?;
        if !output.exists() {
            continue;
        }
        let identity =
            crate::offline_media::physical_file_identity(&output).map_err(
                |denial| match denial {
                    crate::OfflineMediaReadDenial::Io { path, source } => io_denial(&path, source),
                    _ => PhysicalBackupMaterializationDenial::OutputAliasesMaterializationFile {
                        path: output.clone(),
                    },
                },
            )?;
        if sources
            .iter()
            .any(|candidate| candidate.expected_physical_identity() == identity)
            || output_identities.contains(&identity)
        {
            return Err(
                PhysicalBackupMaterializationDenial::OutputAliasesMaterializationFile {
                    path: output,
                },
            );
        }
        output_identities.push(identity);
    }
    Ok(())
}

fn reject_unexpected_staging_entries(
    staging_root: &Path,
    sources: &[crate::PhysicalBackupSource],
) -> Result<(), PhysicalBackupMaterializationDenial> {
    let mut allowed_outputs = Vec::new();
    allowed_outputs
        .try_reserve_exact(sources.len())
        .map_err(|_| PhysicalBackupMaterializationDenial::SourceCollectionAllocationFailed)?;
    allowed_outputs.extend(sources.iter().map(crate::PhysicalBackupSource::output_name));
    allowed_outputs.sort_unstable();
    let entries =
        std::fs::read_dir(staging_root).map_err(|source| io_denial(staging_root, source))?;
    for entry in entries {
        let entry = entry.map_err(|source| io_denial(staging_root, source))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|source| io_denial(&path, source))?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            return Err(PhysicalBackupMaterializationDenial::UnexpectedStagingEntry { path });
        };
        let reserved = matches!(
            name,
            "materialization.session" | "backup.manifest.pending" | "backup.manifest"
        );
        if file_type.is_symlink()
            || !file_type.is_file()
            || (!reserved && allowed_outputs.binary_search(&name).is_err())
        {
            return Err(PhysicalBackupMaterializationDenial::UnexpectedStagingEntry { path });
        }
    }
    Ok(())
}

fn prepare_one_source(
    staging_root: &Path,
    source: crate::PhysicalBackupSource,
    buffer: &mut [u8],
    counters: &mut PhysicalBackupMaterializationCounters,
) -> Result<SourceProgress, PhysicalBackupMaterializationDenial> {
    let output = staging_root.join(source.output_name());
    reject_symbolic_link_if_present(&output)?;
    let observed = output
        .metadata()
        .map(|metadata| metadata.len())
        .unwrap_or(0);
    let admitted_prefix = observed.min(source.expected_bytes());
    truncate_excess_output(&output, observed, admitted_prefix, counters)?;
    let mut hasher = Sha256::new();
    let copied = if admitted_prefix > 0 {
        validate_resume_prefix(
            &source,
            &output,
            admitted_prefix,
            buffer,
            &mut hasher,
            counters,
        )?
    } else {
        0
    };
    super::add_counter(&mut counters.rollback_bytes, admitted_prefix - copied)?;
    if copied > 0 {
        super::add_counter(&mut counters.resumed_artifacts, 1)?;
        super::add_counter(&mut counters.resumed_bytes, copied)?;
    }
    let mut input = open_read(source.source_path())?;
    input
        .seek(SeekFrom::Start(copied))
        .map_err(|error| io_denial(source.source_path(), error))?;
    let mut output_file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(&output)
        .map_err(|error| io_denial(&output, error))?;
    output_file
        .seek(SeekFrom::Start(copied))
        .map_err(|error| io_denial(&output, error))?;
    Ok(SourceProgress {
        source,
        copied,
        hasher,
        input,
        output: output_file,
        durable: false,
    })
}

fn truncate_excess_output(
    output: &Path,
    observed: u64,
    admitted: u64,
    counters: &mut PhysicalBackupMaterializationCounters,
) -> Result<(), PhysicalBackupMaterializationDenial> {
    if observed == admitted {
        return Ok(());
    }
    let file = OpenOptions::new()
        .write(true)
        .open(output)
        .map_err(|source| io_denial(output, source))?;
    file.set_len(admitted)
        .map_err(|source| io_denial(output, source))?;
    file.sync_all()
        .map_err(|source| io_denial(output, source))?;
    super::add_counter(&mut counters.sync_operations, 1)?;
    super::add_counter(&mut counters.rollback_bytes, observed - admitted)
}

fn publication_started(staging_root: &Path) -> bool {
    staging_root.join("backup.manifest.pending").exists()
        || staging_root.join("backup.manifest").exists()
}
