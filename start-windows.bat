@echo off
REM Personal and Educational Free License (PEF License / PEFL) v1.0 - 2026-05-26
REM Copyright (c) 2026 Rubens Rainelli (rubens@rainelli.it | https://github.com/RubensRainelli/)
REM Personal and educational free use permitted. Corporate use prohibited without written authorization and fee.

title Antigravity Mobile Manager (Porta 8420) - PEFL v1.0
cd /d "%~dp0"

if not exist "target\debug\antigravity-mobile-manager.exe" (
    echo [1/2] Compilazione di antigravity-mobile-manager in corso...
    cargo build
)

echo [2/2] Avvio di Antigravity Mobile Manager sulla porta 8420...
"target\debug\antigravity-mobile-manager.exe" serve
pause
