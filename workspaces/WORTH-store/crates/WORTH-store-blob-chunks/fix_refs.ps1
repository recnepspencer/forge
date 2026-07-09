$src = "C:\Users\Esther\Documents\Programming\WORTH_workspace\WORTH\workspaces\worth-store\crates\worth-store-blob-chunks\src"
$replacements = [ordered]@{
    'crate::blob_chunk_test_support' = 'crate::test_support'
    'crate::blob_chunk_physical_test_support' = 'crate::test_support'
    'crate::blob_generation_registry_test_support' = 'crate::lifecycle::generation_registry_test_support'
    'crate::blob_publication_commit_test_support' = 'crate::publication::test_support'
    'crate::blob_retention_reclaim_test_support' = 'crate::retention_reclaim::test_support'
    'crate::blob_reachability_hold_test_support' = 'crate::reachability::hold_test_support'
    'crate::blob_placement_admission' = 'crate::placement::admission'
    'crate::blob_publication_commit::' = 'crate::publication::'
    'crate::blob_reachability_edges::' = 'crate::reachability::edges::'
    'crate::blob_reachability_reclaim_release::' = 'crate::reachability::reclaim_release::'
    'crate::blob_reachability_snapshot::' = 'crate::reachability::snapshot::'
    'crate::blob_lifecycle_authority::' = 'crate::lifecycle::authority::'
    'crate::blob_chunk_dedupe_canonical::' = 'crate::dedupe::canonical::'
    'crate::blob_streaming_performance::' = 'crate::streaming::performance::'
}

Get-ChildItem $src -Recurse -Filter "*.rs" | ForEach-Object {
    $content = Get-Content $_.FullName -Raw
    $original = $content
    foreach ($entry in $replacements.GetEnumerator()) {
        $content = $content.Replace($entry.Key, $entry.Value)
    }
    if ($content -ne $original) {
        Set-Content $_.FullName $content -NoNewline
        Write-Host "Fixed: $($_.FullName.Replace($src + '\', ''))"
    }
}
Write-Host "Reference fix complete."