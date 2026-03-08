# converge-traits

Core traits and types for the [Converge](https://converge.zone) platform.

## Overview

Shared Rust interfaces used across all Converge crates:

- **`LlmProvider`** — trait for LLM backends (complete, name, model, provenance)
- **`LlmRequest` / `LlmResponse`** — structured request/response types with serde support
- **`TokenUsage` / `FinishReason`** — token accounting and stop-reason variants
- **`LlmError` / `LlmErrorKind`** — typed errors with retryable flags
- **`ModelSelectorTrait`** — trait for cost/latency/compliance-aware model selection
- **`AgentRequirements`** — builder for expressing model selection constraints (cost class, latency, sovereignty, compliance)

## Usage

```toml
[dependencies]
converge-traits = { path = "../converge-traits" }
```

```rust
use converge_traits::{LlmProvider, LlmRequest, AgentRequirements};

let req = LlmRequest::new("Hello!")
    .with_system("You are helpful.")
    .with_max_tokens(512);

let requirements = AgentRequirements::balanced();
```

## Crate Features

No feature flags — minimal, zero-overhead shared interfaces only.

## License

MIT — Copyright 2024-2025 Aprio One AB, Sweden
