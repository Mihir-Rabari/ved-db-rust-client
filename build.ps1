# Build script for VedDB Rust Client (Windows)

Write-Host "Building VedDB Rust Client..." -ForegroundColor Green
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to build client" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Build complete!" -ForegroundColor Green
Write-Host "Client library: target\release\" -ForegroundColor Cyan
Write-Host "CLI tool: target\release\veddb-cli.exe" -ForegroundColor Cyan
