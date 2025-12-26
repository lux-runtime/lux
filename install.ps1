# Lux Runtime Installer
# PowerShell script to download and install Lux and LPM
# Run: irm https://raw.githubusercontent.com/lux-runtime/lux/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

$LuxRepo = "lux-runtime/lux"
$LpmRepo = "lux-runtime/lpm"
$InstallDir = "$env:USERPROFILE\.lux\bin"

Write-Host ""
Write-Host "  _               " -ForegroundColor Blue
Write-Host " | |    _   ___  __" -ForegroundColor Blue
Write-Host " | |   | | | \ \/ /" -ForegroundColor Blue
Write-Host " | |___| |_| |>  < " -ForegroundColor Blue
Write-Host " |______\__,_/_/\_\" -ForegroundColor Blue
Write-Host ""
Write-Host " Lux Runtime Installer" -ForegroundColor Cyan
Write-Host ""

function Get-LatestRelease {
    param([string]$Repo)
    $url = "https://api.github.com/repos/$Repo/releases/latest"
    try {
        $response = Invoke-RestMethod -Uri $url -Headers @{ "User-Agent" = "Lux-Installer" }
        return $response
    } catch {
        Write-Host "Error fetching release info from $Repo" -ForegroundColor Red
        return $null
    }
}

function Get-AssetUrl {
    param($Release, [string]$Pattern)
    foreach ($asset in $Release.assets) {
        if ($asset.name -match $Pattern) {
            return $asset.browser_download_url
        }
    }
    return $null
}

function Download-Binary {
    param([string]$Url, [string]$Dest)
    Write-Host "  Downloading from $Url..." -ForegroundColor Gray
    Invoke-WebRequest -Uri $Url -OutFile $Dest -UseBasicParsing
}

# Create install directory
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    Write-Host "[+] Created $InstallDir" -ForegroundColor Green
}

# Download Lux
Write-Host ""
Write-Host "[*] Fetching Lux Runtime..." -ForegroundColor Yellow
$luxRelease = Get-LatestRelease -Repo $LuxRepo

if ($luxRelease) {
    $luxUrl = Get-AssetUrl -Release $luxRelease -Pattern "lux.*windows.*\.exe$|lux-windows.*\.exe$|lux\.exe$"
    if (-not $luxUrl) {
        # Try zip
        $luxUrl = Get-AssetUrl -Release $luxRelease -Pattern "lux.*windows.*\.zip$"
    }
    
    if ($luxUrl) {
        $luxPath = "$InstallDir\lux.exe"
        if ($luxUrl -match "\.zip$") {
            $tempZip = "$env:TEMP\lux.zip"
            Download-Binary -Url $luxUrl -Dest $tempZip
            Expand-Archive -Path $tempZip -DestinationPath $InstallDir -Force
            Remove-Item $tempZip -Force
        } else {
            Download-Binary -Url $luxUrl -Dest $luxPath
        }
        Write-Host "[+] Lux installed: $luxPath" -ForegroundColor Green
    } else {
        Write-Host "[-] Could not find Lux binary in release" -ForegroundColor Red
        Write-Host "    You may need to build from source:" -ForegroundColor Gray
        Write-Host "    git clone https://github.com/lux-runtime/lux" -ForegroundColor Gray
        Write-Host "    cd lux/lux-runtime && cargo build --release" -ForegroundColor Gray
    }
} else {
    Write-Host "[-] No releases found. Build from source:" -ForegroundColor Yellow
    Write-Host "    git clone https://github.com/lux-runtime/lux" -ForegroundColor Gray
}

# Download LPM
Write-Host ""
Write-Host "[*] Fetching LPM (Package Manager)..." -ForegroundColor Yellow
$lpmRelease = Get-LatestRelease -Repo $LpmRepo

if ($lpmRelease) {
    $lpmUrl = Get-AssetUrl -Release $lpmRelease -Pattern "lpm.*windows.*\.exe$|lpm-windows.*\.exe$|lpm\.exe$"
    if (-not $lpmUrl) {
        $lpmUrl = Get-AssetUrl -Release $lpmRelease -Pattern "lpm.*windows.*\.zip$"
    }
    
    if ($lpmUrl) {
        $lpmPath = "$InstallDir\lpm.exe"
        if ($lpmUrl -match "\.zip$") {
            $tempZip = "$env:TEMP\lpm.zip"
            Download-Binary -Url $lpmUrl -Dest $tempZip
            Expand-Archive -Path $tempZip -DestinationPath $InstallDir -Force
            Remove-Item $tempZip -Force
        } else {
            Download-Binary -Url $lpmUrl -Dest $lpmPath
        }
        Write-Host "[+] LPM installed: $lpmPath" -ForegroundColor Green
    } else {
        Write-Host "[-] Could not find LPM binary in release" -ForegroundColor Red
        Write-Host "    Build from source:" -ForegroundColor Gray
        Write-Host "    git clone https://github.com/lux-runtime/lpm" -ForegroundColor Gray
        Write-Host "    cd lpm && cargo build --release" -ForegroundColor Gray
    }
} else {
    Write-Host "[-] No LPM releases found. Build from source:" -ForegroundColor Yellow
    Write-Host "    git clone https://github.com/lux-runtime/lpm" -ForegroundColor Gray
}

# Add to PATH
Write-Host ""
Write-Host "[*] Configuring PATH..." -ForegroundColor Yellow

$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$currentPath;$InstallDir", "User")
    Write-Host "[+] Added $InstallDir to PATH" -ForegroundColor Green
    Write-Host "    Restart your terminal for changes to take effect" -ForegroundColor Gray
} else {
    Write-Host "[=] PATH already configured" -ForegroundColor Gray
}

# Verify
Write-Host ""
Write-Host "[*] Verifying installation..." -ForegroundColor Yellow

if (Test-Path "$InstallDir\lux.exe") {
    $version = & "$InstallDir\lux.exe" --version 2>&1
    Write-Host "[+] Lux: $version" -ForegroundColor Green
}

if (Test-Path "$InstallDir\lpm.exe") {
    $version = & "$InstallDir\lpm.exe" --version 2>&1
    Write-Host "[+] LPM: $version" -ForegroundColor Green
}

Write-Host ""
Write-Host "Installation complete!" -ForegroundColor Cyan
Write-Host ""
Write-Host "Quick start:" -ForegroundColor White
Write-Host "  lpm init        # Create a new project" -ForegroundColor Gray
Write-Host "  lpm run dev     # Run your project" -ForegroundColor Gray
Write-Host ""
Write-Host "Documentation: https://lux-runtime.github.io/docs/" -ForegroundColor Blue
Write-Host "Discord: https://discord.gg/QfU7rweBCC" -ForegroundColor Blue
Write-Host ""
