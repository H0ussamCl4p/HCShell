use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
};


const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const CYAN: &str = "\x1b[36m";

fn main() -> Result<(), Box<dyn Error>> {
    let mut rl = DefaultEditor::new()?;

    // Cross-platform history path: prefer $HOME, then %USERPROFILE% on Windows, else temp dir
    let history_path: PathBuf = if let Some(home) = env::var_os("HOME") {
        let mut p = PathBuf::from(home);
        p.push(".hcshell_history");
        p
    } else if cfg!(windows) {
        if let Some(up) = env::var_os("USERPROFILE") {
            let mut p = PathBuf::from(up);
            p.push(".hcshell_history");
            p
        } else {
            let mut p = env::temp_dir();
            p.push(".hcshell_history");
            p
        }
    } else {
        let mut p = PathBuf::from("/tmp");
        p.push(".hcshell_history");
        p
    };


    println!(
        "{}{}{}{}\n",
        BOLD, CYAN,
        r#"    ╦ ╦╔═╗┌─┐┬ ┬┌─┐┬  ┬  
    ╠═╣║  └─┐├─┤├┤ │  │  
    ╩ ╩╚═╝└─┘┴ ┴└─┘┴─┘┴─┘"#,
        RESET
    );

    println!(
        "{} Welcome to {}HCShell{} — type {}'exit'{} to quit - Made with ❤️ By {}{}{}{}.\n",
        GREEN, BOLD, RESET, BLUE, RESET, BOLD, CYAN, "Houssam Choubik", RESET
    );

    match rl.load_history(&history_path) {
        Ok(_) => {}
        Err(ReadlineError::Io(_)) => {
            fs::File::create(&history_path)?;
        }
        Err(err) => {
            eprintln!("HCShell: Error loading history: {}", err);
        }
    }

    loop {
        let prompt = build_prompt();
        let line = rl.readline(&prompt);

        match line {
            Ok(line) => {
                let input = line.trim();

                if input.is_empty() {
                    continue;
                }

                rl.add_history_entry(input)?;

                let mut commands = input.trim().split(" | ").peekable();
                let mut prev_stdout = None;
                let mut children: Vec<Child> = Vec::new();

                while let Some(command) = commands.next() {
                    let mut parts = command.split_whitespace();
                    let Some(command) = parts.next() else {
                        continue;
                    };
                    let args = parts;

                    match command {
                        "cd" => {
                            let new_dir = args.peekable().peek().map_or("/", |x| *x);
                            let root = Path::new(new_dir);
                            if let Err(e) = env::set_current_dir(root) {
                                eprintln!("{}", e);
                            }

                            prev_stdout = None;
                        }
                        "exit" => {
                            rl.save_history(&history_path)?;
                            return Ok(());
                        }
                        command => {
                            let stdin = match prev_stdout.take() {
                                Some(output) => Stdio::from(output),
                                None => Stdio::inherit(),
                            };

                            let stdout = if commands.peek().is_some() {
                                Stdio::piped()
                            } else {
                                Stdio::inherit()
                            };

                            let child = Command::new(command)
                                .args(args)
                                .stdin(stdin)
                                .stdout(stdout)
                                .spawn();

                            match child {
                                Ok(mut child) => {
                                    prev_stdout = child.stdout.take();
                                    children.push(child);
                                }
                                Err(e) => {
                                    eprintln!("{}", e);
                                    break;
                                }
                            };
                        }
                    }
                }

                for mut child in children {
                    let _ = child.wait();
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("\nExiting HCShell...");
                break;
            }
            Err(e) => {
                eprintln!("HCShell: Error: {:?}", e);
            }
        }
    }

    rl.save_history(&history_path)?;
    Ok(())
}

fn build_prompt() -> String {

    let user = env::var("USER").or_else(|_| env::var("USERNAME")).unwrap_or_else(|_| "user".into());

    let cwd = env::current_dir().map(|p| p.display().to_string()).unwrap_or_else(|_| "?".into());
    let mut short = cwd.clone();
    if let Ok(home) = env::var("HOME") {
        if cwd.starts_with(&home) {
            short = cwd.replacen(&home, "~", 1);
        }
    }

    let comps: Vec<&str> = short.split('/').filter(|s| !s.is_empty()).collect();
    let display = if comps.len() > 3 {
        let last = comps[comps.len().saturating_sub(3)..].join("/");
        format!(".../{last}")
    } else {
        short
    };

    format!("{}{}{}{}{} {}{}{} ",
        BOLD, CYAN, user, RESET, "$",
        BLUE, display, RESET
    )
}