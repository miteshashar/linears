//! Generated resource types - DO NOT EDIT
//! Run `cargo xtask codegen` to regenerate

use clap::ValueEnum;

/// Available query resources derived from Linear's GraphQL schema
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "camelCase")]
pub enum Resource {
    /// Query administrableTeams
    #[value(name = "administrableTeams")]
    AdministrableTeams,
    /// Query agentActivities
    #[value(name = "agentActivities")]
    AgentActivities,
    /// Query agentActivity
    #[value(name = "agentActivity")]
    AgentActivity,
    /// Query agentSession
    #[value(name = "agentSession")]
    AgentSession,
    /// Query agentSessions
    #[value(name = "agentSessions")]
    AgentSessions,
    /// Query applicationInfo
    #[value(name = "applicationInfo")]
    ApplicationInfo,
    /// Query archivedTeams
    #[value(name = "archivedTeams")]
    ArchivedTeams,
    /// Query attachment
    #[value(name = "attachment")]
    Attachment,
    /// Query attachmentIssue
    #[value(name = "attachmentIssue")]
    AttachmentIssue,
    /// Query attachmentSources
    #[value(name = "attachmentSources")]
    AttachmentSources,
    /// Query attachments
    #[value(name = "attachments")]
    Attachments,
    /// Query attachmentsForURL
    #[value(name = "attachmentsForURL")]
    AttachmentsForURL,
    /// Query auditEntries
    #[value(name = "auditEntries")]
    AuditEntries,
    /// Query auditEntryTypes
    #[value(name = "auditEntryTypes")]
    AuditEntryTypes,
    /// Query authenticationSessions
    #[value(name = "authenticationSessions")]
    AuthenticationSessions,
    /// Query availableUsers
    #[value(name = "availableUsers")]
    AvailableUsers,
    /// Query comment
    #[value(name = "comment")]
    Comment,
    /// Query comments
    #[value(name = "comments")]
    Comments,
    /// Query customView
    #[value(name = "customView")]
    CustomView,
    /// Query customViewDetailsSuggestion
    #[value(name = "customViewDetailsSuggestion")]
    CustomViewDetailsSuggestion,
    /// Query customViewHasSubscribers
    #[value(name = "customViewHasSubscribers")]
    CustomViewHasSubscribers,
    /// Query customViews
    #[value(name = "customViews")]
    CustomViews,
    /// Query customer
    #[value(name = "customer")]
    Customer,
    /// Query customerNeed
    #[value(name = "customerNeed")]
    CustomerNeed,
    /// Query customerNeeds
    #[value(name = "customerNeeds")]
    CustomerNeeds,
    /// Query customerStatus
    #[value(name = "customerStatus")]
    CustomerStatus,
    /// Query customerStatuses
    #[value(name = "customerStatuses")]
    CustomerStatuses,
    /// Query customerTier
    #[value(name = "customerTier")]
    CustomerTier,
    /// Query customerTiers
    #[value(name = "customerTiers")]
    CustomerTiers,
    /// Query customers
    #[value(name = "customers")]
    Customers,
    /// Query cycle
    #[value(name = "cycle")]
    Cycle,
    /// Query cycles
    #[value(name = "cycles")]
    Cycles,
    /// Query document
    #[value(name = "document")]
    Document,
    /// Query documentContentHistory
    #[value(name = "documentContentHistory")]
    DocumentContentHistory,
    /// Query documents
    #[value(name = "documents")]
    Documents,
    /// Query emailIntakeAddress
    #[value(name = "emailIntakeAddress")]
    EmailIntakeAddress,
    /// Query emoji
    #[value(name = "emoji")]
    Emoji,
    /// Query emojis
    #[value(name = "emojis")]
    Emojis,
    /// Query entityExternalLink
    #[value(name = "entityExternalLink")]
    EntityExternalLink,
    /// Query externalUser
    #[value(name = "externalUser")]
    ExternalUser,
    /// Query externalUsers
    #[value(name = "externalUsers")]
    ExternalUsers,
    /// Query failuresForOauthWebhooks
    #[value(name = "failuresForOauthWebhooks")]
    FailuresForOauthWebhooks,
    /// Query favorite
    #[value(name = "favorite")]
    Favorite,
    /// Query favorites
    #[value(name = "favorites")]
    Favorites,
    /// Query fetchData
    #[value(name = "fetchData")]
    FetchData,
    /// Query initiative
    #[value(name = "initiative")]
    Initiative,
    /// Query initiativeRelation
    #[value(name = "initiativeRelation")]
    InitiativeRelation,
    /// Query initiativeRelations
    #[value(name = "initiativeRelations")]
    InitiativeRelations,
    /// Query initiativeToProject
    #[value(name = "initiativeToProject")]
    InitiativeToProject,
    /// Query initiativeToProjects
    #[value(name = "initiativeToProjects")]
    InitiativeToProjects,
    /// Query initiativeUpdate
    #[value(name = "initiativeUpdate")]
    InitiativeUpdate,
    /// Query initiativeUpdates
    #[value(name = "initiativeUpdates")]
    InitiativeUpdates,
    /// Query initiatives
    #[value(name = "initiatives")]
    Initiatives,
    /// Query integration
    #[value(name = "integration")]
    Integration,
    /// Query integrationHasScopes
    #[value(name = "integrationHasScopes")]
    IntegrationHasScopes,
    /// Query integrationTemplate
    #[value(name = "integrationTemplate")]
    IntegrationTemplate,
    /// Query integrationTemplates
    #[value(name = "integrationTemplates")]
    IntegrationTemplates,
    /// Query integrations
    #[value(name = "integrations")]
    Integrations,
    /// Query integrationsSettings
    #[value(name = "integrationsSettings")]
    IntegrationsSettings,
    /// Query issue
    #[value(name = "issue")]
    Issue,
    /// Query issueFigmaFileKeySearch
    #[value(name = "issueFigmaFileKeySearch")]
    IssueFigmaFileKeySearch,
    /// Query issueFilterSuggestion
    #[value(name = "issueFilterSuggestion")]
    IssueFilterSuggestion,
    /// Query issueImportCheckCSV
    #[value(name = "issueImportCheckCSV")]
    IssueImportCheckCSV,
    /// Query issueImportCheckSync
    #[value(name = "issueImportCheckSync")]
    IssueImportCheckSync,
    /// Query issueImportJqlCheck
    #[value(name = "issueImportJqlCheck")]
    IssueImportJqlCheck,
    /// Query issueLabel
    #[value(name = "issueLabel")]
    IssueLabel,
    /// Query issueLabels
    #[value(name = "issueLabels")]
    IssueLabels,
    /// Query issuePriorityValues
    #[value(name = "issuePriorityValues")]
    IssuePriorityValues,
    /// Query issueRelation
    #[value(name = "issueRelation")]
    IssueRelation,
    /// Query issueRelations
    #[value(name = "issueRelations")]
    IssueRelations,
    /// Query issueRepositorySuggestions
    #[value(name = "issueRepositorySuggestions")]
    IssueRepositorySuggestions,
    /// Query issueSearch
    #[value(name = "issueSearch")]
    IssueSearch,
    /// Query issueTitleSuggestionFromCustomerRequest
    #[value(name = "issueTitleSuggestionFromCustomerRequest")]
    IssueTitleSuggestionFromCustomerRequest,
    /// Query issueToRelease
    #[value(name = "issueToRelease")]
    IssueToRelease,
    /// Query issueToReleases
    #[value(name = "issueToReleases")]
    IssueToReleases,
    /// Query issueVcsBranchSearch
    #[value(name = "issueVcsBranchSearch")]
    IssueVcsBranchSearch,
    /// Query issues
    #[value(name = "issues")]
    Issues,
    /// Query notification
    #[value(name = "notification")]
    Notification,
    /// Query notificationSubscription
    #[value(name = "notificationSubscription")]
    NotificationSubscription,
    /// Query notificationSubscriptions
    #[value(name = "notificationSubscriptions")]
    NotificationSubscriptions,
    /// Query notifications
    #[value(name = "notifications")]
    Notifications,
    /// Query notificationsUnreadCount
    #[value(name = "notificationsUnreadCount")]
    NotificationsUnreadCount,
    /// Query organization
    #[value(name = "organization")]
    Organization,
    /// Query organizationDomainClaimRequest
    #[value(name = "organizationDomainClaimRequest")]
    OrganizationDomainClaimRequest,
    /// Query organizationExists
    #[value(name = "organizationExists")]
    OrganizationExists,
    /// Query organizationInvite
    #[value(name = "organizationInvite")]
    OrganizationInvite,
    /// Query organizationInviteDetails
    #[value(name = "organizationInviteDetails")]
    OrganizationInviteDetails,
    /// Query organizationInvites
    #[value(name = "organizationInvites")]
    OrganizationInvites,
    /// Query organizationMeta
    #[value(name = "organizationMeta")]
    OrganizationMeta,
    /// Query project
    #[value(name = "project")]
    Project,
    /// Query projectFilterSuggestion
    #[value(name = "projectFilterSuggestion")]
    ProjectFilterSuggestion,
    /// Query projectLabel
    #[value(name = "projectLabel")]
    ProjectLabel,
    /// Query projectLabels
    #[value(name = "projectLabels")]
    ProjectLabels,
    /// Query projectMilestone
    #[value(name = "projectMilestone")]
    ProjectMilestone,
    /// Query projectMilestones
    #[value(name = "projectMilestones")]
    ProjectMilestones,
    /// Query projectRelation
    #[value(name = "projectRelation")]
    ProjectRelation,
    /// Query projectRelations
    #[value(name = "projectRelations")]
    ProjectRelations,
    /// Query projectStatus
    #[value(name = "projectStatus")]
    ProjectStatus,
    /// Query projectStatusProjectCount
    #[value(name = "projectStatusProjectCount")]
    ProjectStatusProjectCount,
    /// Query projectStatuses
    #[value(name = "projectStatuses")]
    ProjectStatuses,
    /// Query projectUpdate
    #[value(name = "projectUpdate")]
    ProjectUpdate,
    /// Query projectUpdates
    #[value(name = "projectUpdates")]
    ProjectUpdates,
    /// Query projects
    #[value(name = "projects")]
    Projects,
    /// Query pushSubscriptionTest
    #[value(name = "pushSubscriptionTest")]
    PushSubscriptionTest,
    /// Query rateLimitStatus
    #[value(name = "rateLimitStatus")]
    RateLimitStatus,
    /// Query release
    #[value(name = "release")]
    Release,
    /// Query releasePipeline
    #[value(name = "releasePipeline")]
    ReleasePipeline,
    /// Query releasePipelines
    #[value(name = "releasePipelines")]
    ReleasePipelines,
    /// Query releaseStage
    #[value(name = "releaseStage")]
    ReleaseStage,
    /// Query releaseStages
    #[value(name = "releaseStages")]
    ReleaseStages,
    /// Query releases
    #[value(name = "releases")]
    Releases,
    /// Query roadmap
    #[value(name = "roadmap")]
    Roadmap,
    /// Query roadmapToProject
    #[value(name = "roadmapToProject")]
    RoadmapToProject,
    /// Query roadmapToProjects
    #[value(name = "roadmapToProjects")]
    RoadmapToProjects,
    /// Query roadmaps
    #[value(name = "roadmaps")]
    Roadmaps,
    /// Query searchDocuments
    #[value(name = "searchDocuments")]
    SearchDocuments,
    /// Query searchIssues
    #[value(name = "searchIssues")]
    SearchIssues,
    /// Query searchProjects
    #[value(name = "searchProjects")]
    SearchProjects,
    /// Query semanticSearch
    #[value(name = "semanticSearch")]
    SemanticSearch,
    /// Query ssoUrlFromEmail
    #[value(name = "ssoUrlFromEmail")]
    SsoUrlFromEmail,
    /// Query team
    #[value(name = "team")]
    Team,
    /// Query teamMembership
    #[value(name = "teamMembership")]
    TeamMembership,
    /// Query teamMemberships
    #[value(name = "teamMemberships")]
    TeamMemberships,
    /// Query teams
    #[value(name = "teams")]
    Teams,
    /// Query template
    #[value(name = "template")]
    Template,
    /// Query templates
    #[value(name = "templates")]
    Templates,
    /// Query templatesForIntegration
    #[value(name = "templatesForIntegration")]
    TemplatesForIntegration,
    /// Query timeSchedule
    #[value(name = "timeSchedule")]
    TimeSchedule,
    /// Query timeSchedules
    #[value(name = "timeSchedules")]
    TimeSchedules,
    /// Query triageResponsibilities
    #[value(name = "triageResponsibilities")]
    TriageResponsibilities,
    /// Query triageResponsibility
    #[value(name = "triageResponsibility")]
    TriageResponsibility,
    /// Query user
    #[value(name = "user")]
    User,
    /// Query userSettings
    #[value(name = "userSettings")]
    UserSettings,
    /// Query users
    #[value(name = "users")]
    Users,
    /// Query verifyGitHubEnterpriseServerInstallation
    #[value(name = "verifyGitHubEnterpriseServerInstallation")]
    VerifyGitHubEnterpriseServerInstallation,
    /// Query viewer
    #[value(name = "viewer")]
    Viewer,
    /// Query webhook
    #[value(name = "webhook")]
    Webhook,
    /// Query webhooks
    #[value(name = "webhooks")]
    Webhooks,
    /// Query workflowState
    #[value(name = "workflowState")]
    WorkflowState,
    /// Query workflowStates
    #[value(name = "workflowStates")]
    WorkflowStates,
}

impl Resource {
    /// Get all available resources
    pub fn all() -> &'static [Resource] {
        use Resource::*;
        &[
            AdministrableTeams,
            AgentActivities,
            AgentActivity,
            AgentSession,
            AgentSessions,
            ApplicationInfo,
            ArchivedTeams,
            Attachment,
            AttachmentIssue,
            AttachmentSources,
            Attachments,
            AttachmentsForURL,
            AuditEntries,
            AuditEntryTypes,
            AuthenticationSessions,
            AvailableUsers,
            Comment,
            Comments,
            CustomView,
            CustomViewDetailsSuggestion,
            CustomViewHasSubscribers,
            CustomViews,
            Customer,
            CustomerNeed,
            CustomerNeeds,
            CustomerStatus,
            CustomerStatuses,
            CustomerTier,
            CustomerTiers,
            Customers,
            Cycle,
            Cycles,
            Document,
            DocumentContentHistory,
            Documents,
            EmailIntakeAddress,
            Emoji,
            Emojis,
            EntityExternalLink,
            ExternalUser,
            ExternalUsers,
            FailuresForOauthWebhooks,
            Favorite,
            Favorites,
            FetchData,
            Initiative,
            InitiativeRelation,
            InitiativeRelations,
            InitiativeToProject,
            InitiativeToProjects,
            InitiativeUpdate,
            InitiativeUpdates,
            Initiatives,
            Integration,
            IntegrationHasScopes,
            IntegrationTemplate,
            IntegrationTemplates,
            Integrations,
            IntegrationsSettings,
            Issue,
            IssueFigmaFileKeySearch,
            IssueFilterSuggestion,
            IssueImportCheckCSV,
            IssueImportCheckSync,
            IssueImportJqlCheck,
            IssueLabel,
            IssueLabels,
            IssuePriorityValues,
            IssueRelation,
            IssueRelations,
            IssueRepositorySuggestions,
            IssueSearch,
            IssueTitleSuggestionFromCustomerRequest,
            IssueToRelease,
            IssueToReleases,
            IssueVcsBranchSearch,
            Issues,
            Notification,
            NotificationSubscription,
            NotificationSubscriptions,
            Notifications,
            NotificationsUnreadCount,
            Organization,
            OrganizationDomainClaimRequest,
            OrganizationExists,
            OrganizationInvite,
            OrganizationInviteDetails,
            OrganizationInvites,
            OrganizationMeta,
            Project,
            ProjectFilterSuggestion,
            ProjectLabel,
            ProjectLabels,
            ProjectMilestone,
            ProjectMilestones,
            ProjectRelation,
            ProjectRelations,
            ProjectStatus,
            ProjectStatusProjectCount,
            ProjectStatuses,
            ProjectUpdate,
            ProjectUpdates,
            Projects,
            PushSubscriptionTest,
            RateLimitStatus,
            Release,
            ReleasePipeline,
            ReleasePipelines,
            ReleaseStage,
            ReleaseStages,
            Releases,
            Roadmap,
            RoadmapToProject,
            RoadmapToProjects,
            Roadmaps,
            SearchDocuments,
            SearchIssues,
            SearchProjects,
            SemanticSearch,
            SsoUrlFromEmail,
            Team,
            TeamMembership,
            TeamMemberships,
            Teams,
            Template,
            Templates,
            TemplatesForIntegration,
            TimeSchedule,
            TimeSchedules,
            TriageResponsibilities,
            TriageResponsibility,
            User,
            UserSettings,
            Users,
            VerifyGitHubEnterpriseServerInstallation,
            Viewer,
            Webhook,
            Webhooks,
            WorkflowState,
            WorkflowStates,
        ]
    }

    /// Get the GraphQL field name for this resource
    pub fn field_name(&self) -> &'static str {
        match self {
            Resource::AdministrableTeams => "administrableTeams",
            Resource::AgentActivities => "agentActivities",
            Resource::AgentActivity => "agentActivity",
            Resource::AgentSession => "agentSession",
            Resource::AgentSessions => "agentSessions",
            Resource::ApplicationInfo => "applicationInfo",
            Resource::ArchivedTeams => "archivedTeams",
            Resource::Attachment => "attachment",
            Resource::AttachmentIssue => "attachmentIssue",
            Resource::AttachmentSources => "attachmentSources",
            Resource::Attachments => "attachments",
            Resource::AttachmentsForURL => "attachmentsForURL",
            Resource::AuditEntries => "auditEntries",
            Resource::AuditEntryTypes => "auditEntryTypes",
            Resource::AuthenticationSessions => "authenticationSessions",
            Resource::AvailableUsers => "availableUsers",
            Resource::Comment => "comment",
            Resource::Comments => "comments",
            Resource::CustomView => "customView",
            Resource::CustomViewDetailsSuggestion => "customViewDetailsSuggestion",
            Resource::CustomViewHasSubscribers => "customViewHasSubscribers",
            Resource::CustomViews => "customViews",
            Resource::Customer => "customer",
            Resource::CustomerNeed => "customerNeed",
            Resource::CustomerNeeds => "customerNeeds",
            Resource::CustomerStatus => "customerStatus",
            Resource::CustomerStatuses => "customerStatuses",
            Resource::CustomerTier => "customerTier",
            Resource::CustomerTiers => "customerTiers",
            Resource::Customers => "customers",
            Resource::Cycle => "cycle",
            Resource::Cycles => "cycles",
            Resource::Document => "document",
            Resource::DocumentContentHistory => "documentContentHistory",
            Resource::Documents => "documents",
            Resource::EmailIntakeAddress => "emailIntakeAddress",
            Resource::Emoji => "emoji",
            Resource::Emojis => "emojis",
            Resource::EntityExternalLink => "entityExternalLink",
            Resource::ExternalUser => "externalUser",
            Resource::ExternalUsers => "externalUsers",
            Resource::FailuresForOauthWebhooks => "failuresForOauthWebhooks",
            Resource::Favorite => "favorite",
            Resource::Favorites => "favorites",
            Resource::FetchData => "fetchData",
            Resource::Initiative => "initiative",
            Resource::InitiativeRelation => "initiativeRelation",
            Resource::InitiativeRelations => "initiativeRelations",
            Resource::InitiativeToProject => "initiativeToProject",
            Resource::InitiativeToProjects => "initiativeToProjects",
            Resource::InitiativeUpdate => "initiativeUpdate",
            Resource::InitiativeUpdates => "initiativeUpdates",
            Resource::Initiatives => "initiatives",
            Resource::Integration => "integration",
            Resource::IntegrationHasScopes => "integrationHasScopes",
            Resource::IntegrationTemplate => "integrationTemplate",
            Resource::IntegrationTemplates => "integrationTemplates",
            Resource::Integrations => "integrations",
            Resource::IntegrationsSettings => "integrationsSettings",
            Resource::Issue => "issue",
            Resource::IssueFigmaFileKeySearch => "issueFigmaFileKeySearch",
            Resource::IssueFilterSuggestion => "issueFilterSuggestion",
            Resource::IssueImportCheckCSV => "issueImportCheckCSV",
            Resource::IssueImportCheckSync => "issueImportCheckSync",
            Resource::IssueImportJqlCheck => "issueImportJqlCheck",
            Resource::IssueLabel => "issueLabel",
            Resource::IssueLabels => "issueLabels",
            Resource::IssuePriorityValues => "issuePriorityValues",
            Resource::IssueRelation => "issueRelation",
            Resource::IssueRelations => "issueRelations",
            Resource::IssueRepositorySuggestions => "issueRepositorySuggestions",
            Resource::IssueSearch => "issueSearch",
            Resource::IssueTitleSuggestionFromCustomerRequest => "issueTitleSuggestionFromCustomerRequest",
            Resource::IssueToRelease => "issueToRelease",
            Resource::IssueToReleases => "issueToReleases",
            Resource::IssueVcsBranchSearch => "issueVcsBranchSearch",
            Resource::Issues => "issues",
            Resource::Notification => "notification",
            Resource::NotificationSubscription => "notificationSubscription",
            Resource::NotificationSubscriptions => "notificationSubscriptions",
            Resource::Notifications => "notifications",
            Resource::NotificationsUnreadCount => "notificationsUnreadCount",
            Resource::Organization => "organization",
            Resource::OrganizationDomainClaimRequest => "organizationDomainClaimRequest",
            Resource::OrganizationExists => "organizationExists",
            Resource::OrganizationInvite => "organizationInvite",
            Resource::OrganizationInviteDetails => "organizationInviteDetails",
            Resource::OrganizationInvites => "organizationInvites",
            Resource::OrganizationMeta => "organizationMeta",
            Resource::Project => "project",
            Resource::ProjectFilterSuggestion => "projectFilterSuggestion",
            Resource::ProjectLabel => "projectLabel",
            Resource::ProjectLabels => "projectLabels",
            Resource::ProjectMilestone => "projectMilestone",
            Resource::ProjectMilestones => "projectMilestones",
            Resource::ProjectRelation => "projectRelation",
            Resource::ProjectRelations => "projectRelations",
            Resource::ProjectStatus => "projectStatus",
            Resource::ProjectStatusProjectCount => "projectStatusProjectCount",
            Resource::ProjectStatuses => "projectStatuses",
            Resource::ProjectUpdate => "projectUpdate",
            Resource::ProjectUpdates => "projectUpdates",
            Resource::Projects => "projects",
            Resource::PushSubscriptionTest => "pushSubscriptionTest",
            Resource::RateLimitStatus => "rateLimitStatus",
            Resource::Release => "release",
            Resource::ReleasePipeline => "releasePipeline",
            Resource::ReleasePipelines => "releasePipelines",
            Resource::ReleaseStage => "releaseStage",
            Resource::ReleaseStages => "releaseStages",
            Resource::Releases => "releases",
            Resource::Roadmap => "roadmap",
            Resource::RoadmapToProject => "roadmapToProject",
            Resource::RoadmapToProjects => "roadmapToProjects",
            Resource::Roadmaps => "roadmaps",
            Resource::SearchDocuments => "searchDocuments",
            Resource::SearchIssues => "searchIssues",
            Resource::SearchProjects => "searchProjects",
            Resource::SemanticSearch => "semanticSearch",
            Resource::SsoUrlFromEmail => "ssoUrlFromEmail",
            Resource::Team => "team",
            Resource::TeamMembership => "teamMembership",
            Resource::TeamMemberships => "teamMemberships",
            Resource::Teams => "teams",
            Resource::Template => "template",
            Resource::Templates => "templates",
            Resource::TemplatesForIntegration => "templatesForIntegration",
            Resource::TimeSchedule => "timeSchedule",
            Resource::TimeSchedules => "timeSchedules",
            Resource::TriageResponsibilities => "triageResponsibilities",
            Resource::TriageResponsibility => "triageResponsibility",
            Resource::User => "user",
            Resource::UserSettings => "userSettings",
            Resource::Users => "users",
            Resource::VerifyGitHubEnterpriseServerInstallation => "verifyGitHubEnterpriseServerInstallation",
            Resource::Viewer => "viewer",
            Resource::Webhook => "webhook",
            Resource::Webhooks => "webhooks",
            Resource::WorkflowState => "workflowState",
            Resource::WorkflowStates => "workflowStates",
        }
    }
}
