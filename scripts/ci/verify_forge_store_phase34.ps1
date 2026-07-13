param(
    [switch]$Workspace
)

$ErrorActionPreference = "Stop"
$root = Resolve-Path (Join-Path $PSScriptRoot "../..")
$store = Join-Path $root "workspaces/forge-store"

function Invoke-Checked {
    param([string]$Label, [string[]]$Arguments)

    Write-Host "==> $Label"
    & cargo @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "$Label failed with exit code $LASTEXITCODE"
    }
}

Push-Location $store
try {
    & python (Join-Path $root "scripts/ci/check_forge_store_phase34_dependencies.py")
    if ($LASTEXITCODE -ne 0) {
        throw "Phase 34 dependency direction failed with exit code $LASTEXITCODE"
    }

    Invoke-Checked "layout indexes check" @("check", "-p", "forge-store-layout-indexes")
    Invoke-Checked "physical isolation check" @(
        "check", "-p", "forge-store-physical-isolation", "--features", "certification-authority"
    )
    Invoke-Checked "layout indexes unit and residue tests" @(
        "test", "-p", "forge-store-layout-indexes", "--lib"
    )
    Invoke-Checked "layout authority compile-fail suites" @(
        "test", "-p", "forge-store-layout-indexes", "--test", "layout_compile_fail"
    )
    Invoke-Checked "physical compaction interlock" @(
        "test", "-p", "forge-store-physical-isolation", "compaction_interlock"
    )
    Invoke-Checked "durable LSM membership authority" @(
        "test", "-p", "forge-store-lsm-authority"
    )
    Invoke-Checked "layout access-path courtroom" @(
        "test", "-p", "forge-store-certification", "--test", "s8_layout_access_path_harness"
    )
    Invoke-Checked "layout corruption and rebuild courtroom" @(
        "test", "-p", "forge-store-certification", "--test", "s8_layout_corruption_rebuild"
    )
    Invoke-Checked "B-tree lookup courtroom" @(
        "test", "-p", "forge-store-certification", "--test", "s8_btree_lookup_authority"
    )
    Invoke-Checked "B-tree replay courtroom" @(
        "test", "-p", "forge-store-certification", "--test", "s8_btree_replay_authority"
    )
    Invoke-Checked "LSM lookup and replay courtroom" @(
        "test", "-p", "forge-store-certification", "--test", "s8_lsm_lookup_authority"
    )
    Invoke-Checked "certification owner topology" @(
        "test", "-p", "forge-store-certification", "--lib", "owner_topology"
    )
    Invoke-Checked "test support compilation" @(
        "test", "-p", "forge-store-test-support", "--no-run"
    )
    Invoke-Checked "physical format compilation" @(
        "test", "-p", "forge-store-physical-format", "--no-run"
    )

    if ($Workspace) {
        Invoke-Checked "Forge Store workspace check" @("check", "--workspace")
    }
}
finally {
    Pop-Location
}
