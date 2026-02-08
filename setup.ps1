# SharpSystem V2 - Environment Setup Script
# This script initializes the .env file from the example

$ExampleFile = ".env.example"
$TargetFile = ".env"

if (-Not (Test-Path $ExampleFile)) {
    Write-Host "Error: .env.example not found!" -ForegroundColor Red
    exit 1
}

if (Test-Path $TargetFile) {
    $Choice = Read-Host ".env already exists. Overwrite? (y/n)"
    if ($Choice -ne "y") {
        Write-Host "Setup cancelled." -ForegroundColor Yellow
        exit 0
    }
}

Copy-Item $ExampleFile $TargetFile
Write-Host "Successfully initialized .env from .env.example" -ForegroundColor Green
Write-Host "Action Required: Please update the values in .env with your local configuration." -ForegroundColor Cyan
