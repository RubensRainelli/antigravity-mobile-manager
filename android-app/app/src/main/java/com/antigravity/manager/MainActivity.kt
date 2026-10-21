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

import android.Manifest
import android.app.Activity
import android.app.AlertDialog
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.graphics.Color
import android.os.Build
import android.os.Bundle
import android.view.Gravity
import android.webkit.WebChromeClient
import android.webkit.WebView
import android.webkit.WebViewClient
import android.widget.Button
import android.widget.EditText
import android.widget.FrameLayout

class MainActivity : Activity() {

    private lateinit var webView: WebView

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            if (checkSelfPermission(Manifest.permission.POST_NOTIFICATIONS) != PackageManager.PERMISSION_GRANTED) {
                requestPermissions(arrayOf(Manifest.permission.POST_NOTIFICATIONS), 100)
            }
        }

        val root = FrameLayout(this)
        root.setBackgroundColor(Color.parseColor("#0b0f19"))

        webView = WebView(this).apply {
            settings.javaScriptEnabled = true
            settings.domStorageEnabled = true
            webViewClient = WebViewClient()
            webChromeClient = WebChromeClient()
        }

        val ipBtn = Button(this).apply {
            text = "IP PC"
            textSize = 11f
            setBackgroundColor(Color.parseColor("#1e2942"))
            setTextColor(Color.WHITE)
            setOnClickListener { showServerUrlDialog() }
        }
        val btnParams = FrameLayout.LayoutParams(
            FrameLayout.LayoutParams.WRAP_CONTENT,
            FrameLayout.LayoutParams.WRAP_CONTENT
        ).apply {
            gravity = Gravity.BOTTOM or Gravity.END
            rightMargin = 24
            bottomMargin = 190
        }

        root.addView(webView, FrameLayout.LayoutParams.MATCH_PARENT, FrameLayout.LayoutParams.MATCH_PARENT)
        root.addView(ipBtn, btnParams)
        setContentView(root)

        // Clausola 3 PEFL v1.0: Controllo e approvazione obbligatoria in primo piano (max 7 giorni)
        ensurePeflAccepted {
            val url = getSavedServerUrl(this)
            if (url.isEmpty()) {
                showServerUrlDialog()
            } else {
                connectAndStartService(url)
            }
        }
    }

    private fun ensurePeflAccepted(onApproved: () -> Unit) {
        if (isPeflValid(this)) {
            onApproved()
            return
        }

        val peflText = """
            Personal and Educational Free License (PEFL v1.0)
            Copyright (c) 2026 Rubens Rainelli
            Email: rubens@rainelli.it | Tel: (+39) 342 811 5882
            GitHub: https://github.com/RubensRainelli/pefl-license-spec

            1. Uso Privato ed Educativo GRATUITO consentito.
            2. Uso Aziendale, Professionale o Commerciale SEVERAMENTE VIETATO senza autorizzazione scritta e pagamento della relativa fee (inclusi enti di culto/religiosi).
            3. Attribuzione obbligatoria a Rubens Rainelli e approvazione in primo piano (Clausola 3).
            4. Il Software è fornito "così com'è" senza alcuna garanzia.
        """.trimIndent()

        AlertDialog.Builder(this)
            .setTitle("Licenza PEFL v1.0 — Rubens Rainelli")
            .setMessage(peflText)
            .setCancelable(false)
            .setPositiveButton("Accetto (Ricorda 7 giorni)") { _, _ ->
                savePeflAcceptance(this, true)
                onApproved()
            }
            .setNeutralButton("Accetto (Solo ora)") { _, _ ->
                savePeflAcceptance(this, false)
                onApproved()
            }
            .setNegativeButton("Rifiuto (Esci)") { _, _ ->
                finish()
            }
            .show()
    }

    private fun connectAndStartService(url: String) {
        webView.loadUrl(url)
        val serviceIntent = Intent(this, AntigravityService::class.java)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            startForegroundService(serviceIntent)
        } else {
            startService(serviceIntent)
        }
    }

    private fun showServerUrlDialog() {
        val current = getSavedServerUrl(this).ifEmpty { "http://192.168.137.1:8420" }
        val input = EditText(this).apply {
            setText(current)
            setPadding(40, 30, 40, 30)
        }

        AlertDialog.Builder(this)
            .setTitle("Indirizzo PC Antigravity (Porta 8420)")
            .setMessage("Inserisci l'URL LAN o Hotspot PC mostrato dal server:")
            .setView(input)
            .setPositiveButton("Connetti") { _, _ ->
                val cleanUrl = input.text.toString().trim().trimEnd('/')
                saveServerUrl(this, cleanUrl)
                connectAndStartService(cleanUrl)
            }
            .setNegativeButton("Annulla", null)
            .show()
    }

    override fun onBackPressed() {
        if (webView.canGoBack()) {
            webView.goBack()
        } else {
            super.onBackPressed()
        }
    }

    companion object {
        private const val PREFS_NAME = "agy_prefs"
        private const val KEY_URL = "server_url"
        private const val KEY_PEFL_TS = "pefl_accepted_at"
        private const val SEVEN_DAYS_MS = 7L * 24L * 60L * 60L * 1000L

        fun getSavedServerUrl(ctx: Context): String {
            return ctx.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
                .getString(KEY_URL, "") ?: ""
        }

        fun saveServerUrl(ctx: Context, url: String) {
            ctx.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
                .edit()
                .putString(KEY_URL, url)
                .apply()
        }

        fun isPeflValid(ctx: Context): Boolean {
            val ts = ctx.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
                .getLong(KEY_PEFL_TS, 0L)
            val now = System.currentTimeMillis()
            return ts > 0L && now >= ts && (now - ts) < SEVEN_DAYS_MS
        }

        fun savePeflAcceptance(ctx: Context, remember7Days: Boolean) {
            val edit = ctx.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE).edit()
            if (remember7Days) {
                edit.putLong(KEY_PEFL_TS, System.currentTimeMillis())
            } else {
                edit.remove(KEY_PEFL_TS)
            }
            edit.apply()
        }
    }
}
