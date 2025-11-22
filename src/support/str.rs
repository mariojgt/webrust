use rand::{distributions::Alphanumeric, Rng};
use heck::{ToSnakeCase, ToKebabCase, ToLowerCamelCase, ToPascalCase, ToTitleCase};

pub struct Str;

#[allow(dead_code)]
impl Str {
    /// Generate a random string of the given length.
    pub fn random(length: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }

    /// Generate a URL friendly "slug" from the given string.
    pub fn slug(text: &str) -> String {
        text.to_kebab_case()
    }

    /// Convert the given string to title case.
    pub fn title(text: &str) -> String {
        text.to_title_case()
    }

    /// Convert the given string to snake case.
    pub fn snake(text: &str) -> String {
        text.to_snake_case()
    }

    /// Convert the given string to camel case.
    pub fn camel(text: &str) -> String {
        text.to_lower_camel_case()
    }

    /// Convert the given string to kebab case.
    pub fn kebab(text: &str) -> String {
        text.to_kebab_case()
    }

    /// Convert the given string to studly case (PascalCase).
    pub fn studly(text: &str) -> String {
        text.to_pascal_case()
    }

    /// Limit the number of characters in a string.
    pub fn limit(text: &str, limit: usize, end: Option<&str>) -> String {
        let end = end.unwrap_or("...");
        if text.chars().count() <= limit {
            return text.to_string();
        }
        let truncated: String = text.chars().take(limit).collect();
        format!("{}{}", truncated, end)
    }

    /// Convert the given string to uppercase.
    pub fn upper(text: &str) -> String {
        text.to_uppercase()
    }

    /// Convert the given string to lowercase.
    pub fn lower(text: &str) -> String {
        text.to_lowercase()
    }

    /// Determine if a given string contains a given substring.
    pub fn contains(haystack: &str, needles: &[&str]) -> bool {
        needles.iter().any(|needle| haystack.contains(needle))
    }

    /// Determine if a given string starts with a given substring.
    pub fn starts_with(haystack: &str, needles: &[&str]) -> bool {
        needles.iter().any(|needle| haystack.starts_with(needle))
    }

    /// Determine if a given string ends with a given substring.
    pub fn ends_with(haystack: &str, needles: &[&str]) -> bool {
        needles.iter().any(|needle| haystack.ends_with(needle))
    }
}
