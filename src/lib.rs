//! Zed extension for the Sema language.
//!
//! Beyond the tree-sitter grammar and language config (declared in
//! `extension.toml`), this provides two runtime integrations backed by the
//! `sema` binary:
//!
//! - **Debug adapter** (`sema dap`) — launch-and-debug `.sema` programs.
//! - **MCP context server** (`sema mcp`) — expose Sema's tools to the agent panel.

use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerId, DebugAdapterBinary, DebugConfig,
    DebugRequest, DebugScenario, DebugTaskDefinition, Project, Result,
    StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest, Worktree,
};

struct SemaExtension;

impl SemaExtension {
    /// Resolve the `sema` binary: prefer a user-provided path, then the
    /// worktree's `PATH`, else fall back to the bare name (Zed resolves it).
    fn sema_binary(worktree: &Worktree, user_path: Option<String>) -> String {
        user_path
            .or_else(|| worktree.which("sema"))
            .unwrap_or_else(|| "sema".to_string())
    }
}

impl zed::Extension for SemaExtension {
    fn new() -> Self {
        SemaExtension
    }

    // ── MCP context server: `sema mcp` ──────────────────────────────────
    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        Ok(Command {
            command: "sema".to_string(),
            args: vec!["mcp".to_string()],
            env: vec![],
        })
    }

    // ── Debug adapter: `sema dap` ───────────────────────────────────────
    fn get_dap_binary(
        &mut self,
        _adapter_name: String,
        config: DebugTaskDefinition,
        user_provided_debug_adapter_path: Option<String>,
        worktree: &Worktree,
    ) -> Result<DebugAdapterBinary, String> {
        Ok(DebugAdapterBinary {
            command: Some(Self::sema_binary(
                worktree,
                user_provided_debug_adapter_path,
            )),
            arguments: vec!["dap".to_string()],
            envs: vec![],
            cwd: None,
            // stdio transport (no TCP connection)
            connection: None,
            request_args: StartDebuggingRequestArguments {
                configuration: config.config,
                request: StartDebuggingRequestArgumentsRequest::Launch,
            },
        })
    }

    fn dap_request_kind(
        &mut self,
        _adapter_name: String,
        _config: serde_json::Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest, String> {
        // Sema's DAP only supports launching a program (no attach).
        Ok(StartDebuggingRequestArgumentsRequest::Launch)
    }

    fn dap_config_to_scenario(&mut self, config: DebugConfig) -> Result<DebugScenario, String> {
        let program = match config.request {
            DebugRequest::Launch(launch) => launch.program,
            DebugRequest::Attach(_) => {
                return Err("Sema debugging only supports launch, not attach".to_string());
            }
        };
        let configuration = serde_json::json!({
            "request": "launch",
            "program": program,
            "stopOnEntry": config.stop_on_entry.unwrap_or(false),
        })
        .to_string();
        Ok(DebugScenario {
            label: config.label,
            adapter: config.adapter,
            build: None,
            config: configuration,
            tcp_connection: None,
        })
    }
}

zed::register_extension!(SemaExtension);
