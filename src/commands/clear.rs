use crate::modules::command::CommandResult;

pub fn clear_command() -> CommandResult {
    clearscreen::clear().expect("failed to clear screen");
    CommandResult::Success
}
