//! Generated resource types - DO NOT EDIT
//! Run `cargo xtask codegen` to regenerate

use clap::ValueEnum;

/// Available query resources derived from Linear's GraphQL schema
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "camelCase")]
pub enum Resource {
    /// Query issue
    #[value(name = "issue")]
    Issue,
    /// Query team
    #[value(name = "team")]
    Team,
    /// Query user
    #[value(name = "user")]
    User,
    /// Query project
    #[value(name = "project")]
    Project,
    /// Query cycle
    #[value(name = "cycle")]
    Cycle,
    /// Query issueLabel
    #[value(name = "issueLabel")]
    IssueLabel,
    /// Query comment
    #[value(name = "comment")]
    Comment,
    /// Query workflow
    #[value(name = "workflow")]
    Workflow,
    /// Query workflowState
    #[value(name = "workflowState")]
    WorkflowState,
    /// Query attachment
    #[value(name = "attachment")]
    Attachment,
    /// Query document
    #[value(name = "document")]
    Document,
    /// Query roadmap
    #[value(name = "roadmap")]
    Roadmap,
    /// Query initiative
    #[value(name = "initiative")]
    Initiative,
    /// Query integration
    #[value(name = "integration")]
    Integration,
    /// Query notification
    #[value(name = "notification")]
    Notification,
    /// Query webhook
    #[value(name = "webhook")]
    Webhook,
    /// Query apiKey
    #[value(name = "apiKey")]
    ApiKey,
    /// Query viewer
    #[value(name = "viewer")]
    Viewer,
    /// Query organization
    #[value(name = "organization")]
    Organization,
}

impl Resource {
    /// Get all available resources
    pub fn all() -> &'static [Resource] {
        use Resource::*;
        &[
            Issue,
            Team,
            User,
            Project,
            Cycle,
            IssueLabel,
            Comment,
            Workflow,
            WorkflowState,
            Attachment,
            Document,
            Roadmap,
            Initiative,
            Integration,
            Notification,
            Webhook,
            ApiKey,
            Viewer,
            Organization,
        ]
    }

    /// Get the GraphQL field name for this resource
    pub fn field_name(&self) -> &'static str {
        match self {
            Resource::Issue => "issue",
            Resource::Team => "team",
            Resource::User => "user",
            Resource::Project => "project",
            Resource::Cycle => "cycle",
            Resource::IssueLabel => "issueLabel",
            Resource::Comment => "comment",
            Resource::Workflow => "workflow",
            Resource::WorkflowState => "workflowState",
            Resource::Attachment => "attachment",
            Resource::Document => "document",
            Resource::Roadmap => "roadmap",
            Resource::Initiative => "initiative",
            Resource::Integration => "integration",
            Resource::Notification => "notification",
            Resource::Webhook => "webhook",
            Resource::ApiKey => "apiKey",
            Resource::Viewer => "viewer",
            Resource::Organization => "organization",
        }
    }
}
