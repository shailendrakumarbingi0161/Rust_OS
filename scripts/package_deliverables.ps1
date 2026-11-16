# Package Deliverables Script
# This PowerShell script creates the final deliverables package.
# It assumes the ISO has been built (build\rustos.iso) and that any screenshots have been placed in docs\screenshots\.
# It also expects deadlock_results.txt to exist in the in‑memory FS after running the lab; the user should copy it to results\.

$repoRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$deliverDir = Join-Path $repoRoot "deliverables"

# Ensure the deliverables directory exists
if (-Not (Test-Path $deliverDir)) { New-Item -ItemType Directory -Path $deliverDir | Out-Null }

# 1. Copy ISO
Copy-Item -Path "build\rustos.iso" -Destination $deliverDir -Force

# 2. Create source archive
$zipPath = Join-Path $deliverDir "source.zip"
if (Test-Path $zipPath) { Remove-Item $zipPath -Force }
Compress-Archive -Path "$repoRoot\*" -DestinationPath $zipPath -Force

# 3. Copy docs folder
Copy-Item -Path "docs" -Destination $deliverDir -Recurse -Force

# 4. Copy screenshots (user should have placed them in docs\screenshots)
Copy-Item -Path "docs\screenshots" -Destination $deliverDir -Recurse -Force

# 5. Copy results (deadlock_results.txt) – user should have exported it from the OS
Copy-Item -Path "results" -Destination $deliverDir -Recurse -Force

Write-Host "Deliverables packaged in $deliverDir" -ForegroundColor Green
