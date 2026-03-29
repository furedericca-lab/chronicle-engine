use serde::{Deserialize, Serialize};

use crate::models::{
    BehavioralRecallMode, Category, DistillArtifact, DistillJobStatus, DistillMode,
    DistillSourceKind, RetrievalTrace,
};

// ─── Recall Simulation ───

/// Recall simulation request DTO for the admin plane.
///
/// Covers both generic and behavioral recall modes in a single request shape.
/// The admin plane uses this instead of the runtime Actor-based recall requests
/// so the browser does not need to provide runtime-only fields like sessionId/sessionKey.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminRecallSimulateRequest {
    /// "generic" or "behavioral"
    pub mode: AdminRecallMode,
    pub query: String,
    #[serde(default = "default_recall_limit")]
    pub limit: u64,
    /// Optional: categories filter (generic mode).
    #[serde(default)]
    pub categories: Option<Vec<Category>>,
    /// Optional: exclude behavioral rows (generic mode).
    #[serde(default)]
    pub exclude_behavioral: Option<bool>,
    /// Optional: max age in days (generic mode).
    #[serde(default)]
    pub max_age_days: Option<u64>,
    /// Optional: behavioral recall mode (behavioral mode).
    #[serde(default)]
    pub behavioral_mode: Option<BehavioralRecallMode>,
    /// Optional: include kinds filter (behavioral mode).
    #[serde(default)]
    pub include_kinds: Option<Vec<crate::models::ReflectionKind>>,
    /// Optional: minimum score threshold.
    #[serde(default)]
    pub min_score: Option<f64>,
}

fn default_recall_limit() -> u64 {
    20
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AdminRecallMode {
    Generic,
    Behavioral,
}

/// Recall simulation response DTO for the admin plane.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminRecallSimulateResponse {
    pub mode: AdminRecallMode,
    pub principal_user_id: String,
    pub principal_agent_id: String,
    pub results: serde_json::Value,
    pub trace: RetrievalTrace,
    pub applied_filters: AdminRecallFilterSummary,
}

/// Summary of applied filters for a recall simulation.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminRecallFilterSummary {
    pub mode: AdminRecallMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<Category>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_behavioral: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age_days: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavioral_mode: Option<BehavioralRecallMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_kinds: Option<Vec<crate::models::ReflectionKind>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_score: Option<f64>,
    pub limit: u64,
}

// ─── Health ───

/// Admin health probe response for the admin plane.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminHealthResponse {
    pub status: &'static str,
    pub service: &'static str,
    pub plane: &'static str,
    pub version: &'static str,
}

// ─── Memory Explorer ───

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminMemoryListItem {
    pub id: String,
    pub principal: crate::models::Principal,
    pub text_preview: String,
    pub category: Option<Category>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavioral_kind: Option<String>,
    pub scope: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub access_count: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::models::MemoryProvenance>,
    pub is_behavioral: bool,
    pub is_distill_derived: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminMemoryListResponse {
    pub items: Vec<AdminMemoryListItem>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminMemoryDetail {
    pub id: String,
    pub principal: crate::models::Principal,
    pub text: String,
    pub category: Option<Category>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavioral_kind: Option<String>,
    pub scope: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub access_count: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::models::MemoryProvenance>,
    pub is_behavioral: bool,
    pub is_distill_derived: bool,
    pub strict_key: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminCreateMemoryRequest {
    pub text: String,
    pub category: Option<Category>,
    pub importance: Option<f64>,
    pub source_kind: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminUpdateMemoryRequest {
    pub text: Option<String>,
    pub category: Option<Category>,
    pub importance: Option<f64>,
}

// ─── Distill Jobs ───

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminDistillJobListItem {
    pub job_id: String,
    pub status: DistillJobStatus,
    pub mode: DistillMode,
    pub source_kind: DistillSourceKind,
    pub created_at: i64,
    pub updated_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<crate::models::DistillJobResultSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<crate::models::JobStatusError>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminDistillJobListResponse {
    pub items: Vec<AdminDistillJobListItem>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminDistillJobDetail {
    pub job_id: String,
    pub status: DistillJobStatus,
    pub mode: DistillMode,
    pub source_kind: DistillSourceKind,
    pub created_at: i64,
    pub updated_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<crate::models::DistillJobResultSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<crate::models::JobStatusError>,
    pub artifacts: Vec<DistillArtifact>,
}

// ─── Transcripts ───

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminTranscriptHead {
    pub transcript_id: String,
    pub principal: crate::models::Principal,
    pub session_key: String,
    pub session_id: String,
    pub message_count: u64,
    pub first_timestamp: i64,
    pub last_timestamp: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminTranscriptListResponse {
    pub items: Vec<AdminTranscriptHead>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminTranscriptMessage {
    pub seq: u64,
    pub role: crate::models::MessageRole,
    pub text: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminTranscriptDetailResponse {
    pub transcript_id: String,
    pub principal: crate::models::Principal,
    pub session_key: String,
    pub session_id: String,
    pub messages: Vec<AdminTranscriptMessage>,
}

// ─── Governance ───

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminGovernanceArtifact {
    pub artifact_id: String,
    pub job_id: String,
    pub kind: crate::models::DistillArtifactKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<crate::models::DistillArtifactSubtype>,
    pub category: Category,
    pub importance: f64,
    pub text: String,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewer_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminGovernanceListResponse {
    pub items: Vec<AdminGovernanceArtifact>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminGovernanceReviewRequest {
    pub review_status: String,
    #[serde(default)]
    pub reviewer_note: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminGovernanceReviewResponse {
    pub artifact_id: String,
    pub review_status: String,
    pub reviewed_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminGovernancePromoteRequest {
    #[serde(default)]
    pub reviewer_note: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminGovernancePromoteResponse {
    pub artifact_id: String,
    pub promoted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persisted_memory_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

// ─── Audit Log ───

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdminAuditEntry {
    pub id: String,
    pub timestamp: i64,
    pub admin_subject: String,
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_principal_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_principal_agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_id: Option<String>,
    pub outcome: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_json: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAuditLogResponse {
    pub items: Vec<AdminAuditEntry>,
}

// ─── Settings ───

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminSettingsResponse {
    pub config: serde_json::Value,
    pub config_toml: String,
    pub editable: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminSettingsUpdateRequest {
    pub config_toml: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminSettingsUpdateResponse {
    pub applied: bool,
    pub restart_required: bool,
    pub summary: String,
}
