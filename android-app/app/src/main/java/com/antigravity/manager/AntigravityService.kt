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

package com.antigravity.manager

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service
import android.content.Context
import android.content.Intent
import android.os.Build
import android.os.IBinder
import org.json.JSONObject
import java.io.BufferedReader
import java.io.InputStreamReader
import java.net.HttpURLConnection
import java.net.URL
import kotlin.concurrent.thread

class AntigravityService : Service() {

    @Volatile
    private var running = false

    override fun onCreate() {
        super.onCreate()
        createChannels()
        startForeground(1, buildForegroundNotification("Connessione ad Antigravity PC (:8420) attiva"))
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        if (!running) {
            running = true
            thread(start = true, isDaemon = true) {
                listenSseLoop()
            }
        }
        return START_STICKY
    }

    override fun onDestroy() {
        running = false
        super.onDestroy()
    }

    override fun onBind(intent: Intent?): IBinder? = null

    private fun listenSseLoop() {
        while (running) {
            val baseUrl = MainActivity.getSavedServerUrl(this)
            if (baseUrl.isEmpty()) {
                Thread.sleep(3000)
                continue
            }

            var conn: HttpURLConnection? = null
            try {
                val url = URL("$baseUrl/api/events")
                conn = (url.openConnection() as HttpURLConnection).apply {
                    connectTimeout = 5000
                    readTimeout = 0
                    setRequestProperty("Accept", "text/event-stream")
                }

                val reader = BufferedReader(InputStreamReader(conn.inputStream))
                var line: String?
                while (running && reader.readLine().also { line = it } != null) {
                    val l = line ?: continue
                    if (l.startsWith("data:")) {
                        val jsonStr = l.removePrefix("data:").trim()
                        handleEventJson(jsonStr)
                    }
                }
            } catch (_: Exception) {
            } finally {
                conn?.disconnect()
            }

            Thread.sleep(4000)
        }
    }

    private fun handleEventJson(jsonStr: String) {
        try {
            val ev = JSONObject(jsonStr)
            when (ev.optString("type")) {
                "confirm_request" -> {
                    val reqId = ev.optString("id")
                    val proj = ev.optString("projectName", "Antigravity")
                    val tool = ev.optString("toolName")
                    val summary = ev.optString("summary")
                    showConfirmationNotification(reqId, "Conferma richiesta ($proj)", "[$tool] $summary")
                }
                "notification" -> {
                    val kind = ev.optString("kind")
                    if (kind != "confirm") {
                        val id = ev.optInt("id", System.currentTimeMillis().toInt())
                        val title = ev.optString("title", "Antigravity")
                        val body = ev.optString("body", "")
                        showStandardNotification(id + 1000, title, body)
                    }
                }
            }
        } catch (_: Exception) {}
    }

    private fun showConfirmationNotification(reqId: String, title: String, body: String) {
        val notifId = reqId.hashCode()
        val openIntent = PendingIntent.getActivity(
            this, 0, Intent(this, MainActivity::class.java),
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        val allowIntent = Intent(this, ActionReceiver::class.java).apply {
            action = "ACTION_DECIDE"
            putExtra("reqId", reqId)
            putExtra("decision", "allow")
            putExtra("notifId", notifId)
        }
        val denyIntent = Intent(this, ActionReceiver::class.java).apply {
            action = "ACTION_DECIDE"
            putExtra("reqId", reqId)
            putExtra("decision", "deny")
            putExtra("notifId", notifId)
        }

        val allowPending = PendingIntent.getBroadcast(
            this, notifId, allowIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )
        val denyPending = PendingIntent.getBroadcast(
            this, notifId + 1, denyIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        val builder = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            Notification.Builder(this, CHANNEL_ALERTS)
        } else {
            Notification.Builder(this)
        }

        val notif = builder
            .setSmallIcon(android.R.drawable.ic_dialog_alert)
            .setContentTitle(title)
            .setContentText(body)
            .setStyle(Notification.BigTextStyle().bigText(body))
            .setContentIntent(openIntent)
            .setAutoCancel(true)
            .addAction(Notification.Action.Builder(0, "✓ Approva", allowPending).build())
            .addAction(Notification.Action.Builder(0, "✕ Rifiuta", denyPending).build())
            .build()

        val nm = getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        nm.notify(notifId, notif)
    }

    private fun showStandardNotification(id: Int, title: String, body: String) {
        val openIntent = PendingIntent.getActivity(
            this, 0, Intent(this, MainActivity::class.java),
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )
        val builder = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            Notification.Builder(this, CHANNEL_ALERTS)
        } else {
            Notification.Builder(this)
        }
        val notif = builder
            .setSmallIcon(android.R.drawable.ic_dialog_info)
            .setContentTitle(title)
            .setContentText(body)
            .setContentIntent(openIntent)
            .setAutoCancel(true)
            .build()

        val nm = getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        nm.notify(id, notif)
    }

    private fun buildForegroundNotification(text: String): Notification {
        val openIntent = PendingIntent.getActivity(
            this, 0, Intent(this, MainActivity::class.java),
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )
        val builder = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            Notification.Builder(this, CHANNEL_BG)
        } else {
            Notification.Builder(this)
        }
        return builder
            .setSmallIcon(android.R.drawable.stat_notify_sync)
            .setContentTitle("Antigravity Mobile Manager (PEFL v1.0)")
            .setContentText(text)
            .setContentIntent(openIntent)
            .build()
    }

    private fun createChannels() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val nm = getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
            nm.createNotificationChannel(
                NotificationChannel(CHANNEL_BG, "Connessione LAN", NotificationManager.IMPORTANCE_LOW)
            )
            nm.createNotificationChannel(
                NotificationChannel(CHANNEL_ALERTS, "Conferme e Avvisi Agenti", NotificationManager.IMPORTANCE_HIGH)
            )
        }
    }

    companion object {
        private const val CHANNEL_BG = "agy_lan_bg"
        private const val CHANNEL_ALERTS = "agy_alerts"
    }
}
