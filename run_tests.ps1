# VedDB Client Test Runner
# PowerShell script to run all tests and examples

Write-Host "🚀 VedDB Client Test Runner" -ForegroundColor Green
Write-Host "=============================" -ForegroundColor Green

# Check if VedDB server is running
Write-Host "`n📡 Checking VedDB server connection..." -ForegroundColor Yellow
$serverRunning = $false
try {
    $connection = New-Object System.Net.Sockets.TcpClient
    $connection.Connect("127.0.0.1", 50051)
    $connection.Close()
    $serverRunning = $true
    Write-Host "   ✅ VedDB server is running" -ForegroundColor Green
} catch {
    Write-Host "   ❌ VedDB server is not running on 127.0.0.1:50051" -ForegroundColor Red
    Write-Host "   Please start the VedDB server first:" -ForegroundColor Yellow
    Write-Host "   cd ../veddb-server && cargo run --release" -ForegroundColor Cyan
    exit 1
}

# Build the project
Write-Host "`n🔨 Building VedDB client..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "   ❌ Build failed" -ForegroundColor Red
    exit 1
}
Write-Host "   ✅ Build successful" -ForegroundColor Green

# Run unit tests
Write-Host "`n🧪 Running unit tests..." -ForegroundColor Yellow
cargo test --lib
if ($LASTEXITCODE -ne 0) {
    Write-Host "   ❌ Unit tests failed" -ForegroundColor Red
    exit 1
}
Write-Host "   ✅ Unit tests passed" -ForegroundColor Green

# Run integration tests (only if server is running)
if ($serverRunning) {
    Write-Host "`n🔗 Running integration tests..." -ForegroundColor Yellow
    cargo test --test integration_test
    if ($LASTEXITCODE -ne 0) {
        Write-Host "   ⚠️  Integration tests failed (server might not be compatible)" -ForegroundColor Yellow
    } else {
        Write-Host "   ✅ Integration tests passed" -ForegroundColor Green
    }
}

# Run examples
Write-Host "`n📚 Running examples..." -ForegroundColor Yellow

Write-Host "   Running basic usage example..." -ForegroundColor Cyan
cargo run --example basic_usage
if ($LASTEXITCODE -eq 0) {
    Write-Host "   ✅ Basic usage example completed" -ForegroundColor Green
} else {
    Write-Host "   ⚠️  Basic usage example failed" -ForegroundColor Yellow
}

Write-Host "   Running comprehensive test script..." -ForegroundColor Cyan
cargo run --example test_script
if ($LASTEXITCODE -eq 0) {
    Write-Host "   ✅ Test script completed" -ForegroundColor Green
} else {
    Write-Host "   ⚠️  Test script failed" -ForegroundColor Yellow
}

Write-Host "   Running connection pooling example..." -ForegroundColor Cyan
cargo run --example pooling
if ($LASTEXITCODE -eq 0) {
    Write-Host "   ✅ Pooling example completed" -ForegroundColor Green
} else {
    Write-Host "   ⚠️  Pooling example failed" -ForegroundColor Yellow
}

# Run benchmarks
Write-Host "`n📊 Running benchmarks..." -ForegroundColor Yellow
cargo bench
if ($LASTEXITCODE -eq 0) {
    Write-Host "   ✅ Benchmarks completed" -ForegroundColor Green
    Write-Host "   📈 Check target/criterion/report/index.html for detailed results" -ForegroundColor Cyan
} else {
    Write-Host "   ⚠️  Benchmarks failed" -ForegroundColor Yellow
}

Write-Host "`n🎉 Test runner completed!" -ForegroundColor Green
Write-Host "📖 Check the README.md for more usage examples" -ForegroundColor Cyan
