/*
 * Personal and Educational Free License (PEF License / PEFL)
 * Version 1.0 - 2026-05-26
 * Copyright (c) 2026 Rubens Rainelli
 *
 * Contact Details:
 *   Email: rubens@rainelli.it
 *   GitHub: https://github.com/RubensRainelli/
 *   Tel: (+39) 342 811 5882
 *
 * Personal and educational free use permitted.
 * Corporate, business or enterprise use prohibited without written authorization and fee.
 * The full license can be found in the LICENSE file.
 */

const PEFL_SEVEN_DAYS_MS = 7 * 24 * 60 * 60 * 1000;
const PEFL_STORAGE_KEY = "pefl_accepted_at_v1";
const API_BASE_STORAGE_KEY = "agy_api_base_url";

const PEFL_TEXT_IT = `Personal and Educational Free License
PEF License (PEFL)
Versione 1.0
Data: 2026-05-26
Repository Ufficiale della Specifica: https://github.com/RubensRainelli/pefl-license-spec

La presente licenza disciplina anche il proprio testo, la propria distribuzione e il proprio utilizzo; pertanto, la licenza stessa non può essere impiegata in ambito aziendale senza autorizzazione scritta e pagamento della relativa fee.

Copyright (c) 2026 Rubens Rainelli

Contatti dell'Autore:
  - Email: rubens@rainelli.it
  - GitHub: https://github.com/RubensRainelli/
  - Telefono: (+39) 342 811 5882

1. Definizioni.
Per "Uso Privato" si intende l'uso personale, domestico, didattico, accademico o da parte di scuole, università ed Enti del Terzo Settore, senza finalità di profitto o di supporto diretto ad attività aziendali.
Sono esclusi dall'Uso Privato gli enti di culto e le organizzazioni religiose, che sono considerati equivalenti a Uso Aziendale.
Per "Uso Aziendale" si intende qualsiasi uso del Software in contesti business, professionali, produttivi o organizzativi, inclusi server, PC, notebook, workstation, infrastrutture, ambienti di test, staging o produzione gestiti da aziende, enti religiosi o soggetti equivalenti.

2. Permessi.
È consentito a chiunque usare, copiare, modificare e ridistribuire il Software gratuitamente per Uso Privato.
È consentito modificare il Software per Uso Privato, purché restino visibili il nome dell'autore, il riferimento alla presente licenza e tutti gli avvisi di copyright originali.
Il Software e la presente licenza possono essere ospitati, archiviati e distribuiti su qualsiasi dispositivo o supporto di qualunque natura.

3. Obblighi di attribuzione e approvazione.
Ogni copia, distribuzione o derivato del Software deve mantenere il nome dell'autore, Rubens Rainelli, e non può rimuovere, oscurare o sostituire l'attribuzione originaria.
La presente licenza deve essere sempre visibile, in primo piano, ad ogni avvio del Software (o almeno una volta ogni 7 giorni qualora sia disponibile un meccanismo di persistenza locale per memorizzare in modo sicuro la data di accettazione), e deve essere esplicitamente approvata dall'utente prima di ogni utilizzo.
Ogni redistribuzione deve includere la licenza nella sua forma attuale, senza alterarne il testo, la versione o le condizioni.

4. Divieto d'uso aziendale senza autorizzazione.
Qualsiasi Uso Aziendale del Software, o qualsiasi utilizzo del Software su macchine aziendali o comunque non destinate a esclusivo uso privato, è vietato salvo autorizzazione scritta e pagamento della relativa usage fee concordata con l'autore.
In assenza di tale autorizzazione, l'uso in ambienti business è proibito anche se il Software non viene venduto o distribuito a terzi.

5. Redistribuzione.
Chiunque può ridistribuire il Software, con o senza modifiche, purché siano rispettati i termini della presente licenza, incluso il mantenimento dell'attribuzione e il mantenimento della licenza attuale.
È vietato distribuire il Software con una licenza diversa da quella presente, salvo consenso scritto dell'autore.

6. Nessuna garanzia.
Il Software è fornito "così com'è", senza alcuna garanzia di alcun tipo, espressa o implicita.
L'autore non risponde di perdite di dati, malfunzionamenti, incompatibilità, interruzioni o altri effetti derivanti dall'uso del Software.

7. Responsabilità.
L'uso del Software avviene sotto esclusiva responsabilità di chi lo utilizza.
Qualsiasi danno, perdita o conseguenza derivante dall'uso, dall'abuso o dall'interpretazione del Software è a carico dell'utente.

8. Accettazione.
L'uso, la copia, la modifica o la redistribuzione del Software implica l'accettazione integrale dei termini della presente licenza.`;

const PEFL_TEXT_EN = `Personal and Educational Free License
PEF License (PEFL)
Version 1.0
Date: 2026-05-26
Official Specification Repository: https://github.com/RubensRainelli/pefl-license-spec

This license also governs its own text, its own distribution, and its own use; therefore, the license itself cannot be used in a corporate environment without written authorization and payment of the corresponding fee.

Copyright (c) 2026 Rubens Rainelli

Author's Contact Details:
  - Email: rubens@rainelli.it
  - GitHub: https://github.com/RubensRainelli/
  - Phone: (+39) 342 811 5882

1. Definitions.
"Personal Use" refers to personal, domestic, educational, academic use, or use by schools, universities, and Non-Profit Organizations / Third Sector Entities, without profit motives or direct support for corporate activities.
Places of worship and religious organizations are explicitly excluded from Personal Use and are considered equivalent to Corporate Use.
"Corporate Use" refers to any use of the Software in business, professional, productive, or organizational contexts, including servers, PCs, notebooks, workstations, infrastructures, testing, staging, or production environments managed by corporations, companies, religious entities, or equivalent subjects.

2. Permissions.
Anyone is allowed to use, copy, modify, and redistribute the Software free of charge for Personal Use.
Modifying the Software is permitted for Personal Use, provided that the author's name, reference to this license, and all original copyright notices remain visible.
The Software and this license may be hosted, archived, and distributed on any device or medium of any nature.

3. Attribution and Approval Obligations.
Every copy, distribution, or derivative work of the Software must retain the author's name, Rubens Rainelli, and cannot remove, obscure, or replace the original attribution.
This license must always be visible, in the foreground, at every launch of the Software (or at least once every 7 days if a local persistence mechanism is available to securely store the acceptance date), and must be explicitly approved by the user before any use.
Every redistribution must include the license in its current form, without altering the text, version, or conditions.

4. Prohibition of Corporate Use without Authorization.
Any Corporate Use of the Software, or any use of the Software on corporate-owned machines or machines not intended for exclusive personal use, is prohibited except with written authorization and payment of the corresponding usage fee agreed upon with the author.
In the absence of such authorization, use in business environments is prohibited even if the Software is not sold or distributed to third parties.

5. Redistribution.
Anyone may redistribute the Software, with or without modifications, provided that the terms of this license are respected, including retaining the attribution and retaining the current license.
Distributing the Software under a different license is prohibited without the author's written consent.

6. No Warranty.
The Software is provided "as is", without warranty of any kind, express or implied.
The author is not liable for data loss, malfunctions, incompatibility, interruptions, or any other effects deriving from the use of the Software.

7. Liability.
The use of the Software is at the sole responsibility of the user.
Any damage, loss, or consequence resulting from the use, abuse, or interpretation of the Software is a responsibility of the user.

8. Acceptance.
Using, copying, modifying, or redistributing the Software implies full acceptance of the terms of this license.`;

let appState = {
  apiBaseUrl: "",
  peflApproved: false,
  conversations: [],
  projects: [],
  pendingConfirmations: [],
  notifications: [],
  selectedProject: "ALL",
  currentConversationId: null,
  eventSource: null,
  swRegistration: null,
};

document.addEventListener("DOMContentLoaded", () => {
  initServiceWorker();
  initApiBaseUrl();
  updateNotificationBanner();
  checkPeflComplianceOnStartup();
});

// ============================================================================
// 1. CLAUSOLA 3 PEFL v1.0 - CONTROLLO EULA OBBLIGATORIO IN PRIMO PIANO
// ============================================================================

function checkPeflComplianceOnStartup() {
  switchPeflLang("it");
  try {
    const tsStr = localStorage.getItem(PEFL_STORAGE_KEY);
    if (tsStr) {
      const ts = parseInt(tsStr, 10);
      const now = Date.now();
      if (!isNaN(ts) && now >= ts && now - ts < PEFL_SEVEN_DAYS_MS) {
        appState.peflApproved = true;
        document.getElementById("pefl-modal").classList.add("hidden");
        startAppAfterPeflApproval();
        return;
      }
    }
  } catch (_) {}

  // Mostra la licenza in primo piano prima di qualsiasi utilizzo
  showPeflModal(true);
}

function switchPeflLang(lang) {
  const box = document.getElementById("pefl-license-text");
  if (box) {
    box.textContent = lang === "en" ? PEFL_TEXT_EN : PEFL_TEXT_IT;
  }
}

function showPeflModal(fromDeclined) {
  if (fromDeclined) {
    document.getElementById("pefl-declined-screen").classList.add("hidden");
  }
  document.getElementById("pefl-modal").classList.remove("hidden");
}

function acceptPeflLicense() {
  const remember = document.getElementById("pefl-remember-7d").checked;
  try {
    if (remember) {
      localStorage.setItem(PEFL_STORAGE_KEY, String(Date.now()));
    } else {
      localStorage.removeItem(PEFL_STORAGE_KEY);
    }
  } catch (_) {}

  appState.peflApproved = true;
  document.getElementById("pefl-modal").classList.add("hidden");
  document.getElementById("pefl-declined-screen").classList.add("hidden");

  // Registra l'approvazione anche sul PC se connesso
  if (appState.apiBaseUrl) {
    fetch(`${appState.apiBaseUrl}/api/pefl/accept`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ remember7Days: remember }),
    }).catch(() => {});
  }

  startAppAfterPeflApproval();
}

function declinePeflLicense() {
  appState.peflApproved = false;
  try {
    localStorage.removeItem(PEFL_STORAGE_KEY);
  } catch (_) {}
  document.getElementById("pefl-modal").classList.add("hidden");
  document.getElementById("pefl-declined-screen").classList.remove("hidden");
}

// ============================================================================
// 2. INIZIALIZZAZIONE PWA, SERVICE WORKER E URL BACKEND (GITHUB PAGES + LAN)
// ============================================================================

function initServiceWorker() {
  if ("serviceWorker" in navigator) {
    navigator.serviceWorker
      .register("./sw.js")
      .then((reg) => {
        appState.swRegistration = reg;
      })
      .catch(() => {});

    navigator.serviceWorker.addEventListener("message", (event) => {
      if (event.data && event.data.type === "OPEN_CONVERSATION" && event.data.conversationId) {
        openConversationDetail(event.data.conversationId);
      }
    });
  }
}

function initApiBaseUrl() {
  const params = new URLSearchParams(window.location.search);
  const queryServer = params.get("server");
  if (queryServer) {
    const clean = queryServer.trim().replace(/\/+$/, "");
    localStorage.setItem(API_BASE_STORAGE_KEY, clean);
    appState.apiBaseUrl = clean;
  } else {
    const saved = localStorage.getItem(API_BASE_STORAGE_KEY);
    if (saved) {
      appState.apiBaseUrl = saved.trim().replace(/\/+$/, "");
    } else if (window.location.port === "8420") {
      // Se aperta direttamente dal server locale sulla porta 8420
      appState.apiBaseUrl = window.location.origin;
    } else {
      // Se aperta da GitHub Pages (es. https://*.github.io) senza IP salvato
      appState.apiBaseUrl = "http://192.168.1.120:8420";
    }
  }

  const settingsInput = document.getElementById("settings-api-url");
  if (settingsInput) settingsInput.value = appState.apiBaseUrl;
}

function startAppAfterPeflApproval() {
  fetchState();
  connectSSE();
  setInterval(() => {
    if (appState.peflApproved) fetchState();
  }, 6000);
}

function openServerConfigModal() {
  document.getElementById("modal-api-url").value = appState.apiBaseUrl;
  document.getElementById("server-config-modal").classList.remove("hidden");
}

function closeServerConfigModal() {
  document.getElementById("server-config-modal").classList.add("hidden");
}

function submitServerConfigModal(e) {
  e.preventDefault();
  const val = document.getElementById("modal-api-url").value.trim().replace(/\/+$/, "");
  if (!val) return;
  applyNewApiBaseUrl(val);
  closeServerConfigModal();
}

function saveApiBaseFromSettings() {
  const val = document.getElementById("settings-api-url").value.trim().replace(/\/+$/, "");
  if (!val) return;
  applyNewApiBaseUrl(val);
}

function applyNewApiBaseUrl(newUrl) {
  appState.apiBaseUrl = newUrl;
  localStorage.setItem(API_BASE_STORAGE_KEY, newUrl);
  document.getElementById("settings-api-url").value = newUrl;
  document.getElementById("modal-api-url").value = newUrl;
  fetchState();
  connectSSE();
}

// ============================================================================
// 3. SINCRONIZZAZIONE STATO & STREAMING EVENTI (SSE)
// ============================================================================

async function fetchState() {
  if (!appState.peflApproved || !appState.apiBaseUrl) return;
  try {
    const res = await fetch(`${appState.apiBaseUrl}/api/state`);
    const data = await res.json();
    appState.conversations = data.conversations || [];
    appState.projects = data.projects || [];
    appState.pendingConfirmations = data.pendingConfirmations || [];
    appState.notifications = data.notifications || [];
    appState.lanUrls = data.lanUrls || [];

    const urlsSummary =
      appState.lanUrls.length > 0
        ? appState.lanUrls.map((u) => `${u.label}: ${u.url}`).join(" | ")
        : data.lanUrl;
    document.getElementById("server-info").textContent = `${data.os.toUpperCase()} • ${urlsSummary}`;
    document.getElementById("lan-url-box").textContent = data.lanUrl;

    renderDetectedLanUrls();

    const radio = document.querySelector(`input[name="confirmMode"][value="${data.confirmMode}"]`);
    if (radio) radio.checked = true;

    renderProjectsFilter();
    renderConversationsList();
    renderConfirmations();
    renderNotifications();
  } catch (_) {
    document.getElementById("sse-dot").classList.remove("online");
    document.getElementById("server-info").textContent = `Offline (${appState.apiBaseUrl})`;
  }
}

function renderDetectedLanUrls() {
  const list = appState.lanUrls || [];
  const settingsStrip = document.getElementById("detected-ips-list");
  const modalStrip = document.getElementById("modal-detected-ips");
  if (list.length === 0) {
    if (settingsStrip) settingsStrip.innerHTML = "";
    if (modalStrip) modalStrip.innerHTML = "";
    return;
  }
  const html = list
    .map(
      (item) =>
        `<button type="button" class="chip ${appState.apiBaseUrl === item.url ? "active" : ""}" onclick="applyNewApiBaseUrl('${escapeAttr(item.url)}')">🖧 ${escapeHtml(item.label)}: <strong>${escapeHtml(item.ip)}:8420</strong></button>`
    )
    .join("");
  if (settingsStrip) settingsStrip.innerHTML = html;
  if (modalStrip) modalStrip.innerHTML = html;
}

function connectSSE() {
  if (!appState.peflApproved || !appState.apiBaseUrl) return;
  if (appState.eventSource) {
    try {
      appState.eventSource.close();
    } catch (_) {}
  }

  const dot = document.getElementById("sse-dot");
  const es = new EventSource(`${appState.apiBaseUrl}/api/events`);
  appState.eventSource = es;

  es.onopen = () => dot.classList.add("online");
  es.onerror = () => dot.classList.remove("online");

  es.onmessage = (e) => {
    dot.classList.add("online");
    try {
      const ev = JSON.parse(e.data);
      if (ev.type === "confirm_request" || ev.type === "confirm_resolved") {
        fetchState();
        if (ev.type === "confirm_request") {
          showBrowserNotification(`⚠️ Conferma richiesta (${ev.projectName})`, `[${ev.toolName}] ${ev.summary}`, {
            isConfirm: true,
            reqId: ev.id,
            conversationId: ev.conversationId,
          });
        }
      } else if (ev.type === "notification") {
        fetchState();
        if (ev.kind !== "confirm") {
          showBrowserNotification(ev.title, ev.body, {
            isConfirm: false,
            conversationId: ev.conversationId,
          });
        }
      } else if (ev.type === "transcript_updated") {
        fetchState();
        if (appState.currentConversationId && appState.currentConversationId === ev.conversationId) {
          loadConversationDetail(appState.currentConversationId, false);
        }
      }
    } catch (_) {}
  };
}

// ============================================================================
// 4. NOTIFICHE BROWSER / PWA (CON PULSANTI APPROVA / RIFIUTA)
// ============================================================================

function updateNotificationBanner() {
  const banner = document.getElementById("notif-permission-banner");
  const statusText = document.getElementById("notif-status-text");

  if (!("Notification" in window)) {
    if (banner) banner.classList.add("hidden");
    if (statusText) statusText.textContent = "Questo browser non supporta l'API Web Notifications.";
    return;
  }

  if (Notification.permission === "granted") {
    if (banner) banner.classList.add("hidden");
    if (statusText) statusText.textContent = "✅ Notifiche Browser attive (con supporto bottoni Approva/Rifiuta da Service Worker).";
  } else if (Notification.permission === "denied") {
    if (banner) banner.classList.add("hidden");
    if (statusText) statusText.textContent = "❌ Permesso notifiche negato nelle impostazioni del browser.";
  } else {
    if (banner) banner.classList.remove("hidden");
    if (statusText) statusText.textContent = "🔔 Premi 'Permessi 🔔' per abilitare gli avvisi in tempo reale sul browser.";
  }
}

async function requestBrowserNotificationPermission() {
  if (!("Notification" in window)) return;
  const perm = await Notification.requestPermission();
  updateNotificationBanner();
  if (perm === "granted") {
    showBrowserNotification("Antigravity Mobile Manager PWA", "Notifiche Browser attivate con successo!", {});
  }
}

function sendTestBrowserNotification() {
  if (!("Notification" in window)) return;
  if (Notification.permission !== "granted") {
    requestBrowserNotificationPermission();
    return;
  }
  showBrowserNotification(
    "Test Notifica Antigravity Mobile Manager",
    "Le notifiche browser funzionano correttamente sulla tua rete locale / Hotspot PC!",
    {}
  );
}

async function showBrowserNotification(title, body, meta = {}) {
  if (!("Notification" in window) || Notification.permission !== "granted") return;

  const options = {
    body,
    icon: "./icon.svg",
    badge: "./icon.svg",
    vibrate: [200, 100, 200],
    requireInteraction: !!meta.isConfirm,
    tag: meta.reqId || `agy-${Date.now()}`,
    data: {
      reqId: meta.reqId || "",
      conversationId: meta.conversationId || "",
      apiBaseUrl: appState.apiBaseUrl,
    },
  };

  if (meta.isConfirm) {
    options.actions = [
      { action: "allow", title: "✓ Approva" },
      { action: "deny", title: "✕ Rifiuta" },
    ];
  }

  try {
    if (appState.swRegistration && appState.swRegistration.showNotification) {
      await appState.swRegistration.showNotification(title, options);
      return;
    }
    if ("serviceWorker" in navigator) {
      const reg = await navigator.serviceWorker.ready;
      if (reg && reg.showNotification) {
        await reg.showNotification(title, options);
        return;
      }
    }
  } catch (_) {}

  // Fallback se il Service Worker non è attivo
  try {
    new Notification(title, { body, icon: "./icon.svg" });
  } catch (_) {}
}

// ============================================================================
// 5. RENDERING PROGETTI, AGENTI, DETTAGLIO E CONFERME
// ============================================================================

function renderProjectsFilter() {
  const container = document.getElementById("projects-filter");
  let html = `<button class="chip ${appState.selectedProject === "ALL" ? "active" : ""}" onclick="selectProject('ALL')">Tutti (${appState.conversations.length})</button>`;

  for (const p of appState.projects) {
    const active = appState.selectedProject === p.name ? "active" : "";
    html += `<button class="chip ${active}" onclick="selectProject('${escapeAttr(p.name)}')">${escapeHtml(p.name)} (${p.conversationCount})</button>`;
  }
  container.innerHTML = html;
}

function selectProject(name) {
  appState.selectedProject = name;
  renderProjectsFilter();
  renderConversationsList();
}

function renderConversationsList() {
  const container = document.getElementById("conversations-list");
  const filtered = appState.conversations.filter(
    (c) => appState.selectedProject === "ALL" || c.projectName === appState.selectedProject
  );

  if (filtered.length === 0) {
    container.innerHTML = `<div class="card"><p class="text-muted">Nessun agente trovato o PC non raggiungibile su <code>${escapeHtml(appState.apiBaseUrl)}</code>.</p></div>`;
    return;
  }

  const statusLabels = {
    ACTIVE: "IN ESECUZIONE",
    WAITING_CONFIRM: "ATTESA CONFERMA",
    WAITING_QUESTION: "DOMANDA AGENTE",
    IDLE: "INATTIVO",
  };

  container.innerHTML = filtered
    .map(
      (c) => `
      <div class="card conv-card" onclick="openConversationDetail('${escapeAttr(c.id)}')">
        <div class="conv-top">
          <div>
            <span class="project-tag">${escapeHtml(c.projectName)}</span>
            ${c.isSubagent ? `<span class="subagent-tag">SUBAGENT</span>` : ""}
          </div>
          <span class="status-pill status-${c.status}">${statusLabels[c.status] || c.status}</span>
        </div>
        <div class="conv-title">${escapeHtml(c.title)}</div>
        <div class="conv-preview">${escapeHtml(c.lastMessage || "Nessun messaggio")}</div>
      </div>
    `
    )
    .join("");
}

async function openConversationDetail(convId) {
  appState.currentConversationId = convId;
  document.getElementById("conversation-detail-view").classList.remove("hidden");
  await loadConversationDetail(convId, true);
}

function closeConversationDetail() {
  appState.currentConversationId = null;
  document.getElementById("conversation-detail-view").classList.add("hidden");
}

function refreshCurrentConversation() {
  if (appState.currentConversationId) {
    loadConversationDetail(appState.currentConversationId, true);
  }
}

async function loadConversationDetail(convId, scrollToBottom) {
  const res = await fetch(`${appState.apiBaseUrl}/api/conversations/${encodeURIComponent(convId)}`);
  const data = await res.json();

  document.getElementById("detail-project-name").textContent = data.projectName || "Conversazione";
  document.getElementById("detail-conv-id").textContent = data.parentId
    ? `Subagent di ${data.parentId.slice(0, 8)} • ${convId.slice(0, 8)}`
    : `ID: ${convId.slice(0, 8)}`;

  const artBar = document.getElementById("detail-artifacts-bar");
  if (data.artifacts && data.artifacts.length > 0) {
    artBar.classList.remove("hidden");
    artBar.innerHTML = data.artifacts
      .map(
        (fname) =>
          `<button class="chip" onclick="openArtifact('${escapeAttr(convId)}', '${escapeAttr(fname)}')">📄 ${escapeHtml(fname)}</button>`
      )
      .join("");
  } else {
    artBar.classList.add("hidden");
  }

  const stream = document.getElementById("detail-steps");
  const steps = data.steps || [];

  stream.innerHTML = steps
    .map((s) => {
      if (s.type === "USER_INPUT") {
        return `<div class="msg-bubble msg-user">${escapeHtml(s.content)}</div>`;
      }

      const thinkingHtml = s.thinking
        ? `<details class="thinking-box"><summary>Ragionamento agente</summary>${escapeHtml(s.thinking)}</details>`
        : "";

      let toolsHtml = "";
      let questionsHtml = "";

      if (s.toolCalls && s.toolCalls.length > 0) {
        toolsHtml = `<div>${s.toolCalls.map((t) => `<span class="tool-pill">⚙ ${escapeHtml(t.name)}</span>`).join("")}</div>`;
        for (const t of s.toolCalls) {
          if (t.name === "ask_question" && t.args) {
            questionsHtml += renderInteractiveQuestions(t.args);
          }
        }
      }

      return `
        <div class="msg-bubble msg-agent">
          ${thinkingHtml}
          ${toolsHtml}
          <div>${escapeHtml(s.content)}</div>
          ${questionsHtml}
        </div>
      `;
    })
    .join("");

  if (scrollToBottom) {
    stream.scrollTop = stream.scrollHeight;
  }
}

function renderInteractiveQuestions(rawArgsStr) {
  try {
    const args = JSON.parse(rawArgsStr);
    const questions = typeof args.questions === "string" ? JSON.parse(args.questions) : args.questions;
    if (!Array.isArray(questions)) return "";

    return questions
      .map(
        (q) => `
        <div class="question-card">
          <strong>❓ ${escapeHtml(q.question || "Domanda dall'agente")}</strong>
          <div class="question-options">
            ${(q.options || [])
              .map(
                (opt) =>
                  `<button class="btn btn-secondary btn-sm" onclick="sendQuickReply('${escapeAttr(opt)}')">${escapeHtml(opt)}</button>`
              )
              .join("")}
          </div>
        </div>
      `
      )
      .join("");
  } catch (_) {
    return "";
  }
}

async function sendQuickReply(optionText) {
  const input = document.getElementById("detail-prompt-input");
  input.value = optionText;
  await sendPromptToCurrent(new Event("submit"));
}

async function openArtifact(convId, fileName) {
  const res = await fetch(
    `${appState.apiBaseUrl}/api/conversations/${encodeURIComponent(convId)}/artifact?name=${encodeURIComponent(fileName)}`
  );
  const data = await res.json();
  document.getElementById("artifact-modal-title").textContent = fileName;
  document.getElementById("artifact-modal-content").textContent = data.content || data.error || "";
  document.getElementById("artifact-modal").classList.remove("hidden");
}

function closeArtifactModal() {
  document.getElementById("artifact-modal").classList.add("hidden");
}

async function sendPromptToCurrent(e) {
  if (e && e.preventDefault) e.preventDefault();
  const input = document.getElementById("detail-prompt-input");
  const btn = document.getElementById("detail-send-btn");
  const prompt = input.value.trim();
  if (!prompt || !appState.currentConversationId) return;

  btn.disabled = true;
  btn.textContent = "...";
  try {
    await fetch(`${appState.apiBaseUrl}/api/prompt`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        conversationId: appState.currentConversationId,
        prompt,
      }),
    });
    input.value = "";
    setTimeout(() => loadConversationDetail(appState.currentConversationId, true), 700);
  } finally {
    btn.disabled = false;
    btn.textContent = "Invia";
  }
}

function openNewConversationModal() {
  document.getElementById("new-conv-modal").classList.remove("hidden");
}

function closeNewConversationModal() {
  document.getElementById("new-conv-modal").classList.add("hidden");
}

async function submitNewConversation(e) {
  e.preventDefault();
  const title = document.getElementById("new-conv-title").value.trim();
  const model = document.getElementById("new-conv-model").value;
  const prompt = document.getElementById("new-conv-prompt").value.trim();
  const btn = document.getElementById("new-conv-submit");
  if (!prompt) return;

  btn.disabled = true;
  btn.textContent = "Avvio...";
  try {
    await fetch(`${appState.apiBaseUrl}/api/prompt`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        conversationId: "new",
        title,
        model,
        prompt,
      }),
    });
    document.getElementById("new-conv-prompt").value = "";
    closeNewConversationModal();
    setTimeout(fetchState, 1000);
  } finally {
    btn.disabled = false;
    btn.textContent = "Avvia Agente";
  }
}

function renderConfirmations() {
  const list = appState.pendingConfirmations;
  const badge = document.getElementById("nav-confirm-badge");
  const banner = document.getElementById("urgent-confirm-banner");
  const container = document.getElementById("confirmations-list");

  if (list.length > 0) {
    badge.textContent = list.length;
    badge.classList.remove("hidden");

    const first = list[0];
    banner.classList.remove("hidden");
    banner.innerHTML = `
      <div class="urgent-banner-inner">
        <div>
          <strong>⚠️ Conferma richiesta (${escapeHtml(first.projectName)})</strong>
          <div style="font-size:0.82rem;margin-top:2px;"><code>${escapeHtml(first.toolName)}</code>: ${escapeHtml(first.summary)}</div>
        </div>
        <div class="urgent-actions">
          <button class="btn btn-success btn-sm" onclick="decideConfirmation('${escapeAttr(first.id)}', 'allow')">✓ Approva</button>
          <button class="btn btn-danger btn-sm" onclick="decideConfirmation('${escapeAttr(first.id)}', 'deny')">✕ Rifiuta</button>
          <button class="btn btn-secondary btn-sm" onclick="switchTab('tab-confirmations')">Dettagli</button>
        </div>
      </div>
    `;
  } else {
    badge.classList.add("hidden");
    banner.classList.add("hidden");
  }

  if (list.length === 0) {
    container.innerHTML = `<div class="card"><p class="text-muted">Nessuna richiesta di conferma in attesa.</p></div>`;
    return;
  }

  container.innerHTML = list
    .map(
      (c) => `
      <div class="card">
        <div class="conv-top">
          <span class="project-tag">${escapeHtml(c.projectName)}</span>
          <span class="tool-pill">${escapeHtml(c.toolName)}</span>
        </div>
        <div class="conv-title">${escapeHtml(c.summary)}</div>
        <div class="code-box">${escapeHtml(c.commandLine || "")}</div>
        ${
          c.toolName === "run_command"
            ? `<label class="field-label">Modifica comando prima di eseguire (opzionale):</label>
               <input type="text" class="input" id="ov-${escapeAttr(c.id)}" value="${escapeAttr(c.commandLine)}" />`
            : ""
        }
        <div style="display:flex;gap:8px;">
          <button class="btn btn-success" style="flex:1" onclick="decideConfirmation('${escapeAttr(c.id)}', 'allow')">✓ Approva</button>
          <button class="btn btn-danger" style="flex:1" onclick="decideConfirmation('${escapeAttr(c.id)}', 'deny')">✕ Rifiuta</button>
        </div>
      </div>
    `
    )
    .join("");
}

async function decideConfirmation(reqId, decision) {
  const ovInput = document.getElementById(`ov-${reqId}`);
  const overwriteCommand = ovInput ? ovInput.value.trim() : "";

  await fetch(`${appState.apiBaseUrl}/api/confirmations/${encodeURIComponent(reqId)}/decide`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      decision,
      overwriteCommand,
      reason: decision === "deny" ? "Rifiutato dall'utente via PWA/Browser" : "",
    }),
  });
  fetchState();
}

async function triggerTestConfirmation() {
  await fetch(`${appState.apiBaseUrl}/api/hooks/test-confirm`, { method: "POST" });
  setTimeout(fetchState, 250);
}

function renderNotifications() {
  const container = document.getElementById("notifications-list");
  if (appState.notifications.length === 0) {
    container.innerHTML = `<div class="card"><p class="text-muted">Nessuna notifica recente.</p></div>`;
    return;
  }

  container.innerHTML = appState.notifications
    .map(
      (n) => `
      <div class="card" onclick="${n.conversationId && n.conversationId !== 'test-session' ? `openConversationDetail('${escapeAttr(n.conversationId)}')` : ""}">
        <div class="conv-top">
          <strong>${escapeHtml(n.title)}</strong>
          <small class="text-muted">${new Date(n.timestamp * 1000).toLocaleTimeString()}</small>
        </div>
        <div class="conv-preview">${escapeHtml(n.body)}</div>
      </div>
    `
    )
    .join("");
}

async function saveConfirmMode(confirmMode) {
  await fetch(`${appState.apiBaseUrl}/api/settings`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ confirmMode }),
  });
}

function switchTab(tabId) {
  closeConversationDetail();
  document.querySelectorAll(".tab-view").forEach((el) => el.classList.remove("active"));
  document.querySelectorAll(".nav-btn").forEach((el) => el.classList.remove("active"));
  document.getElementById(tabId).classList.add("active");
  const btn = document.querySelector(`.nav-btn[data-tab="${tabId}"]`);
  if (btn) btn.classList.add("active");
}

function escapeHtml(str) {
  return String(str || "")
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

function escapeAttr(str) {
  return String(str || "").replace(/'/g, "\\'").replace(/"/g, "&quot;");
}
