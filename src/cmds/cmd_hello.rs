use crate::{error::CommandError, utils};

pub fn run(name: Option<String>) -> Result<(), CommandError> {
    let name = utils::normalize_name(name)?;
    println!("hello, {name}");
    Ok(())
}