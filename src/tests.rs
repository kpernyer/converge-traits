// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT

use crate::{
    AgentRequirements, ComplianceLevel, CostClass, DataSovereignty, FinishReason, LlmError,
    LlmErrorKind, LlmProvider, LlmRequest, LlmResponse, ModelSelectorTrait, TokenUsage,
};

// =============================================================================
// LlmRequest tests
// =============================================================================

#[test]
fn llm_request_defaults() {
    let req = LlmRequest::new("hello");
    assert_eq!(req.prompt, "hello");
    assert_eq!(req.max_tokens, 1024);
    assert!((req.temperature - 0.7).abs() < f64::EPSILON);
    assert!(req.system.is_none());
    assert!(req.stop_sequences.is_empty());
}

#[test]
fn llm_request_builder() {
    let req = LlmRequest::new("prompt")
        .with_system("system")
        .with_max_tokens(512)
        .with_temperature(0.0)
        .with_stop_sequence("STOP");

    assert_eq!(req.system.as_deref(), Some("system"));
    assert_eq!(req.max_tokens, 512);
    assert!((req.temperature - 0.0).abs() < f64::EPSILON);
    assert_eq!(req.stop_sequences, vec!["STOP"]);
}

#[test]
fn llm_request_serde_roundtrip() {
    let req = LlmRequest::new("test").with_max_tokens(256).with_system("sys");
    let json = serde_json::to_string(&req).expect("serialize");
    let back: LlmRequest = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.prompt, req.prompt);
    assert_eq!(back.max_tokens, req.max_tokens);
    assert_eq!(back.system, req.system);
}

// =============================================================================
// LlmResponse / FinishReason / TokenUsage tests
// =============================================================================

#[test]
fn finish_reason_serde_stable() {
    let cases = [
        (FinishReason::Stop, "\"stop\""),
        (FinishReason::MaxTokens, "\"max_tokens\""),
        (FinishReason::StopSequence, "\"stop_sequence\""),
        (FinishReason::ContentFilter, "\"content_filter\""),
    ];
    for (variant, expected) in &cases {
        let serialized = serde_json::to_string(variant).expect("serialize");
        assert_eq!(&serialized, expected);
        let back: FinishReason = serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(&back, variant);
    }
}

#[test]
fn token_usage_default_is_zero() {
    let usage = TokenUsage::default();
    assert_eq!(usage.prompt_tokens, 0);
    assert_eq!(usage.completion_tokens, 0);
    assert_eq!(usage.total_tokens, 0);
}

// =============================================================================
// LlmError tests
// =============================================================================

#[test]
fn llm_error_auth_not_retryable() {
    let err = LlmError::auth("bad key");
    assert_eq!(err.kind, LlmErrorKind::Authentication);
    assert!(!err.retryable);
}

#[test]
fn llm_error_rate_limit_is_retryable() {
    let err = LlmError::rate_limit("too many");
    assert_eq!(err.kind, LlmErrorKind::RateLimit);
    assert!(err.retryable);
}

#[test]
fn llm_error_network_is_retryable() {
    let err = LlmError::network("timeout");
    assert_eq!(err.kind, LlmErrorKind::Network);
    assert!(err.retryable);
}

#[test]
fn llm_error_parse_not_retryable() {
    let err = LlmError::parse("bad json");
    assert_eq!(err.kind, LlmErrorKind::ParseError);
    assert!(!err.retryable);
}

#[test]
fn llm_error_display_contains_message() {
    let err = LlmError::provider("something went wrong");
    let display = format!("{err}");
    assert!(display.contains("something went wrong"));
}

#[test]
fn llm_error_serde_roundtrip() {
    let err = LlmError::new(LlmErrorKind::Timeout, "timed out", true);
    let json = serde_json::to_string(&err).expect("serialize");
    let back: LlmError = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.kind, LlmErrorKind::Timeout);
    assert_eq!(back.message, "timed out");
    assert!(back.retryable);
}

// =============================================================================
// CostClass tests
// =============================================================================

#[test]
fn cost_class_allowed_classes_inclusive() {
    assert_eq!(CostClass::VeryLow.allowed_classes(), vec![CostClass::VeryLow]);
    assert_eq!(CostClass::Low.allowed_classes().len(), 2);
    assert_eq!(CostClass::Medium.allowed_classes().len(), 3);
    assert_eq!(CostClass::High.allowed_classes().len(), 4);
    assert_eq!(CostClass::VeryHigh.allowed_classes().len(), 5);
}

#[test]
fn cost_class_ordering() {
    assert!(CostClass::VeryLow < CostClass::Low);
    assert!(CostClass::Low < CostClass::Medium);
    assert!(CostClass::Medium < CostClass::High);
    assert!(CostClass::High < CostClass::VeryHigh);
}

// =============================================================================
// AgentRequirements tests
// =============================================================================

#[test]
fn agent_requirements_fast_cheap() {
    let req = AgentRequirements::fast_cheap();
    assert_eq!(req.max_cost_class, CostClass::VeryLow);
    assert_eq!(req.max_latency_ms, 2000);
    assert!(!req.requires_reasoning);
    assert_eq!(req.data_sovereignty, DataSovereignty::Any);
    assert_eq!(req.compliance, ComplianceLevel::None);
}

#[test]
fn agent_requirements_balanced() {
    let req = AgentRequirements::balanced();
    assert!(req.min_quality > 0.0);
}

#[test]
fn agent_requirements_powerful() {
    let req = AgentRequirements::powerful();
    assert!(req.requires_reasoning);
    assert!(req.min_quality >= 0.9);
}

#[test]
fn agent_requirements_builder_methods() {
    let req = AgentRequirements::fast_cheap()
        .with_web_search(true)
        .with_data_sovereignty(DataSovereignty::EU)
        .with_compliance(ComplianceLevel::GDPR)
        .with_multilingual(true)
        .with_quality(0.85);

    assert!(req.requires_web_search);
    assert_eq!(req.data_sovereignty, DataSovereignty::EU);
    assert_eq!(req.compliance, ComplianceLevel::GDPR);
    assert!(req.requires_multilingual);
    assert!((req.min_quality - 0.85).abs() < f64::EPSILON);
}

// =============================================================================
// ModelSelectorTrait impl test (mock)
// =============================================================================

struct FixedSelector {
    provider: &'static str,
    model: &'static str,
}

impl ModelSelectorTrait for FixedSelector {
    fn select(&self, _requirements: &AgentRequirements) -> Result<(String, String), LlmError> {
        Ok((self.provider.to_string(), self.model.to_string()))
    }
}

#[test]
fn model_selector_trait_returns_provider_and_model() {
    let selector = FixedSelector { provider: "anthropic", model: "claude-3-haiku" };
    let (provider, model) = selector.select(&AgentRequirements::fast_cheap()).expect("select");
    assert_eq!(provider, "anthropic");
    assert_eq!(model, "claude-3-haiku");
}

// =============================================================================
// LlmProvider impl test (mock)
// =============================================================================

struct MockProvider;

impl LlmProvider for MockProvider {
    fn name(&self) -> &'static str { "mock" }
    fn model(&self) -> &str { "mock-1" }
    fn complete(&self, request: &LlmRequest) -> Result<LlmResponse, LlmError> {
        Ok(LlmResponse {
            content: format!("echo: {}", request.prompt),
            model: self.model().to_string(),
            usage: TokenUsage { prompt_tokens: 10, completion_tokens: 5, total_tokens: 15 },
            finish_reason: FinishReason::Stop,
        })
    }
}

#[test]
fn llm_provider_complete_and_provenance() {
    let p = MockProvider;
    let req = LlmRequest::new("hi");
    let resp = p.complete(&req).expect("complete");
    assert_eq!(resp.content, "echo: hi");
    assert_eq!(resp.finish_reason, FinishReason::Stop);
    assert_eq!(resp.usage.total_tokens, 15);

    let prov = p.provenance("req-42");
    assert!(prov.contains("mock-1"));
    assert!(prov.contains("req-42"));
}
