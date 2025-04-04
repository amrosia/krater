// NOT production-ready, this crate is under development

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod scan;
use crate::scan::Scan;

#[tokio::main]
async fn main() {
    let mut rl = DefaultEditor::new().unwrap_or_else(|e| {
        eprintln!("Error initializing readline: {}", e);
        std::process::exit(1);
    });
    
    // Create interrupt flag
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    // Set up CTRL+C handler
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("\nInterrupted current command");
    }).expect("Error setting Ctrl-C handler");

    let version = env!("CARGO_PKG_VERSION");
    println!("Krater v{}", version);
    println!("Github: https://github.com/amrosia/krater");
    println!("---");
    loop {
        // Reset interrupt flag
        running.store(true, Ordering::SeqCst);
        
        match rl.readline("krater > ") {
            Ok(input) => {
                if !input.trim().is_empty() {
                    if let Err(e) = rl.add_history_entry(&input) {
                        eprintln!("Error adding to history: {}", e);
                    }
                }
                let input = input.trim().to_string();
                
                let mut args = input.split_whitespace();
                if let Some(command) = args.next() {
                    match command {
                        "scan" => {
                            let running = running.clone();
                            let handle = tokio::spawn(async move {
                                Scan::run(&input, running).await
                            });

                            match handle.await {
                                Ok(_) => println!("Scan executed successfully"),
                                Err(e) => eprintln!("Scan was interrupted: {}", e),
                            }
                        }
                        "help" => {
                            dbg!("Help command received");
                        }
                        "exit" | "quit" => {
                            println!("Exiting...");
                            break;
                        }
                        _ => {
                            if !running.load(Ordering::SeqCst) {
                                continue;
                            }
                            if let Err(e) = std::process::Command::new("sh")
                                .arg("-c")
                                .arg(&input)
                                .status()
                            {
                                eprintln!("Failed to execute command: {}", e);
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("\nUse 'exit' or 'quit' to exit");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("\nUse 'exit' or 'quit' to exit");
                continue;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        }
    }
}