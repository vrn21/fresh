use std::{
    env, fs,
    io::{self, Write},
    path::Path,
    process::{Child, Command, Stdio},
};

use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!(
        r#"
      ___           ___           ___           ___           ___     
     /\  \         /\  \         /\  \         /\  \         /\__\    
    /::\  \       /::\  \       /::\  \       /::\  \       /:/  /    
   /:/\:\  \     /:/\:\  \     /:/\:\  \     /:/\ \  \     /:/__/     
  /::\~\:\  \   /::\~\:\  \   /::\~\:\  \   _\:\~\ \  \   /::\  \ ___ 
 /:/\:\ \:\__\ /:/\:\ \:\__\ /:/\:\ \:\__\ /\ \:\ \ \__\ /:/\:\  /\__\
 \/__\:\ \/__/ \/_|::\/:/  / \:\~\:\ \/__/ \:\ \:\ \/__/ \/__\:\:/  /
      \:\__\      |:|::/  /   \:\ \:\__\    \:\ \:\__\        \::/  / 
       \/__/      |:|\/__/     \:\ \/__/     \:\/:/  /        /:/  /  
                  |:|  |        \:\__\        \::/  /        /:/  /   
                   \|__|         \/__/         \/__/         \/__/    
 "#,
    );
    println!(
        " \n
        Welcome to freSH, FRiendly Easy SHell! Type 'exit' to quit.\n"
    );

    let mut rl = DefaultEditor::new()?;
    let history_path = "/tmp/.fresh_history";

    match rl.load_history(history_path) {
        Ok(_) => {}
        Err(ReadlineError::Io(_)) => {
            // History file doesn't exist, create it
            fs::File::create(history_path)?;
        }
        Err(err) => {
            eprintln!("fresh: Error loading history: {}", err);
        }
    }

    loop {
        print!("> ");
        let line = rl.readline("> ");

        match line {
            Ok(line) => {
                let input = line.trim();

                if input.is_empty() {
                    continue;
                }

                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let input = input.trim();

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
                        "exit" => return Ok(()),
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
                // Handle Ctrl-C or Ctrl-D gracefully
                println!("\nExiting fresh...");
                rl.save_history(history_path)?;
                break;
            }
            Err(e) => {
                eprintln!("fresh: Error: {:?}", e);
            }
        }
    }

    Ok(())
}
