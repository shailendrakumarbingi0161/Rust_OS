# Create or update a VirtualBox VM for RustOS and start it headlessly.

$vmName = "RustOS"
$isoPath = Join-Path -Path $PSScriptRoot -ChildPath "build\rustos.iso"
$serialLog = Join-Path -Path $PSScriptRoot -ChildPath "build\serial.log"

if (-not (VBoxManage list vms | Select-String -Pattern "\"$vmName\"")) {
    Write-Host "Creating VM `$vmName`..."
    VBoxManage createvm --name $vmName --register
    VBoxManage modifyvm $vmName --ostype "Other_64" --memory 128 --cpus 1 --nic none
    VBoxManage storagectl $vmName --name "IDE Controller" --add ide
    VBoxManage storageattach $vmName --storagectl "IDE Controller" --port 0 --device 0 --type dvddrive --medium "$isoPath"
    VBoxManage modifyvm $vmName --uart1 0x3F8 4 --uartmode1 file "$serialLog"
} else {
    Write-Host "VM `$vmName` already exists - updating ISO and serial log."
    VBoxManage storageattach $vmName --storagectl "IDE Controller" --port 0 --device 0 --type dvddrive --medium "$isoPath" --force
    VBoxManage modifyvm $vmName --uartmode1 file "$serialLog"
}

Write-Host "Starting VM `$vmName` (headless)..."
VBoxManage startvm $vmName --type headless
