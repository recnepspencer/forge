$ErrorActionPreference = "Stop"

$root = Resolve-Path (Join-Path $PSScriptRoot "..\..")
Set-Location $root

Write-Host "[worth-topo-domain-structure] cargo fmt"
cargo fmt --package worth-topo --check

Write-Host "[worth-topo-domain-structure] cargo check"
cargo check -p worth-topo --quiet

Write-Host "[worth-topo-domain-structure] structure guards"
cargo test -p worth-topo certification::structure_guard --quiet

Write-Host "[worth-topo-domain-structure] facade privacy compile-fail contracts"
cargo test -p worth-topo --test ui --quiet

Write-Host "[worth-topo-domain-structure] worth-topo full suite"
cargo test -p worth-topo --quiet

Write-Host "[worth-topo-domain-structure] worth-topo rust line caps"
$violations = @()
git ls-files "crates/worth-topo/**/*.rs" | Sort-Object | ForEach-Object {
    if (-not (Test-Path $_)) {
        return
    }
    $lineCount = (Get-Content $_).Count
    if ($lineCount -gt 400) {
        $violations += "FAIL: $_ is $lineCount lines (cap 400)"
    }
}

if ($violations.Count -gt 0) {
    $violations | ForEach-Object { Write-Host $_ }
    exit 1
}

Write-Host "[worth-topo-domain-structure] PASS"
