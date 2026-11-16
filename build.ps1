# PowerShell script to build the OS ISO using the Docker container
# Usage: .\build.ps1

$repoRoot = $PSScriptRoot
Set-Location $repoRoot

# Build Docker image
Write-Host "Building Docker image rustos-builder..."
docker build -t rustos-builder .

# Run the build inside the container, mounting the repo at /src
Write-Host "Running build inside container..."
docker run --rm -v "${repoRoot}:/src" rustos-builder /bin/bash -c "/src/scripts/build.sh"

Write-Host "Build completed. ISO is at build/rustos.iso"
