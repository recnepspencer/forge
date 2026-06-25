$ErrorActionPreference = "Stop"

function Invoke-Step {
    param(
        [Parameter(Mandatory = $true)]
        [string] $Label,

        [Parameter(Mandatory = $true)]
        [string[]] $Command
    )

    Write-Host ""
    Write-Host "==> $Label"
    Write-Host ($Command -join " ")

    $Executable = $Command[0]
    $Arguments = @()
    if ($Command.Length -gt 1) {
        $Arguments = $Command[1..($Command.Length - 1)]
    }

    $PreviousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    try {
        & $Executable @Arguments
    } finally {
        $ErrorActionPreference = $PreviousErrorActionPreference
    }
    if ($LASTEXITCODE -ne 0) {
        throw "$Label failed with exit code $LASTEXITCODE"
    }
}

Invoke-Step "forge-query compile/check gate" @(
    "cargo", "check", "-p", "forge-query", "--tests", "--message-format", "short"
)
Invoke-Step "forge-query test compilation gate" @(
    "cargo", "test", "-p", "forge-query", "--tests", "--no-run", "--message-format", "short"
)
Invoke-Step "forge-query fast unit gate" @(
    "cargo", "test", "-p", "forge-query", "--lib", "--", "--format", "terse"
)

Invoke-Step "worth-spatial compile/check gate" @(
    "cargo", "check", "-p", "worth-spatial", "--tests", "--message-format", "short"
)
Invoke-Step "worth-spatial test compilation gate" @(
    "cargo", "test", "-p", "worth-spatial", "--tests", "--no-run", "--message-format", "short"
)
Invoke-Step "worth-spatial fast unit gate" @(
    "cargo", "test", "-p", "worth-spatial", "--lib", "--", "--format", "terse"
)
Invoke-Step "worth-spatial compile-fail boundary gate" @(
    "cargo", "test", "-p", "worth-spatial", "--test", "ui", "--", "--format", "terse"
)

Write-Host ""
Write-Host "Worth fast iteration lane passed."
