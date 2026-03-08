// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT

use crate::llm::LlmError;
use serde::{Deserialize, Serialize};

/// Cost class of an LLM.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CostClass {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

impl CostClass {
    #[must_use]
    pub fn allowed_classes(self) -> Vec<CostClass> {
        match self {
            Self::VeryLow => vec![Self::VeryLow],
            Self::Low => vec![Self::VeryLow, Self::Low],
            Self::Medium => vec![Self::VeryLow, Self::Low, Self::Medium],
            Self::High => vec![Self::VeryLow, Self::Low, Self::Medium, Self::High],
            Self::VeryHigh => vec![
                Self::VeryLow,
                Self::Low,
                Self::Medium,
                Self::High,
                Self::VeryHigh,
            ],
        }
    }
}

/// Data sovereignty requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataSovereignty {
    Any,
    US,
    EU,
    Switzerland,
    China,
    OnPremises,
}

/// Compliance level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComplianceLevel {
    None,
    GDPR,
    HIPAA,
    SOC2,
}

/// Requirements for model selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRequirements {
    pub max_cost_class: CostClass,
    pub max_latency_ms: u32,
    pub requires_reasoning: bool,
    pub requires_web_search: bool,
    pub min_quality: f64,
    pub data_sovereignty: DataSovereignty,
    pub compliance: ComplianceLevel,
    pub requires_multilingual: bool,
}

impl AgentRequirements {
    #[must_use]
    pub fn new(max_cost_class: CostClass, max_latency_ms: u32, requires_reasoning: bool) -> Self {
        Self {
            max_cost_class,
            max_latency_ms,
            requires_reasoning,
            requires_web_search: false,
            min_quality: 0.0,
            data_sovereignty: DataSovereignty::Any,
            compliance: ComplianceLevel::None,
            requires_multilingual: false,
        }
    }

    #[must_use]
    pub fn fast_cheap() -> Self {
        Self::new(CostClass::VeryLow, 2000, false)
    }

    #[must_use]
    pub fn balanced() -> Self {
        Self::new(CostClass::Low, 5000, false).with_quality(0.8)
    }

    #[must_use]
    pub fn powerful() -> Self {
        Self::new(CostClass::High, 10000, true).with_quality(0.9)
    }

    #[must_use]
    pub fn with_quality(mut self, quality: f64) -> Self {
        self.min_quality = quality;
        self
    }

    #[must_use]
    pub fn with_web_search(mut self, required: bool) -> Self {
        self.requires_web_search = required;
        self
    }

    #[must_use]
    pub fn with_data_sovereignty(mut self, sovereignty: DataSovereignty) -> Self {
        self.data_sovereignty = sovereignty;
        self
    }

    #[must_use]
    pub fn with_compliance(mut self, compliance: ComplianceLevel) -> Self {
        self.compliance = compliance;
        self
    }

    #[must_use]
    pub fn with_multilingual(mut self, required: bool) -> Self {
        self.requires_multilingual = required;
        self
    }
}

/// Trait for model selection.
pub trait ModelSelectorTrait: Send + Sync {
    /// Selects a model (provider, model_id) satisfying the requirements.
    fn select(&self, requirements: &AgentRequirements) -> Result<(String, String), LlmError>;
}
