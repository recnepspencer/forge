$ErrorActionPreference = "Stop"
$src = "C:\Users\Esther\Documents\Programming\WORTH_workspace\WORTH\workspaces\worth-store\crates\worth-store-blob-chunks\src"

function Ensure-Dir($path) {
    if (-not (Test-Path $path)) { New-Item -ItemType Directory -Path $path -Force | Out-Null }
}

function Move-File($from, $to) {
    $fromPath = Join-Path $src $from
    $toPath = Join-Path $src $to
    if (-not (Test-Path $fromPath)) {
        Write-Host "SKIP (missing): $from"
        return
    }
    Ensure-Dir (Split-Path $toPath -Parent)
    if (Test-Path $toPath) { Remove-Item $toPath -Force }
    Move-Item $fromPath $toPath
    Write-Host "MOVED: $from -> $to"
}

# Top-level lifecycle dirs
@(
    "compile_fail", "chunk_identity", "chunk_integrity", "dedupe", "streaming", "lifecycle",
    "publication", "reachability", "placement", "recovery", "retention_reclaim",
    "compaction", "corruption", "handoffs", "test_support"
) | ForEach-Object { Ensure-Dir (Join-Path $src $_) }

# Rename existing subdirs
$renames = @{
    "blob_compaction" = "compaction"
    "blob_corruption" = "corruption"
    "blob_publication_commit" = "publication"
    "blob_placement_admission" = "placement/admission"
    "blob_placement_movement" = "placement/movement"
    "blob_recovery_records" = "recovery/records"
    "blob_resume_session" = "recovery/resume_session"
}

foreach ($entry in $renames.GetEnumerator()) {
    $from = Join-Path $src $entry.Key
    $to = Join-Path $src $entry.Value
    if (Test-Path $from) {
        Ensure-Dir (Split-Path $to -Parent)
        if (Test-Path $to) { Remove-Item $to -Recurse -Force }
        Move-Item $from $to
        Write-Host "RENAMED DIR: $($entry.Key) -> $($entry.Value)"
    }
}

# retention_reclaim: merge wrapper + subdir
$rrWrapper = Join-Path $src "blob_retention_reclaim.rs"
$rrDir = Join-Path $src "blob_retention_reclaim"
$rrTarget = Join-Path $src "retention_reclaim"
if (Test-Path $rrDir) {
    Get-ChildItem $rrDir -File | ForEach-Object {
        Move-Item $_.FullName (Join-Path $rrTarget $_.Name) -Force
    }
    Remove-Item $rrDir -Recurse -Force
}
if (Test-Path $rrWrapper) {
    Move-Item $rrWrapper (Join-Path $rrTarget "mod.rs") -Force
}

# compile_fail standalone files
$compileFailMoves = @{
    "blob_chunk_integrity_compile_fail.rs" = "compile_fail/integrity.rs"
    "blob_chunk_root_compile_fail.rs" = "compile_fail/root.rs"
    "blob_corruption_compile_fail.rs" = "compile_fail/corruption.rs"
    "blob_generation_registry_compile_fail.rs" = "compile_fail/generation_registry.rs"
    "blob_publication_commit_compile_fail.rs" = "compile_fail/publication.rs"
    "blob_reachability_compile_fail.rs" = "compile_fail/reachability.rs"
    "blob_recovery_records_compile_fail.rs" = "compile_fail/recovery_records.rs"
    "blob_retention_reclaim_compile_fail.rs" = "compile_fail/retention_reclaim.rs"
    "blob_streaming_read_compile_fail.rs" = "compile_fail/streaming_read.rs"
    "security_metadata_compile_fail.rs" = "compile_fail/security_metadata.rs"
}
foreach ($entry in $compileFailMoves.GetEnumerator()) {
    Move-File $entry.Key $entry.Value
}
if (Test-Path (Join-Path $src "placement/movement/compile_fail.rs")) {
    Move-File "placement/movement/compile_fail.rs" "compile_fail/placement_movement.rs"
}

# chunk_identity
@{
    "blob_chunk_bytes.rs" = "chunk_identity/bytes.rs"
    "blob_chunk_identity.rs" = "chunk_identity/identity.rs"
    "blob_chunk_security_metadata.rs" = "chunk_identity/security_metadata.rs"
    "blob_chunk_scope.rs" = "chunk_identity/scope.rs"
    "blob_scoped_chunk.rs" = "chunk_identity/scoped_chunk.rs"
    "blob_chunk_counters.rs" = "chunk_identity/counters.rs"
    "blob_chunk_denial.rs" = "chunk_identity/denial.rs"
    "blob_chunk_scope_tests.rs" = "chunk_identity/scope_tests.rs"
    "blob_chunk_security_metadata_tests.rs" = "chunk_identity/security_metadata_tests.rs"
} | ForEach-Object { $_.GetEnumerator() } | ForEach-Object { Move-File $_.Key $_.Value }

# chunk_integrity
@{
    "blob_chunk_integrity.rs" = "chunk_integrity/integrity.rs"
    "blob_chunk_integrity_denial.rs" = "chunk_integrity/denial.rs"
    "blob_chunk_integrity_tests.rs" = "chunk_integrity/tests.rs"
} | ForEach-Object { $_.GetEnumerator() } | ForEach-Object { Move-File $_.Key $_.Value }

# dedupe
@{
    "blob_chunk_dedupe.rs" = "dedupe/admission.rs"
    "blob_chunk_dedupe_byte_comparison.rs" = "dedupe/byte_comparison.rs"
    "blob_chunk_dedupe_canonical.rs" = "dedupe/canonical.rs"
    "blob_chunk_dedupe_collision.rs" = "dedupe/collision.rs"
    "blob_chunk_dedupe_counters.rs" = "dedupe/counters.rs"
    "blob_chunk_dedupe_index_posture.rs" = "dedupe/index_posture.rs"
    "blob_chunk_dedupe_policy.rs" = "dedupe/policy.rs"
    "blob_chunk_dedupe_receipt.rs" = "dedupe/receipt.rs"
    "blob_chunk_dedupe_reference_edges.rs" = "dedupe/reference_edges.rs"
    "blob_chunk_collision_verification.rs" = "dedupe/collision_verification.rs"
    "blob_chunk_canonical_basis.rs" = "dedupe/canonical_basis.rs"
    "blob_chunk_canonical_comparison_basis.rs" = "dedupe/canonical_comparison_basis.rs"
    "blob_chunk_root_comparison.rs" = "dedupe/root_comparison.rs"
    "blob_chunk_root_counters.rs" = "dedupe/root_counters.rs"
    "blob_chunk_root_denial.rs" = "dedupe/root_denial.rs"
    "blob_chunk_root_publication.rs" = "dedupe/root_publication.rs"
    "blob_chunk_root_publication_tests.rs" = "dedupe/root_publication_tests.rs"
    "blob_chunk_reference_accounting.rs" = "dedupe/reference_accounting.rs"
} | ForEach-Object { $_.GetEnumerator() } | ForEach-Object { Move-File $_.Key $_.Value }
if (Test-Path (Join-Path $src "blob_chunk_dedupe_reference_edges")) {
    $dedupeRefEdgesDir = Join-Path $src "dedupe/reference_edges"
    Ensure-Dir $dedupeRefEdgesDir
    Get-ChildItem (Join-Path $src "blob_chunk_dedupe_reference_edges") -File | ForEach-Object {
        Move-Item $_.FullName (Join-Path $dedupeRefEdgesDir $_.Name) -Force
    }
    Remove-Item (Join-Path $src "blob_chunk_dedupe_reference_edges") -Recurse -Force
}

# streaming
@{
    "blob_chunk_streaming.rs" = "streaming/chunk_streaming.rs"
    "blob_chunk_sequence.rs" = "streaming/sequence.rs"
    "blob_chunk_rule.rs" = "streaming/rule.rs"
    "blob_streaming_counters.rs" = "streaming/counters.rs"
    "blob_streaming_denial.rs" = "streaming/denial.rs"
    "blob_streaming_equivalence_tests.rs" = "streaming/equivalence_tests.rs"
    "blob_streaming_frontier.rs" = "streaming/frontier.rs"
    "blob_streaming_ingest.rs" = "streaming/ingest.rs"
    "blob_streaming_ingest_tests.rs" = "streaming/ingest_tests.rs"
    "blob_streaming_performance.rs" = "streaming/performance.rs"
    "blob_streaming_pressure_tests.rs" = "streaming/pressure_tests.rs"
    "blob_streaming_read.rs" = "streaming/read.rs"
    "blob_streaming_read_admission.rs" = "streaming/read_admission.rs"
    "blob_streaming_read_counters.rs" = "streaming/read_counters.rs"
    "blob_streaming_read_denial.rs" = "streaming/read_denial.rs"
    "blob_streaming_read_observation.rs" = "streaming/read_observation.rs"
    "blob_streaming_read_performance.rs" = "streaming/read_performance.rs"
    "blob_streaming_read_pressure_tests.rs" = "streaming/read_pressure_tests.rs"
    "blob_streaming_read_request.rs" = "streaming/read_request.rs"
    "blob_streaming_read_tests.rs" = "streaming/read_tests.rs"
    "blob_streaming_read_verification.rs" = "streaming/read_verification.rs"
    "blob_streaming_request.rs" = "streaming/request.rs"
    "blob_streaming_residency.rs" = "streaming/residency.rs"
    "blob_streaming_resume.rs" = "streaming/resume.rs"
    "blob_streaming_resume_tests.rs" = "streaming/resume_tests.rs"
    "blob_streaming_source.rs" = "streaming/source.rs"
    "large_record_streaming_envelope.rs" = "streaming/large_record_envelope.rs"
} | ForEach-Object { $_.GetEnumerator() } | ForEach-Object { Move-File $_.Key $_.Value }

# lifecycle
@{
    "blob_lifecycle_authority.rs" = "lifecycle/authority.rs"
    "blob_lifecycle_counters.rs" = "lifecycle/counters.rs"
    "blob_lifecycle_denial.rs" = "lifecycle/denial.rs"
    "blob_lifecycle_identity.rs" = "lifecycle/identity.rs"
    "blob_lifecycle_progression.rs" = "lifecycle/progression.rs"
    "blob_lifecycle_receipts.rs" = "lifecycle/receipts.rs"
    "blob_lifecycle_boundary_tests.rs" = "lifecycle/boundary_tests.rs"
    "blob_generation_classification.rs" = "lifecycle/generation_classification.rs"
    "blob_generation_registry.rs" = "lifecycle/generation_registry.rs"
    "blob_generation_registry_authority.rs" = "lifecycle/generation_registry_authority.rs"
    "blob_generation_registry_counters.rs" = "lifecycle/generation_registry_counters.rs"
    "blob_generation_registry_denial.rs" = "lifecycle/generation_registry_denial.rs"
    "blob_generation_registry_test_support.rs" = "lifecycle/generation_registry_test_support.rs"
    "blob_generation_registry_tests.rs" = "lifecycle/generation_registry_tests.rs"
} | ForEach-Object { $_.GetEnumerator() } | ForEach-Object { Move-File $_.Key $_.Value }

# publication root tests
@{
    "blob_publication_commit_tests.rs" = "publication/tests.rs"
    "blob_publication_commit_test_support.rs" = "publication/test_support.rs"
} | ForEach-Object { $_.GetEnumerator() } | ForEach-Object { Move-File $_.Key $_.Value }

# reachability
@{
    "blob_reachability_authority_tests.rs" = "reachability/authority_tests.rs"
    "blob_reachability_checkpoint_tests.rs" = "reachability/checkpoint_tests.rs"
    "blob_reachability_counters.rs" = "reachability/counters.rs"
    "blob_reachability_dedupe_release_tests.rs" = "reachability/dedupe_release_tests.rs"
    "blob_reachability_denial.rs" = "reachability/denial.rs"
    "blob_reachability_edges.rs" = "reachability/edges.rs"
    "blob_reachability_hold_test_support.rs" = "reachability/hold_test_support.rs"
    "blob_reachability_holds.rs" = "reachability/holds.rs"
    "blob_reachability_proof.rs" = "reachability/proof.rs"
    "blob_reachability_reclaim_release.rs" = "reachability/reclaim_release.rs"
    "blob_reachability_registry.rs" = "reachability/registry.rs"
    "blob_reachability_snapshot.rs" = "reachability/snapshot.rs"
    "blob_reachability_tests.rs" = "reachability/tests.rs"
} | ForEach-Object { $_.GetEnumerator() } | ForEach-Object { Move-File $_.Key $_.Value }
if (Test-Path (Join-Path $src "blob_reachability_edges")) {
    $reachEdgesDir = Join-Path $src "reachability/edges"
    Ensure-Dir $reachEdgesDir
    Get-ChildItem (Join-Path $src "blob_reachability_edges") -File | ForEach-Object {
        Move-Item $_.FullName (Join-Path $reachEdgesDir $_.Name) -Force
    }
    Remove-Item (Join-Path $src "blob_reachability_edges") -Recurse -Force
}
if (Test-Path (Join-Path $src "blob_reachability_registry")) {
    $reachRegistryDir = Join-Path $src "reachability/registry"
    Ensure-Dir $reachRegistryDir
    Get-ChildItem (Join-Path $src "blob_reachability_registry") -File | ForEach-Object {
        Move-Item $_.FullName (Join-Path $reachRegistryDir $_.Name) -Force
    }
    Remove-Item (Join-Path $src "blob_reachability_registry") -Recurse -Force
}

# placement proof
Move-File "blob_placement_proof.rs" "placement/proof.rs"

# recovery root tests
@{
    "blob_recovery_record_generation_tests.rs" = "recovery/record_generation_tests.rs"
    "blob_recovery_records_residue_tests.rs" = "recovery/records_residue_tests.rs"
    "blob_recovery_records_tests.rs" = "recovery/records_tests.rs"
} | ForEach-Object { $_.GetEnumerator() } | ForEach-Object { Move-File $_.Key $_.Value }

# retention_reclaim root tests
@{
    "blob_retention_reclaim_test_support.rs" = "retention_reclaim/test_support.rs"
    "blob_retention_reclaim_tests.rs" = "retention_reclaim/tests.rs"
} | ForEach-Object { $_.GetEnumerator() } | ForEach-Object { Move-File $_.Key $_.Value }

# corruption root tests
@{
    "blob_corruption_shared_reference_tests.rs" = "corruption/shared_reference_tests.rs"
    "blob_corruption_tests.rs" = "corruption/tests.rs"
} | ForEach-Object { $_.GetEnumerator() } | ForEach-Object { Move-File $_.Key $_.Value }

# handoffs
@{
    "s6_background_pressure.rs" = "handoffs/background_pressure.rs"
    "s6_reclaim_handoff.rs" = "handoffs/reclaim_handoff.rs"
    "s7_blob_security_handoff.rs" = "handoffs/blob_security_handoff.rs"
    "s7_harness_vocab.rs" = "handoffs/harness_vocab.rs"
} | ForEach-Object { $_.GetEnumerator() } | ForEach-Object { Move-File $_.Key $_.Value }

# test_support
@{
    "blob_chunk_test_support.rs" = "test_support/chunk.rs"
    "blob_chunk_physical_test_support.rs" = "test_support/physical.rs"
} | ForEach-Object { $_.GetEnumerator() } | ForEach-Object { Move-File $_.Key $_.Value }

Write-Host "Migration complete."