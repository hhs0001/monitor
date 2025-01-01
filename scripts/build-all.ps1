# Definir cores para output
$Green = [System.ConsoleColor]::Green
$Blue = [System.ConsoleColor]::Blue

Write-Host "Escolha a plataforma para compilar:" -ForegroundColor $Blue
Write-Host "1. Windows (x64)"
Write-Host "2. Linux (x64)"
Write-Host "3. macOS (x64 + ARM)"
Write-Host "4. Todas as plataformas"

$choice = Read-Host "Digite sua escolha (1-4)"

switch ($choice) {
    "1" { 
        & "$PSScriptRoot\build-windows.ps1"
    }
    "2" {
        if (Get-Command wsl -ErrorAction SilentlyContinue) {
            wsl bash "$PSScriptRoot/build-linux.sh"
        } else {
            Write-Host "WSL não está instalado. Necessário para compilar para Linux." -ForegroundColor Red
        }
    }
    "3" {
        Write-Host "Compilação para macOS requer um ambiente macOS." -ForegroundColor Yellow
        & "$PSScriptRoot\build-macos.sh"
    }
    "4" {
        & "$PSScriptRoot\build-windows.ps1"
        if (Get-Command wsl -ErrorAction SilentlyContinue) {
            wsl bash "$PSScriptRoot/build-linux.sh"
        }
        & "$PSScriptRoot\build-macos.sh"
    }
    default {
        Write-Host "Escolha inválida" -ForegroundColor Red
    }
} 