// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error from an LLM provider.
#[derive(Debug, Clone, Serialize, Deserialize, Error)]
#[error("{kind:?}: {message}")]
pub struct LlmError {
    /// Error kind.
    pub kind: LlmErrorKind,
    /// Human-readable message.
    pub message: String,
    /// Whether the request can be retried.
    pub retryable: bool,
}

/// Kind of LLM error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LlmErrorKind {
    /// Invalid API key.
    Authentication,
    /// Rate limit exceeded.
    RateLimit,
    /// Invalid request parameters.
    InvalidRequest,
    /// Model not available.
    ModelNotFound,
    /// Network or connection error.
    Network,
    /// Provider returned an error.
    ProviderError,
    /// Response couldn't be parsed.
    ParseError,
    /// Request timed out.
    Timeout,
}

impl LlmError {
    pub fn new(kind: LlmErrorKind, message: impl Into<String>, retryable: bool) -> Self {
        Self {
            kind,
            message: message.into(),
            retryable,
        }
    }

    pub fn auth(message: impl Into<String>) -> Self {
        Self::new(LlmErrorKind::Authentication, message, false)
    }

    pub fn rate_limit(message: impl Into<String>) -> Self {
        Self::new(LlmErrorKind::RateLimit, message, true)
    }

    pub fn network(message: impl Into<String>) -> Self {
        Self::new(LlmErrorKind::Network, message, true)
    }

    pub fn parse(message: impl Into<String>) -> Self {
        Self::new(LlmErrorKind::ParseError, message, false)
    }

    pub fn provider(message: impl Into<String>) -> Self {
        Self::new(LlmErrorKind::ProviderError, message, false)
    }
}
