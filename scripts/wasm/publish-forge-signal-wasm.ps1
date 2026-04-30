param(
    [Parameter(Mandatory = $false)]
    [string]$Scope = $env:FORGE_SIGNAL_WASM_SCOPE,

    [Parameter(Mandatory = $false)]
    [string]$PackageName = $env:FORGE_SIGNAL_WASM_PACKAGE_NAME,

    [Parameter(Mandatory = $false)]
    [string]$RepositoryUrl = $env:FORGE_SIGNAL_WASM_REPOSITORY_URL,

    [Parameter(Mandatory = $false)]
    [string]$Registry = $(if ($env:FORGE_SIGNAL_WASM_REGISTRY) { $env:FORGE_SIGNAL_WASM_REGISTRY } else { "https://registry.npmjs.org" }),

    [Parameter(Mandatory = $false)]
    [string]$Access = $(if ($env:FORGE_SIGNAL_WASM_PUBLISH_ACCESS) { $env:FORGE_SIGNAL_WASM_PUBLISH_ACCESS } else { "public" }),

    [Parameter(Mandatory = $false)]
    [string]$NoticeMode = $(if ($env:FORGE_SIGNAL_WASM_NOTICE_MODE) { $env:FORGE_SIGNAL_WASM_NOTICE_MODE } else { "none" }),

    [Parameter(Mandatory = $false)]
    [string]$CrateDir = "crates/forge-signal-wasm",

    [Parameter(Mandatory = $false)]
    [string]$OutDir = "pkg",

    [Parameter(Mandatory = $false)]
    [switch]$SkipVerify,

    [Parameter(Mandatory = $false)]
    [switch]$SkipPublish
)

$ErrorActionPreference = "Stop"

if (-not $RepositoryUrl) {
    $RepositoryUrl = "https://github.com/recnepspencer/forge.git"
}

$packageNameWasExplicit =
    $PSBoundParameters.ContainsKey("PackageName") -or
    [bool]$env:FORGE_SIGNAL_WASM_PACKAGE_NAME

$scopeWasExplicit =
    $PSBoundParameters.ContainsKey("Scope") -or
    [bool]$env:FORGE_SIGNAL_WASM_SCOPE

if (-not $packageNameWasExplicit) {
    if (-not $scopeWasExplicit) {
        $PackageName = "forge-signal-wasm"
    }
    else {
        $PackageName = $null
    }
}

$cratePath = Resolve-Path $CrateDir

wasm-pack build $cratePath --target bundler --release --out-dir $OutDir

$pkgPath = Join-Path $cratePath $OutDir
$env:FORGE_SIGNAL_WASM_SCOPE = $Scope
$env:FORGE_SIGNAL_WASM_PACKAGE_NAME = $PackageName
$env:FORGE_SIGNAL_WASM_REPOSITORY_URL = $RepositoryUrl
$env:FORGE_SIGNAL_WASM_REGISTRY = $Registry
$env:FORGE_SIGNAL_WASM_PUBLISH_ACCESS = $Access
$env:FORGE_SIGNAL_WASM_NOTICE_MODE = $NoticeMode
node scripts/wasm/prepare-forge-signal-wasm-package.mjs $pkgPath

if (-not $SkipVerify) {
    node scripts/wasm/verify-forge-signal-wasm-package.mjs $pkgPath
}

Push-Location $pkgPath
try {
    if ($SkipPublish) {
        Write-Host "Skipping npm publish; package built, prepared, and verified at $pkgPath"
    }
    elseif ($Access) {
        npm publish --access $Access
    }
    else {
        npm publish
    }
}
finally {
    Pop-Location
}
