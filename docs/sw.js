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

const CACHE_NAME = "antigravity-mobile-manager-pwa-v1";
const ASSETS = [
  "./",
  "./index.html",
  "./style.css",
  "./app.js",
  "./manifest.json",
  "./icon.svg"
];

self.addEventListener("install", (event) => {
  self.skipWaiting();
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => cache.addAll(ASSETS).catch(() => {}))
  );
});

self.addEventListener("activate", (event) => {
  event.waitUntil(self.clients.claim());
});

self.addEventListener("fetch", (event) => {
  // Non intercettare chiamate API o stream SSE verso il PC sulla porta 8420
  if (event.request.url.includes("/api/")) {
    return;
  }
  event.respondWith(
    fetch(event.request).catch(() => caches.match(event.request))
  );
});

// Gestione click sulle notifiche Browser (inclusi i pulsanti Approva / Rifiuta)
self.addEventListener("notificationclick", (event) => {
  const notif = event.notification;
  const data = notif.data || {};
  const action = event.action; // 'allow' | 'deny' | ''
  notif.close();

  if ((action === "allow" || action === "deny") && data.reqId && data.apiBaseUrl) {
    event.waitUntil(
      fetch(`${data.apiBaseUrl}/api/confirmations/${encodeURIComponent(data.reqId)}/decide`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          decision: action,
          reason: `Deciso da notifica Browser PWA (${action})`
        })
      }).catch(() => {})
    );
    return;
  }

  // Se l'utente tocca il corpo della notifica, porta in primo piano la PWA
  event.waitUntil(
    self.clients.matchAll({ type: "window", includeUncontrolled: true }).then((clientList) => {
      for (const client of clientList) {
        if ("focus" in client) {
          if (data.conversationId) {
            client.postMessage({ type: "OPEN_CONVERSATION", conversationId: data.conversationId });
          }
          return client.focus();
        }
      }
      if (self.clients.openWindow) {
        return self.clients.openWindow("./index.html");
      }
    })
  );
});
