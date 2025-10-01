# VedDB Rust Client Installation Script for Windows

param(
    [string]$InstallPath = "$env:ProgramFiles\VedDB\Client",
    [switch]$AddToPath = $true
)

Write-Host "`n=== VedDB Rust Client Installation ===" -ForegroundColor Cyan
Write-Host ""

# Check if running as administrator
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "Warning: Not running as administrator. Installing for current user only." -ForegroundColor Yellow
    $InstallPath = "$env:LOCALAPPDATA\VedDB\Client"
}

# Step 1: Build the client
Write-Host "[1/4] Building VedDB client..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "  Build successful" -ForegroundColor Green

# Step 2: Create installation directory
Write-Host "`n[2/4] Creating installation directory..." -ForegroundColor Yellow
if (-not (Test-Path $InstallPath)) {
    New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
}
Write-Host "  Installation path: $InstallPath" -ForegroundColor Green

# Step 3: Copy binaries and libraries
Write-Host "`n[3/4] Installing binaries..." -ForegroundColor Yellow
if (Test-Path "target\release\veddb-cli.exe") {
    Copy-Item "target\release\veddb-cli.exe" -Destination "$InstallPath\" -Force
    Write-Host "  Copied veddb-cli.exe" -ForegroundColor Green
}
if (Test-Path "target\release\veddb_client.dll") {
    Copy-Item "target\release\veddb_client.dll" -Destination "$InstallPath\" -Force
    Write-Host "  Copied veddb_client.dll" -ForegroundColor Green
}
if (Test-Path "target\release\veddb_client.lib") {
    Copy-Item "target\release\veddb_client.lib" -Destination "$InstallPath\" -Force
    Write-Host "  Copied veddb_client.lib" -ForegroundColor Green
}

# Step 4: Set environment variables
Write-Host "`n[4/4] Setting environment variables..." -ForegroundColor Yellow

$scope = if ($isAdmin) { "Machine" } else { "User" }

# Set VEDDB_CLIENT_HOME
[Environment]::SetEnvironmentVariable("VEDDB_CLIENT_HOME", $InstallPath, $scope)
Write-Host "  VEDDB_CLIENT_HOME = $InstallPath" -ForegroundColor Green

# Set VEDDB_CLIENT_VERSION
$version = "0.1.1"
[Environment]::SetEnvironmentVariable("VEDDB_CLIENT_VERSION", $version, $scope)
Write-Host "  VEDDB_CLIENT_VERSION = $version" -ForegroundColor Green

# Add to PATH
if ($AddToPath) {
    $currentPath = [Environment]::GetEnvironmentVariable("Path", $scope)
    if ($currentPath -notlike "*$InstallPath*") {
        $newPath = "$currentPath;$InstallPath"
        [Environment]::SetEnvironmentVariable("Path", $newPath, $scope)
        Write-Host "  Added to PATH" -ForegroundColor Green
    } else {
        Write-Host "  Already in PATH" -ForegroundColor Yellow
    }
}

# Create uninstall script
$uninstallScript = @"
# VedDB Client Uninstall Script
Write-Host "Uninstalling VedDB Client..." -ForegroundColor Yellow

# Remove installation directory
if (Test-Path "$InstallPath") {
    Remove-Item -Path "$InstallPath" -Recurse -Force
    Write-Host "Removed installation directory" -ForegroundColor Green
}

# Remove environment variables
`$scope = "$scope"
[Environment]::SetEnvironmentVariable("VEDDB_CLIENT_HOME", `$null, `$scope)
[Environment]::SetEnvironmentVariable("VEDDB_CLIENT_VERSION", `$null, `$scope)

# Remove from PATH
`$currentPath = [Environment]::GetEnvironmentVariable("Path", `$scope)
`$newPath = `$currentPath -replace [regex]::Escape("$InstallPath;?"), ""
[Environment]::SetEnvironmentVariable("Path", `$newPath, `$scope)

Write-Host "VedDB Client uninstalled successfully" -ForegroundColor Green
"@

Set-Content -Path "$InstallPath\uninstall.ps1" -Value $uninstallScript

# Summary
Write-Host "`n=== Installation Complete ===" -ForegroundColor Green
Write-Host ""
Write-Host "VedDB Client has been installed to: $InstallPath" -ForegroundColor Cyan
Write-Host ""
Write-Host "Environment variables set:" -ForegroundColor Yellow
Write-Host "  VEDDB_CLIENT_HOME    = $InstallPath"
Write-Host "  VEDDB_CLIENT_VERSION = $version"
Write-Host ""
Write-Host "To use the client, open a NEW terminal and run:" -ForegroundColor Yellow
Write-Host "  veddb-cli --help" -ForegroundColor White
Write-Host ""
Write-Host "To uninstall, run:" -ForegroundColor Yellow
Write-Host "  $InstallPath\uninstall.ps1" -ForegroundColor White
Write-Host ""
