#[cfg(test)]

#[test]
fn test_commands() {
    // dummy data

    use super::command::CommandResult;
    use crate::commands;

    use commands::cd::cd_command;
    use commands::clear::clear_command;
    use commands::clock::clock_command;
    use commands::cp::cp_command;
    use commands::exec::exec_command;
    use commands::head::head_command;
    use commands::help::help_command;
    use commands::ls::ls_command;
    use commands::mkdir::mkdir_command;
    use commands::mv::mv_command;
    use commands::neofetch::neofetch_command;
    use commands::pwd::pwd_command;
    use commands::read::read_command;
    use commands::rm::rm_command;
    use commands::rmdir::rmdir_command;
    use commands::tail::tail_command;
    use commands::touch::touch_command;
    use commands::wc::wc_command;

    use crossterm::style::Stylize;

    let mut env_args: Vec<&str> = Vec::new();
    let test_directory: &str = "tests";
    let mut i : u8 = 0;
    let commands: [&str; 19] = ["cd", "clear", "clock", "cp", "exec", "exit", "head", "help", "ls", "mkdir", "mv", "neofetch", "pwd", "read", "rm", "rmdir", "tail", "touch", "wc"];
    let mut result : CommandResult;
    env_args.push("");
    env_args.push(test_directory);
    
    // delete test directory
    println!("ℹ Test {}/{} - {}", i, commands.len(), "delete test directory".blue());
    result = rmdir_command(env_args.clone());
    assert_eq!(result, CommandResult::Success);

    // mkdir
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "mkdir".blue());
    result = mkdir_command(env_args.clone());
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "mkdir".green());

    // cd
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "cd".blue());
    result = cd_command(env_args.clone());
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "cd".green());

    // clear
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "clear".blue());
    result = clear_command();
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "clear".green());

    // clock
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "clock".blue());
    result = clock_command();
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "clock".green());

    // touch
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "touch".blue());
    result = touch_command(vec!["", "test.txt"]);
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "touch".green());

    // write (skipped, interactive)
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "write".blue());
    println!("✔ Test {}/{} - {} skipped", i, commands.len(), "write".yellow());
    
    // read
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "read".blue());
    result = read_command(vec!["", "test.txt"]);
    assert_eq!(result, CommandResult::Success);

    // cp
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "cp".blue());
    result = cp_command(vec!["", "test.txt", "test2.txt"]);
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "cp".green());
    
    // rm
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "rm".blue());
    result = rm_command(vec!["", "test.txt"]);
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "rm".green());

    // mv
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "mv".blue());
    result = mv_command(vec!["", "test2.txt", "test3.txt"]);
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "mv".green());

    // wc
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "wc".blue());
    result = wc_command(vec!["", "test3.txt"]);
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "wc".green());

    // head
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "head".blue());
    result = head_command(vec!["", "test3.txt"]);
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "head".green());

    // tail
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "tail".blue());
    result = tail_command(vec!["", "test3.txt"]);
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "tail".green());

    // ls
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "ls".blue());
    result = ls_command(vec!["-lah"]);
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "ls".green());

    // pwd
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "pwd".blue());
    result = pwd_command();
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "pwd".green());

    // neofetch
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "neofetch".blue());
    result = neofetch_command();
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "neofetch".green());

    // edit (skipped, interactive)
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "edit".blue());
    println!("✔ Test {}/{} - {} skipped", i, commands.len(), "edit".yellow());

    // help
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "help".blue());
    result = help_command();
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "help".green());

    // exec
    i += 1;
    println!("ℹ Test {}/{} - {}", i, commands.len(), "exec".blue());
    result = exec_command(vec!["", "echo", "Hello, World!"]);
    assert_eq!(result, CommandResult::Success);
    println!("✔ Test {}/{} - {} passed", i, commands.len(), "exec".green());

    // display info about the test
    println!("\n📝 Test info:");
    println!("Passed: {}/{}", i, commands.len());
    println!("Skipped: 2/{}", commands.len());
    println!("Failed: 0/{}", commands.len());
    println!("Total: {}/{}", i, commands.len());
    let success_rate = (i as f32 / commands.len() as f32) * 100.0;
    println!("Success rate: {:.2}%", success_rate);
    
    
}
