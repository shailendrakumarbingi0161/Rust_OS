# Create or update a VirtualBox VM for RustOS with Intel e1000 NAT Network and start it.

$vmName = "RustOS"
$repoRoot = Split-Path $PSScriptRoot -Parent
$isoPath = Join-Path -Path $repoRoot -ChildPath "build\rustos.iso"
$serialLog = Join-Path -Path $repoRoot -ChildPath "build\serial.log"

# Ensure old state is cleared
VBoxManage controlvm $vmName poweroff 2>$null

$vmList = VBoxManage list vms
if ($vmList -notmatch '"RustOS"') {
    Write-Host "Creating VM $vmName..."
    VBoxManage createvm --name $vmName --register
    VBoxManage modifyvm $vmName --ostype "Other_64" --memory 128 --cpus 1 --nic1 nat --nictype1 82540EM
    VBoxManage storagectl $vmName --name "IDE Controller" --add ide
    VBoxManage storageattach $vmName --storagectl "IDE Controller" --port 0 --device 0 --type dvddrive --medium "$isoPath"
    VBoxManage modifyvm $vmName --uart1 0x3F8 4 --uartmode1 file "$serialLog"
} else {
    Write-Host "VM $vmName already exists - updating ISO, NIC, and serial log."
    VBoxManage modifyvm $vmName --nic1 nat --nictype1 82540EM
    VBoxManage storageattach $vmName --storagectl "IDE Controller" --port 0 --device 0 --type dvddrive --medium "$isoPath" --force
    VBoxManage modifyvm $vmName --uartmode1 file "$serialLog"
}

Write-Host "Starting VM $vmName (headless)..."
VBoxManage startvm $vmName --type headless
