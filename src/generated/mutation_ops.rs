//! Generated mutation operations - DO NOT EDIT
//! Run `cargo xtask codegen` to regenerate

use clap::ValueEnum;

/// Available mutation operations derived from Linear's GraphQL schema
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "camelCase")]
pub enum MutationOp {
    /// Execute issueCreate mutation
    #[value(name = "issueCreate")]
    IssueCreate,
    /// Execute issueUpdate mutation
    #[value(name = "issueUpdate")]
    IssueUpdate,
    /// Execute issueDelete mutation
    #[value(name = "issueDelete")]
    IssueDelete,
    /// Execute issueArchive mutation
    #[value(name = "issueArchive")]
    IssueArchive,
    /// Execute issueUnarchive mutation
    #[value(name = "issueUnarchive")]
    IssueUnarchive,
    /// Execute commentCreate mutation
    #[value(name = "commentCreate")]
    CommentCreate,
    /// Execute commentUpdate mutation
    #[value(name = "commentUpdate")]
    CommentUpdate,
    /// Execute commentDelete mutation
    #[value(name = "commentDelete")]
    CommentDelete,
    /// Execute teamCreate mutation
    #[value(name = "teamCreate")]
    TeamCreate,
    /// Execute teamUpdate mutation
    #[value(name = "teamUpdate")]
    TeamUpdate,
    /// Execute teamDelete mutation
    #[value(name = "teamDelete")]
    TeamDelete,
    /// Execute projectCreate mutation
    #[value(name = "projectCreate")]
    ProjectCreate,
    /// Execute projectUpdate mutation
    #[value(name = "projectUpdate")]
    ProjectUpdate,
    /// Execute projectDelete mutation
    #[value(name = "projectDelete")]
    ProjectDelete,
    /// Execute projectArchive mutation
    #[value(name = "projectArchive")]
    ProjectArchive,
    /// Execute projectUnarchive mutation
    #[value(name = "projectUnarchive")]
    ProjectUnarchive,
    /// Execute cycleCreate mutation
    #[value(name = "cycleCreate")]
    CycleCreate,
    /// Execute cycleUpdate mutation
    #[value(name = "cycleUpdate")]
    CycleUpdate,
    /// Execute cycleDelete mutation
    #[value(name = "cycleDelete")]
    CycleDelete,
    /// Execute cycleArchive mutation
    #[value(name = "cycleArchive")]
    CycleArchive,
    /// Execute issueLabelCreate mutation
    #[value(name = "issueLabelCreate")]
    IssueLabelCreate,
    /// Execute issueLabelUpdate mutation
    #[value(name = "issueLabelUpdate")]
    IssueLabelUpdate,
    /// Execute issueLabelDelete mutation
    #[value(name = "issueLabelDelete")]
    IssueLabelDelete,
    /// Execute attachmentCreate mutation
    #[value(name = "attachmentCreate")]
    AttachmentCreate,
    /// Execute attachmentUpdate mutation
    #[value(name = "attachmentUpdate")]
    AttachmentUpdate,
    /// Execute attachmentDelete mutation
    #[value(name = "attachmentDelete")]
    AttachmentDelete,
    /// Execute documentCreate mutation
    #[value(name = "documentCreate")]
    DocumentCreate,
    /// Execute documentUpdate mutation
    #[value(name = "documentUpdate")]
    DocumentUpdate,
    /// Execute documentDelete mutation
    #[value(name = "documentDelete")]
    DocumentDelete,
    /// Execute workflowStateCreate mutation
    #[value(name = "workflowStateCreate")]
    WorkflowStateCreate,
    /// Execute workflowStateUpdate mutation
    #[value(name = "workflowStateUpdate")]
    WorkflowStateUpdate,
    /// Execute workflowStateArchive mutation
    #[value(name = "workflowStateArchive")]
    WorkflowStateArchive,
    /// Execute webhookCreate mutation
    #[value(name = "webhookCreate")]
    WebhookCreate,
    /// Execute webhookUpdate mutation
    #[value(name = "webhookUpdate")]
    WebhookUpdate,
    /// Execute webhookDelete mutation
    #[value(name = "webhookDelete")]
    WebhookDelete,
    /// Execute apiKeyCreate mutation
    #[value(name = "apiKeyCreate")]
    ApiKeyCreate,
    /// Execute apiKeyDelete mutation
    #[value(name = "apiKeyDelete")]
    ApiKeyDelete,
}

impl MutationOp {
    /// Get all available mutation operations
    pub fn all() -> &'static [MutationOp] {
        use MutationOp::*;
        &[
            IssueCreate,
            IssueUpdate,
            IssueDelete,
            IssueArchive,
            IssueUnarchive,
            CommentCreate,
            CommentUpdate,
            CommentDelete,
            TeamCreate,
            TeamUpdate,
            TeamDelete,
            ProjectCreate,
            ProjectUpdate,
            ProjectDelete,
            ProjectArchive,
            ProjectUnarchive,
            CycleCreate,
            CycleUpdate,
            CycleDelete,
            CycleArchive,
            IssueLabelCreate,
            IssueLabelUpdate,
            IssueLabelDelete,
            AttachmentCreate,
            AttachmentUpdate,
            AttachmentDelete,
            DocumentCreate,
            DocumentUpdate,
            DocumentDelete,
            WorkflowStateCreate,
            WorkflowStateUpdate,
            WorkflowStateArchive,
            WebhookCreate,
            WebhookUpdate,
            WebhookDelete,
            ApiKeyCreate,
            ApiKeyDelete,
        ]
    }

    /// Get the GraphQL operation name
    pub fn operation_name(&self) -> &'static str {
        match self {
            MutationOp::IssueCreate => "issueCreate",
            MutationOp::IssueUpdate => "issueUpdate",
            MutationOp::IssueDelete => "issueDelete",
            MutationOp::IssueArchive => "issueArchive",
            MutationOp::IssueUnarchive => "issueUnarchive",
            MutationOp::CommentCreate => "commentCreate",
            MutationOp::CommentUpdate => "commentUpdate",
            MutationOp::CommentDelete => "commentDelete",
            MutationOp::TeamCreate => "teamCreate",
            MutationOp::TeamUpdate => "teamUpdate",
            MutationOp::TeamDelete => "teamDelete",
            MutationOp::ProjectCreate => "projectCreate",
            MutationOp::ProjectUpdate => "projectUpdate",
            MutationOp::ProjectDelete => "projectDelete",
            MutationOp::ProjectArchive => "projectArchive",
            MutationOp::ProjectUnarchive => "projectUnarchive",
            MutationOp::CycleCreate => "cycleCreate",
            MutationOp::CycleUpdate => "cycleUpdate",
            MutationOp::CycleDelete => "cycleDelete",
            MutationOp::CycleArchive => "cycleArchive",
            MutationOp::IssueLabelCreate => "issueLabelCreate",
            MutationOp::IssueLabelUpdate => "issueLabelUpdate",
            MutationOp::IssueLabelDelete => "issueLabelDelete",
            MutationOp::AttachmentCreate => "attachmentCreate",
            MutationOp::AttachmentUpdate => "attachmentUpdate",
            MutationOp::AttachmentDelete => "attachmentDelete",
            MutationOp::DocumentCreate => "documentCreate",
            MutationOp::DocumentUpdate => "documentUpdate",
            MutationOp::DocumentDelete => "documentDelete",
            MutationOp::WorkflowStateCreate => "workflowStateCreate",
            MutationOp::WorkflowStateUpdate => "workflowStateUpdate",
            MutationOp::WorkflowStateArchive => "workflowStateArchive",
            MutationOp::WebhookCreate => "webhookCreate",
            MutationOp::WebhookUpdate => "webhookUpdate",
            MutationOp::WebhookDelete => "webhookDelete",
            MutationOp::ApiKeyCreate => "apiKeyCreate",
            MutationOp::ApiKeyDelete => "apiKeyDelete",
        }
    }
}
