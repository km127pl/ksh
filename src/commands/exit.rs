use std::collections::HashMap;

use crate::modules::aliases::save_aliases;
use crate::modules::command::CommandResult;

pub fn exit_command(aliases: &HashMap<String, String>) -> CommandResult {
    save_aliases(aliases);
    std::process::exit(0);
}
