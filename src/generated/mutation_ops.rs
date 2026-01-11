//! Generated mutation operations - DO NOT EDIT
//! Run `cargo xtask codegen` to regenerate

use clap::ValueEnum;

/// Available mutation operations derived from Linear's GraphQL schema
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "camelCase")]
pub enum MutationOp {
    /// Execute agentActivityCreate mutation
    #[value(name = "agentActivityCreate")]
    AgentActivityCreate,
    /// Execute agentActivityCreatePrompt mutation
    #[value(name = "agentActivityCreatePrompt")]
    AgentActivityCreatePrompt,
    /// Execute agentSessionCreate mutation
    #[value(name = "agentSessionCreate")]
    AgentSessionCreate,
    /// Execute agentSessionCreateOnComment mutation
    #[value(name = "agentSessionCreateOnComment")]
    AgentSessionCreateOnComment,
    /// Execute agentSessionCreateOnIssue mutation
    #[value(name = "agentSessionCreateOnIssue")]
    AgentSessionCreateOnIssue,
    /// Execute agentSessionUpdate mutation
    #[value(name = "agentSessionUpdate")]
    AgentSessionUpdate,
    /// Execute agentSessionUpdateExternalUrl mutation
    #[value(name = "agentSessionUpdateExternalUrl")]
    AgentSessionUpdateExternalUrl,
    /// Execute airbyteIntegrationConnect mutation
    #[value(name = "airbyteIntegrationConnect")]
    AirbyteIntegrationConnect,
    /// Execute asksWebFormsAuth mutation
    #[value(name = "asksWebFormsAuth")]
    AsksWebFormsAuth,
    /// Execute attachmentCreate mutation
    #[value(name = "attachmentCreate")]
    AttachmentCreate,
    /// Execute attachmentDelete mutation
    #[value(name = "attachmentDelete")]
    AttachmentDelete,
    /// Execute attachmentLinkDiscord mutation
    #[value(name = "attachmentLinkDiscord")]
    AttachmentLinkDiscord,
    /// Execute attachmentLinkFront mutation
    #[value(name = "attachmentLinkFront")]
    AttachmentLinkFront,
    /// Execute attachmentLinkGitHubIssue mutation
    #[value(name = "attachmentLinkGitHubIssue")]
    AttachmentLinkGitHubIssue,
    /// Execute attachmentLinkGitHubPR mutation
    #[value(name = "attachmentLinkGitHubPR")]
    AttachmentLinkGitHubPR,
    /// Execute attachmentLinkGitLabMR mutation
    #[value(name = "attachmentLinkGitLabMR")]
    AttachmentLinkGitLabMR,
    /// Execute attachmentLinkIntercom mutation
    #[value(name = "attachmentLinkIntercom")]
    AttachmentLinkIntercom,
    /// Execute attachmentLinkJiraIssue mutation
    #[value(name = "attachmentLinkJiraIssue")]
    AttachmentLinkJiraIssue,
    /// Execute attachmentLinkSalesforce mutation
    #[value(name = "attachmentLinkSalesforce")]
    AttachmentLinkSalesforce,
    /// Execute attachmentLinkSlack mutation
    #[value(name = "attachmentLinkSlack")]
    AttachmentLinkSlack,
    /// Execute attachmentLinkURL mutation
    #[value(name = "attachmentLinkURL")]
    AttachmentLinkURL,
    /// Execute attachmentLinkZendesk mutation
    #[value(name = "attachmentLinkZendesk")]
    AttachmentLinkZendesk,
    /// Execute attachmentSyncToSlack mutation
    #[value(name = "attachmentSyncToSlack")]
    AttachmentSyncToSlack,
    /// Execute attachmentUpdate mutation
    #[value(name = "attachmentUpdate")]
    AttachmentUpdate,
    /// Execute commentCreate mutation
    #[value(name = "commentCreate")]
    CommentCreate,
    /// Execute commentDelete mutation
    #[value(name = "commentDelete")]
    CommentDelete,
    /// Execute commentResolve mutation
    #[value(name = "commentResolve")]
    CommentResolve,
    /// Execute commentUnresolve mutation
    #[value(name = "commentUnresolve")]
    CommentUnresolve,
    /// Execute commentUpdate mutation
    #[value(name = "commentUpdate")]
    CommentUpdate,
    /// Execute contactCreate mutation
    #[value(name = "contactCreate")]
    ContactCreate,
    /// Execute contactSalesCreate mutation
    #[value(name = "contactSalesCreate")]
    ContactSalesCreate,
    /// Execute createCsvExportReport mutation
    #[value(name = "createCsvExportReport")]
    CreateCsvExportReport,
    /// Execute createInitiativeUpdateReminder mutation
    #[value(name = "createInitiativeUpdateReminder")]
    CreateInitiativeUpdateReminder,
    /// Execute createOrganizationFromOnboarding mutation
    #[value(name = "createOrganizationFromOnboarding")]
    CreateOrganizationFromOnboarding,
    /// Execute createProjectUpdateReminder mutation
    #[value(name = "createProjectUpdateReminder")]
    CreateProjectUpdateReminder,
    /// Execute customViewCreate mutation
    #[value(name = "customViewCreate")]
    CustomViewCreate,
    /// Execute customViewDelete mutation
    #[value(name = "customViewDelete")]
    CustomViewDelete,
    /// Execute customViewUpdate mutation
    #[value(name = "customViewUpdate")]
    CustomViewUpdate,
    /// Execute customerCreate mutation
    #[value(name = "customerCreate")]
    CustomerCreate,
    /// Execute customerDelete mutation
    #[value(name = "customerDelete")]
    CustomerDelete,
    /// Execute customerMerge mutation
    #[value(name = "customerMerge")]
    CustomerMerge,
    /// Execute customerNeedArchive mutation
    #[value(name = "customerNeedArchive")]
    CustomerNeedArchive,
    /// Execute customerNeedCreate mutation
    #[value(name = "customerNeedCreate")]
    CustomerNeedCreate,
    /// Execute customerNeedCreateFromAttachment mutation
    #[value(name = "customerNeedCreateFromAttachment")]
    CustomerNeedCreateFromAttachment,
    /// Execute customerNeedDelete mutation
    #[value(name = "customerNeedDelete")]
    CustomerNeedDelete,
    /// Execute customerNeedUnarchive mutation
    #[value(name = "customerNeedUnarchive")]
    CustomerNeedUnarchive,
    /// Execute customerNeedUpdate mutation
    #[value(name = "customerNeedUpdate")]
    CustomerNeedUpdate,
    /// Execute customerStatusCreate mutation
    #[value(name = "customerStatusCreate")]
    CustomerStatusCreate,
    /// Execute customerStatusDelete mutation
    #[value(name = "customerStatusDelete")]
    CustomerStatusDelete,
    /// Execute customerStatusUpdate mutation
    #[value(name = "customerStatusUpdate")]
    CustomerStatusUpdate,
    /// Execute customerTierCreate mutation
    #[value(name = "customerTierCreate")]
    CustomerTierCreate,
    /// Execute customerTierDelete mutation
    #[value(name = "customerTierDelete")]
    CustomerTierDelete,
    /// Execute customerTierUpdate mutation
    #[value(name = "customerTierUpdate")]
    CustomerTierUpdate,
    /// Execute customerUnsync mutation
    #[value(name = "customerUnsync")]
    CustomerUnsync,
    /// Execute customerUpdate mutation
    #[value(name = "customerUpdate")]
    CustomerUpdate,
    /// Execute customerUpsert mutation
    #[value(name = "customerUpsert")]
    CustomerUpsert,
    /// Execute cycleArchive mutation
    #[value(name = "cycleArchive")]
    CycleArchive,
    /// Execute cycleCreate mutation
    #[value(name = "cycleCreate")]
    CycleCreate,
    /// Execute cycleShiftAll mutation
    #[value(name = "cycleShiftAll")]
    CycleShiftAll,
    /// Execute cycleStartUpcomingCycleToday mutation
    #[value(name = "cycleStartUpcomingCycleToday")]
    CycleStartUpcomingCycleToday,
    /// Execute cycleUpdate mutation
    #[value(name = "cycleUpdate")]
    CycleUpdate,
    /// Execute documentCreate mutation
    #[value(name = "documentCreate")]
    DocumentCreate,
    /// Execute documentDelete mutation
    #[value(name = "documentDelete")]
    DocumentDelete,
    /// Execute documentUnarchive mutation
    #[value(name = "documentUnarchive")]
    DocumentUnarchive,
    /// Execute documentUpdate mutation
    #[value(name = "documentUpdate")]
    DocumentUpdate,
    /// Execute emailIntakeAddressCreate mutation
    #[value(name = "emailIntakeAddressCreate")]
    EmailIntakeAddressCreate,
    /// Execute emailIntakeAddressDelete mutation
    #[value(name = "emailIntakeAddressDelete")]
    EmailIntakeAddressDelete,
    /// Execute emailIntakeAddressRotate mutation
    #[value(name = "emailIntakeAddressRotate")]
    EmailIntakeAddressRotate,
    /// Execute emailIntakeAddressUpdate mutation
    #[value(name = "emailIntakeAddressUpdate")]
    EmailIntakeAddressUpdate,
    /// Execute emailTokenUserAccountAuth mutation
    #[value(name = "emailTokenUserAccountAuth")]
    EmailTokenUserAccountAuth,
    /// Execute emailUnsubscribe mutation
    #[value(name = "emailUnsubscribe")]
    EmailUnsubscribe,
    /// Execute emailUserAccountAuthChallenge mutation
    #[value(name = "emailUserAccountAuthChallenge")]
    EmailUserAccountAuthChallenge,
    /// Execute emojiCreate mutation
    #[value(name = "emojiCreate")]
    EmojiCreate,
    /// Execute emojiDelete mutation
    #[value(name = "emojiDelete")]
    EmojiDelete,
    /// Execute entityExternalLinkCreate mutation
    #[value(name = "entityExternalLinkCreate")]
    EntityExternalLinkCreate,
    /// Execute entityExternalLinkDelete mutation
    #[value(name = "entityExternalLinkDelete")]
    EntityExternalLinkDelete,
    /// Execute entityExternalLinkUpdate mutation
    #[value(name = "entityExternalLinkUpdate")]
    EntityExternalLinkUpdate,
    /// Execute favoriteCreate mutation
    #[value(name = "favoriteCreate")]
    FavoriteCreate,
    /// Execute favoriteDelete mutation
    #[value(name = "favoriteDelete")]
    FavoriteDelete,
    /// Execute favoriteUpdate mutation
    #[value(name = "favoriteUpdate")]
    FavoriteUpdate,
    /// Execute fileUpload mutation
    #[value(name = "fileUpload")]
    FileUpload,
    /// Execute fileUploadDangerouslyDelete mutation
    #[value(name = "fileUploadDangerouslyDelete")]
    FileUploadDangerouslyDelete,
    /// Execute gitAutomationStateCreate mutation
    #[value(name = "gitAutomationStateCreate")]
    GitAutomationStateCreate,
    /// Execute gitAutomationStateDelete mutation
    #[value(name = "gitAutomationStateDelete")]
    GitAutomationStateDelete,
    /// Execute gitAutomationStateUpdate mutation
    #[value(name = "gitAutomationStateUpdate")]
    GitAutomationStateUpdate,
    /// Execute gitAutomationTargetBranchCreate mutation
    #[value(name = "gitAutomationTargetBranchCreate")]
    GitAutomationTargetBranchCreate,
    /// Execute gitAutomationTargetBranchDelete mutation
    #[value(name = "gitAutomationTargetBranchDelete")]
    GitAutomationTargetBranchDelete,
    /// Execute gitAutomationTargetBranchUpdate mutation
    #[value(name = "gitAutomationTargetBranchUpdate")]
    GitAutomationTargetBranchUpdate,
    /// Execute googleUserAccountAuth mutation
    #[value(name = "googleUserAccountAuth")]
    GoogleUserAccountAuth,
    /// Execute imageUploadFromUrl mutation
    #[value(name = "imageUploadFromUrl")]
    ImageUploadFromUrl,
    /// Execute importFileUpload mutation
    #[value(name = "importFileUpload")]
    ImportFileUpload,
    /// Execute initiativeArchive mutation
    #[value(name = "initiativeArchive")]
    InitiativeArchive,
    /// Execute initiativeCreate mutation
    #[value(name = "initiativeCreate")]
    InitiativeCreate,
    /// Execute initiativeDelete mutation
    #[value(name = "initiativeDelete")]
    InitiativeDelete,
    /// Execute initiativeRelationCreate mutation
    #[value(name = "initiativeRelationCreate")]
    InitiativeRelationCreate,
    /// Execute initiativeRelationDelete mutation
    #[value(name = "initiativeRelationDelete")]
    InitiativeRelationDelete,
    /// Execute initiativeRelationUpdate mutation
    #[value(name = "initiativeRelationUpdate")]
    InitiativeRelationUpdate,
    /// Execute initiativeToProjectCreate mutation
    #[value(name = "initiativeToProjectCreate")]
    InitiativeToProjectCreate,
    /// Execute initiativeToProjectDelete mutation
    #[value(name = "initiativeToProjectDelete")]
    InitiativeToProjectDelete,
    /// Execute initiativeToProjectUpdate mutation
    #[value(name = "initiativeToProjectUpdate")]
    InitiativeToProjectUpdate,
    /// Execute initiativeUnarchive mutation
    #[value(name = "initiativeUnarchive")]
    InitiativeUnarchive,
    /// Execute initiativeUpdate mutation
    #[value(name = "initiativeUpdate")]
    InitiativeUpdate,
    /// Execute initiativeUpdateArchive mutation
    #[value(name = "initiativeUpdateArchive")]
    InitiativeUpdateArchive,
    /// Execute initiativeUpdateCreate mutation
    #[value(name = "initiativeUpdateCreate")]
    InitiativeUpdateCreate,
    /// Execute initiativeUpdateUnarchive mutation
    #[value(name = "initiativeUpdateUnarchive")]
    InitiativeUpdateUnarchive,
    /// Execute initiativeUpdateUpdate mutation
    #[value(name = "initiativeUpdateUpdate")]
    InitiativeUpdateUpdate,
    /// Execute integrationArchive mutation
    #[value(name = "integrationArchive")]
    IntegrationArchive,
    /// Execute integrationAsksConnectChannel mutation
    #[value(name = "integrationAsksConnectChannel")]
    IntegrationAsksConnectChannel,
    /// Execute integrationCustomerDataAttributesRefresh mutation
    #[value(name = "integrationCustomerDataAttributesRefresh")]
    IntegrationCustomerDataAttributesRefresh,
    /// Execute integrationDelete mutation
    #[value(name = "integrationDelete")]
    IntegrationDelete,
    /// Execute integrationDiscord mutation
    #[value(name = "integrationDiscord")]
    IntegrationDiscord,
    /// Execute integrationFigma mutation
    #[value(name = "integrationFigma")]
    IntegrationFigma,
    /// Execute integrationFront mutation
    #[value(name = "integrationFront")]
    IntegrationFront,
    /// Execute integrationGitHubEnterpriseServerConnect mutation
    #[value(name = "integrationGitHubEnterpriseServerConnect")]
    IntegrationGitHubEnterpriseServerConnect,
    /// Execute integrationGitHubPersonal mutation
    #[value(name = "integrationGitHubPersonal")]
    IntegrationGitHubPersonal,
    /// Execute integrationGithubCommitCreate mutation
    #[value(name = "integrationGithubCommitCreate")]
    IntegrationGithubCommitCreate,
    /// Execute integrationGithubConnect mutation
    #[value(name = "integrationGithubConnect")]
    IntegrationGithubConnect,
    /// Execute integrationGithubImportConnect mutation
    #[value(name = "integrationGithubImportConnect")]
    IntegrationGithubImportConnect,
    /// Execute integrationGithubImportRefresh mutation
    #[value(name = "integrationGithubImportRefresh")]
    IntegrationGithubImportRefresh,
    /// Execute integrationGitlabConnect mutation
    #[value(name = "integrationGitlabConnect")]
    IntegrationGitlabConnect,
    /// Execute integrationGong mutation
    #[value(name = "integrationGong")]
    IntegrationGong,
    /// Execute integrationGoogleCalendarPersonalConnect mutation
    #[value(name = "integrationGoogleCalendarPersonalConnect")]
    IntegrationGoogleCalendarPersonalConnect,
    /// Execute integrationGoogleSheets mutation
    #[value(name = "integrationGoogleSheets")]
    IntegrationGoogleSheets,
    /// Execute integrationIntercom mutation
    #[value(name = "integrationIntercom")]
    IntegrationIntercom,
    /// Execute integrationIntercomDelete mutation
    #[value(name = "integrationIntercomDelete")]
    IntegrationIntercomDelete,
    /// Execute integrationIntercomSettingsUpdate mutation
    #[value(name = "integrationIntercomSettingsUpdate")]
    IntegrationIntercomSettingsUpdate,
    /// Execute integrationJiraPersonal mutation
    #[value(name = "integrationJiraPersonal")]
    IntegrationJiraPersonal,
    /// Execute integrationJiraUpdate mutation
    #[value(name = "integrationJiraUpdate")]
    IntegrationJiraUpdate,
    /// Execute integrationLaunchDarklyConnect mutation
    #[value(name = "integrationLaunchDarklyConnect")]
    IntegrationLaunchDarklyConnect,
    /// Execute integrationLaunchDarklyPersonalConnect mutation
    #[value(name = "integrationLaunchDarklyPersonalConnect")]
    IntegrationLaunchDarklyPersonalConnect,
    /// Execute integrationLoom mutation
    #[value(name = "integrationLoom")]
    IntegrationLoom,
    /// Execute integrationMcpServerConnect mutation
    #[value(name = "integrationMcpServerConnect")]
    IntegrationMcpServerConnect,
    /// Execute integrationMcpServerPersonalConnect mutation
    #[value(name = "integrationMcpServerPersonalConnect")]
    IntegrationMcpServerPersonalConnect,
    /// Execute integrationOpsgenieConnect mutation
    #[value(name = "integrationOpsgenieConnect")]
    IntegrationOpsgenieConnect,
    /// Execute integrationOpsgenieRefreshScheduleMappings mutation
    #[value(name = "integrationOpsgenieRefreshScheduleMappings")]
    IntegrationOpsgenieRefreshScheduleMappings,
    /// Execute integrationPagerDutyConnect mutation
    #[value(name = "integrationPagerDutyConnect")]
    IntegrationPagerDutyConnect,
    /// Execute integrationPagerDutyRefreshScheduleMappings mutation
    #[value(name = "integrationPagerDutyRefreshScheduleMappings")]
    IntegrationPagerDutyRefreshScheduleMappings,
    /// Execute integrationRequest mutation
    #[value(name = "integrationRequest")]
    IntegrationRequest,
    /// Execute integrationSalesforce mutation
    #[value(name = "integrationSalesforce")]
    IntegrationSalesforce,
    /// Execute integrationSalesforceMetadataRefresh mutation
    #[value(name = "integrationSalesforceMetadataRefresh")]
    IntegrationSalesforceMetadataRefresh,
    /// Execute integrationSentryConnect mutation
    #[value(name = "integrationSentryConnect")]
    IntegrationSentryConnect,
    /// Execute integrationSettingsUpdate mutation
    #[value(name = "integrationSettingsUpdate")]
    IntegrationSettingsUpdate,
    /// Execute integrationSlack mutation
    #[value(name = "integrationSlack")]
    IntegrationSlack,
    /// Execute integrationSlackAsks mutation
    #[value(name = "integrationSlackAsks")]
    IntegrationSlackAsks,
    /// Execute integrationSlackCustomViewNotifications mutation
    #[value(name = "integrationSlackCustomViewNotifications")]
    IntegrationSlackCustomViewNotifications,
    /// Execute integrationSlackCustomerChannelLink mutation
    #[value(name = "integrationSlackCustomerChannelLink")]
    IntegrationSlackCustomerChannelLink,
    /// Execute integrationSlackImportEmojis mutation
    #[value(name = "integrationSlackImportEmojis")]
    IntegrationSlackImportEmojis,
    /// Execute integrationSlackInitiativePost mutation
    #[value(name = "integrationSlackInitiativePost")]
    IntegrationSlackInitiativePost,
    /// Execute integrationSlackOrAsksUpdateSlackTeamName mutation
    #[value(name = "integrationSlackOrAsksUpdateSlackTeamName")]
    IntegrationSlackOrAsksUpdateSlackTeamName,
    /// Execute integrationSlackOrgInitiativeUpdatesPost mutation
    #[value(name = "integrationSlackOrgInitiativeUpdatesPost")]
    IntegrationSlackOrgInitiativeUpdatesPost,
    /// Execute integrationSlackOrgProjectUpdatesPost mutation
    #[value(name = "integrationSlackOrgProjectUpdatesPost")]
    IntegrationSlackOrgProjectUpdatesPost,
    /// Execute integrationSlackPersonal mutation
    #[value(name = "integrationSlackPersonal")]
    IntegrationSlackPersonal,
    /// Execute integrationSlackPost mutation
    #[value(name = "integrationSlackPost")]
    IntegrationSlackPost,
    /// Execute integrationSlackProjectPost mutation
    #[value(name = "integrationSlackProjectPost")]
    IntegrationSlackProjectPost,
    /// Execute integrationSlackWorkflowAccessUpdate mutation
    #[value(name = "integrationSlackWorkflowAccessUpdate")]
    IntegrationSlackWorkflowAccessUpdate,
    /// Execute integrationTemplateCreate mutation
    #[value(name = "integrationTemplateCreate")]
    IntegrationTemplateCreate,
    /// Execute integrationTemplateDelete mutation
    #[value(name = "integrationTemplateDelete")]
    IntegrationTemplateDelete,
    /// Execute integrationUpdate mutation
    #[value(name = "integrationUpdate")]
    IntegrationUpdate,
    /// Execute integrationZendesk mutation
    #[value(name = "integrationZendesk")]
    IntegrationZendesk,
    /// Execute integrationsSettingsCreate mutation
    #[value(name = "integrationsSettingsCreate")]
    IntegrationsSettingsCreate,
    /// Execute integrationsSettingsUpdate mutation
    #[value(name = "integrationsSettingsUpdate")]
    IntegrationsSettingsUpdate,
    /// Execute issueAddLabel mutation
    #[value(name = "issueAddLabel")]
    IssueAddLabel,
    /// Execute issueArchive mutation
    #[value(name = "issueArchive")]
    IssueArchive,
    /// Execute issueBatchCreate mutation
    #[value(name = "issueBatchCreate")]
    IssueBatchCreate,
    /// Execute issueBatchUpdate mutation
    #[value(name = "issueBatchUpdate")]
    IssueBatchUpdate,
    /// Execute issueCreate mutation
    #[value(name = "issueCreate")]
    IssueCreate,
    /// Execute issueDelete mutation
    #[value(name = "issueDelete")]
    IssueDelete,
    /// Execute issueDescriptionUpdateFromFront mutation
    #[value(name = "issueDescriptionUpdateFromFront")]
    IssueDescriptionUpdateFromFront,
    /// Execute issueExternalSyncDisable mutation
    #[value(name = "issueExternalSyncDisable")]
    IssueExternalSyncDisable,
    /// Execute issueImportCreateAsana mutation
    #[value(name = "issueImportCreateAsana")]
    IssueImportCreateAsana,
    /// Execute issueImportCreateCSVJira mutation
    #[value(name = "issueImportCreateCSVJira")]
    IssueImportCreateCSVJira,
    /// Execute issueImportCreateClubhouse mutation
    #[value(name = "issueImportCreateClubhouse")]
    IssueImportCreateClubhouse,
    /// Execute issueImportCreateGithub mutation
    #[value(name = "issueImportCreateGithub")]
    IssueImportCreateGithub,
    /// Execute issueImportCreateJira mutation
    #[value(name = "issueImportCreateJira")]
    IssueImportCreateJira,
    /// Execute issueImportCreateLinearV2 mutation
    #[value(name = "issueImportCreateLinearV2")]
    IssueImportCreateLinearV2,
    /// Execute issueImportDelete mutation
    #[value(name = "issueImportDelete")]
    IssueImportDelete,
    /// Execute issueImportProcess mutation
    #[value(name = "issueImportProcess")]
    IssueImportProcess,
    /// Execute issueImportUpdate mutation
    #[value(name = "issueImportUpdate")]
    IssueImportUpdate,
    /// Execute issueLabelCreate mutation
    #[value(name = "issueLabelCreate")]
    IssueLabelCreate,
    /// Execute issueLabelDelete mutation
    #[value(name = "issueLabelDelete")]
    IssueLabelDelete,
    /// Execute issueLabelRestore mutation
    #[value(name = "issueLabelRestore")]
    IssueLabelRestore,
    /// Execute issueLabelRetire mutation
    #[value(name = "issueLabelRetire")]
    IssueLabelRetire,
    /// Execute issueLabelUpdate mutation
    #[value(name = "issueLabelUpdate")]
    IssueLabelUpdate,
    /// Execute issueRelationCreate mutation
    #[value(name = "issueRelationCreate")]
    IssueRelationCreate,
    /// Execute issueRelationDelete mutation
    #[value(name = "issueRelationDelete")]
    IssueRelationDelete,
    /// Execute issueRelationUpdate mutation
    #[value(name = "issueRelationUpdate")]
    IssueRelationUpdate,
    /// Execute issueReminder mutation
    #[value(name = "issueReminder")]
    IssueReminder,
    /// Execute issueRemoveLabel mutation
    #[value(name = "issueRemoveLabel")]
    IssueRemoveLabel,
    /// Execute issueSubscribe mutation
    #[value(name = "issueSubscribe")]
    IssueSubscribe,
    /// Execute issueToReleaseCreate mutation
    #[value(name = "issueToReleaseCreate")]
    IssueToReleaseCreate,
    /// Execute issueToReleaseDelete mutation
    #[value(name = "issueToReleaseDelete")]
    IssueToReleaseDelete,
    /// Execute issueToReleaseDeleteByIssueAndRelease mutation
    #[value(name = "issueToReleaseDeleteByIssueAndRelease")]
    IssueToReleaseDeleteByIssueAndRelease,
    /// Execute issueUnarchive mutation
    #[value(name = "issueUnarchive")]
    IssueUnarchive,
    /// Execute issueUnsubscribe mutation
    #[value(name = "issueUnsubscribe")]
    IssueUnsubscribe,
    /// Execute issueUpdate mutation
    #[value(name = "issueUpdate")]
    IssueUpdate,
    /// Execute jiraIntegrationConnect mutation
    #[value(name = "jiraIntegrationConnect")]
    JiraIntegrationConnect,
    /// Execute joinOrganizationFromOnboarding mutation
    #[value(name = "joinOrganizationFromOnboarding")]
    JoinOrganizationFromOnboarding,
    /// Execute leaveOrganization mutation
    #[value(name = "leaveOrganization")]
    LeaveOrganization,
    /// Execute logout mutation
    #[value(name = "logout")]
    Logout,
    /// Execute logoutAllSessions mutation
    #[value(name = "logoutAllSessions")]
    LogoutAllSessions,
    /// Execute logoutOtherSessions mutation
    #[value(name = "logoutOtherSessions")]
    LogoutOtherSessions,
    /// Execute logoutSession mutation
    #[value(name = "logoutSession")]
    LogoutSession,
    /// Execute notificationArchive mutation
    #[value(name = "notificationArchive")]
    NotificationArchive,
    /// Execute notificationArchiveAll mutation
    #[value(name = "notificationArchiveAll")]
    NotificationArchiveAll,
    /// Execute notificationCategoryChannelSubscriptionUpdate mutation
    #[value(name = "notificationCategoryChannelSubscriptionUpdate")]
    NotificationCategoryChannelSubscriptionUpdate,
    /// Execute notificationMarkReadAll mutation
    #[value(name = "notificationMarkReadAll")]
    NotificationMarkReadAll,
    /// Execute notificationMarkUnreadAll mutation
    #[value(name = "notificationMarkUnreadAll")]
    NotificationMarkUnreadAll,
    /// Execute notificationSnoozeAll mutation
    #[value(name = "notificationSnoozeAll")]
    NotificationSnoozeAll,
    /// Execute notificationSubscriptionCreate mutation
    #[value(name = "notificationSubscriptionCreate")]
    NotificationSubscriptionCreate,
    /// Execute notificationSubscriptionDelete mutation
    #[value(name = "notificationSubscriptionDelete")]
    NotificationSubscriptionDelete,
    /// Execute notificationSubscriptionUpdate mutation
    #[value(name = "notificationSubscriptionUpdate")]
    NotificationSubscriptionUpdate,
    /// Execute notificationUnarchive mutation
    #[value(name = "notificationUnarchive")]
    NotificationUnarchive,
    /// Execute notificationUnsnoozeAll mutation
    #[value(name = "notificationUnsnoozeAll")]
    NotificationUnsnoozeAll,
    /// Execute notificationUpdate mutation
    #[value(name = "notificationUpdate")]
    NotificationUpdate,
    /// Execute organizationCancelDelete mutation
    #[value(name = "organizationCancelDelete")]
    OrganizationCancelDelete,
    /// Execute organizationDelete mutation
    #[value(name = "organizationDelete")]
    OrganizationDelete,
    /// Execute organizationDeleteChallenge mutation
    #[value(name = "organizationDeleteChallenge")]
    OrganizationDeleteChallenge,
    /// Execute organizationDomainClaim mutation
    #[value(name = "organizationDomainClaim")]
    OrganizationDomainClaim,
    /// Execute organizationDomainCreate mutation
    #[value(name = "organizationDomainCreate")]
    OrganizationDomainCreate,
    /// Execute organizationDomainDelete mutation
    #[value(name = "organizationDomainDelete")]
    OrganizationDomainDelete,
    /// Execute organizationDomainUpdate mutation
    #[value(name = "organizationDomainUpdate")]
    OrganizationDomainUpdate,
    /// Execute organizationDomainVerify mutation
    #[value(name = "organizationDomainVerify")]
    OrganizationDomainVerify,
    /// Execute organizationInviteCreate mutation
    #[value(name = "organizationInviteCreate")]
    OrganizationInviteCreate,
    /// Execute organizationInviteDelete mutation
    #[value(name = "organizationInviteDelete")]
    OrganizationInviteDelete,
    /// Execute organizationInviteUpdate mutation
    #[value(name = "organizationInviteUpdate")]
    OrganizationInviteUpdate,
    /// Execute organizationStartTrial mutation
    #[value(name = "organizationStartTrial")]
    OrganizationStartTrial,
    /// Execute organizationStartTrialForPlan mutation
    #[value(name = "organizationStartTrialForPlan")]
    OrganizationStartTrialForPlan,
    /// Execute organizationUpdate mutation
    #[value(name = "organizationUpdate")]
    OrganizationUpdate,
    /// Execute passkeyLoginFinish mutation
    #[value(name = "passkeyLoginFinish")]
    PasskeyLoginFinish,
    /// Execute passkeyLoginStart mutation
    #[value(name = "passkeyLoginStart")]
    PasskeyLoginStart,
    /// Execute projectAddLabel mutation
    #[value(name = "projectAddLabel")]
    ProjectAddLabel,
    /// Execute projectArchive mutation
    #[value(name = "projectArchive")]
    ProjectArchive,
    /// Execute projectCreate mutation
    #[value(name = "projectCreate")]
    ProjectCreate,
    /// Execute projectDelete mutation
    #[value(name = "projectDelete")]
    ProjectDelete,
    /// Execute projectExternalSyncDisable mutation
    #[value(name = "projectExternalSyncDisable")]
    ProjectExternalSyncDisable,
    /// Execute projectLabelCreate mutation
    #[value(name = "projectLabelCreate")]
    ProjectLabelCreate,
    /// Execute projectLabelDelete mutation
    #[value(name = "projectLabelDelete")]
    ProjectLabelDelete,
    /// Execute projectLabelRestore mutation
    #[value(name = "projectLabelRestore")]
    ProjectLabelRestore,
    /// Execute projectLabelRetire mutation
    #[value(name = "projectLabelRetire")]
    ProjectLabelRetire,
    /// Execute projectLabelUpdate mutation
    #[value(name = "projectLabelUpdate")]
    ProjectLabelUpdate,
    /// Execute projectMilestoneCreate mutation
    #[value(name = "projectMilestoneCreate")]
    ProjectMilestoneCreate,
    /// Execute projectMilestoneDelete mutation
    #[value(name = "projectMilestoneDelete")]
    ProjectMilestoneDelete,
    /// Execute projectMilestoneMove mutation
    #[value(name = "projectMilestoneMove")]
    ProjectMilestoneMove,
    /// Execute projectMilestoneUpdate mutation
    #[value(name = "projectMilestoneUpdate")]
    ProjectMilestoneUpdate,
    /// Execute projectReassignStatus mutation
    #[value(name = "projectReassignStatus")]
    ProjectReassignStatus,
    /// Execute projectRelationCreate mutation
    #[value(name = "projectRelationCreate")]
    ProjectRelationCreate,
    /// Execute projectRelationDelete mutation
    #[value(name = "projectRelationDelete")]
    ProjectRelationDelete,
    /// Execute projectRelationUpdate mutation
    #[value(name = "projectRelationUpdate")]
    ProjectRelationUpdate,
    /// Execute projectRemoveLabel mutation
    #[value(name = "projectRemoveLabel")]
    ProjectRemoveLabel,
    /// Execute projectStatusArchive mutation
    #[value(name = "projectStatusArchive")]
    ProjectStatusArchive,
    /// Execute projectStatusCreate mutation
    #[value(name = "projectStatusCreate")]
    ProjectStatusCreate,
    /// Execute projectStatusUnarchive mutation
    #[value(name = "projectStatusUnarchive")]
    ProjectStatusUnarchive,
    /// Execute projectStatusUpdate mutation
    #[value(name = "projectStatusUpdate")]
    ProjectStatusUpdate,
    /// Execute projectUnarchive mutation
    #[value(name = "projectUnarchive")]
    ProjectUnarchive,
    /// Execute projectUpdate mutation
    #[value(name = "projectUpdate")]
    ProjectUpdate,
    /// Execute projectUpdateArchive mutation
    #[value(name = "projectUpdateArchive")]
    ProjectUpdateArchive,
    /// Execute projectUpdateCreate mutation
    #[value(name = "projectUpdateCreate")]
    ProjectUpdateCreate,
    /// Execute projectUpdateDelete mutation
    #[value(name = "projectUpdateDelete")]
    ProjectUpdateDelete,
    /// Execute projectUpdateUnarchive mutation
    #[value(name = "projectUpdateUnarchive")]
    ProjectUpdateUnarchive,
    /// Execute projectUpdateUpdate mutation
    #[value(name = "projectUpdateUpdate")]
    ProjectUpdateUpdate,
    /// Execute pushSubscriptionCreate mutation
    #[value(name = "pushSubscriptionCreate")]
    PushSubscriptionCreate,
    /// Execute pushSubscriptionDelete mutation
    #[value(name = "pushSubscriptionDelete")]
    PushSubscriptionDelete,
    /// Execute reactionCreate mutation
    #[value(name = "reactionCreate")]
    ReactionCreate,
    /// Execute reactionDelete mutation
    #[value(name = "reactionDelete")]
    ReactionDelete,
    /// Execute refreshGoogleSheetsData mutation
    #[value(name = "refreshGoogleSheetsData")]
    RefreshGoogleSheetsData,
    /// Execute releaseArchive mutation
    #[value(name = "releaseArchive")]
    ReleaseArchive,
    /// Execute releaseCreate mutation
    #[value(name = "releaseCreate")]
    ReleaseCreate,
    /// Execute releasePipelineArchive mutation
    #[value(name = "releasePipelineArchive")]
    ReleasePipelineArchive,
    /// Execute releasePipelineCreate mutation
    #[value(name = "releasePipelineCreate")]
    ReleasePipelineCreate,
    /// Execute releasePipelineDelete mutation
    #[value(name = "releasePipelineDelete")]
    ReleasePipelineDelete,
    /// Execute releasePipelineUnarchive mutation
    #[value(name = "releasePipelineUnarchive")]
    ReleasePipelineUnarchive,
    /// Execute releasePipelineUpdate mutation
    #[value(name = "releasePipelineUpdate")]
    ReleasePipelineUpdate,
    /// Execute releaseStageArchive mutation
    #[value(name = "releaseStageArchive")]
    ReleaseStageArchive,
    /// Execute releaseStageCreate mutation
    #[value(name = "releaseStageCreate")]
    ReleaseStageCreate,
    /// Execute releaseStageUnarchive mutation
    #[value(name = "releaseStageUnarchive")]
    ReleaseStageUnarchive,
    /// Execute releaseStageUpdate mutation
    #[value(name = "releaseStageUpdate")]
    ReleaseStageUpdate,
    /// Execute releaseUnarchive mutation
    #[value(name = "releaseUnarchive")]
    ReleaseUnarchive,
    /// Execute releaseUpdate mutation
    #[value(name = "releaseUpdate")]
    ReleaseUpdate,
    /// Execute resendOrganizationInvite mutation
    #[value(name = "resendOrganizationInvite")]
    ResendOrganizationInvite,
    /// Execute resendOrganizationInviteByEmail mutation
    #[value(name = "resendOrganizationInviteByEmail")]
    ResendOrganizationInviteByEmail,
    /// Execute roadmapArchive mutation
    #[value(name = "roadmapArchive")]
    RoadmapArchive,
    /// Execute roadmapCreate mutation
    #[value(name = "roadmapCreate")]
    RoadmapCreate,
    /// Execute roadmapDelete mutation
    #[value(name = "roadmapDelete")]
    RoadmapDelete,
    /// Execute roadmapToProjectCreate mutation
    #[value(name = "roadmapToProjectCreate")]
    RoadmapToProjectCreate,
    /// Execute roadmapToProjectDelete mutation
    #[value(name = "roadmapToProjectDelete")]
    RoadmapToProjectDelete,
    /// Execute roadmapToProjectUpdate mutation
    #[value(name = "roadmapToProjectUpdate")]
    RoadmapToProjectUpdate,
    /// Execute roadmapUnarchive mutation
    #[value(name = "roadmapUnarchive")]
    RoadmapUnarchive,
    /// Execute roadmapUpdate mutation
    #[value(name = "roadmapUpdate")]
    RoadmapUpdate,
    /// Execute samlTokenUserAccountAuth mutation
    #[value(name = "samlTokenUserAccountAuth")]
    SamlTokenUserAccountAuth,
    /// Execute teamCreate mutation
    #[value(name = "teamCreate")]
    TeamCreate,
    /// Execute teamCyclesDelete mutation
    #[value(name = "teamCyclesDelete")]
    TeamCyclesDelete,
    /// Execute teamDelete mutation
    #[value(name = "teamDelete")]
    TeamDelete,
    /// Execute teamKeyDelete mutation
    #[value(name = "teamKeyDelete")]
    TeamKeyDelete,
    /// Execute teamMembershipCreate mutation
    #[value(name = "teamMembershipCreate")]
    TeamMembershipCreate,
    /// Execute teamMembershipDelete mutation
    #[value(name = "teamMembershipDelete")]
    TeamMembershipDelete,
    /// Execute teamMembershipUpdate mutation
    #[value(name = "teamMembershipUpdate")]
    TeamMembershipUpdate,
    /// Execute teamUnarchive mutation
    #[value(name = "teamUnarchive")]
    TeamUnarchive,
    /// Execute teamUpdate mutation
    #[value(name = "teamUpdate")]
    TeamUpdate,
    /// Execute templateCreate mutation
    #[value(name = "templateCreate")]
    TemplateCreate,
    /// Execute templateDelete mutation
    #[value(name = "templateDelete")]
    TemplateDelete,
    /// Execute templateUpdate mutation
    #[value(name = "templateUpdate")]
    TemplateUpdate,
    /// Execute timeScheduleCreate mutation
    #[value(name = "timeScheduleCreate")]
    TimeScheduleCreate,
    /// Execute timeScheduleDelete mutation
    #[value(name = "timeScheduleDelete")]
    TimeScheduleDelete,
    /// Execute timeScheduleRefreshIntegrationSchedule mutation
    #[value(name = "timeScheduleRefreshIntegrationSchedule")]
    TimeScheduleRefreshIntegrationSchedule,
    /// Execute timeScheduleUpdate mutation
    #[value(name = "timeScheduleUpdate")]
    TimeScheduleUpdate,
    /// Execute timeScheduleUpsertExternal mutation
    #[value(name = "timeScheduleUpsertExternal")]
    TimeScheduleUpsertExternal,
    /// Execute triageResponsibilityCreate mutation
    #[value(name = "triageResponsibilityCreate")]
    TriageResponsibilityCreate,
    /// Execute triageResponsibilityDelete mutation
    #[value(name = "triageResponsibilityDelete")]
    TriageResponsibilityDelete,
    /// Execute triageResponsibilityUpdate mutation
    #[value(name = "triageResponsibilityUpdate")]
    TriageResponsibilityUpdate,
    /// Execute updateIntegrationSlackScopes mutation
    #[value(name = "updateIntegrationSlackScopes")]
    UpdateIntegrationSlackScopes,
    /// Execute userChangeRole mutation
    #[value(name = "userChangeRole")]
    UserChangeRole,
    /// Execute userDemoteAdmin mutation
    #[value(name = "userDemoteAdmin")]
    UserDemoteAdmin,
    /// Execute userDemoteMember mutation
    #[value(name = "userDemoteMember")]
    UserDemoteMember,
    /// Execute userDiscordConnect mutation
    #[value(name = "userDiscordConnect")]
    UserDiscordConnect,
    /// Execute userExternalUserDisconnect mutation
    #[value(name = "userExternalUserDisconnect")]
    UserExternalUserDisconnect,
    /// Execute userFlagUpdate mutation
    #[value(name = "userFlagUpdate")]
    UserFlagUpdate,
    /// Execute userPromoteAdmin mutation
    #[value(name = "userPromoteAdmin")]
    UserPromoteAdmin,
    /// Execute userPromoteMember mutation
    #[value(name = "userPromoteMember")]
    UserPromoteMember,
    /// Execute userSettingsFlagsReset mutation
    #[value(name = "userSettingsFlagsReset")]
    UserSettingsFlagsReset,
    /// Execute userSettingsUpdate mutation
    #[value(name = "userSettingsUpdate")]
    UserSettingsUpdate,
    /// Execute userSuspend mutation
    #[value(name = "userSuspend")]
    UserSuspend,
    /// Execute userUnlinkFromIdentityProvider mutation
    #[value(name = "userUnlinkFromIdentityProvider")]
    UserUnlinkFromIdentityProvider,
    /// Execute userUnsuspend mutation
    #[value(name = "userUnsuspend")]
    UserUnsuspend,
    /// Execute userUpdate mutation
    #[value(name = "userUpdate")]
    UserUpdate,
    /// Execute viewPreferencesCreate mutation
    #[value(name = "viewPreferencesCreate")]
    ViewPreferencesCreate,
    /// Execute viewPreferencesDelete mutation
    #[value(name = "viewPreferencesDelete")]
    ViewPreferencesDelete,
    /// Execute viewPreferencesUpdate mutation
    #[value(name = "viewPreferencesUpdate")]
    ViewPreferencesUpdate,
    /// Execute webhookCreate mutation
    #[value(name = "webhookCreate")]
    WebhookCreate,
    /// Execute webhookDelete mutation
    #[value(name = "webhookDelete")]
    WebhookDelete,
    /// Execute webhookUpdate mutation
    #[value(name = "webhookUpdate")]
    WebhookUpdate,
    /// Execute workflowStateArchive mutation
    #[value(name = "workflowStateArchive")]
    WorkflowStateArchive,
    /// Execute workflowStateCreate mutation
    #[value(name = "workflowStateCreate")]
    WorkflowStateCreate,
    /// Execute workflowStateUpdate mutation
    #[value(name = "workflowStateUpdate")]
    WorkflowStateUpdate,
}

impl MutationOp {
    /// Get all available mutation operations
    pub fn all() -> &'static [MutationOp] {
        use MutationOp::*;
        &[
            AgentActivityCreate,
            AgentActivityCreatePrompt,
            AgentSessionCreate,
            AgentSessionCreateOnComment,
            AgentSessionCreateOnIssue,
            AgentSessionUpdate,
            AgentSessionUpdateExternalUrl,
            AirbyteIntegrationConnect,
            AsksWebFormsAuth,
            AttachmentCreate,
            AttachmentDelete,
            AttachmentLinkDiscord,
            AttachmentLinkFront,
            AttachmentLinkGitHubIssue,
            AttachmentLinkGitHubPR,
            AttachmentLinkGitLabMR,
            AttachmentLinkIntercom,
            AttachmentLinkJiraIssue,
            AttachmentLinkSalesforce,
            AttachmentLinkSlack,
            AttachmentLinkURL,
            AttachmentLinkZendesk,
            AttachmentSyncToSlack,
            AttachmentUpdate,
            CommentCreate,
            CommentDelete,
            CommentResolve,
            CommentUnresolve,
            CommentUpdate,
            ContactCreate,
            ContactSalesCreate,
            CreateCsvExportReport,
            CreateInitiativeUpdateReminder,
            CreateOrganizationFromOnboarding,
            CreateProjectUpdateReminder,
            CustomViewCreate,
            CustomViewDelete,
            CustomViewUpdate,
            CustomerCreate,
            CustomerDelete,
            CustomerMerge,
            CustomerNeedArchive,
            CustomerNeedCreate,
            CustomerNeedCreateFromAttachment,
            CustomerNeedDelete,
            CustomerNeedUnarchive,
            CustomerNeedUpdate,
            CustomerStatusCreate,
            CustomerStatusDelete,
            CustomerStatusUpdate,
            CustomerTierCreate,
            CustomerTierDelete,
            CustomerTierUpdate,
            CustomerUnsync,
            CustomerUpdate,
            CustomerUpsert,
            CycleArchive,
            CycleCreate,
            CycleShiftAll,
            CycleStartUpcomingCycleToday,
            CycleUpdate,
            DocumentCreate,
            DocumentDelete,
            DocumentUnarchive,
            DocumentUpdate,
            EmailIntakeAddressCreate,
            EmailIntakeAddressDelete,
            EmailIntakeAddressRotate,
            EmailIntakeAddressUpdate,
            EmailTokenUserAccountAuth,
            EmailUnsubscribe,
            EmailUserAccountAuthChallenge,
            EmojiCreate,
            EmojiDelete,
            EntityExternalLinkCreate,
            EntityExternalLinkDelete,
            EntityExternalLinkUpdate,
            FavoriteCreate,
            FavoriteDelete,
            FavoriteUpdate,
            FileUpload,
            FileUploadDangerouslyDelete,
            GitAutomationStateCreate,
            GitAutomationStateDelete,
            GitAutomationStateUpdate,
            GitAutomationTargetBranchCreate,
            GitAutomationTargetBranchDelete,
            GitAutomationTargetBranchUpdate,
            GoogleUserAccountAuth,
            ImageUploadFromUrl,
            ImportFileUpload,
            InitiativeArchive,
            InitiativeCreate,
            InitiativeDelete,
            InitiativeRelationCreate,
            InitiativeRelationDelete,
            InitiativeRelationUpdate,
            InitiativeToProjectCreate,
            InitiativeToProjectDelete,
            InitiativeToProjectUpdate,
            InitiativeUnarchive,
            InitiativeUpdate,
            InitiativeUpdateArchive,
            InitiativeUpdateCreate,
            InitiativeUpdateUnarchive,
            InitiativeUpdateUpdate,
            IntegrationArchive,
            IntegrationAsksConnectChannel,
            IntegrationCustomerDataAttributesRefresh,
            IntegrationDelete,
            IntegrationDiscord,
            IntegrationFigma,
            IntegrationFront,
            IntegrationGitHubEnterpriseServerConnect,
            IntegrationGitHubPersonal,
            IntegrationGithubCommitCreate,
            IntegrationGithubConnect,
            IntegrationGithubImportConnect,
            IntegrationGithubImportRefresh,
            IntegrationGitlabConnect,
            IntegrationGong,
            IntegrationGoogleCalendarPersonalConnect,
            IntegrationGoogleSheets,
            IntegrationIntercom,
            IntegrationIntercomDelete,
            IntegrationIntercomSettingsUpdate,
            IntegrationJiraPersonal,
            IntegrationJiraUpdate,
            IntegrationLaunchDarklyConnect,
            IntegrationLaunchDarklyPersonalConnect,
            IntegrationLoom,
            IntegrationMcpServerConnect,
            IntegrationMcpServerPersonalConnect,
            IntegrationOpsgenieConnect,
            IntegrationOpsgenieRefreshScheduleMappings,
            IntegrationPagerDutyConnect,
            IntegrationPagerDutyRefreshScheduleMappings,
            IntegrationRequest,
            IntegrationSalesforce,
            IntegrationSalesforceMetadataRefresh,
            IntegrationSentryConnect,
            IntegrationSettingsUpdate,
            IntegrationSlack,
            IntegrationSlackAsks,
            IntegrationSlackCustomViewNotifications,
            IntegrationSlackCustomerChannelLink,
            IntegrationSlackImportEmojis,
            IntegrationSlackInitiativePost,
            IntegrationSlackOrAsksUpdateSlackTeamName,
            IntegrationSlackOrgInitiativeUpdatesPost,
            IntegrationSlackOrgProjectUpdatesPost,
            IntegrationSlackPersonal,
            IntegrationSlackPost,
            IntegrationSlackProjectPost,
            IntegrationSlackWorkflowAccessUpdate,
            IntegrationTemplateCreate,
            IntegrationTemplateDelete,
            IntegrationUpdate,
            IntegrationZendesk,
            IntegrationsSettingsCreate,
            IntegrationsSettingsUpdate,
            IssueAddLabel,
            IssueArchive,
            IssueBatchCreate,
            IssueBatchUpdate,
            IssueCreate,
            IssueDelete,
            IssueDescriptionUpdateFromFront,
            IssueExternalSyncDisable,
            IssueImportCreateAsana,
            IssueImportCreateCSVJira,
            IssueImportCreateClubhouse,
            IssueImportCreateGithub,
            IssueImportCreateJira,
            IssueImportCreateLinearV2,
            IssueImportDelete,
            IssueImportProcess,
            IssueImportUpdate,
            IssueLabelCreate,
            IssueLabelDelete,
            IssueLabelRestore,
            IssueLabelRetire,
            IssueLabelUpdate,
            IssueRelationCreate,
            IssueRelationDelete,
            IssueRelationUpdate,
            IssueReminder,
            IssueRemoveLabel,
            IssueSubscribe,
            IssueToReleaseCreate,
            IssueToReleaseDelete,
            IssueToReleaseDeleteByIssueAndRelease,
            IssueUnarchive,
            IssueUnsubscribe,
            IssueUpdate,
            JiraIntegrationConnect,
            JoinOrganizationFromOnboarding,
            LeaveOrganization,
            Logout,
            LogoutAllSessions,
            LogoutOtherSessions,
            LogoutSession,
            NotificationArchive,
            NotificationArchiveAll,
            NotificationCategoryChannelSubscriptionUpdate,
            NotificationMarkReadAll,
            NotificationMarkUnreadAll,
            NotificationSnoozeAll,
            NotificationSubscriptionCreate,
            NotificationSubscriptionDelete,
            NotificationSubscriptionUpdate,
            NotificationUnarchive,
            NotificationUnsnoozeAll,
            NotificationUpdate,
            OrganizationCancelDelete,
            OrganizationDelete,
            OrganizationDeleteChallenge,
            OrganizationDomainClaim,
            OrganizationDomainCreate,
            OrganizationDomainDelete,
            OrganizationDomainUpdate,
            OrganizationDomainVerify,
            OrganizationInviteCreate,
            OrganizationInviteDelete,
            OrganizationInviteUpdate,
            OrganizationStartTrial,
            OrganizationStartTrialForPlan,
            OrganizationUpdate,
            PasskeyLoginFinish,
            PasskeyLoginStart,
            ProjectAddLabel,
            ProjectArchive,
            ProjectCreate,
            ProjectDelete,
            ProjectExternalSyncDisable,
            ProjectLabelCreate,
            ProjectLabelDelete,
            ProjectLabelRestore,
            ProjectLabelRetire,
            ProjectLabelUpdate,
            ProjectMilestoneCreate,
            ProjectMilestoneDelete,
            ProjectMilestoneMove,
            ProjectMilestoneUpdate,
            ProjectReassignStatus,
            ProjectRelationCreate,
            ProjectRelationDelete,
            ProjectRelationUpdate,
            ProjectRemoveLabel,
            ProjectStatusArchive,
            ProjectStatusCreate,
            ProjectStatusUnarchive,
            ProjectStatusUpdate,
            ProjectUnarchive,
            ProjectUpdate,
            ProjectUpdateArchive,
            ProjectUpdateCreate,
            ProjectUpdateDelete,
            ProjectUpdateUnarchive,
            ProjectUpdateUpdate,
            PushSubscriptionCreate,
            PushSubscriptionDelete,
            ReactionCreate,
            ReactionDelete,
            RefreshGoogleSheetsData,
            ReleaseArchive,
            ReleaseCreate,
            ReleasePipelineArchive,
            ReleasePipelineCreate,
            ReleasePipelineDelete,
            ReleasePipelineUnarchive,
            ReleasePipelineUpdate,
            ReleaseStageArchive,
            ReleaseStageCreate,
            ReleaseStageUnarchive,
            ReleaseStageUpdate,
            ReleaseUnarchive,
            ReleaseUpdate,
            ResendOrganizationInvite,
            ResendOrganizationInviteByEmail,
            RoadmapArchive,
            RoadmapCreate,
            RoadmapDelete,
            RoadmapToProjectCreate,
            RoadmapToProjectDelete,
            RoadmapToProjectUpdate,
            RoadmapUnarchive,
            RoadmapUpdate,
            SamlTokenUserAccountAuth,
            TeamCreate,
            TeamCyclesDelete,
            TeamDelete,
            TeamKeyDelete,
            TeamMembershipCreate,
            TeamMembershipDelete,
            TeamMembershipUpdate,
            TeamUnarchive,
            TeamUpdate,
            TemplateCreate,
            TemplateDelete,
            TemplateUpdate,
            TimeScheduleCreate,
            TimeScheduleDelete,
            TimeScheduleRefreshIntegrationSchedule,
            TimeScheduleUpdate,
            TimeScheduleUpsertExternal,
            TriageResponsibilityCreate,
            TriageResponsibilityDelete,
            TriageResponsibilityUpdate,
            UpdateIntegrationSlackScopes,
            UserChangeRole,
            UserDemoteAdmin,
            UserDemoteMember,
            UserDiscordConnect,
            UserExternalUserDisconnect,
            UserFlagUpdate,
            UserPromoteAdmin,
            UserPromoteMember,
            UserSettingsFlagsReset,
            UserSettingsUpdate,
            UserSuspend,
            UserUnlinkFromIdentityProvider,
            UserUnsuspend,
            UserUpdate,
            ViewPreferencesCreate,
            ViewPreferencesDelete,
            ViewPreferencesUpdate,
            WebhookCreate,
            WebhookDelete,
            WebhookUpdate,
            WorkflowStateArchive,
            WorkflowStateCreate,
            WorkflowStateUpdate,
        ]
    }

    /// Get the GraphQL operation name
    pub fn operation_name(&self) -> &'static str {
        match self {
            MutationOp::AgentActivityCreate => "agentActivityCreate",
            MutationOp::AgentActivityCreatePrompt => "agentActivityCreatePrompt",
            MutationOp::AgentSessionCreate => "agentSessionCreate",
            MutationOp::AgentSessionCreateOnComment => "agentSessionCreateOnComment",
            MutationOp::AgentSessionCreateOnIssue => "agentSessionCreateOnIssue",
            MutationOp::AgentSessionUpdate => "agentSessionUpdate",
            MutationOp::AgentSessionUpdateExternalUrl => "agentSessionUpdateExternalUrl",
            MutationOp::AirbyteIntegrationConnect => "airbyteIntegrationConnect",
            MutationOp::AsksWebFormsAuth => "asksWebFormsAuth",
            MutationOp::AttachmentCreate => "attachmentCreate",
            MutationOp::AttachmentDelete => "attachmentDelete",
            MutationOp::AttachmentLinkDiscord => "attachmentLinkDiscord",
            MutationOp::AttachmentLinkFront => "attachmentLinkFront",
            MutationOp::AttachmentLinkGitHubIssue => "attachmentLinkGitHubIssue",
            MutationOp::AttachmentLinkGitHubPR => "attachmentLinkGitHubPR",
            MutationOp::AttachmentLinkGitLabMR => "attachmentLinkGitLabMR",
            MutationOp::AttachmentLinkIntercom => "attachmentLinkIntercom",
            MutationOp::AttachmentLinkJiraIssue => "attachmentLinkJiraIssue",
            MutationOp::AttachmentLinkSalesforce => "attachmentLinkSalesforce",
            MutationOp::AttachmentLinkSlack => "attachmentLinkSlack",
            MutationOp::AttachmentLinkURL => "attachmentLinkURL",
            MutationOp::AttachmentLinkZendesk => "attachmentLinkZendesk",
            MutationOp::AttachmentSyncToSlack => "attachmentSyncToSlack",
            MutationOp::AttachmentUpdate => "attachmentUpdate",
            MutationOp::CommentCreate => "commentCreate",
            MutationOp::CommentDelete => "commentDelete",
            MutationOp::CommentResolve => "commentResolve",
            MutationOp::CommentUnresolve => "commentUnresolve",
            MutationOp::CommentUpdate => "commentUpdate",
            MutationOp::ContactCreate => "contactCreate",
            MutationOp::ContactSalesCreate => "contactSalesCreate",
            MutationOp::CreateCsvExportReport => "createCsvExportReport",
            MutationOp::CreateInitiativeUpdateReminder => "createInitiativeUpdateReminder",
            MutationOp::CreateOrganizationFromOnboarding => "createOrganizationFromOnboarding",
            MutationOp::CreateProjectUpdateReminder => "createProjectUpdateReminder",
            MutationOp::CustomViewCreate => "customViewCreate",
            MutationOp::CustomViewDelete => "customViewDelete",
            MutationOp::CustomViewUpdate => "customViewUpdate",
            MutationOp::CustomerCreate => "customerCreate",
            MutationOp::CustomerDelete => "customerDelete",
            MutationOp::CustomerMerge => "customerMerge",
            MutationOp::CustomerNeedArchive => "customerNeedArchive",
            MutationOp::CustomerNeedCreate => "customerNeedCreate",
            MutationOp::CustomerNeedCreateFromAttachment => "customerNeedCreateFromAttachment",
            MutationOp::CustomerNeedDelete => "customerNeedDelete",
            MutationOp::CustomerNeedUnarchive => "customerNeedUnarchive",
            MutationOp::CustomerNeedUpdate => "customerNeedUpdate",
            MutationOp::CustomerStatusCreate => "customerStatusCreate",
            MutationOp::CustomerStatusDelete => "customerStatusDelete",
            MutationOp::CustomerStatusUpdate => "customerStatusUpdate",
            MutationOp::CustomerTierCreate => "customerTierCreate",
            MutationOp::CustomerTierDelete => "customerTierDelete",
            MutationOp::CustomerTierUpdate => "customerTierUpdate",
            MutationOp::CustomerUnsync => "customerUnsync",
            MutationOp::CustomerUpdate => "customerUpdate",
            MutationOp::CustomerUpsert => "customerUpsert",
            MutationOp::CycleArchive => "cycleArchive",
            MutationOp::CycleCreate => "cycleCreate",
            MutationOp::CycleShiftAll => "cycleShiftAll",
            MutationOp::CycleStartUpcomingCycleToday => "cycleStartUpcomingCycleToday",
            MutationOp::CycleUpdate => "cycleUpdate",
            MutationOp::DocumentCreate => "documentCreate",
            MutationOp::DocumentDelete => "documentDelete",
            MutationOp::DocumentUnarchive => "documentUnarchive",
            MutationOp::DocumentUpdate => "documentUpdate",
            MutationOp::EmailIntakeAddressCreate => "emailIntakeAddressCreate",
            MutationOp::EmailIntakeAddressDelete => "emailIntakeAddressDelete",
            MutationOp::EmailIntakeAddressRotate => "emailIntakeAddressRotate",
            MutationOp::EmailIntakeAddressUpdate => "emailIntakeAddressUpdate",
            MutationOp::EmailTokenUserAccountAuth => "emailTokenUserAccountAuth",
            MutationOp::EmailUnsubscribe => "emailUnsubscribe",
            MutationOp::EmailUserAccountAuthChallenge => "emailUserAccountAuthChallenge",
            MutationOp::EmojiCreate => "emojiCreate",
            MutationOp::EmojiDelete => "emojiDelete",
            MutationOp::EntityExternalLinkCreate => "entityExternalLinkCreate",
            MutationOp::EntityExternalLinkDelete => "entityExternalLinkDelete",
            MutationOp::EntityExternalLinkUpdate => "entityExternalLinkUpdate",
            MutationOp::FavoriteCreate => "favoriteCreate",
            MutationOp::FavoriteDelete => "favoriteDelete",
            MutationOp::FavoriteUpdate => "favoriteUpdate",
            MutationOp::FileUpload => "fileUpload",
            MutationOp::FileUploadDangerouslyDelete => "fileUploadDangerouslyDelete",
            MutationOp::GitAutomationStateCreate => "gitAutomationStateCreate",
            MutationOp::GitAutomationStateDelete => "gitAutomationStateDelete",
            MutationOp::GitAutomationStateUpdate => "gitAutomationStateUpdate",
            MutationOp::GitAutomationTargetBranchCreate => "gitAutomationTargetBranchCreate",
            MutationOp::GitAutomationTargetBranchDelete => "gitAutomationTargetBranchDelete",
            MutationOp::GitAutomationTargetBranchUpdate => "gitAutomationTargetBranchUpdate",
            MutationOp::GoogleUserAccountAuth => "googleUserAccountAuth",
            MutationOp::ImageUploadFromUrl => "imageUploadFromUrl",
            MutationOp::ImportFileUpload => "importFileUpload",
            MutationOp::InitiativeArchive => "initiativeArchive",
            MutationOp::InitiativeCreate => "initiativeCreate",
            MutationOp::InitiativeDelete => "initiativeDelete",
            MutationOp::InitiativeRelationCreate => "initiativeRelationCreate",
            MutationOp::InitiativeRelationDelete => "initiativeRelationDelete",
            MutationOp::InitiativeRelationUpdate => "initiativeRelationUpdate",
            MutationOp::InitiativeToProjectCreate => "initiativeToProjectCreate",
            MutationOp::InitiativeToProjectDelete => "initiativeToProjectDelete",
            MutationOp::InitiativeToProjectUpdate => "initiativeToProjectUpdate",
            MutationOp::InitiativeUnarchive => "initiativeUnarchive",
            MutationOp::InitiativeUpdate => "initiativeUpdate",
            MutationOp::InitiativeUpdateArchive => "initiativeUpdateArchive",
            MutationOp::InitiativeUpdateCreate => "initiativeUpdateCreate",
            MutationOp::InitiativeUpdateUnarchive => "initiativeUpdateUnarchive",
            MutationOp::InitiativeUpdateUpdate => "initiativeUpdateUpdate",
            MutationOp::IntegrationArchive => "integrationArchive",
            MutationOp::IntegrationAsksConnectChannel => "integrationAsksConnectChannel",
            MutationOp::IntegrationCustomerDataAttributesRefresh => "integrationCustomerDataAttributesRefresh",
            MutationOp::IntegrationDelete => "integrationDelete",
            MutationOp::IntegrationDiscord => "integrationDiscord",
            MutationOp::IntegrationFigma => "integrationFigma",
            MutationOp::IntegrationFront => "integrationFront",
            MutationOp::IntegrationGitHubEnterpriseServerConnect => "integrationGitHubEnterpriseServerConnect",
            MutationOp::IntegrationGitHubPersonal => "integrationGitHubPersonal",
            MutationOp::IntegrationGithubCommitCreate => "integrationGithubCommitCreate",
            MutationOp::IntegrationGithubConnect => "integrationGithubConnect",
            MutationOp::IntegrationGithubImportConnect => "integrationGithubImportConnect",
            MutationOp::IntegrationGithubImportRefresh => "integrationGithubImportRefresh",
            MutationOp::IntegrationGitlabConnect => "integrationGitlabConnect",
            MutationOp::IntegrationGong => "integrationGong",
            MutationOp::IntegrationGoogleCalendarPersonalConnect => "integrationGoogleCalendarPersonalConnect",
            MutationOp::IntegrationGoogleSheets => "integrationGoogleSheets",
            MutationOp::IntegrationIntercom => "integrationIntercom",
            MutationOp::IntegrationIntercomDelete => "integrationIntercomDelete",
            MutationOp::IntegrationIntercomSettingsUpdate => "integrationIntercomSettingsUpdate",
            MutationOp::IntegrationJiraPersonal => "integrationJiraPersonal",
            MutationOp::IntegrationJiraUpdate => "integrationJiraUpdate",
            MutationOp::IntegrationLaunchDarklyConnect => "integrationLaunchDarklyConnect",
            MutationOp::IntegrationLaunchDarklyPersonalConnect => "integrationLaunchDarklyPersonalConnect",
            MutationOp::IntegrationLoom => "integrationLoom",
            MutationOp::IntegrationMcpServerConnect => "integrationMcpServerConnect",
            MutationOp::IntegrationMcpServerPersonalConnect => "integrationMcpServerPersonalConnect",
            MutationOp::IntegrationOpsgenieConnect => "integrationOpsgenieConnect",
            MutationOp::IntegrationOpsgenieRefreshScheduleMappings => "integrationOpsgenieRefreshScheduleMappings",
            MutationOp::IntegrationPagerDutyConnect => "integrationPagerDutyConnect",
            MutationOp::IntegrationPagerDutyRefreshScheduleMappings => "integrationPagerDutyRefreshScheduleMappings",
            MutationOp::IntegrationRequest => "integrationRequest",
            MutationOp::IntegrationSalesforce => "integrationSalesforce",
            MutationOp::IntegrationSalesforceMetadataRefresh => "integrationSalesforceMetadataRefresh",
            MutationOp::IntegrationSentryConnect => "integrationSentryConnect",
            MutationOp::IntegrationSettingsUpdate => "integrationSettingsUpdate",
            MutationOp::IntegrationSlack => "integrationSlack",
            MutationOp::IntegrationSlackAsks => "integrationSlackAsks",
            MutationOp::IntegrationSlackCustomViewNotifications => "integrationSlackCustomViewNotifications",
            MutationOp::IntegrationSlackCustomerChannelLink => "integrationSlackCustomerChannelLink",
            MutationOp::IntegrationSlackImportEmojis => "integrationSlackImportEmojis",
            MutationOp::IntegrationSlackInitiativePost => "integrationSlackInitiativePost",
            MutationOp::IntegrationSlackOrAsksUpdateSlackTeamName => "integrationSlackOrAsksUpdateSlackTeamName",
            MutationOp::IntegrationSlackOrgInitiativeUpdatesPost => "integrationSlackOrgInitiativeUpdatesPost",
            MutationOp::IntegrationSlackOrgProjectUpdatesPost => "integrationSlackOrgProjectUpdatesPost",
            MutationOp::IntegrationSlackPersonal => "integrationSlackPersonal",
            MutationOp::IntegrationSlackPost => "integrationSlackPost",
            MutationOp::IntegrationSlackProjectPost => "integrationSlackProjectPost",
            MutationOp::IntegrationSlackWorkflowAccessUpdate => "integrationSlackWorkflowAccessUpdate",
            MutationOp::IntegrationTemplateCreate => "integrationTemplateCreate",
            MutationOp::IntegrationTemplateDelete => "integrationTemplateDelete",
            MutationOp::IntegrationUpdate => "integrationUpdate",
            MutationOp::IntegrationZendesk => "integrationZendesk",
            MutationOp::IntegrationsSettingsCreate => "integrationsSettingsCreate",
            MutationOp::IntegrationsSettingsUpdate => "integrationsSettingsUpdate",
            MutationOp::IssueAddLabel => "issueAddLabel",
            MutationOp::IssueArchive => "issueArchive",
            MutationOp::IssueBatchCreate => "issueBatchCreate",
            MutationOp::IssueBatchUpdate => "issueBatchUpdate",
            MutationOp::IssueCreate => "issueCreate",
            MutationOp::IssueDelete => "issueDelete",
            MutationOp::IssueDescriptionUpdateFromFront => "issueDescriptionUpdateFromFront",
            MutationOp::IssueExternalSyncDisable => "issueExternalSyncDisable",
            MutationOp::IssueImportCreateAsana => "issueImportCreateAsana",
            MutationOp::IssueImportCreateCSVJira => "issueImportCreateCSVJira",
            MutationOp::IssueImportCreateClubhouse => "issueImportCreateClubhouse",
            MutationOp::IssueImportCreateGithub => "issueImportCreateGithub",
            MutationOp::IssueImportCreateJira => "issueImportCreateJira",
            MutationOp::IssueImportCreateLinearV2 => "issueImportCreateLinearV2",
            MutationOp::IssueImportDelete => "issueImportDelete",
            MutationOp::IssueImportProcess => "issueImportProcess",
            MutationOp::IssueImportUpdate => "issueImportUpdate",
            MutationOp::IssueLabelCreate => "issueLabelCreate",
            MutationOp::IssueLabelDelete => "issueLabelDelete",
            MutationOp::IssueLabelRestore => "issueLabelRestore",
            MutationOp::IssueLabelRetire => "issueLabelRetire",
            MutationOp::IssueLabelUpdate => "issueLabelUpdate",
            MutationOp::IssueRelationCreate => "issueRelationCreate",
            MutationOp::IssueRelationDelete => "issueRelationDelete",
            MutationOp::IssueRelationUpdate => "issueRelationUpdate",
            MutationOp::IssueReminder => "issueReminder",
            MutationOp::IssueRemoveLabel => "issueRemoveLabel",
            MutationOp::IssueSubscribe => "issueSubscribe",
            MutationOp::IssueToReleaseCreate => "issueToReleaseCreate",
            MutationOp::IssueToReleaseDelete => "issueToReleaseDelete",
            MutationOp::IssueToReleaseDeleteByIssueAndRelease => "issueToReleaseDeleteByIssueAndRelease",
            MutationOp::IssueUnarchive => "issueUnarchive",
            MutationOp::IssueUnsubscribe => "issueUnsubscribe",
            MutationOp::IssueUpdate => "issueUpdate",
            MutationOp::JiraIntegrationConnect => "jiraIntegrationConnect",
            MutationOp::JoinOrganizationFromOnboarding => "joinOrganizationFromOnboarding",
            MutationOp::LeaveOrganization => "leaveOrganization",
            MutationOp::Logout => "logout",
            MutationOp::LogoutAllSessions => "logoutAllSessions",
            MutationOp::LogoutOtherSessions => "logoutOtherSessions",
            MutationOp::LogoutSession => "logoutSession",
            MutationOp::NotificationArchive => "notificationArchive",
            MutationOp::NotificationArchiveAll => "notificationArchiveAll",
            MutationOp::NotificationCategoryChannelSubscriptionUpdate => "notificationCategoryChannelSubscriptionUpdate",
            MutationOp::NotificationMarkReadAll => "notificationMarkReadAll",
            MutationOp::NotificationMarkUnreadAll => "notificationMarkUnreadAll",
            MutationOp::NotificationSnoozeAll => "notificationSnoozeAll",
            MutationOp::NotificationSubscriptionCreate => "notificationSubscriptionCreate",
            MutationOp::NotificationSubscriptionDelete => "notificationSubscriptionDelete",
            MutationOp::NotificationSubscriptionUpdate => "notificationSubscriptionUpdate",
            MutationOp::NotificationUnarchive => "notificationUnarchive",
            MutationOp::NotificationUnsnoozeAll => "notificationUnsnoozeAll",
            MutationOp::NotificationUpdate => "notificationUpdate",
            MutationOp::OrganizationCancelDelete => "organizationCancelDelete",
            MutationOp::OrganizationDelete => "organizationDelete",
            MutationOp::OrganizationDeleteChallenge => "organizationDeleteChallenge",
            MutationOp::OrganizationDomainClaim => "organizationDomainClaim",
            MutationOp::OrganizationDomainCreate => "organizationDomainCreate",
            MutationOp::OrganizationDomainDelete => "organizationDomainDelete",
            MutationOp::OrganizationDomainUpdate => "organizationDomainUpdate",
            MutationOp::OrganizationDomainVerify => "organizationDomainVerify",
            MutationOp::OrganizationInviteCreate => "organizationInviteCreate",
            MutationOp::OrganizationInviteDelete => "organizationInviteDelete",
            MutationOp::OrganizationInviteUpdate => "organizationInviteUpdate",
            MutationOp::OrganizationStartTrial => "organizationStartTrial",
            MutationOp::OrganizationStartTrialForPlan => "organizationStartTrialForPlan",
            MutationOp::OrganizationUpdate => "organizationUpdate",
            MutationOp::PasskeyLoginFinish => "passkeyLoginFinish",
            MutationOp::PasskeyLoginStart => "passkeyLoginStart",
            MutationOp::ProjectAddLabel => "projectAddLabel",
            MutationOp::ProjectArchive => "projectArchive",
            MutationOp::ProjectCreate => "projectCreate",
            MutationOp::ProjectDelete => "projectDelete",
            MutationOp::ProjectExternalSyncDisable => "projectExternalSyncDisable",
            MutationOp::ProjectLabelCreate => "projectLabelCreate",
            MutationOp::ProjectLabelDelete => "projectLabelDelete",
            MutationOp::ProjectLabelRestore => "projectLabelRestore",
            MutationOp::ProjectLabelRetire => "projectLabelRetire",
            MutationOp::ProjectLabelUpdate => "projectLabelUpdate",
            MutationOp::ProjectMilestoneCreate => "projectMilestoneCreate",
            MutationOp::ProjectMilestoneDelete => "projectMilestoneDelete",
            MutationOp::ProjectMilestoneMove => "projectMilestoneMove",
            MutationOp::ProjectMilestoneUpdate => "projectMilestoneUpdate",
            MutationOp::ProjectReassignStatus => "projectReassignStatus",
            MutationOp::ProjectRelationCreate => "projectRelationCreate",
            MutationOp::ProjectRelationDelete => "projectRelationDelete",
            MutationOp::ProjectRelationUpdate => "projectRelationUpdate",
            MutationOp::ProjectRemoveLabel => "projectRemoveLabel",
            MutationOp::ProjectStatusArchive => "projectStatusArchive",
            MutationOp::ProjectStatusCreate => "projectStatusCreate",
            MutationOp::ProjectStatusUnarchive => "projectStatusUnarchive",
            MutationOp::ProjectStatusUpdate => "projectStatusUpdate",
            MutationOp::ProjectUnarchive => "projectUnarchive",
            MutationOp::ProjectUpdate => "projectUpdate",
            MutationOp::ProjectUpdateArchive => "projectUpdateArchive",
            MutationOp::ProjectUpdateCreate => "projectUpdateCreate",
            MutationOp::ProjectUpdateDelete => "projectUpdateDelete",
            MutationOp::ProjectUpdateUnarchive => "projectUpdateUnarchive",
            MutationOp::ProjectUpdateUpdate => "projectUpdateUpdate",
            MutationOp::PushSubscriptionCreate => "pushSubscriptionCreate",
            MutationOp::PushSubscriptionDelete => "pushSubscriptionDelete",
            MutationOp::ReactionCreate => "reactionCreate",
            MutationOp::ReactionDelete => "reactionDelete",
            MutationOp::RefreshGoogleSheetsData => "refreshGoogleSheetsData",
            MutationOp::ReleaseArchive => "releaseArchive",
            MutationOp::ReleaseCreate => "releaseCreate",
            MutationOp::ReleasePipelineArchive => "releasePipelineArchive",
            MutationOp::ReleasePipelineCreate => "releasePipelineCreate",
            MutationOp::ReleasePipelineDelete => "releasePipelineDelete",
            MutationOp::ReleasePipelineUnarchive => "releasePipelineUnarchive",
            MutationOp::ReleasePipelineUpdate => "releasePipelineUpdate",
            MutationOp::ReleaseStageArchive => "releaseStageArchive",
            MutationOp::ReleaseStageCreate => "releaseStageCreate",
            MutationOp::ReleaseStageUnarchive => "releaseStageUnarchive",
            MutationOp::ReleaseStageUpdate => "releaseStageUpdate",
            MutationOp::ReleaseUnarchive => "releaseUnarchive",
            MutationOp::ReleaseUpdate => "releaseUpdate",
            MutationOp::ResendOrganizationInvite => "resendOrganizationInvite",
            MutationOp::ResendOrganizationInviteByEmail => "resendOrganizationInviteByEmail",
            MutationOp::RoadmapArchive => "roadmapArchive",
            MutationOp::RoadmapCreate => "roadmapCreate",
            MutationOp::RoadmapDelete => "roadmapDelete",
            MutationOp::RoadmapToProjectCreate => "roadmapToProjectCreate",
            MutationOp::RoadmapToProjectDelete => "roadmapToProjectDelete",
            MutationOp::RoadmapToProjectUpdate => "roadmapToProjectUpdate",
            MutationOp::RoadmapUnarchive => "roadmapUnarchive",
            MutationOp::RoadmapUpdate => "roadmapUpdate",
            MutationOp::SamlTokenUserAccountAuth => "samlTokenUserAccountAuth",
            MutationOp::TeamCreate => "teamCreate",
            MutationOp::TeamCyclesDelete => "teamCyclesDelete",
            MutationOp::TeamDelete => "teamDelete",
            MutationOp::TeamKeyDelete => "teamKeyDelete",
            MutationOp::TeamMembershipCreate => "teamMembershipCreate",
            MutationOp::TeamMembershipDelete => "teamMembershipDelete",
            MutationOp::TeamMembershipUpdate => "teamMembershipUpdate",
            MutationOp::TeamUnarchive => "teamUnarchive",
            MutationOp::TeamUpdate => "teamUpdate",
            MutationOp::TemplateCreate => "templateCreate",
            MutationOp::TemplateDelete => "templateDelete",
            MutationOp::TemplateUpdate => "templateUpdate",
            MutationOp::TimeScheduleCreate => "timeScheduleCreate",
            MutationOp::TimeScheduleDelete => "timeScheduleDelete",
            MutationOp::TimeScheduleRefreshIntegrationSchedule => "timeScheduleRefreshIntegrationSchedule",
            MutationOp::TimeScheduleUpdate => "timeScheduleUpdate",
            MutationOp::TimeScheduleUpsertExternal => "timeScheduleUpsertExternal",
            MutationOp::TriageResponsibilityCreate => "triageResponsibilityCreate",
            MutationOp::TriageResponsibilityDelete => "triageResponsibilityDelete",
            MutationOp::TriageResponsibilityUpdate => "triageResponsibilityUpdate",
            MutationOp::UpdateIntegrationSlackScopes => "updateIntegrationSlackScopes",
            MutationOp::UserChangeRole => "userChangeRole",
            MutationOp::UserDemoteAdmin => "userDemoteAdmin",
            MutationOp::UserDemoteMember => "userDemoteMember",
            MutationOp::UserDiscordConnect => "userDiscordConnect",
            MutationOp::UserExternalUserDisconnect => "userExternalUserDisconnect",
            MutationOp::UserFlagUpdate => "userFlagUpdate",
            MutationOp::UserPromoteAdmin => "userPromoteAdmin",
            MutationOp::UserPromoteMember => "userPromoteMember",
            MutationOp::UserSettingsFlagsReset => "userSettingsFlagsReset",
            MutationOp::UserSettingsUpdate => "userSettingsUpdate",
            MutationOp::UserSuspend => "userSuspend",
            MutationOp::UserUnlinkFromIdentityProvider => "userUnlinkFromIdentityProvider",
            MutationOp::UserUnsuspend => "userUnsuspend",
            MutationOp::UserUpdate => "userUpdate",
            MutationOp::ViewPreferencesCreate => "viewPreferencesCreate",
            MutationOp::ViewPreferencesDelete => "viewPreferencesDelete",
            MutationOp::ViewPreferencesUpdate => "viewPreferencesUpdate",
            MutationOp::WebhookCreate => "webhookCreate",
            MutationOp::WebhookDelete => "webhookDelete",
            MutationOp::WebhookUpdate => "webhookUpdate",
            MutationOp::WorkflowStateArchive => "workflowStateArchive",
            MutationOp::WorkflowStateCreate => "workflowStateCreate",
            MutationOp::WorkflowStateUpdate => "workflowStateUpdate",
        }
    }
}
