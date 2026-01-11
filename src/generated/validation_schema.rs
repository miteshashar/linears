//! Generated validation schema - DO NOT EDIT
//! Run `cargo xtask codegen` to regenerate

use super::Resource;

/// Get valid filter keys for a resource type.
/// Returns None if the resource doesn't support filtering.
pub fn get_valid_filter_keys(resource: Resource) -> Option<&'static [&'static str]> {
    match resource {
        Resource::AdministrableTeams => Some(&["and", "createdAt", "description", "id", "issues", "key", "name", "or", "parent", "private", "updatedAt"]),
        Resource::AgentActivities => Some(&["agentSessionId", "and", "createdAt", "id", "or", "sourceComment", "type", "updatedAt"]),
        Resource::Attachments => Some(&["and", "createdAt", "creator", "id", "or", "sourceType", "subtitle", "title", "updatedAt", "url"]),
        Resource::AuditEntries => Some(&["actor", "and", "countryCode", "createdAt", "id", "ip", "or", "type", "updatedAt"]),
        Resource::Comments => Some(&["and", "body", "createdAt", "documentContent", "id", "issue", "needs", "or", "parent", "projectUpdate", "reactions", "updatedAt", "user"]),
        Resource::CustomViews => Some(&["and", "createdAt", "creator", "hasFacet", "id", "modelName", "name", "or", "shared", "team", "updatedAt"]),
        Resource::CustomerNeeds => Some(&["and", "comment", "createdAt", "customer", "id", "issue", "or", "priority", "project", "updatedAt"]),
        Resource::Customers => Some(&["and", "createdAt", "domains", "externalIds", "id", "name", "needs", "or", "owner", "revenue", "size", "slackChannelId", "status", "tier", "updatedAt"]),
        Resource::Cycles => Some(&["and", "completedAt", "createdAt", "endsAt", "id", "inheritedFromId", "isActive", "isFuture", "isInCooldown", "isNext", "isPast", "isPrevious", "issues", "name", "number", "or", "startsAt", "team", "updatedAt"]),
        Resource::Documents => Some(&["and", "createdAt", "creator", "id", "initiative", "issue", "or", "project", "slugId", "title", "updatedAt"]),
        Resource::InitiativeUpdates => Some(&["and", "createdAt", "id", "initiative", "or", "reactions", "updatedAt", "user"]),
        Resource::Initiatives => Some(&["activityType", "ancestors", "and", "createdAt", "creator", "health", "healthWithAge", "id", "name", "or", "owner", "slugId", "status", "targetDate", "teams", "updatedAt"]),
        Resource::IssueLabels => Some(&["and", "createdAt", "creator", "id", "isGroup", "name", "or", "parent", "team", "updatedAt"]),
        Resource::IssueSearch => Some(&["accumulatedStateUpdatedAt", "addedToCycleAt", "addedToCyclePeriod", "ageTime", "and", "archivedAt", "assignee", "attachments", "autoArchivedAt", "autoClosedAt", "canceledAt", "children", "comments", "completedAt", "createdAt", "creator", "customerCount", "customerImportantCount", "cycle", "cycleTime", "delegate", "description", "dueDate", "estimate", "hasBlockedByRelations", "hasBlockingRelations", "hasDuplicateRelations", "hasRelatedRelations", "hasSuggestedAssignees", "hasSuggestedLabels", "hasSuggestedProjects", "hasSuggestedRelatedIssues", "hasSuggestedSimilarIssues", "hasSuggestedTeams", "id", "labels", "lastAppliedTemplate", "leadTime", "needs", "number", "or", "parent", "priority", "project", "projectMilestone", "reactions", "recurringIssueTemplate", "searchableContent", "slaStatus", "snoozedBy", "snoozedUntilAt", "sourceMetadata", "startedAt", "state", "subscribers", "suggestions", "team", "title", "triageTime", "triagedAt", "updatedAt"]),
        Resource::Issues => Some(&["accumulatedStateUpdatedAt", "addedToCycleAt", "addedToCyclePeriod", "ageTime", "and", "archivedAt", "assignee", "attachments", "autoArchivedAt", "autoClosedAt", "canceledAt", "children", "comments", "completedAt", "createdAt", "creator", "customerCount", "customerImportantCount", "cycle", "cycleTime", "delegate", "description", "dueDate", "estimate", "hasBlockedByRelations", "hasBlockingRelations", "hasDuplicateRelations", "hasRelatedRelations", "hasSuggestedAssignees", "hasSuggestedLabels", "hasSuggestedProjects", "hasSuggestedRelatedIssues", "hasSuggestedSimilarIssues", "hasSuggestedTeams", "id", "labels", "lastAppliedTemplate", "leadTime", "needs", "number", "or", "parent", "priority", "project", "projectMilestone", "reactions", "recurringIssueTemplate", "searchableContent", "slaStatus", "snoozedBy", "snoozedUntilAt", "sourceMetadata", "startedAt", "state", "subscribers", "suggestions", "team", "title", "triageTime", "triagedAt", "updatedAt"]),
        Resource::Notifications => Some(&["and", "archivedAt", "createdAt", "id", "or", "type", "updatedAt"]),
        Resource::ProjectLabels => Some(&["and", "createdAt", "creator", "id", "isGroup", "name", "or", "parent", "updatedAt"]),
        Resource::ProjectMilestones => Some(&["and", "createdAt", "id", "name", "or", "project", "targetDate", "updatedAt"]),
        Resource::ProjectUpdates => Some(&["and", "createdAt", "id", "or", "project", "reactions", "updatedAt", "user"]),
        Resource::Projects => Some(&["accessibleTeams", "activityType", "and", "canceledAt", "completedAt", "completedProjectMilestones", "createdAt", "creator", "customerCount", "customerImportantCount", "hasBlockedByRelations", "hasBlockingRelations", "hasDependedOnByRelations", "hasDependsOnRelations", "hasRelatedRelations", "hasViolatedRelations", "health", "healthWithAge", "id", "initiatives", "issues", "labels", "lastAppliedTemplate", "lead", "members", "name", "needs", "nextProjectMilestone", "or", "priority", "projectMilestones", "projectUpdates", "roadmaps", "searchableContent", "slugId", "startDate", "startedAt", "state", "status", "targetDate", "updatedAt"]),
        Resource::SearchIssues => Some(&["accumulatedStateUpdatedAt", "addedToCycleAt", "addedToCyclePeriod", "ageTime", "and", "archivedAt", "assignee", "attachments", "autoArchivedAt", "autoClosedAt", "canceledAt", "children", "comments", "completedAt", "createdAt", "creator", "customerCount", "customerImportantCount", "cycle", "cycleTime", "delegate", "description", "dueDate", "estimate", "hasBlockedByRelations", "hasBlockingRelations", "hasDuplicateRelations", "hasRelatedRelations", "hasSuggestedAssignees", "hasSuggestedLabels", "hasSuggestedProjects", "hasSuggestedRelatedIssues", "hasSuggestedSimilarIssues", "hasSuggestedTeams", "id", "labels", "lastAppliedTemplate", "leadTime", "needs", "number", "or", "parent", "priority", "project", "projectMilestone", "reactions", "recurringIssueTemplate", "searchableContent", "slaStatus", "snoozedBy", "snoozedUntilAt", "sourceMetadata", "startedAt", "state", "subscribers", "suggestions", "team", "title", "triageTime", "triagedAt", "updatedAt"]),
        Resource::Teams => Some(&["and", "createdAt", "description", "id", "issues", "key", "name", "or", "parent", "private", "updatedAt"]),
        Resource::Users => Some(&["active", "admin", "and", "app", "assignedIssues", "createdAt", "displayName", "email", "id", "invited", "isInvited", "isMe", "name", "or", "owner", "updatedAt"]),
        Resource::WorkflowStates => Some(&["and", "createdAt", "description", "id", "issues", "name", "or", "position", "team", "type", "updatedAt"]),
        _ => None,
    }
}

/// Validate filter keys and return unknown keys with suggestions.
/// Returns Ok(()) if all keys are valid, Err with suggestions otherwise.
pub fn validate_filter_keys(
    resource: Resource,
    filter: &serde_json::Value,
) -> Result<(), Vec<(String, Option<String>)>> {
    let valid_keys = match get_valid_filter_keys(resource) {
        Some(keys) => keys,
        None => return Ok(()), // No validation available
    };

    let mut errors: Vec<(String, Option<String>)> = Vec::new();

    if let Some(obj) = filter.as_object() {
        for key in obj.keys() {
            if !valid_keys.contains(&key.as_str()) {
                // Find closest match using simple similarity
                let suggestion = find_closest_match(key, valid_keys);
                errors.push((key.clone(), suggestion));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Find the closest matching key using simple Levenshtein-like heuristics
fn find_closest_match(input: &str, valid_keys: &[&str]) -> Option<String> {
    let input_lower = input.to_lowercase();

    // First try: exact case-insensitive match
    for key in valid_keys {
        if key.to_lowercase() == input_lower {
            return Some(key.to_string());
        }
    }

    // Second try: prefix match
    for key in valid_keys {
        if key.to_lowercase().starts_with(&input_lower) || input_lower.starts_with(&key.to_lowercase()) {
            return Some(key.to_string());
        }
    }

    // Third try: Levenshtein distance
    let mut best_match: Option<(&str, usize)> = None;
    for key in valid_keys {
        let distance = levenshtein_distance(&input_lower, &key.to_lowercase());
        // Only suggest if distance is reasonable (less than half the key length + 2)
        let threshold = (key.len() / 2).max(2) + 1;
        if distance <= threshold {
            match best_match {
                None => best_match = Some((key, distance)),
                Some((_, best_dist)) if distance < best_dist => best_match = Some((key, distance)),
                _ => {}
            }
        }
    }

    best_match.map(|(key, _)| key.to_string())
}

/// Compute Levenshtein edit distance between two strings
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();

    if m == 0 { return n; }
    if n == 0 { return m; }

    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr: Vec<usize> = vec![0; n + 1];

    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1)          // deletion
                .min(curr[j - 1] + 1)         // insertion
                .min(prev[j - 1] + cost);     // substitution
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    prev[n]
}
