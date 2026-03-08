// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT

pub use crate::error::{LlmError, LlmErrorKind};
use serde::{Deserialize, Serialize};

/// Request to an LLM provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmRequest {
    /// The user prompt.
    pub prompt: String,
    /// Optional system prompt.
    pub system: Option<String>,
    /// Maximum tokens to generate.
    pub max_tokens: u32,
    /// Temperature (0.0 = deterministic, 1.0 = creative).
    pub temperature: f64,
    /// Optional stop sequences.
    pub stop_sequences: Vec<String>,
}

impl LlmRequest {
    #[must_use]
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            system: None,
            max_tokens: 1024,
            temperature: 0.7,
            stop_sequences: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    #[must_use]
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    #[must_use]
    pub fn with_temperature(mut self, temperature: f64) -> Self {
        self.temperature = temperature;
        self
    }

    #[must_use]
    pub fn with_stop_sequence(mut self, stop: impl Into<String>) -> Self {
        self.stop_sequences.push(stop.into());
        self
    }
}

/// Response from an LLM provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    /// The generated content.
    pub content: String,
    /// The model that generated this response.
    pub model: String,
    /// Token usage statistics.
    pub usage: TokenUsage,
    /// Finish reason.
    pub finish_reason: FinishReason,
}

/// Token usage statistics.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Tokens in the prompt.
    pub prompt_tokens: u32,
    /// Tokens in the completion.
    pub completion_tokens: u32,
    /// Total tokens used.
    pub total_tokens: u32,
}

/// Reason the generation stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    /// Natural end of response.
    Stop,
    /// Hit max_tokens limit.
    MaxTokens,
    /// Hit a stop sequence.
    StopSequence,
    /// Content was filtered.
    ContentFilter,
}

/// Trait for LLM providers.
pub trait LlmProvider: Send + Sync {
    /// The name of this provider (e.g., "anthropic", "openai").
    fn name(&self) -> &'static str;

    /// The model being used (e.g., "claude-3-opus", "gpt-4").
    fn model(&self) -> &str;

    /// Sends a completion request to the LLM.
    ///
    /// # Errors
    ///
    /// Returns `LlmError` if the request fails.
    fn complete(&self, request: &LlmRequest) -> Result<LlmResponse, LlmError>;

    /// Returns a provenance string for tracking (e.g., "claude-3-opus:abc123").
    fn provenance(&self, request_id: &str) -> String {
        format!("{}:{}", self.model(), request_id)
    }
}
