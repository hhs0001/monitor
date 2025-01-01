# Definir cores para output
$Green = [System.ConsoleColor]::Green
$Red = [System.ConsoleColor]::Red
$Blue = [System.ConsoleColor]::Blue

Write-Host "Compilando System Monitor para Windows..." -ForegroundColor $Blue

# Criar diretório de output se não existir
if (-not (Test-Path "../builds")) {
    New-Item -ItemType Directory -Path "../builds"
}

# Compilar para Windows (x64)
Write-Host "Compilando para Windows (x64)..." -ForegroundColor $Green
cargo build --release

# Verificar se a compilação foi bem sucedida
if ($LASTEXITCODE -eq 0) {
    # Copiar o executável para a pasta builds
    Copy-Item "../target/release/monitor.exe" "../builds/monitor-windows-x64.exe" -Force
    
    Write-Host "`nCompilação concluída com sucesso!" -ForegroundColor $Green
    Write-Host "O executável está em: builds/monitor-windows-x64.exe" -ForegroundColor $Blue
} else {
    Write-Host "`nErro durante a compilação!" -ForegroundColor $Red
    exit 1
}

# Pausar para ver o resultado
Write-Host "`nPressione qualquer tecla para sair..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown") 