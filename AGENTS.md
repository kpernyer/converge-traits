# AGENTS - converge-traits

Start here for tasks in this repo. Keep this file short; load details just-in-time.

## Quick Start

- Check: `cargo check`
- Run: `cargo test`
- Single test: `cargo test test_name`
- Run before handoff: `cargo fmt -- --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test`

## Load On Demand

- Shared workflow: `../docs/agent/workflow.md`
- Rust commands/style/testing: `../docs/agent/rust.md`
- Repo manifests: `Cargo.toml`

## Local Rules

- This crate defines shared interfaces only — no implementations
- Breaking trait changes require coordination with all dependent crates (converge-llm, converge-provider, etc.)
- If present, load `.cursorrules`, `.cursor/rules/`, and `.github/copilot-instructions.md`
