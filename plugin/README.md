# 📱 Antigravity Mobile Manager (Plugin per Google Antigravity)

**Controlla Google Antigravity dallo smartphone via Rete Locale (LAN) o Hotspot PC sulla porta `8420` tramite Web App PWA o App Android nativa.**

* **Autore**: **Rubens Rainelli** ([rubens@rainelli.it](mailto:rubens@rainelli.it) | [GitHub](https://github.com/RubensRainelli/) | Tel: `(+39) 342 811 5882`)
* **Licenza**: **Personal and Educational Free License (PEFL) v1.0**

---

## ✨ Funzionalità del Plugin

1. **🔔 Notifiche Push in Tempo Reale (`Stop` Hook & Transcript Watcher)**:
   * Avvisa istantaneamente sullo smartphone quando un agente termina il lavoro, incontra un errore o pone una domanda.
2. **🛡️ Approvazione / Rifiuto Comandi da Remoto (`PreToolUse` Hook)**:
   * Intercetta `run_command`, `edit_file` e `write_file` prima dell'esecuzione.
   * Permette di **Approvare (`✓ Approva`)**, **Rifiutare (`✕ Rifiuta`)** o **Modificare il comando** direttamente dalla notifica del telefono o dalla dashboard.
3. **🖧 Rilevamento Multi-Rete Automatico (Wi-Fi LAN + Hotspot PC)**:
   * Rileva e mostra automaticamente sia l'IP della rete Wi-Fi/Ethernet (es. `192.168.1.x:8420`) sia l'IP dell'**Hotspot Mobile del PC Windows/Linux** (es. `192.168.137.1:8420`).
4. **💬 Invio Nuovi Prompt & Visualizzazione Agenti**:
   * Avvia nuove conversazioni, rispondi agli agenti attivi e ispeziona ragionamenti (`thinking`), tool call, subagenti e artefatti `.md`.

---

## 🚀 Avvio Rapido

1. Avvia il server sul PC eseguendo **`start-windows.bat`** (Windows) o **`./start-linux.sh`** (Linux), oppure da terminale:
   ```bash
   antigravity-mobile-manager serve
   ```
2. Apri sullo smartphone l'indirizzo mostrato nel terminale (es. `http://192.168.137.1:8420` se usi l'Hotspot del PC, oppure `http://192.168.1.120:8420` su rete Wi-Fi).

---

> **Disclaimer**: *Antigravity Mobile Manager è un progetto indipendente creato da Rubens Rainelli e NON è sostenuto, affiliato, sponsorizzato o correlato a Google LLC.*
