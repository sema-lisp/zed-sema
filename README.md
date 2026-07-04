<div align="center">

<img src="https://sema-lang.com/logo.svg" alt="Sema" height="64">

# Sema for Zed

**[Sema](https://sema-lang.com) support for [Zed](https://zed.dev)** — grammar, debugger, and MCP server.

[![CI](https://img.shields.io/github/actions/workflow/status/sema-lisp/zed-sema/ci.yml?branch=main&label=CI&logo=github)](https://github.com/sema-lisp/zed-sema/actions)
[![License](https://img.shields.io/github/license/sema-lisp/zed-sema?color=c8a855)](LICENSE)
[![Website](https://img.shields.io/badge/website-sema--lang.com-c8a855)](https://sema-lang.com)

</div>

Language support for [Sema](https://sema-lang.com) — a Lisp dialect with first-class LLM primitives — in the [Zed](https://zed.dev) editor.

## Install

From inside Zed: `cmd-shift-P` → `zed: extensions` → search for **Sema** → Install.

> The extension is pending publish to the Zed extension registry. Until it lands there, install it as a dev extension:
>
> 1. Clone this repository: `git clone https://github.com/sema-lisp/zed-sema.git`
> 2. In Zed, open the command palette (`cmd-shift-P`) → `zed: install dev extension`
> 3. Select the cloned `zed-sema` directory.

The [tree-sitter-sema](https://github.com/sema-lisp/tree-sitter-sema) grammar is fetched automatically by Zed at the commit pinned in `extension.toml` — no manual grammar setup is required.

## Features

- **Syntax highlighting** — special forms, LLM primitives (`llm/chat`, `defagent`, `deftool`, …), slash-namespaced builtins (`string/trim`, `json/encode`), keyword literals (`:foo`), booleans, `nil`, numbers, characters, strings, and quote/quasiquote/unquote operators.
- **Comments** — `;` line comments and `#| … |#` block comments, with TODO/FIXME highlighting injected inside them.
- **Auto-pairs & bracket matching** — `()`, `[]`, `{}`, and `""`.
- **Code outline** — top-level `define`, `defun`, `defn`, `defmacro`, `defagent`, `deftool`, `define-record-type`, `lambda`/`fn`, block forms (`let`, `let*`, `letrec`, `begin`, `do`, `cond`, `case`, `when`, `unless`, `try`), and `module`/`import`.
- **Auto-indent** — 2-space indentation for lists, vectors, and hash maps; outdent on closing delimiters.
- **Vim text objects** — `af`/`if` for function definitions (`define`, `defun`, `lambda`, `fn`, `defmacro`), `ac`/`ic` for agents and tools (`defagent`, `deftool`), plus comment objects.
- **Runnables & tasks** — a gutter play button (▶) to run the whole file (`sema-run`) or evaluate the selected form (`sema-run-form`), backed by the `sema` CLI.
- **Debugging (DAP)** — launch-and-debug `.sema` programs via Sema's debug adapter (`sema dap`): breakpoints, stepping, and variable inspection. Pick **Sema** in Zed's debugger, or add a launch config (see below).
- **MCP server** — the extension registers Sema's MCP context server (`sema mcp`), exposing Sema's tools (eval, build, notebook, docs) to Zed's agent panel.
- **Secret redaction** — string arguments to `llm/configure`, `llm/define-provider`, and `llm/auto-configure` are hidden during screen sharing.

## Requirements

The syntax queries work on their own, but the **run tasks** and **language server** need the `sema` CLI on your `PATH`:

```sh
cargo install sema-lang
```

See [sema-lang.com](https://sema-lang.com) for other install options.

### Running Sema files

A play button (▶) appears in the gutter — click it (or use the task runner) to run the current file. To evaluate just a selection, use the **sema eval form** task. Override the command in your project's `.zed/tasks.json` if you need a custom binary during development:

```json
[
  {
    "label": "sema run (dev)",
    "command": "sema",
    "args": ["$ZED_FILE"],
    "tags": ["sema-run"]
  }
]
```

### Language Server (LSP)

Sema ships a built-in language server (`sema lsp`). Enable it in your Zed settings (`cmd-,`):

```json
{
  "lsp": {
    "sema": {
      "binary": {
        "path": "sema",
        "arguments": ["lsp"]
      }
    }
  },
  "languages": {
    "Sema": {
      "language_servers": ["sema"]
    }
  }
}
```

### Debugging

Pick **Sema** when starting a debug session, or add a launch config to `.zed/debug.json`:

```json
[
  {
    "label": "Debug Sema Program",
    "adapter": "sema",
    "request": "launch",
    "program": "$ZED_FILE",
    "stopOnEntry": false
  }
]
```

The adapter runs `sema dap` over stdio (the `sema` binary is resolved from your `PATH`, or a per-adapter override).

### MCP server

The `sema` MCP context server is registered automatically; enable it in Zed's agent panel to give the assistant access to Sema's tools (eval, build, notebook, docs). It runs `sema mcp`.

## Links

- **Website** — [sema-lang.com](https://sema-lang.com)
- **Playground** — [sema.run](https://sema.run)
- **Documentation** — [sema-lang.com/docs](https://sema-lang.com/docs/)
- **Grammar** — [tree-sitter-sema](https://github.com/sema-lisp/tree-sitter-sema)
- **Repository** — [sema-lisp/zed-sema](https://github.com/sema-lisp/zed-sema)

## License

[MIT](LICENSE) © [Helge Sverre](https://github.com/HelgeSverre)
