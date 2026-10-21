@echo off
REM Personal and Educational Free License (PEF License / PEFL) v1.0 - 2026-05-26
REM Copyright (c) 2026 Rubens Rainelli (rubens@rainelli.it | https://github.com/RubensRainelli/)
REM Personal and educational free use permitted. Corporate use prohibited without written authorization and fee.

title Installazione Plugin Antigravity Mobile Manager
cd /d "%~dp0"

echo ============================================================================
echo   Installazione Plugin: Antigravity Mobile Manager (antigravity-mobile-manager)
echo   Autore: Rubens Rainelli (PEFL v1.0)
echo ============================================================================
echo.

echo [1/3] Compilazione del binario antigravity-mobile-manager.exe...
cargo build
if errorlevel 1 (
    echo [ERRORE] Compilazione fallita. Verifica che Rust/Cargo sia installato.
    pause
    exit /b 1
)

set "BIN_DIR=%USERPROFILE%\.gemini\antigravity\bin"
set "WIN_APPS_DIR=%USERPROFILE%\AppData\Local\Microsoft\WindowsApps"
set "OLD_PLUGIN_DIR=%USERPROFILE%\.gemini\config\plugins\antigravity-manager"
set "PLUGIN_DIR=%USERPROFILE%\.gemini\config\plugins\antigravity-mobile-manager"

if exist "%OLD_PLUGIN_DIR%" (
    rmdir /s /q "%OLD_PLUGIN_DIR%"
)

if not exist "%BIN_DIR%" mkdir "%BIN_DIR%"
if not exist "%PLUGIN_DIR%" mkdir "%PLUGIN_DIR%"
if not exist "%PLUGIN_DIR%\assets" mkdir "%PLUGIN_DIR%\assets"

echo [2/3] Copia di antigravity-mobile-manager.exe nel PATH utente e nel plugin...
copy /Y "target\debug\antigravity-mobile-manager.exe" "%BIN_DIR%\antigravity-mobile-manager.exe" >nul
copy /Y "target\debug\antigravity-mobile-manager.exe" "%PLUGIN_DIR%\antigravity-mobile-manager.exe" >nul
if exist "%WIN_APPS_DIR%" (
    copy /Y "target\debug\antigravity-mobile-manager.exe" "%WIN_APPS_DIR%\antigravity-mobile-manager.exe" >nul
)

echo [3/3] Installazione manifest plugin.json, README.md, logo.svg e hooks.json...
copy /Y "plugin\plugin.json" "%PLUGIN_DIR%\plugin.json" >nul
copy /Y "plugin\hooks.json" "%PLUGIN_DIR%\hooks.json" >nul
copy /Y "plugin\README.md" "%PLUGIN_DIR%\README.md" >nul
copy /Y "plugin\assets\logo.svg" "%PLUGIN_DIR%\assets\logo.svg" >nul

echo.
echo [OK] Plugin "Antigravity Mobile Manager" installato con successo in:
echo      - %PLUGIN_DIR%
echo      - %BIN_DIR%\antigravity-mobile-manager.exe
echo.
echo Per avviare il server sulla porta 8420 esegui: start-windows.bat
pause
