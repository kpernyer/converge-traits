// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT

pub mod error;
pub mod llm;
pub mod selection;

pub use error::{LlmError, LlmErrorKind};
pub use llm::{FinishReason, LlmProvider, LlmRequest, LlmResponse, TokenUsage};
pub use selection::{AgentRequirements, ComplianceLevel, CostClass, DataSovereignty, ModelSelectorTrait};
