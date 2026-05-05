# fledge-plugin-scratch

Throwaway scratch notes scoped to your current repo. Opens a fresh markdown file in `$EDITOR`, autosaved under `~/.fledge/scratches/<repo>/<timestamp>.md`.

A plugin for [fledge](https://github.com/CorvidLabs/fledge).

## Install

```bash
fledge plugins install CorvidLabs/fledge-plugin-scratch
```

## Usage

```bash
fledge scratch                # resume the most recent scratch (or create one if none)
fledge scratch new            # force-create a new scratch
fledge scratch list           # list scratches for this repo (newest first)
fledge scratch open 1         # open the 1st most recent scratch
fledge scratch path           # print the scratch directory for this repo
```

The default command resumes — running `fledge scratch` repeatedly keeps you in the same file. Use `scratch new` when you want a fresh one.

If you're not inside a git repo, scratches go to a `_global` bucket.

## Picking your editor

Scratch uses your `$EDITOR` env var (falls back to `vi`). Works with terminal *and* GUI editors:

```bash
export EDITOR="code --wait"   # VS Code
export EDITOR="cursor --wait" # Cursor
export EDITOR="subl -w"       # Sublime
export EDITOR="nano"          # nano
export EDITOR="vim"           # vim
export EDITOR="hx"            # Helix
```

GUI editors need a "wait" flag (`--wait`, `-w`) so the editor blocks until you close the file — otherwise scratch returns immediately and you don't see the "Saved:" confirmation.

## Build from source

```bash
git clone https://github.com/CorvidLabs/fledge-plugin-scratch.git
cd fledge-plugin-scratch
cargo build --release
```

The binary is at `target/release/fledge-scratch`.

## Contributing

1. Fork the repo and create a feature branch.
2. Run `cargo clippy -- -D warnings` and `cargo fmt --check` before pushing.
3. Open a PR against `main`.

## License

MIT
