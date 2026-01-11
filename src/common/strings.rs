//! String transformation utilities

/// Convert a string to PascalCase
/// Examples: "issue_create" -> "IssueCreate", "my-field" -> "MyField"
pub fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' || c == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

/// Convert a string to camelCase
/// Examples: "issue_create" -> "issueCreate", "MyField" -> "myField"
pub fn to_camel_case(s: &str) -> String {
    let pascal = to_pascal_case(s);
    let mut chars = pascal.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_lowercase().collect::<String>() + chars.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("issue_create"), "IssueCreate");
        assert_eq!(to_pascal_case("my-field"), "MyField");
        assert_eq!(to_pascal_case("already"), "Already");
        assert_eq!(to_pascal_case(""), "");
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("issue_create"), "issueCreate");
        assert_eq!(to_camel_case("MyField"), "myField");
        assert_eq!(to_camel_case("already"), "already");
        assert_eq!(to_camel_case(""), "");
    }
}
