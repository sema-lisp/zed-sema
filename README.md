# Sema for Zed

[![CI](https://img.shields.io/github/actions/workflow/status/sema-lisp/zed-sema/ci.yml?branch=main&logo=github&label=CI)](https://github.com/sema-lisp/zed-sema/actions/workflows/ci.yml)
[![License](https://img.shields.io/github/license/sema-lisp/zed-sema)](LICENSE)

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

## Roadmap

- **Debug adapter (DAP)** — Sema ships a Debug Adapter (`sema dap`) with breakpoints, stepping, and variable inspection. Wiring it into Zed requires a `zed_extension_api` Rust component (Zed exposes debug adapters through compiled extensions, not query files), so it is planned but not yet implemented here. Tracked in the issues.

## Links

- Website — [sema-lang.com](https://sema-lang.com)
- Playground — [sema.run](https://sema.run)
- Grammar — [tree-sitter-sema](https://github.com/sema-lisp/tree-sitter-sema)
- This extension — [sema-lisp/zed-sema](https://github.com/sema-lisp/zed-sema)
