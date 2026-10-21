#!/usr/bin/env bash
# Personal and Educational Free License (PEF License / PEFL) v1.0 - 2026-05-26
# Copyright (c) 2026 Rubens Rainelli (rubens@rainelli.it | https://github.com/RubensRainelli/)
# Personal and educational free use permitted. Corporate use prohibited without written authorization and fee.

set -e
cd "$(dirname "$0")"

if [ ! -f "target/debug/antigravity-mobile-manager" ]; then
    echo "[1/2] Compilazione di antigravity-mobile-manager in corso..."
    cargo build
fi

echo "[2/2] Avvio di Antigravity Mobile Manager sulla porta 8420..."
./target/debug/antigravity-mobile-manager serve
