$ErrorActionPreference = "Stop"

Write-Host ""
Write-Host " ▗▄▄▖ ▗▄▖ ▗▖ ▗▖▗▖  ▗▖▗▄▄▄   ▗▄▄▖" -ForegroundColor Cyan
Write-Host "▐▌   ▐▌ ▐▌▐▌ ▐▌▐▛▚▖▐▌▐▌  █ ▐▌   " -ForegroundColor Cyan
Write-Host " ▝▀▚▖▐▌ ▐▌▐▌ ▐▌▐▌ ▝▜▌▐▌  █  ▝▀▚▖" -ForegroundColor Cyan
Write-Host "▗▄▄▞▘▝▚▄▞▘▝▚▄▞▘▐▌  ▐▌▐▙▄▄▀ ▗▄▄▞▘" -ForegroundColor Cyan
Write-Host ""

$arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else {
    Write-Host "  ✖ Unsupported architecture" -ForegroundColor Red
    exit 1
}

$target = "windows-$arch"
$repo = "alhaymex/sounds"
$binaryName = "sounds.exe"

Write-Host "  ▸ Fetching latest release for $target..." -ForegroundColor Cyan
$release = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases/latest"
$version = $release.tag_name

if (-not $version) {
    Write-Host "  ✖ Could not fetch latest release tag." -ForegroundColor Red
    exit 1
}

Write-Host "  ✔ Found $version" -ForegroundColor Green

$downloadUrl = "https://github.com/$repo/releases/download/$version/sounds-$version-$target.zip"
$tmpDir = Join-Path $env:TEMP "sounds-install"
$zipPath = Join-Path $tmpDir "sounds.zip"

New-Item -ItemType Directory -Force -Path $tmpDir | Out-Null

Write-Host "  ▸ Downloading sounds $version..." -ForegroundColor Cyan
Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath -UseBasicParsing

Write-Host "  ✔ Downloaded" -ForegroundColor Green

Expand-Archive -Path $zipPath -DestinationPath $tmpDir -Force

$installDir = Join-Path $env:LOCALAPPDATA "sounds"
New-Item -ItemType Directory -Force -Path $installDir | Out-Null
Move-Item -Path (Join-Path $tmpDir $binaryName) -Destination (Join-Path $installDir $binaryName) -Force

# add to path
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
    Write-Host "  ▸ Added $installDir to PATH (restart your terminal)" -ForegroundColor Cyan
}

Remove-Item -Recurse -Force $tmpDir

Write-Host ""
Write-Host "  ✔ sounds $version installed successfully! 🎵" -ForegroundColor Green
Write-Host "    Run 'sounds' to get started." -ForegroundColor DarkGray
Write-Host ""
