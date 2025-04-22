// src/main.rs
// Rust CLI tool "gst": --skip-worktree management tig like TUI

use clap::{Parser, Subcommand};
use std::process::Command;
use std::{error::Error, io::{Write, stdout}};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::{backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::Modifier, text::Span, widgets::{Block, Borders, List, ListItem}, Terminal};

#[derive(Parser)]
#[command(name = "gst", about = "Manage skip-worktree flags with a TUI", version)]
struct Cli {
    /// CLI mode: skip/resume bulk operation only
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Set skip-worktree on files
    Skip { files: Vec<String> },
    /// Unset skip-worktree on files
    Resume { files: Vec<String> },
}

struct FileItem {
    path: String,
    skipped: bool,
}

fn list_skip_worktree() -> Result<Vec<FileItem>, Box<dyn Error>> {
    let output = Command::new("git")
        .args(["ls-files", "-v"])
        .output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let mut items = Vec::new();
    for line in text.lines() {
        if line.len() > 2 {
            let flag = line.chars().next().unwrap();
            let path = line[2..].to_string();
            let skipped = flag == 'S';
            items.push(FileItem { path, skipped });
        }
    }
    Ok(items)
}

fn toggle_skip(path: &str, skip: bool) {
    let flag = if skip { "--skip-worktree" } else { "--no-skip-worktree" };
    let _ = Command::new("git")
        .args(["update-index", flag, path])
        .status();
}

fn run_tui() -> Result<(), Box<dyn Error>> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut items = list_skip_worktree()?;
    let mut cursor = 0;

    let result = loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(3),
                    Constraint::Length(1),
                ])
                .split(size);

            let list_items: Vec<ListItem> = items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let marker = if item.skipped { "[x]" } else { "[ ]" };
                    let content = Span::raw(format!("{} {}", marker, item.path));
                    let mut li = ListItem::new(content);
                    if i == cursor {
                        li = li.style(ratatui::style::Style::default().add_modifier(Modifier::REVERSED));
                    }
                    li
                })
                .collect();

            let list = List::new(list_items)
                .block(Block::default().borders(Borders::ALL).title("skip-worktree Manager"));
            f.render_widget(list, chunks[0]);

            // Help message at bottom
            let help_text = "j/k: move  u: toggle skip-worktree  !: clear all  q: quit";
            let help = Span::raw(help_text);
            f.render_widget(
                Block::default()
                    .title(help)
                    .borders(Borders::BOTTOM),
                chunks[1],
            );
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => {
                        cursor = (cursor + 1).min(items.len().saturating_sub(1));
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        cursor = cursor.saturating_sub(1);
                    }
                    KeyCode::Char('u') => {
                        let skip = !items[cursor].skipped;
                        toggle_skip(&items[cursor].path, skip);
                        items[cursor].skipped = skip;
                    }
                    KeyCode::Char('!') => {
                        // Confirm before bulk clear
                        terminal.clear()?;
                        terminal.backend_mut().write_all(b"\nClear all skip-worktree flags? (y/n): ")?;
                        terminal.backend_mut().flush()?;

                        loop {
                            if let Event::Key(conf) = event::read()? {
                                match conf.code {
                                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                                        // Bulk clear skip-worktree
                                        let mut ls = Command::new("git")
                                            .args(["ls-files", "-z"])
                                            .stdout(std::process::Stdio::piped())
                                            .spawn()?;
                                        let _ = Command::new("git")
                                            .args(["update-index", "-z", "--no-skip-worktree", "--stdin"])
                                            .stdin(ls.stdout.take().unwrap())
                                            .status()?;
                                        for item in items.iter_mut() {
                                            item.skipped = false;
                                        }
                                        break;
                                    }
                                    KeyCode::Char('n') | KeyCode::Char('N') => {
                                        break;
                                    }
                                    _ => continue,
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    };

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        crossterm::cursor::Show
    )?;

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Skip { files }) => {
            for f in files {
                toggle_skip(f, true);
            }
        }
        Some(Commands::Resume { files }) => {
            for f in files {
                toggle_skip(f, false);
            }
        }
        None => run_tui()?,
    }
    Ok(())
}
