# -------------------------------------------------
# Run RustOS ISO in VirtualBox (PowerShell helper)
# -------------------------------------------------

$vboxExePath = "C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"
if (-not (Test-Path $vboxExePath)) {
    # Check PATH
    $cmd = Get-Command VBoxManage.exe -ErrorAction SilentlyContinue
    if ($cmd) {
        $vboxExePath = $cmd.Source
    } else {
        Write-Error "VBoxManage.exe not found at $vboxExePath or in PATH."
        exit 1
    }
}

# Resolve paths
$scriptDir = $PSScriptRoot
if (-not $scriptDir) { $scriptDir = (Get-Location).Path }
$projectRoot = (Get-Item $scriptDir).FullName
if ((Get-Item $projectRoot).Name -eq "scripts") {
    $projectRoot = (Get-Item $projectRoot).Parent.FullName
}

$buildDir = Join-Path $projectRoot "build"
$isoPath  = Join-Path $buildDir "rustos.iso"
$uartLog  = Join-Path $buildDir "serial.log"

if (-not (Test-Path $isoPath)) {
    Write-Error "ISO not found at $isoPath. Please build it first using Docker."
    exit 1
}

# Ensure build dir and log file exist
if (-not (Test-Path $buildDir)) {
    New-Item -ItemType Directory -Path $buildDir -Force | Out-Null
}
if (-not (Test-Path $uartLog)) {
    New-Item -ItemType File -Path $uartLog -Force | Out-Null
}

$vmName = "RustOS"

# Check if VM already exists
$vms = & $vboxExePath list vms
$vmExists = $vms | Select-String -Pattern "`"$vmName`"" -SimpleMatch

if (-not $vmExists) {
    Write-Host "Creating Virtual Machine: $vmName ..."
    & $vboxExePath createvm --name $vmName --ostype Other --register

    & $vboxExePath modifyvm $vmName `
        --memory 128 `
        --boot1 dvd `
        --firmware bios

    & $vboxExePath storagectl $vmName `
        --name "SATA Controller" `
        --add sata `
        --controller IntelAHCI

    & $vboxExePath storageattach $vmName `
        --storagectl "SATA Controller" `
        --port 0 `
        --device 0 `
        --type dvddrive `
        --medium $isoPath

    # COM1 serial output to serial.log
    & $vboxExePath modifyvm $vmName `
        --uart1 0x3F8 4 `
        --uartmode1 file $uartLog
} else {
    # Ensure current ISO is attached
    Write-Host "Re-attaching latest ISO to $vmName ..."
    & $vboxExePath storageattach $vmName `
        --storagectl "SATA Controller" `
        --port 0 `
        --device 0 `
        --type dvddrive `
        --medium $isoPath 2>$null
}

Write-Host "Starting VirtualBox VM: $vmName ..."
& $vboxExePath startvm $vmName --type gui
Write-Host "✅ VM started successfully!"
