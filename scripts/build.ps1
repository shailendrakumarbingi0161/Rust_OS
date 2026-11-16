# PowerShell script to build the OS ISO using the Docker container
# Usage: .\scripts\build.ps1

# Determine repository root (one level up from the scripts folder)
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Split-Path -Parent $scriptDir
Set-Location $repoRoot

# Build Docker image (if not already built)
Write-Host "Building Docker image rustos-builder..."
docker build -t rustos-builder .

# Run the build inside the container, mounting the repo at /src
Write-Host "Running build inside container..."
# The container expects the source at /src and will execute the script /src/scripts/build.sh
docker run --rm -v "${repoRoot}:/src" rustos-builder /bin/bash -c "/src/scripts/build.sh"

Write-Host "Build completed. ISO is at build/rustos.iso"
