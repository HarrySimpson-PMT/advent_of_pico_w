# Define variables
$pwd = Get-Location

#write current directory
Write-Host "Current directory: $pwd"

# Step 1: Build the pico_solver for thumbv6m - this is in the Node subdirectory
write-host "Building node for thumbv6m..."
cd $pwd\Server
cargo run
cd ..

