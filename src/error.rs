use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error(transparent)]
    Command(#[from] CommandError),
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("name cannot be empty")]
    EmptyName,

    #[error("parameter `{param}` has invalid value `{value}`")]
    InvalidName { param: &'static str, value: String },
}