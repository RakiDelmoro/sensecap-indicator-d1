# Check WSL Status PowerShell Script
# Run this in PowerShell to check your WSL installation

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  WSL Installation Checker" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check if WSL is installed
$wslInstalled = Get-Command wsl -ErrorAction SilentlyContinue

if (-not $wslInstalled) {
    Write-Host "❌ WSL is NOT installed on your system" -ForegroundColor Red
    Write-Host ""
    Write-Host "To install WSL2, run the following in PowerShell as Administrator:" -ForegroundColor Yellow
    Write-Host "    wsl --install" -ForegroundColor Green
    Write-Host ""
    Write-Host "Then restart your computer and run this script again." -ForegroundColor Yellow
    exit 1
}

Write-Host "✅ WSL is installed" -ForegroundColor Green
Write-Host ""

# Check WSL version
Write-Host "Checking WSL version..." -ForegroundColor Cyan
$wslVersion = wsl --version 2>&1
Write-Host $wslVersion
Write-Host ""

# Check WSL status
Write-Host "Checking WSL status..." -ForegroundColor Cyan
$wslStatus = wsl --status 2>&1
Write-Host $wslStatus
Write-Host ""

# List installed distributions
Write-Host "Installed WSL distributions:" -ForegroundColor Cyan
$distros = wsl --list --verbose 2>&1
if ($distros -match "NAME") {
    Write-Host $distros
} else {
    Write-Host "  No distributions found" -ForegroundColor Yellow
}
Write-Host ""

# Check if Ubuntu is installed
$ubuntuInstalled = wsl --list | Select-String -Pattern "Ubuntu" -Quiet
if ($ubuntuInstalled) {
    Write-Host "✅ Ubuntu is installed" -ForegroundColor Green
} else {
    Write-Host "⚠️  Ubuntu not found. You can install it with:" -ForegroundColor Yellow
    Write-Host "    wsl --install -d Ubuntu" -ForegroundColor Green
}
Write-Host ""

# Check if WSL2 is default version
$defaultVersion = wsl --status | Select-String -Pattern "Default Version: (\d)" | ForEach-Object { $_.Matches.Groups[1].Value }
if ($defaultVersion -eq "2") {
    Write-Host "✅ WSL2 is the default version" -ForegroundColor Green
} else {
    Write-Host "⚠️  WSL1 is the default. To set WSL2 as default:" -ForegroundColor Yellow
    Write-Host "    wsl --set-default-version 2" -ForegroundColor Green
}
Write-Host ""

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Next Steps:" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
if ($ubuntuInstalled -and $defaultVersion -eq "2") {
    Write-Host "✅ You're ready to go! Open Ubuntu in WSL2:" -ForegroundColor Green
    Write-Host "    wsl" -ForegroundColor Green
    Write-Host ""
    Write-Host "Then navigate to your simulator folder and run:" -ForegroundColor Yellow
    Write-Host "    ./build.sh" -ForegroundColor Green
} else {
    Write-Host "Please complete the installation steps above, then run this script again." -ForegroundColor Yellow
}
