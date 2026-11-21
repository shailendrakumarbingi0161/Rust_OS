# scripts/enlarge_vm.ps1
$csharpCode = @"
using System;
using System.Runtime.InteropServices;

public class WinHelper {
    [DllImport("user32.dll")]
    public static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);
    [DllImport("user32.dll")]
    public static extern bool SetForegroundWindow(IntPtr hWnd);
}
"@

Add-Type -TypeDefinition $csharpCode -Language CSharp

$procs = Get-Process | Where-Object { $_.MainWindowTitle -match "RustOS" }
if ($procs) {
    foreach ($p in $procs) {
        Write-Host "Maximizing window: $($p.MainWindowTitle) (HWND: $($p.MainWindowHandle))"
        [WinHelper]::ShowWindow($p.MainWindowHandle, 3) # 3 = SW_MAXIMIZE
        [WinHelper]::SetForegroundWindow($p.MainWindowHandle)
    }
} else {
    Write-Host "No process with MainWindowTitle matching 'RustOS' found. Looking for VirtualBoxVM..."
    $vmProcs = Get-Process | Where-Object { $_.ProcessName -match "VirtualBox" }
    foreach ($vp in $vmProcs) {
        Write-Host "Process $($vp.ProcessName) ID $($vp.Id) Title: '$($vp.MainWindowTitle)'"
        if ($vp.MainWindowHandle -ne 0) {
            Write-Host "Maximizing HWND $($vp.MainWindowHandle)"
            [WinHelper]::ShowWindow($vp.MainWindowHandle, 3)
            [WinHelper]::SetForegroundWindow($vp.MainWindowHandle)
        }
    }
}

VBoxManage list runningvms
