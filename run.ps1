# Define variables
$pwd = Get-Location

#write current directory
Write-Host "Current directory: $pwd"

$REMOTE_DEPLOY_SCRIPT = "D:\Source\Rust\AdventOfPicoW\pico_solver\scripts\remote_deploy.ps1"

# Step 1: Build the pico_solver for thumbv6m - this is in the Node subdirectory
write-host "Building node for thumbv6m..."
cd $pwd\Node
cargo build

# Step 2: Open remote_deploy.ps1 in a new terminal
Write-Host "Launching remote deployment script in a new terminal..."
Start-Process powershell -ArgumentList "-NoExit", "-File", $REMOTE_DEPLOY_SCRIPT
cd ..

cd $pwd\Server
cargo run
cd ..

