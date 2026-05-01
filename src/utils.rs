use crate::error::CommandError;

pub fn normalize_name(name: Option<String>) -> Result<String, CommandError> {
    let raw_name = name.unwrap_or_else(|| "world".to_string());
    let normalized = raw_name.trim().to_string();

    if normalized.is_empty() {
        return Err(CommandError::EmptyName);
    }

    if !normalized
        .chars()
        .all(|character| character.is_alphabetic() || character.is_whitespace() || matches!(character, '-' | '\''))
    {
        return Err(CommandError::InvalidName {
            param: "name",
            value: raw_name,
        });
    }

    Ok(normalized)
}