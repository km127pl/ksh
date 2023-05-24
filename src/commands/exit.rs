use std::collections::HashMap;

use crate::modules::aliases::save_aliases;

pub fn exit_command(aliases: &HashMap<String, String>) {
    println!("Exiting...");
    save_aliases(aliases);
    std::process::exit(0);
}
