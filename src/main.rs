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

//! # Antigravity Mobile Manager (`antigravity-mobile-manager`)
//!
//! Unico binario cross-platform (Windows & Linux) a **zero dipendenze esterne**
//! che espone sulla porta **8420** il server compagno LAN & Hotspot PC per controllare
//! Google Antigravity dallo smartphone (via PWA o App Android nativa).
//!
//! ## Modalità di esecuzione:
//! 1. `antigravity-mobile-manager` (oppure `antigravity-mobile-manager serve`) -> Avvia il server HTTP + SSE su `:8420`
//! 2. `antigravity-mobile-manager hook pre-tool` -> Hook `PreToolUse` per conferma comandi/file dal telefono
//! 3. `antigravity-mobile-manager hook stop`     -> Hook `Stop` per notifica immediata di fine task

mod json;
mod pefl_guard;

use json::Json;
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const PORT: u16 = 8420;

// ============================================================================
// STRUTTURE DATI CONDIVISE (Stato del Server)
// ============================================================================

#[derive(Clone)]
struct PendingConfirmation {
    id: String,
    conversation_id: String,
    project_name: String,
    tool_name: String,
    summary: String,
    command_line: String,
    created_at: u64,
    decision: Option<ApprovalDecision>,
}

#[derive(Clone)]
struct ApprovalDecision {
    allow: bool,
    reason: String,
    overwrite_command: Option<String>,
}

#[derive(Clone)]
struct NotificationItem {
    id: String,
    kind: String,
    title: String,
    body: String,
    conversation_id: String,
    timestamp: u64,
}

struct AppState {
    /// Richieste di conferma (PreToolUse) in attesa
    approvals: HashMap<String, PendingConfirmation>,
    /// Storico recente delle notifiche (ultime 50)
    notifications: Vec<NotificationItem>,
    /// Client SSE (Server-Sent Events) connessi dallo smartphone
    sse_clients: Vec<TcpStream>,
    /// Modalità conferme: "smart" | "commands_only" | "auto_allow"
    confirm_mode: String,
}

type SharedState = Arc<(Mutex<AppState>, Condvar)>;

// ============================================================================
// ENTRYPOINT PRINCIPALE
// ============================================================================

fn main() {
    let args: Vec<String> = env::args().collect();

    // Modalità Hook (invocata in background da Antigravity IDE):
    if args.len() >= 3 && args[1] == "hook" {
        run_hook_mode(&args[2]);
        return;
    }

    // Modalità Foreground (`serve`): Verifica obbligatoria Licenza PEFL v1.0 (Clausola 3)
    pefl_guard::verify_pefl_compliance();

    run_server_mode();
}

// ============================================================================
// 1. MODALITÀ HOOK (`antigravity-mobile-manager hook pre-tool` / `stop`)
// ============================================================================

fn run_hook_mode(hook_type: &str) {
    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input);

    // Invia i dati dell'hook al server locale sulla porta 8420.
    // Se il server non è attivo, lascia procedere Antigravity normalmente (fail-open).
    let endpoint = match hook_type {
        "pre-tool" => "/api/internal/hook/pre-tool",
        "stop" => "/api/internal/hook/stop",
        _ => return,
    };

    if let Ok(response_body) = http_post_local(endpoint, &input) {
        if !response_body.trim().is_empty() {
            println!("{}", response_body.trim());
        }
    }
}

fn http_post_local(path: &str, body: &str) -> Result<String, std::io::Error> {
    let addr = format!("127.0.0.1:{}", PORT);
    let mut stream = TcpStream::connect_timeout(
        &addr.parse().unwrap(),
        Duration::from_millis(600),
    )?;
    // Timeout di lettura lungo (5 minuti) per attendere la risposta dell'utente sullo smartphone
    let _ = stream.set_read_timeout(Some(Duration::from_secs(300)));

    let req = format!(
        "POST {} HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        path,
        PORT,
        body.len(),
        body
    );
    stream.write_all(req.as_bytes())?;
    stream.flush()?;

    let mut raw_resp = String::new();
    stream.read_to_string(&mut raw_resp)?;

    if let Some(idx) = raw_resp.find("\r\n\r\n") {
        Ok(raw_resp[idx + 4..].to_string())
    } else {
        Ok(String::new())
    }
}

// ============================================================================
// 2. MODALITÀ SERVER (`:8420`)
// ============================================================================

fn run_server_mode() {
    let lan_interfaces = detect_all_lan_ips();
    let brain_dir = get_antigravity_brain_dir();

    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║  🚀 Antigravity Mobile Manager v1.0.0 - Attivo sulla porta {:<9}║", PORT);
    println!("╠══════════════════════════════════════════════════════════════════════╣");
    println!("║  📱 Indirizzi di rete rilevati per lo Smartphone (Porta {}):      ║", PORT);
    for (label, ip) in &lan_interfaces {
        let url = format!("http://{}:{}", ip, PORT);
        println!("║     • {:<22} ➜  {:<36}║", truncate_str(label, 22), url);
    }
    println!("║  🌐 Da GitHub Pages PWA: inserisci uno degli IP qui sopra nell'app  ║");
    println!("║  📂 Cartella Conversazioni: {:<40} ║", truncate_str(&brain_dir.display().to_string(), 40));
    println!("╚══════════════════════════════════════════════════════════════════════╝\n");

    let state: SharedState = Arc::new((
        Mutex::new(AppState {
            approvals: HashMap::new(),
            notifications: Vec::new(),
            sse_clients: Vec::new(),
            confirm_mode: "smart".to_string(),
        }),
        Condvar::new(),
    ));

    // Thread di monitoraggio continuo delle conversazioni (rileva cambi di stato, errori o domande)
    let watcher_state = Arc::clone(&state);
    thread::spawn(move || {
        conversation_watcher_loop(watcher_state);
    });

    let bind_addr = format!("0.0.0.0:{}", PORT);
    let listener = TcpListener::bind(&bind_addr).unwrap_or_else(|e| {
        eprintln!(
            "❌ Impossibile avviare il server su {}: {}. Verifica che la porta {} sia libera.",
            bind_addr, e, PORT
        );
        std::process::exit(1);
    });

    for stream in listener.incoming().flatten() {
        let st = Arc::clone(&state);
        thread::spawn(move || {
            handle_http_client(stream, st);
        });
    }
}

// ============================================================================
// GESTIONE RICHIESTE HTTP + CORS + PWA + API REST + SSE
// ============================================================================

fn handle_http_client(mut stream: TcpStream, state: SharedState) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(30)));
    let mut reader = match stream.try_clone() {
        Ok(s) => BufReader::new(s),
        Err(_) => return,
    };

    // 1. Legge la Request Line
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() || request_line.is_empty() {
        return;
    }
    let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }
    let method = parts[0];
    let full_path = parts[1];
    let (path, query) = match full_path.split_once('?') {
        Some((p, q)) => (p, q),
        None => (full_path, ""),
    };

    // 2. Legge gli Header HTTP
    let mut content_length = 0usize;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).is_err() || line == "\r\n" || line == "\n" || line.is_empty() {
            break;
        }
        let lower = line.to_ascii_lowercase();
        if lower.starts_with("content-length:") {
            if let Some(val) = line.split(':').nth(1) {
                content_length = val.trim().parse::<usize>().unwrap_or(0);
            }
        }
    }

    // 3. Risposta immediata preflight CORS (fondamentale per la PWA su GitHub Pages!)
    if method == "OPTIONS" {
        let resp = "HTTP/1.1 204 No Content\r\n\
            Access-Control-Allow-Origin: *\r\n\
            Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
            Access-Control-Allow-Headers: Content-Type, Authorization, X-Requested-With\r\n\
            Access-Control-Allow-Private-Network: true\r\n\
            Content-Length: 0\r\n\r\n";
        let _ = stream.write_all(resp.as_bytes());
        return;
    }

    // 4. Legge eventuale Body
    let mut body = String::new();
    if content_length > 0 && content_length < 2_000_000 {
        let mut buf = vec![0u8; content_length];
        if reader.read_exact(&mut buf).is_ok() {
            body = String::from_utf8_lossy(&buf).to_string();
        }
    }

    // 5. Routing API & Static PWA
    match (method, path) {
        ("GET", "/api/events") => {
            handle_sse_stream(stream, state);
        }
        ("GET", "/api/state") => {
            let payload = build_dashboard_state(&state);
            send_json(&mut stream, 200, &payload);
        }
        ("GET", p) if p.starts_with("/api/conversations/") && p.ends_with("/artifact") => {
            let trimmed = &p["/api/conversations/".len()..p.len() - "/artifact".len()];
            let conv_id = trimmed.trim_matches('/');
            let file_name = parse_query_param(query, "name");
            let detail = build_artifact_detail(conv_id, &file_name);
            send_json(&mut stream, 200, &detail);
        }
        ("GET", p) if p.starts_with("/api/conversations/") => {
            let conv_id = &p["/api/conversations/".len()..];
            let detail = build_conversation_detail(conv_id);
            send_json(&mut stream, 200, &detail);
        }
        ("POST", "/api/settings") => {
            if let Some(json) = Json::parse_opt(&body) {
                if let Some(mode) = json.get_str("confirmMode") {
                    if let Ok(mut guard) = state.0.lock() {
                        guard.confirm_mode = mode.to_string();
                    }
                }
            }
            send_json(&mut stream, 200, &Json::obj(vec![("ok", Json::Bool(true))]));
        }
        ("POST", "/api/pefl/accept") => {
            send_json(&mut stream, 200, &Json::obj(vec![("ok", Json::Bool(true))]));
        }
        ("POST", p) if p.starts_with("/api/confirmations/") && p.ends_with("/decide") => {
            let req_id = &p["/api/confirmations/".len()..p.len() - "/decide".len()];
            handle_approval_decision(&mut stream, &state, Some(req_id.trim_matches('/')), &body);
        }
        ("POST", "/api/approvals/decide") => {
            handle_approval_decision(&mut stream, &state, None, &body);
        }
        ("POST", "/api/prompt") | ("POST", "/api/prompt/send") => {
            handle_send_prompt(&mut stream, &state, &body);
        }
        ("POST", "/api/hooks/test-confirm") | ("POST", "/api/test-notification") => {
            handle_test_notification(&mut stream, &state);
        }
        ("POST", "/api/internal/hook/pre-tool") => {
            handle_hook_pre_tool(&mut stream, &state, &body);
        }
        ("POST", "/api/internal/hook/stop") => {
            handle_hook_stop(&mut stream, &state, &body);
        }
        ("GET", _) => {
            serve_static_pwa(&mut stream, path);
        }
        _ => {
            send_json(
                &mut stream,
                404,
                &Json::obj(vec![("error", Json::Str("Endpoint non trovato".into()))]),
            );
        }
    }
}

// ============================================================================
// SERVER-SENT EVENTS (Notifiche Push Real-Time su LAN / Hotspot)
// ============================================================================

fn handle_sse_stream(mut stream: TcpStream, state: SharedState) {
    let _ = stream.set_read_timeout(None);
    let headers = "HTTP/1.1 200 OK\r\n\
        Content-Type: text/event-stream\r\n\
        Cache-Control: no-cache\r\n\
        Connection: keep-alive\r\n\
        Access-Control-Allow-Origin: *\r\n\
        Access-Control-Allow-Private-Network: true\r\n\r\n";

    if stream.write_all(headers.as_bytes()).is_err() {
        return;
    }
    let _ = stream.write_all(b"data: {\"type\":\"connected\",\"status\":\"ok\"}\n\n");
    let _ = stream.flush();

    if let Ok(mut guard) = state.0.lock() {
        guard.sse_clients.push(stream);
    }
}

fn broadcast_sse_data(state: &SharedState, data: &Json) {
    let msg = format!("data: {}\n\n", data.stringify());
    if let Ok(mut guard) = state.0.lock() {
        guard.sse_clients.retain_mut(|client| {
            client.write_all(msg.as_bytes()).is_ok() && client.flush().is_ok()
        });
    }
}

fn push_notification(state: &SharedState, kind: &str, title: String, body: String, conv_id: String) {
    let item = NotificationItem {
        id: format!("notif_{}", now_millis()),
        kind: kind.to_string(),
        title: title.clone(),
        body: body.clone(),
        conversation_id: conv_id.clone(),
        timestamp: now_secs(),
    };

    if let Ok(mut guard) = state.0.lock() {
        guard.notifications.insert(0, item.clone());
        if guard.notifications.len() > 50 {
            guard.notifications.truncate(50);
        }
    }

    let ev = Json::obj(vec![
        ("type", Json::Str("notification".into())),
        ("id", Json::Str(item.id)),
        ("kind", Json::Str(item.kind)),
        ("title", Json::Str(title)),
        ("body", Json::Str(body)),
        ("conversationId", Json::Str(conv_id)),
        ("timestamp", Json::Number(item.timestamp as f64)),
    ]);
    broadcast_sse_data(state, &ev);
}

// ============================================================================
// LOGICA HOOKS: PRE-TOOL (Conferme da Smartphone) & STOP (Notifiche Fine Task)
// ============================================================================

fn handle_hook_pre_tool(stream: &mut TcpStream, state: &SharedState, body: &str) {
    let parsed = match Json::parse_opt(body) {
        Some(j) => j,
        None => {
            send_json(stream, 200, &Json::Object(BTreeMap::new()));
            return;
        }
    };

    let tool_name = parsed.get_str("tool_name").unwrap_or("unknown").to_string();
    let conv_id = parsed.get_str("conversation_id").unwrap_or("").to_string();
    let cwd = parsed.get_str("cwd").unwrap_or("").to_string();
    let project_name = if !cwd.is_empty() {
        Path::new(&cwd)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Antigravity")
            .to_string()
    } else {
        "Antigravity".to_string()
    };

    let tool_input = parsed.get("tool_input");

    // Estrae comando e descrizione
    let cmd_line = tool_input
        .and_then(|ti| {
            ti.get_str("CommandLine")
                .or_else(|| ti.get_str("command"))
                .or_else(|| ti.get_str("Path"))
        })
        .unwrap_or("")
        .to_string();
    let summary = tool_input
        .and_then(|ti| ti.get_str("toolSummary").or_else(|| ti.get_str("Path")))
        .unwrap_or(&tool_name)
        .to_string();

    let confirm_mode = state
        .0
        .lock()
        .map(|g| g.confirm_mode.clone())
        .unwrap_or_else(|_| "smart".to_string());

    let should_confirm = match confirm_mode.as_str() {
        "auto_allow" => false,
        "commands_only" => tool_name == "run_command" || tool_name == "Bash",
        _ => {
            // "smart" (default): chiede conferma per comandi terminale o scritture file
            tool_name == "run_command"
                || tool_name == "Bash"
                || tool_name == "edit_file"
                || tool_name == "write_file"
        }
    };

    if !should_confirm {
        send_json(stream, 200, &Json::Object(BTreeMap::new()));
        return;
    }

    let req_id = format!("req_{}", now_millis());
    let pending = PendingConfirmation {
        id: req_id.clone(),
        conversation_id: conv_id.clone(),
        project_name: project_name.clone(),
        tool_name: tool_name.clone(),
        summary: summary.clone(),
        command_line: cmd_line.clone(),
        created_at: now_secs(),
        decision: None,
    };

    {
        let mut guard = state.0.lock().unwrap();
        guard.approvals.insert(req_id.clone(), pending);
    }

    // Invia notifica immediata a tutti gli smartphone connessi (compatibile con PWA + Android Service)
    let ev = Json::obj(vec![
        ("type", Json::Str("confirm_request".into())),
        ("id", Json::Str(req_id.clone())),
        ("projectName", Json::Str(project_name)),
        ("toolName", Json::Str(tool_name.clone())),
        ("summary", Json::Str(summary.clone())),
        ("commandLine", Json::Str(cmd_line.clone())),
        ("conversationId", Json::Str(conv_id)),
        ("timestamp", Json::Number(now_secs() as f64)),
    ]);
    broadcast_sse_data(state, &ev);
    println!("📲 [Richiesta Conferma inviata allo Smartphone] {} -> {}", summary, cmd_line);

    // Attende la decisione dello smartphone (fino a 180 secondi)
    let decision = {
        let (lock, cvar) = &**state;
        let mut guard = lock.lock().unwrap();
        let timeout = Duration::from_secs(180);
        loop {
            if let Some(item) = guard.approvals.get(&req_id) {
                if let Some(dec) = &item.decision {
                    break Some(dec.clone());
                }
            }
            let (new_guard, timeout_res) = cvar.wait_timeout(guard, timeout).unwrap();
            guard = new_guard;
            if timeout_res.timed_out() {
                break None;
            }
        }
    };

    // Rimuove dalla coda attiva
    if let Ok(mut guard) = state.0.lock() {
        guard.approvals.remove(&req_id);
    }

    // Costruisce la risposta conforme alle specifiche Hook `PreToolUse` di Antigravity
    match decision {
        Some(dec) if dec.allow => {
            let mut specific = vec![
                ("hookEventName", Json::Str("PreToolUse".into())),
                ("permissionDecision", Json::Str("allow".into())),
                ("permissionDecisionReason", Json::Str(dec.reason)),
            ];
            if let Some(updated_cmd) = dec.overwrite_command {
                if !updated_cmd.trim().is_empty() && (tool_name == "run_command" || tool_name == "Bash") {
                    specific.push((
                        "updatedInput",
                        Json::obj(vec![("CommandLine", Json::Str(updated_cmd))]),
                    ));
                }
            }
            let out = Json::obj(vec![("hookSpecificOutput", Json::obj(specific))]);
            send_json(stream, 200, &out);
        }
        Some(dec) => {
            let out = Json::obj(vec![(
                "hookSpecificOutput",
                Json::obj(vec![
                    ("hookEventName", Json::Str("PreToolUse".into())),
                    ("permissionDecision", Json::Str("deny".into())),
                    ("permissionDecisionReason", Json::Str(dec.reason)),
                ]),
            )]);
            send_json(stream, 200, &out);
        }
        None => {
            let out = Json::obj(vec![]);
            send_json(stream, 200, &out);
        }
    }
}

fn handle_hook_stop(stream: &mut TcpStream, state: &SharedState, body: &str) {
    let conv_id = Json::parse_opt(body)
        .and_then(|j| j.get_str("conversation_id").map(|s| s.to_string()))
        .unwrap_or_else(|| "Sessione attiva".to_string());

    push_notification(
        state,
        "stop",
        "✅ Antigravity ha completato il task".to_string(),
        format!(
            "L'agente ({}) ha terminato ed è in attesa di istruzioni.",
            &conv_id[..conv_id.len().min(8)]
        ),
        conv_id,
    );
    send_json(stream, 200, &Json::Object(BTreeMap::new()));
}

fn handle_approval_decision(
    stream: &mut TcpStream,
    state: &SharedState,
    url_req_id: Option<&str>,
    body: &str,
) {
    let parsed = match Json::parse_opt(body) {
        Some(j) => j,
        None => {
            send_json(stream, 400, &Json::obj(vec![("error", Json::Str("JSON non valido".into()))]));
            return;
        }
    };

    let id = url_req_id
        .map(|s| s.to_string())
        .or_else(|| parsed.get_str("id").map(|s| s.to_string()))
        .unwrap_or_default();

    let allow = if let Some(dec_str) = parsed.get_str("decision") {
        dec_str == "allow"
    } else {
        parsed.get_bool("allow").unwrap_or(false)
    };

    let overwrite_command = parsed
        .get_str("overwriteCommand")
        .or_else(|| parsed.get_str("updatedCommand"))
        .map(|s| s.to_string());

    let reason = parsed
        .get_str("reason")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            if allow {
                "Approvato da remoto tramite Antigravity Mobile Manager".to_string()
            } else {
                "Rifiutato da remoto tramite Antigravity Mobile Manager".to_string()
            }
        });

    let mut found = false;
    {
        let (lock, cvar) = &**state;
        if let Ok(mut guard) = lock.lock() {
            if let Some(item) = guard.approvals.get_mut(&id) {
                item.decision = Some(ApprovalDecision {
                    allow,
                    reason: reason.clone(),
                    overwrite_command,
                });
                found = true;
                cvar.notify_all();
            }
        }
    }

    let event = Json::obj(vec![
        ("type", Json::Str("confirm_resolved".into())),
        ("id", Json::Str(id)),
        ("allow", Json::Bool(allow)),
        ("reason", Json::Str(reason)),
    ]);
    broadcast_sse_data(state, &event);

    send_json(
        stream,
        200,
        &Json::obj(vec![("ok", Json::Bool(found)), ("allow", Json::Bool(allow))]),
    );
}

fn handle_test_notification(stream: &mut TcpStream, state: &SharedState) {
    // Crea una richiesta di conferma simulata in background così l'utente può testare
    // immediatamente i pulsanti "Approva" e "Rifiuta" dal telefono o dalla notifica browser.
    thread::spawn(move || {
        let fake_hook = Json::obj(vec![
            ("tool_name", Json::Str("run_command".into())),
            ("cwd", Json::Str("Antigravity Mobile Manager".into())),
            ("conversation_id", Json::Str("test-session".into())),
            (
                "tool_input",
                Json::obj(vec![
                    ("CommandLine", Json::Str("cargo test --workspace --release".into())),
                    ("toolSummary", Json::Str("Esecuzione test suite di verifica".into())),
                ]),
            ),
        ]);
        let _ = http_post_local("/api/internal/hook/pre-tool", &fake_hook.stringify());
    });

    push_notification(
        state,
        "info",
        "🔔 Test Notifica Antigravity Mobile Manager".to_string(),
        "Connessione con lo smartphone attiva! È stata generata anche una richiesta di conferma di prova.".to_string(),
        "test-session".to_string(),
    );

    send_json(stream, 200, &Json::obj(vec![("ok", Json::Bool(true))]));
}

// ============================================================================
// INVIO NUOVI PROMPT (`agy` CLI / `agentapi` / Accodamento locale)
// ============================================================================

fn handle_send_prompt(stream: &mut TcpStream, state: &SharedState, body: &str) {
    let parsed = match Json::parse_opt(body) {
        Some(j) => j,
        None => {
            send_json(stream, 400, &Json::obj(vec![("error", Json::Str("JSON non valido".into()))]));
            return;
        }
    };

    let prompt = parsed.get_str("prompt").unwrap_or("").trim().to_string();
    let conversation_id = parsed.get_str("conversationId").unwrap_or("").trim().to_string();
    let workspace = parsed.get_str("workspace").unwrap_or("").trim().to_string();

    if prompt.is_empty() {
        send_json(stream, 400, &Json::obj(vec![("error", Json::Str("Prompt vuoto".into()))]));
        return;
    }

    println!(
        "💬 [Nuovo Prompt dallo Smartphone] Conv: '{}' | Prompt: '{}'",
        conversation_id, prompt
    );

    // 1. Tenta l'invio diretto tramite `agy` CLI (`agentapi`) se disponibile nel sistema
    let cli_result = dispatch_via_agy_cli(&conversation_id, &workspace, &prompt);

    // 2. Salva sempre anche nella coda locale di Antigravity Mobile Manager
    let inbox_path = save_to_remote_inbox(&conversation_id, &workspace, &prompt);

    let delivery_msg = match cli_result {
        Ok(msg) => msg,
        Err(_) => format!("Prompt registrato e pronto ({}).", inbox_path.display()),
    };

    push_notification(
        state,
        "prompt_sent",
        "🚀 Prompt inviato ad Antigravity".to_string(),
        prompt,
        conversation_id,
    );

    send_json(
        stream,
        200,
        &Json::obj(vec![
            ("ok", Json::Bool(true)),
            ("message", Json::Str(delivery_msg)),
        ]),
    );
}

fn dispatch_via_agy_cli(conversation_id: &str, workspace: &str, prompt: &str) -> Result<String, String> {
    let bin = if cfg!(target_os = "windows") { "agy.cmd" } else { "agy" };

    if !conversation_id.is_empty() && conversation_id != "new" {
        let status = Command::new(bin)
            .args(["chat", "--reuse-window", "-m", prompt])
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() {
            return Ok("Prompt inviato alla sessione attiva di Antigravity IDE.".to_string());
        }
    } else {
        let mut cmd = Command::new(bin);
        cmd.arg("chat").arg("-m").arg(prompt);
        if !workspace.is_empty() {
            cmd.current_dir(workspace);
        }
        let status = cmd.status().map_err(|e| e.to_string())?;
        if status.success() {
            return Ok("Nuova conversazione avviata in Antigravity IDE.".to_string());
        }
    }
    Err("CLI agy non disponibile nel PATH".to_string())
}

fn save_to_remote_inbox(conversation_id: &str, workspace: &str, prompt: &str) -> PathBuf {
    let dir = get_antigravity_base_dir().join("mobile_inbox");
    let _ = fs::create_dir_all(&dir);
    let path = dir.join("latest_mobile_prompt.json");

    let record = Json::obj(vec![
        ("timestamp", Json::Number(now_secs() as f64)),
        ("conversationId", Json::Str(conversation_id.to_string())),
        ("workspace", Json::Str(workspace.to_string())),
        ("prompt", Json::Str(prompt.to_string())),
    ]);
    let _ = fs::write(&path, record.stringify());
    path
}

// ============================================================================
// LETTURA PROGETTI, CONVERSAZIONI, SUBAGENTI E TRANSCRIPT JSONL
// ============================================================================

fn build_dashboard_state(state: &SharedState) -> Json {
    let (approvals_json, notifications_json, confirm_mode, pending_conv_ids) = {
        let guard = state.0.lock().unwrap();
        let mut list = Vec::new();
        let mut conv_ids = Vec::new();
        for a in guard.approvals.values() {
            if a.decision.is_none() {
                if !a.conversation_id.is_empty() {
                    conv_ids.push(a.conversation_id.clone());
                }
                list.push(Json::obj(vec![
                    ("id", Json::Str(a.id.clone())),
                    ("conversationId", Json::Str(a.conversation_id.clone())),
                    ("projectName", Json::Str(a.project_name.clone())),
                    ("toolName", Json::Str(a.tool_name.clone())),
                    ("summary", Json::Str(a.summary.clone())),
                    ("commandLine", Json::Str(a.command_line.clone())),
                    ("createdAt", Json::Number(a.created_at as f64)),
                ]));
            }
        }
        let notifs: Vec<Json> = guard
            .notifications
            .iter()
            .map(|n| {
                Json::obj(vec![
                    ("id", Json::Str(n.id.clone())),
                    ("kind", Json::Str(n.kind.clone())),
                    ("title", Json::Str(n.title.clone())),
                    ("body", Json::Str(n.body.clone())),
                    ("conversationId", Json::Str(n.conversation_id.clone())),
                    ("timestamp", Json::Number(n.timestamp as f64)),
                ])
            })
            .collect();
        (
            Json::Array(list),
            Json::Array(notifs),
            guard.confirm_mode.clone(),
            conv_ids,
        )
    };

    let conversations = scan_conversations(&pending_conv_ids);

    // Calcola il conteggio conversazioni per ciascun progetto
    let mut project_counts: BTreeMap<String, usize> = BTreeMap::new();
    for c in &conversations {
        if let Some(pname) = c.get_str("projectName") {
            *project_counts.entry(pname.to_string()).or_insert(0) += 1;
        }
    }
    let projects_json: Vec<Json> = project_counts
        .into_iter()
        .map(|(name, count)| {
            Json::obj(vec![
                ("name", Json::Str(name)),
                ("conversationCount", Json::Number(count as f64)),
            ])
        })
        .collect();

    let lan_interfaces = detect_all_lan_ips();
    let primary_ip = lan_interfaces
        .first()
        .map(|(_, ip)| ip.clone())
        .unwrap_or_else(|| "127.0.0.1".to_string());

    let lan_urls_json: Vec<Json> = lan_interfaces
        .iter()
        .map(|(label, ip)| {
            Json::obj(vec![
                ("label", Json::Str(label.clone())),
                ("ip", Json::Str(ip.clone())),
                ("url", Json::Str(format!("http://{}:{}", ip, PORT))),
            ])
        })
        .collect();

    Json::obj(vec![
        ("serverVersion", Json::Str("1.0.0".into())),
        ("os", Json::Str(env::consts::OS.into())),
        ("lanIp", Json::Str(primary_ip.clone())),
        ("port", Json::Number(PORT as f64)),
        ("lanUrl", Json::Str(format!("http://{}:{}", primary_ip, PORT))),
        ("lanUrls", Json::Array(lan_urls_json)),
        ("confirmMode", Json::Str(confirm_mode)),
        ("pendingConfirmations", approvals_json),
        ("notifications", notifications_json),
        ("projects", Json::Array(projects_json)),
        ("conversations", Json::Array(conversations)),
        ("timestamp", Json::Number(now_secs() as f64)),
    ])
}

fn scan_conversations(pending_conv_ids: &[String]) -> Vec<Json> {
    let brain_dir = get_antigravity_brain_dir();
    let mut entries_with_time: Vec<(u64, Json)> = Vec::new();

    let read_dir = match fs::read_dir(&brain_dir) {
        Ok(rd) => rd,
        Err(_) => return Vec::new(),
    };

    for entry in read_dir.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let conv_id = match path.file_name().and_then(|n| n.to_str()) {
            Some(s) => s.to_string(),
            None => continue,
        };

        let transcript_path = path
            .join(".system_generated")
            .join("logs")
            .join("transcript.jsonl");

        if !transcript_path.exists() {
            continue;
        }

        let modified_secs = fs::metadata(&transcript_path)
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let has_pending_confirm = pending_conv_ids.iter().any(|id| id == &conv_id);
        let summary = summarize_transcript(
            &conv_id,
            &path,
            &transcript_path,
            modified_secs,
            has_pending_confirm,
        );
        entries_with_time.push((modified_secs, summary));
    }

    entries_with_time.sort_by(|a, b| b.0.cmp(&a.0));
    entries_with_time.into_iter().take(25).map(|(_, j)| j).collect()
}

fn summarize_transcript(
    conv_id: &str,
    conv_dir: &Path,
    transcript_path: &Path,
    modified_secs: u64,
    has_pending_confirm: bool,
) -> Json {
    let content = fs::read_to_string(transcript_path).unwrap_or_default();
    let mut title = String::new();
    let mut workspace = String::new();
    let mut last_message = String::new();
    let mut step_count = 0usize;
    let mut is_subagent = false;
    let mut waiting_question = false;

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        step_count += 1;

        if workspace.is_empty() && line.contains("Active Workspaces:") {
            if let Some(pos) = line.find("Active Workspaces:") {
                let snippet = &line[pos..];
                for part in snippet.split("\\n") {
                    let trimmed = part.trim_start_matches('-').trim();
                    if trimmed.contains(":\\") || trimmed.starts_with('/') {
                        workspace = trimmed.to_string();
                        break;
                    }
                }
            }
        }
        if line.contains("Subagent") && step_count <= 2 {
            is_subagent = true;
        }

        if let Some(step) = Json::parse_opt(line) {
            let step_type = step.get_str("type").unwrap_or("");

            if step_type == "USER_INPUT" {
                waiting_question = false;
                if let Some(c) = step.get_str("content") {
                    let clean = clean_user_request_tags(c);
                    if !clean.is_empty() {
                        if title.is_empty() {
                            title = truncate_str(&clean, 80);
                        }
                        last_message = truncate_str(&clean, 140);
                    }
                }
            } else if step_type == "PLANNER_RESPONSE" {
                if let Some(c) = step.get_str("content") {
                    if !c.trim().is_empty() {
                        last_message = truncate_str(c.trim(), 160);
                    }
                }
                if let Some(tool_calls) = step.get_arr("tool_calls") {
                    for tc in tool_calls {
                        let name = tc.get_str("name").unwrap_or("");
                        if name.contains("ask_question") {
                            waiting_question = true;
                        }
                    }
                }
            }
        }
    }

    if title.is_empty() {
        title = format!("Conversazione {}", &conv_id[..conv_id.len().min(8)]);
    }

    let project_name = if !workspace.is_empty() {
        Path::new(&workspace)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&workspace)
            .to_string()
    } else {
        "Antigravity IDE".to_string()
    };

    let mut artifacts = Vec::new();
    if let Ok(rd) = fs::read_dir(conv_dir) {
        for f in rd.flatten() {
            if let Some(name) = f.file_name().to_str() {
                if name.ends_with(".md") {
                    artifacts.push(Json::Str(name.to_string()));
                }
            }
        }
    }

    let is_active = now_secs().saturating_sub(modified_secs) < 20;
    let status_str = if has_pending_confirm {
        "WAITING_CONFIRM"
    } else if waiting_question {
        "WAITING_QUESTION"
    } else if is_active {
        "ACTIVE"
    } else {
        "IDLE"
    };

    Json::obj(vec![
        ("id", Json::Str(conv_id.to_string())),
        ("title", Json::Str(title)),
        ("projectName", Json::Str(project_name)),
        ("workspace", Json::Str(workspace)),
        ("isSubagent", Json::Bool(is_subagent)),
        ("status", Json::Str(status_str.to_string())),
        ("lastMessage", Json::Str(last_message)),
        ("stepCount", Json::Number(step_count as f64)),
        ("updatedAt", Json::Number(modified_secs as f64)),
        ("artifacts", Json::Array(artifacts)),
    ])
}

fn build_conversation_detail(conv_id: &str) -> Json {
    let safe_id: String = conv_id
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect();
    let conv_dir = get_antigravity_brain_dir().join(&safe_id);
    let transcript_path = conv_dir
        .join(".system_generated")
        .join("logs")
        .join("transcript.jsonl");

    let content = fs::read_to_string(&transcript_path).unwrap_or_default();
    let mut steps: Vec<Json> = Vec::new();
    let mut workspace = String::new();

    for line in content.lines() {
        if workspace.is_empty() && line.contains("Active Workspaces:") {
            if let Some(pos) = line.find("Active Workspaces:") {
                let snippet = &line[pos..];
                for part in snippet.split("\\n") {
                    let trimmed = part.trim_start_matches('-').trim();
                    if trimmed.contains(":\\") || trimmed.starts_with('/') {
                        workspace = trimmed.to_string();
                        break;
                    }
                }
            }
        }

        let step = match Json::parse_opt(line) {
            Some(s) => s,
            None => continue,
        };
        let step_type = step.get_str("type").unwrap_or("");

        if step_type == "USER_INPUT" {
            let text = clean_user_request_tags(step.get_str("content").unwrap_or(""));
            if !text.is_empty() {
                steps.push(Json::obj(vec![
                    ("type", Json::Str("USER_INPUT".into())),
                    ("content", Json::Str(text)),
                ]));
            }
        } else if step_type == "PLANNER_RESPONSE" {
            let text = step.get_str("content").unwrap_or("").trim().to_string();
            let thinking = step.get_str("thinking").unwrap_or("").trim().to_string();
            let mut tools = Vec::new();
            if let Some(tcs) = step.get_arr("tool_calls") {
                for tc in tcs {
                    let tname = tc
                        .get_str("name")
                        .unwrap_or("tool")
                        .trim_start_matches("default_api:");
                    let args_str = tc
                        .get("arguments")
                        .or_else(|| tc.get("args"))
                        .map(|a| a.stringify())
                        .unwrap_or_default();
                    tools.push(Json::obj(vec![
                        ("name", Json::Str(tname.to_string())),
                        ("args", Json::Str(args_str)),
                    ]));
                }
            }
            if !text.is_empty() || !tools.is_empty() {
                steps.push(Json::obj(vec![
                    ("type", Json::Str("PLANNER_RESPONSE".into())),
                    ("content", Json::Str(text)),
                    ("thinking", Json::Str(truncate_str(&thinking, 450))),
                    ("toolCalls", Json::Array(tools)),
                ]));
            }
        }
    }

    let len = steps.len();
    let recent = if len > 45 {
        steps.into_iter().skip(len - 45).collect()
    } else {
        steps
    };

    let project_name = if !workspace.is_empty() {
        Path::new(&workspace)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&workspace)
            .to_string()
    } else {
        "Antigravity IDE".to_string()
    };

    let mut artifacts = Vec::new();
    if let Ok(rd) = fs::read_dir(&conv_dir) {
        for f in rd.flatten() {
            if let Some(name) = f.file_name().to_str() {
                if name.ends_with(".md") {
                    artifacts.push(Json::Str(name.to_string()));
                }
            }
        }
    }

    Json::obj(vec![
        ("id", Json::Str(safe_id)),
        ("projectName", Json::Str(project_name)),
        ("steps", Json::Array(recent)),
        ("artifacts", Json::Array(artifacts)),
    ])
}

fn build_artifact_detail(conv_id: &str, file_name: &str) -> Json {
    let safe_id: String = conv_id
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect();
    let safe_name: String = file_name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
        .collect();

    let path = get_antigravity_brain_dir().join(&safe_id).join(&safe_name);
    match fs::read_to_string(&path) {
        Ok(content) => Json::obj(vec![
            ("name", Json::Str(safe_name)),
            ("content", Json::Str(content)),
        ]),
        Err(_) => Json::obj(vec![("error", Json::Str("Artefatto non trovato".into()))]),
    }
}

// ============================================================================
// MONITORAGGIO BACKGROUND DELLE CONVERSAZIONI (Notifiche Automatiche)
// ============================================================================

fn conversation_watcher_loop(state: SharedState) {
    let mut last_seen_steps: HashMap<String, usize> = HashMap::new();
    let mut initialized = false;

    loop {
        thread::sleep(Duration::from_secs(3));
        let convs = scan_conversations(&[]);

        for conv in &convs {
            let id = conv.get_str("id").unwrap_or("").to_string();
            let steps = conv.get_num("stepCount").unwrap_or(0.0) as usize;
            let title = conv.get_str("title").unwrap_or("Agente").to_string();
            let last_msg = conv.get_str("lastMessage").unwrap_or("").to_string();
            let conv_status = conv.get_str("status").unwrap_or("IDLE");

            if let Some(&prev_steps) = last_seen_steps.get(&id) {
                if initialized && steps > prev_steps {
                    let ev = Json::obj(vec![
                        ("type", Json::Str("transcript_updated".into())),
                        ("conversationId", Json::Str(id.clone())),
                    ]);
                    broadcast_sse_data(&state, &ev);

                    if conv_status == "WAITING_QUESTION" || last_msg.ends_with('?') {
                        push_notification(
                            &state,
                            "question",
                            format!("❓ Domanda da {}", truncate_str(&title, 35)),
                            last_msg,
                            id.clone(),
                        );
                    }
                }
            }
            last_seen_steps.insert(id, steps);
        }
        initialized = true;
    }
}

// ============================================================================
// SERVER FILE STATICI PWA (Cartella `docs/`)
// ============================================================================

fn serve_static_pwa(stream: &mut TcpStream, req_path: &str) {
    let clean = match req_path {
        "/" | "/index.html" => "index.html",
        p => p.trim_start_matches('/'),
    };
    if clean.contains("..") {
        send_json(stream, 403, &Json::obj(vec![("error", Json::Str("Forbidden".into()))]));
        return;
    }

    let candidates = [
        PathBuf::from("docs").join(clean),
        env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("docs").join(clean)))
            .unwrap_or_else(|| PathBuf::from("docs").join(clean)),
    ];

    for file_path in &candidates {
        if let Ok(bytes) = fs::read(file_path) {
            let mime = match file_path.extension().and_then(|e| e.to_str()).unwrap_or("") {
                "html" => "text/html; charset=utf-8",
                "css" => "text/css; charset=utf-8",
                "js" => "application/javascript; charset=utf-8",
                "json" | "webmanifest" => "application/manifest+json; charset=utf-8",
                "svg" => "image/svg+xml",
                "png" => "image/png",
                _ => "application/octet-stream",
            };
            let header = format!(
                "HTTP/1.1 200 OK\r\n\
                Content-Type: {}\r\n\
                Content-Length: {}\r\n\
                Cache-Control: no-cache\r\n\
                Access-Control-Allow-Origin: *\r\n\
                Access-Control-Allow-Private-Network: true\r\n\
                Connection: close\r\n\r\n",
                mime,
                bytes.len()
            );
            let _ = stream.write_all(header.as_bytes());
            let _ = stream.write_all(&bytes);
            return;
        }
    }

    send_json(
        stream,
        404,
        &Json::obj(vec![("error", Json::Str("File PWA non trovato".into()))]),
    );
}

// ============================================================================
// UTILITY DI RETE MULTI-IP (LAN + HOTSPOT PC WINDOWS & LINUX)
// ============================================================================

fn send_json(stream: &mut TcpStream, status: u16, data: &Json) {
    let status_text = match status {
        200 => "OK",
        400 => "Bad Request",
        403 => "Forbidden",
        404 => "Not Found",
        _ => "OK",
    };
    let body = data.stringify();
    let resp = format!(
        "HTTP/1.1 {} {}\r\n\
        Content-Type: application/json; charset=utf-8\r\n\
        Content-Length: {}\r\n\
        Access-Control-Allow-Origin: *\r\n\
        Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
        Access-Control-Allow-Headers: Content-Type\r\n\
        Access-Control-Allow-Private-Network: true\r\n\
        Connection: close\r\n\r\n{}",
        status,
        status_text,
        body.len(),
        body
    );
    let _ = stream.write_all(resp.as_bytes());
    let _ = stream.flush();
}

/// Rileva tutti gli indirizzi IPv4 attivi sul PC (inclusi Hotspot Mobile di Windows `192.168.137.1`,
/// Wi-Fi ed Ethernet) restituendo coppie `(Etichetta Interfaccia, Indirizzo IPv4)`.
fn detect_all_lan_ips() -> Vec<(String, String)> {
    let mut results: Vec<(String, String)> = Vec::new();

    if cfg!(target_os = "windows") {
        if let Ok(output) = Command::new("ipconfig").output() {
            let text = String::from_utf8_lossy(&output.stdout);
            let mut current_adapter = String::from("Rete LAN");

            for raw_line in text.lines() {
                let line = raw_line.trim_end();
                if line.is_empty() {
                    continue;
                }
                if !line.starts_with(' ') && !line.starts_with('\t') && line.ends_with(':') {
                    let header = line.trim_end_matches(':').trim();
                    current_adapter = classify_windows_adapter(header);
                } else if line.contains("IPv4") && line.contains(':') {
                    if let Some(ip_part) = line.split(':').last() {
                        let ip = ip_part
                            .trim()
                            .trim_end_matches("(Preferito)")
                            .trim_end_matches("(Preferred)")
                            .trim();
                        if is_valid_private_ipv4(ip) && !results.iter().any(|(_, existing)| existing == ip) {
                            let label = if ip.starts_with("192.168.137.") {
                                "Hotspot PC Windows".to_string()
                            } else {
                                current_adapter.clone()
                            };
                            results.push((label, ip.to_string()));
                        }
                    }
                }
            }
        }
    } else {
        if let Ok(output) = Command::new("ip").args(["-o", "-4", "addr", "show"]).output() {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let iface = parts[1];
                    if iface == "lo" || iface.starts_with("docker") || iface.starts_with("veth") {
                        continue;
                    }
                    if let Some(ip) = parts[3].split('/').next() {
                        if is_valid_private_ipv4(ip) && !results.iter().any(|(_, existing)| existing == ip) {
                            let label = if iface.starts_with("wl") || iface.contains("ap") || iface.contains("hot") {
                                format!("Wi-Fi / Hotspot ({})", iface)
                            } else {
                                format!("Rete ({})", iface)
                            };
                            results.push((label, ip.to_string()));
                        }
                    }
                }
            }
        }
    }

    if let Ok(sock) = UdpSocket::bind("0.0.0.0:0") {
        if sock.connect("8.8.8.8:80").is_ok() {
            if let Ok(local_addr) = sock.local_addr() {
                let ip = local_addr.ip().to_string();
                if !results.iter().any(|(_, existing)| existing == &ip) {
                    results.push(("Rete Principale".to_string(), ip));
                }
            }
        }
    }

    // Ordina mettendo prima l'Hotspot PC (se attivo) così chi usa l'hotspot del PC lo vede subito in cima,
    // seguito dalla rete Wi-Fi / Ethernet.
    results.sort_by_key(|(label, ip)| {
        if ip.starts_with("192.168.137.") || label.contains("Hotspot") {
            0
        } else if label.contains("Wi-Fi") {
            1
        } else {
            2
        }
    });

    if results.is_empty() {
        results.push(("Localhost".to_string(), "127.0.0.1".to_string()));
    }

    results
}

fn classify_windows_adapter(header: &str) -> String {
    let lower = header.to_ascii_lowercase();
    if lower.contains("connessione alla rete locale (lan)*")
        || lower.contains("local area connection*")
        || lower.contains("wi-fi direct")
    {
        "Hotspot PC Windows".to_string()
    } else if lower.contains("wi-fi") || lower.contains("wireless") || lower.contains("wlan") {
        "Wi-Fi".to_string()
    } else if lower.contains("ethernet") {
        "Ethernet LAN".to_string()
    } else {
        header.to_string()
    }
}

fn is_valid_private_ipv4(ip: &str) -> bool {
    let octets: Vec<u8> = ip.split('.').filter_map(|s| s.parse::<u8>().ok()).collect();
    if octets.len() != 4 {
        return false;
    }
    if octets[0] == 127 || (octets[0] == 169 && octets[1] == 254) {
        return false;
    }
    true
}

fn parse_query_param(query: &str, key: &str) -> String {
    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            if k == key {
                return v.replace("%20", " ");
            }
        }
    }
    String::new()
}

fn get_antigravity_base_dir() -> PathBuf {
    let home = env::var("USERPROFILE")
        .or_else(|_| env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".gemini").join("antigravity")
}

fn get_antigravity_brain_dir() -> PathBuf {
    get_antigravity_base_dir().join("brain")
}

fn clean_user_request_tags(raw: &str) -> String {
    if let (Some(start), Some(end)) = (raw.find("<USER_REQUEST>"), raw.find("</USER_REQUEST>")) {
        raw[start + 14..end].trim().to_string()
    } else {
        raw.trim().to_string()
    }
}

fn truncate_str(s: &str, max_chars: usize) -> String {
    let count = s.chars().count();
    if count <= max_chars {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(max_chars.saturating_sub(1)).collect();
        out.push('…');
        out
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn now_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}
