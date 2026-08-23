[CmdletBinding()]
param(
    [string]$Root = ".",
    [string]$ManifestPath
)

$ErrorActionPreference = "Stop"

function Invoke-GitStatus {
    param([string]$RepositoryRoot)

    $startInfo = [System.Diagnostics.ProcessStartInfo]::new()
    $startInfo.FileName = "git"
    $startInfo.WorkingDirectory = $RepositoryRoot
    $startInfo.UseShellExecute = $false
    $startInfo.RedirectStandardOutput = $true
    $startInfo.RedirectStandardError = $true
    [void]$startInfo.ArgumentList.Add("status")
    [void]$startInfo.ArgumentList.Add("--porcelain=v1")
    [void]$startInfo.ArgumentList.Add("-z")
    [void]$startInfo.ArgumentList.Add("--untracked-files=all")

    $process = [System.Diagnostics.Process]::new()
    $process.StartInfo = $startInfo
    [void]$process.Start()
    $stdout = $process.StandardOutput.ReadToEnd()
    $stderr = $process.StandardError.ReadToEnd()
    $process.WaitForExit()
    if ($process.ExitCode -ne 0) {
        throw "git status failed: $stderr"
    }
    return $stdout
}

function Normalize-RepositoryPath {
    param([string]$Path)

    $normalized = $Path.Replace("\", "/").Normalize(
        [System.Text.NormalizationForm]::FormC
    )
    if ($normalized.IndexOfAny([char[]]@("`t", "`r", "`n", [char]0)) -ge 0) {
        throw "dirty path cannot be represented in the manifest: $Path"
    }
    return $normalized
}

function Add-RustPath {
    param(
        [System.Collections.Generic.Dictionary[string, string]]$Paths,
        [string]$Path
    )

    $normalized = Normalize-RepositoryPath $Path
    if (-not $normalized.EndsWith(".rs", [System.StringComparison]::Ordinal)) {
        return
    }
    if ($Paths.ContainsKey($normalized) -and $Paths[$normalized] -ne $Path) {
        throw "NFC path collision: '$Path' and '$($Paths[$normalized])'"
    }
    $Paths[$normalized] = $Path
}

$repositoryRoot = (Resolve-Path -LiteralPath $Root).Path
$statusFields = (Invoke-GitStatus $repositoryRoot).Split([char]0)
$paths = [System.Collections.Generic.Dictionary[string, string]]::new(
    [System.StringComparer]::Ordinal
)

for ($index = 0; $index -lt $statusFields.Length; $index++) {
    $field = $statusFields[$index]
    if ([string]::IsNullOrEmpty($field)) {
        continue
    }
    if ($field.Length -lt 4 -or $field[2] -ne " ") {
        throw "unexpected porcelain status entry: $field"
    }

    $status = $field.Substring(0, 2)
    $currentPath = $field.Substring(3)
    Add-RustPath $paths $currentPath

    if ($status.Contains("R") -or $status.Contains("C")) {
        $index++
        if ($index -ge $statusFields.Length) {
            throw "rename/copy status omitted its source path"
        }
        if ($status.Contains("R")) {
            Add-RustPath $paths $statusFields[$index]
        }
    }
}

$utf8 = [System.Text.UTF8Encoding]::new($false)
$sha256 = [System.Security.Cryptography.SHA256]::Create()
$entriesBySortKey = [System.Collections.Generic.Dictionary[string, string]]::new(
    [System.StringComparer]::Ordinal
)

foreach ($normalizedPath in $paths.Keys) {
    $sortKey = [Convert]::ToHexString($utf8.GetBytes($normalizedPath))
    $workingPath = Join-Path $repositoryRoot $paths[$normalizedPath]
    if (Test-Path -LiteralPath $workingPath -PathType Leaf) {
        $fileBytes = [System.IO.File]::ReadAllBytes($workingPath)
        $fileHash = [Convert]::ToHexString($sha256.ComputeHash($fileBytes)).ToLowerInvariant()
        $entry = "FILE`t$normalizedPath`t$fileHash"
    }
    else {
        $entry = "DELETE`t$normalizedPath"
    }
    if ($entriesBySortKey.ContainsKey($sortKey)) {
        throw "UTF-8 path collision for '$normalizedPath'"
    }
    $entriesBySortKey[$sortKey] = $entry
}

$sortKeys = [string[]]$entriesBySortKey.Keys
[Array]::Sort($sortKeys, [System.StringComparer]::Ordinal)
$manifest = [string]::Join("`n", ($sortKeys | ForEach-Object { $entriesBySortKey[$_] }))
$manifestBytes = $utf8.GetBytes($manifest)
$manifestHash = [Convert]::ToHexString($sha256.ComputeHash($manifestBytes)).ToLowerInvariant()

if ($ManifestPath) {
    $resolvedManifestPath = [System.IO.Path]::GetFullPath($ManifestPath, $repositoryRoot)
    [System.IO.File]::WriteAllBytes($resolvedManifestPath, $manifestBytes)
}

$fileCount = ($entriesBySortKey.Values | Where-Object { $_.StartsWith("FILE`t") }).Count
$deletionCount = $entriesBySortKey.Count - $fileCount
[pscustomobject]@{
    Sha256 = $manifestHash
    Entries = $entriesBySortKey.Count
    Files = $fileCount
    Deletions = $deletionCount
    Rule = "NFC slash paths; UTF-8 byte order; raw file SHA-256; LF rows; no trailing LF"
}
