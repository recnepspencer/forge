param(
    [Parameter(Mandatory = $false)]
    [string]$Scope = $env:WORTH_SIGNAL_WASM_SCOPE,

    [Parameter(Mandatory = $false)]
    [string]$PackageName = $env:WORTH_SIGNAL_WASM_PACKAGE_NAME,

    [Parameter(Mandatory = $false)]
    [string]$RepositoryUrl = $env:WORTH_SIGNAL_WASM_REPOSITORY_URL,

    [Parameter(Mandatory = $false)]
    [string]$Registry = $(if ($env:WORTH_SIGNAL_WASM_REGISTRY) { $env:WORTH_SIGNAL_WASM_REGISTRY } else { "https://registry.npmjs.org" }),

    [Parameter(Mandatory = $false)]
    [string]$Access = $(if ($env:WORTH_SIGNAL_WASM_PUBLISH_ACCESS) { $env:WORTH_SIGNAL_WASM_PUBLISH_ACCESS } else { "public" }),

    [Parameter(Mandatory = $false)]
    [string]$NoticeMode = $(if ($env:WORTH_SIGNAL_WASM_NOTICE_MODE) { $env:WORTH_SIGNAL_WASM_NOTICE_MODE } else { "none" }),

    [Parameter(Mandatory = $false)]
    [string]$CrateDir = "crates/worth-signal-wasm",

    [Parameter(Mandatory = $false)]
    [string]$OutDir = "pkg",

    [Parameter(Mandatory = $false)]
    [switch]$SkipVerify,

    [Parameter(Mandatory = $false)]
    [switch]$SkipPublish
)

$ErrorActionPreference = "Stop"

if (-not $RepositoryUrl) {
    $RepositoryUrl = "https://github.com/recnepspencer/WORTH.git"
}

$packageNameWasExplicit =
    $PSBoundParameters.ContainsKey("PackageName") -or
    [bool]$env:WORTH_SIGNAL_WASM_PACKAGE_NAME

$scopeWasExplicit =
    $PSBoundParameters.ContainsKey("Scope") -or
    [bool]$env:WORTH_SIGNAL_WASM_SCOPE

if (-not $packageNameWasExplicit) {
    if (-not $scopeWasExplicit) {
        $PackageName = "worth-signal-wasm"
    }
    else {
        $PackageName = $null
    }
}

$cratePath = Resolve-Path $CrateDir

wasm-pack build $cratePath --target bundler --release --out-dir $OutDir

$pkgPath = Join-Path $cratePath $OutDir
$env:WORTH_SIGNAL_WASM_SCOPE = $Scope
$env:WORTH_SIGNAL_WASM_PACKAGE_NAME = $PackageName
$env:WORTH_SIGNAL_WASM_REPOSITORY_URL = $RepositoryUrl
$env:WORTH_SIGNAL_WASM_REGISTRY = $Registry
$env:WORTH_SIGNAL_WASM_PUBLISH_ACCESS = $Access
$env:WORTH_SIGNAL_WASM_NOTICE_MODE = $NoticeMode
node scripts/wasm/prepare-worth-signal-wasm-package.mjs $pkgPath

if (-not $SkipVerify) {
    node scripts/wasm/verify-worth-signal-wasm-package.mjs $pkgPath
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
