# Boot test for RustOS in VirtualBox.
# Starts the VM, waits for it to boot, powers it off, and checks the serial log for the boot marker.

$vmName = "RustOS"
$serialLog = Join-Path -Path $PSScriptRoot -ChildPath "build\serial.log"
$bootMarker = "Hello, RustOS!"

# Ensure any previous instance is powered off.
VBoxManage controlvm $vmName poweroff 2>$null

# Remove old serial log.
if (Test-Path $serialLog) { Remove-Item $serialLog }

# Start the VM headlessly.
Write-Host "Starting VM $vmName for boot test..."
VBoxManage startvm $vmName --type headless

# Wait for boot.
$waitSeconds = 10
Write-Host "Waiting $waitSeconds seconds for boot..."
Start-Sleep -Seconds $waitSeconds

# Power off the VM.
Write-Host "Powering off VM $vmName..."
VBoxManage controlvm $vmName acpipowerbutton
Start-Sleep -Seconds 2
VBoxManage controlvm $vmName poweroff

# Check serial log for boot marker.
if (Test-Path $serialLog) {
    $content = Get-Content $serialLog -Raw
    if ($content -match $bootMarker) {
        Write-Host "Boot test succeeded: marker found in serial log."
    } else {
        Write-Host "Boot test failed: marker not found in serial log."
        Write-Host "Serial log content:"
        Write-Host $content
    }
} else {
    Write-Host "Serial log not found at $serialLog. Boot may have failed."
}
