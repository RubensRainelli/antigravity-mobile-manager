#!/usr/bin/env bash
# Personal and Educational Free License (PEF License / PEFL) v1.0 - 2026-05-26
# Copyright (c) 2026 Rubens Rainelli (rubens@rainelli.it | https://github.com/RubensRainelli/)
# Personal and educational free use permitted. Corporate use prohibited without written authorization and fee.

set -e
cd "$(dirname "$0")"

echo "============================================================================"
echo "  Installazione Plugin: Antigravity Mobile Manager (antigravity-mobile-manager)"
echo "  Autore: Rubens Rainelli (PEFL v1.0)"
echo "============================================================================"

echo "[1/3] Compilazione del binario antigravity-mobile-manager..."
cargo build

BIN_DIR="$HOME/.local/bin"
OLD_PLUGIN_DIR="$HOME/.gemini/config/plugins/antigravity-manager"
PLUGIN_DIR="$HOME/.gemini/config/plugins/antigravity-mobile-manager"

rm -rf "$OLD_PLUGIN_DIR"
mkdir -p "$BIN_DIR"
mkdir -p "$PLUGIN_DIR/assets"

echo "[2/3] Copia di antigravity-mobile-manager in $BIN_DIR e $PLUGIN_DIR..."
cp -f "target/debug/antigravity-mobile-manager" "$BIN_DIR/antigravity-mobile-manager"
chmod +x "$BIN_DIR/antigravity-mobile-manager"
cp -f "target/debug/antigravity-mobile-manager" "$PLUGIN_DIR/antigravity-mobile-manager"
chmod +x "$PLUGIN_DIR/antigravity-mobile-manager"

echo "[3/3] Installazione manifest plugin.json, README.md, logo.svg e hooks.json..."
cp -f "plugin/plugin.json" "$PLUGIN_DIR/plugin.json"
cp -f "plugin/hooks.json" "$PLUGIN_DIR/hooks.json"
cp -f "plugin/README.md" "$PLUGIN_DIR/README.md"
cp -f "plugin/assets/logo.svg" "$PLUGIN_DIR/assets/logo.svg"

echo ""
echo "[OK] Plugin Antigravity Mobile Manager installato con successo in:"
echo "     - $PLUGIN_DIR"
echo "     - $BIN_DIR/antigravity-mobile-manager"
echo "Per avviare il server sulla porta 8420 esegui: ./start-linux.sh"
