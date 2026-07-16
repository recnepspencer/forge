param(
    [string]$ToolCache
)

$ErrorActionPreference = "Stop"
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "../..")).Path
$crateRoot = Join-Path $repoRoot "workspaces/worth-store/crates/worth-store-formal-models"
$certificationRoot = Join-Path $repoRoot "workspaces/worth-store/crates/worth-store-certification"
$toolchainPath = Join-Path $crateRoot "formal-toolchain.toml"
$toolchain = Get-Content -Raw -LiteralPath $toolchainPath

function Read-ToolchainValue([string]$Name) {
    $match = [regex]::Match($toolchain, "(?m)^$Name\s*=\s*`"([^`"]+)`"$")
    if (-not $match.Success) {
        throw "missing $Name in $toolchainPath"
    }
    return $match.Groups[1].Value
}

$version = Read-ToolchainValue "version"
$downloadUrl = Read-ToolchainValue "download_url"
$expectedSha256 = Read-ToolchainValue "sha256"
$mainClass = Read-ToolchainValue "main_class"
$modelRelative = Read-ToolchainValue "model"
$configurationRelative = Read-ToolchainValue "configuration"

if (-not $ToolCache) {
    $ToolCache = Join-Path $repoRoot "workspaces/worth-store/target/formal-tools"
}
New-Item -ItemType Directory -Force -Path $ToolCache | Out-Null
$jarPath = Join-Path $ToolCache "tla2tools-$version.jar"

if (-not (Test-Path -LiteralPath $jarPath)) {
    Invoke-WebRequest -Uri $downloadUrl -OutFile $jarPath
}

$actualSha256 = (Get-FileHash -Algorithm SHA256 -LiteralPath $jarPath).Hash.ToLowerInvariant()
if ($actualSha256 -ne $expectedSha256) {
    throw "TLC digest mismatch: expected $expectedSha256, got $actualSha256"
}

$java = Get-Command java -ErrorAction SilentlyContinue
if (-not $java) {
    throw "Java is required to run pinned TLC $version; install a JDK and put java on PATH"
}

function Invoke-CheckedModel(
    [string]$ModelPath,
    [string]$ConfigurationPath,
    [string]$StateName
) {
    if (-not (Test-Path -LiteralPath $ConfigurationPath)) {
        throw "missing TLC configuration for $ModelPath"
    }
    $stateDirectory = Join-Path $ToolCache "states/$StateName"
    New-Item -ItemType Directory -Force -Path $stateDirectory | Out-Null
    & $java.Source -cp $jarPath $mainClass -deadlock -workers auto -metadir $stateDirectory -config $ConfigurationPath $ModelPath
    if ($LASTEXITCODE -ne 0) {
        throw "pinned TLC $version rejected $ModelPath"
    }
}

$modelPath = Join-Path $crateRoot $modelRelative
$configurationPath = Join-Path $crateRoot $configurationRelative
Invoke-CheckedModel $modelPath $configurationPath "toolchain-smoke"

$protocolRoot = Join-Path $crateRoot "src/protocols"
$protocolModels = Get-ChildItem -LiteralPath $protocolRoot -Recurse -File -Filter "*.tla"
& cargo run --quiet --manifest-path (Join-Path $certificationRoot "Cargo.toml") `
    --bin worth_store_protocol_closeout -- `
    $java.Source $jarPath (Join-Path $ToolCache "states")
if ($LASTEXITCODE -ne 0) {
    throw "protocol closeout did not check legal models, reject mutants, and bind owner evidence"
}

Write-Output "verified TLC $version ($actualSha256) across $($protocolModels.Count) protocol models"
