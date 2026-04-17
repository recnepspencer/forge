param(
    [Parameter(Mandatory = $false)]
    [string]$Scope = $(if ($env:FORGE_SIGNAL_WASM_SCOPE) { $env:FORGE_SIGNAL_WASM_SCOPE } else { "aust-group" }),

    [Parameter(Mandatory = $false)]
    [string]$RepositoryUrl = $env:FORGE_SIGNAL_WASM_REPOSITORY_URL,

    [Parameter(Mandatory = $false)]
    [string]$CrateDir = "crates/forge-signal-wasm",

    [Parameter(Mandatory = $false)]
    [string]$OutDir = "pkg"
)

$ErrorActionPreference = "Stop"

if (-not $Scope) {
    throw "Set FORGE_SIGNAL_WASM_SCOPE or pass -Scope with your GitHub username or org."
}

if (-not $RepositoryUrl) {
    $RepositoryUrl = "https://github.com/AuST-Group/forge.git"
}

if (-not $env:NODE_AUTH_TOKEN) {
    throw "Set NODE_AUTH_TOKEN to a GitHub personal access token (classic) with write:packages."
}

$cratePath = Resolve-Path $CrateDir

wasm-pack build $cratePath --target bundler --release --out-dir $OutDir

$pkgPath = Join-Path $cratePath $OutDir
$env:FORGE_SIGNAL_WASM_SCOPE = $Scope
$env:FORGE_SIGNAL_WASM_REPOSITORY_URL = $RepositoryUrl
node scripts/wasm/prepare-forge-signal-wasm-package.mjs $pkgPath

Push-Location $pkgPath
try {
    npm publish
}
finally {
    Pop-Location
}
