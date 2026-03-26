//! AI-OS CLI - Terminal interface

mod commands;

use aios_ai::{AIMiddleware, AIResponse};
use aios_memory::MemoryStore;
use anyhow::Result;
use rustyline::Editor;
use std::sync::Arc;
use std::io::Write;
use tracing::error;

/// Run the CLI REPL
pub async fn run(ai: AIMiddleware, memory: MemoryStore) -> Result<()> {
    let mut rl: Editor<(), rustyline::history::FileHistory> = Editor::new().expect("Failed to create editor");

    // Load history if available
    let history_path = ".aios/history.txt";
    if std::path::Path::new(history_path).exists() {
        let _ = rl.load_history(history_path);
    }

    println!("╔═══════════════════════════════════════╗");
    println!("║           AI-OS Terminal               ║");
    println!("║   Your AI-Native Operating System     ║");
    println!("╚═══════════════════════════════════════╝");
    println!();
    println!("Type your message and press Enter.");
    println!("Press Ctrl+C or Ctrl+D to exit.");
    println!();

    let ai = Arc::new(ai);
    let _memory = Arc::new(memory);

    loop {
        let prompt = "❯ ";
        let readline: Result<String, rustyline::error::ReadlineError> = rl.readline(prompt);

        match readline {
            Ok(line) => {
                let input: &str = line.trim();
                if input.is_empty() {
                    continue;
                }

                // Add to history
                rl.add_history_entry(input).ok();

                // Save history
                let _ = rl.save_history(history_path);

                // Process command
                if input == "/exit" || input == "/quit" {
                    println!("Goodbye!");
                    break;
                }

                if input == "/clear" {
                    print!("\x1B[2J\x1B[1H");
                    std::io::stdout().flush().ok();
                    continue;
                }

                if let Some(response) = handle_input(&ai, input).await {
                    print_response(&response);
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("\nUse /exit to quit.");
                continue;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("\nGoodbye!");
                break;
            }
            Err(e) => {
                error!("Readline error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

/// Handle user input and get AI response
async fn handle_input(
    ai: &AIMiddleware,
    input: &str,
) -> Option<AIResponse> {
    print!("Thinking... ");
    std::io::stdout().flush().ok();

    // Get response from AI
    let response = match ai.complete(input).await {
        Ok(r) => r,
        Err(e) => {
            error!("AI error: {}", e);
            return None;
        }
    };

    Some(response)
}

/// Print AI response with formatting
fn print_response(response: &AIResponse) {
    println!();
    println!("{}", response.content);
    println!();

    // Print usage stats in debug mode
    if tracing::enabled!(tracing::Level::DEBUG) {
        println!(
            "[Debug: {} tokens used - {} in, {} out]",
            response.usage.input_tokens + response.usage.output_tokens,
            response.usage.input_tokens,
            response.usage.output_tokens
        );
    }
}
