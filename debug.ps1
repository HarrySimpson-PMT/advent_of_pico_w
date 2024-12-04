# Define variables
$PICO_PROJECT = "pico_solver"
$HOST_PROJECT = "primary_solver"
$PICO_TARGET = "thumbv6m-none-eabi"
$REMOTE_DEPLOY_SCRIPT = "D:\Source\Rust\AdventOfPicoW\pico_solver\scripts\remote_deploy.ps1"

# Step 1: Build the pico_solver for thumbv7m
Write-Host "Building $PICO_PROJECT for $PICO_TARGET..."
cargo build --package $PICO_PROJECT --target $PICO_TARGET --release

# Step 2: Open remote_deploy.ps1 in a new terminal
Write-Host "Launching remote deployment script in a new terminal..."
Start-Process powershell -ArgumentList "-NoExit", "-File", $REMOTE_DEPLOY_SCRIPT

# Step 3: Run the primary_solver in debug mode
Write-Host "Running $HOST_PROJECT in debug mode..."
cargo run --package $HOST_PROJECT
