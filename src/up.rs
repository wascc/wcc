use log::{info, LevelFilter};
use structopt::clap::AppSettings;
use structopt::StructOpt;
use text_io::read;
use tui_logger::*;

use std::{
    error::Error,
    io,
    sync::atomic::{AtomicBool, Ordering},
    sync::mpsc,
    sync::Arc,
    thread,
    time::Duration,
};
use termion::{
    event::Key, input::MouseTerminal, input::TermRead, raw::IntoRawMode, screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Terminal,
};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Debug, StructOpt, Clone)]
#[structopt(
    global_settings(&[AppSettings::ColoredHelp, AppSettings::VersionlessSubcommands]),
    name = "up")]
pub struct UpCli {
    #[structopt(flatten)]
    command: UpCommand,
}

#[derive(StructOpt, Debug, Clone)]
struct UpCommand {}

pub fn handle_command(cli: UpCli) -> Result<()> {
    match cli.command {
        UpCommand { .. } => handle_up(),
    }
}

#[derive(StructOpt, Debug, Clone)]
#[structopt(global_settings(&[AppSettings::NoBinaryName]))]
struct ReplCli {
    #[structopt(flatten)]
    cmd: ReplCliCommand,
}

#[derive(StructOpt, Debug, Clone)]
enum ReplCliCommand {
    #[structopt(name = "link")]
    Link(LinkCommand),

    #[structopt(name = "call")]
    Call(CallCommand),

    #[structopt(name = "start")]
    Start(StartCommand),

    #[structopt(name = "exit")]
    Exit,
}

#[derive(StructOpt, Debug, Clone)]
struct LinkCommand {
    url: String,
    capid: String,
    env: Vec<String>,
}

#[derive(StructOpt, Debug, Clone)]
struct CallCommand {
    url: String,
    op: String,
    payload: Vec<String>,
}

#[derive(StructOpt, Debug, Clone)]
struct StartCommand {
    url: String,
}

/// Launches REPL environment
fn handle_up() -> Result<()> {
    // Early initialization of the logger

    // Set max_log_level to Trace
    // init_logger(LevelFilter::Trace).unwrap();

    // Set default level for unknown targets to Trace
    // set_default_level(LevelFilter::Trace);

    // Terminal initialization
    // let stdout = io::stdout().into_raw_mode()?;
    // let stdout = MouseTerminal::from(stdout);
    // let stdout = AlternateScreen::from(stdout);
    // let backend = TermionBackend::new(stdout);
    // let mut terminal = Terminal::new(backend)?;

    // let exit_key = Key::Char('q');

    // Setup event handlers
    // let (tx, rx) = mpsc::channel();
    // let ignore_exit_key = Arc::new(AtomicBool::new(false));
    // let input_handle = {
    //     let tx = tx.clone();
    //     let ignore_exit_key = ignore_exit_key.clone();
    //     thread::spawn(move || {
    //         let stdin = io::stdin();
    //         for evt in stdin.keys() {
    //             if let Ok(key) = evt {
    //                 if let Err(err) = tx.send(key) {
    //                     eprintln!("{}", err);
    //                     return;
    //                 }
    //                 if !ignore_exit_key.load(Ordering::Relaxed) && key == exit_key {
    //                     return;
    //                 }
    //             }
    //         }
    //     })
    // };

    // Event loop
    // loop {
    // terminal.draw(|f| {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    // let size = f.size();
    // let block = Block::default()
    //     .borders(Borders::ALL)
    //     .title("waSCC REPL")
    //     .border_type(BorderType::Rounded);
    // f.render_widget(block, size);
    // let chunks = Layout::default()
    //     .direction(Direction::Vertical)
    //     .margin(1)
    //     .constraints([Constraint::Percentage(100), Constraint::Percentage(50)].as_ref())
    //     .split(f.size());

    // let top_chunks = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
    //     .split(chunks[0]);
    // let block = Block::default().title(vec![]);
    // f.render_widget(block, top_chunks[0]);

    // let block = Block::default().borders(Borders::LEFT).title(Span::styled(
    //     "Logs",
    //     Style::default().add_modifier(Modifier::BOLD),
    // ));
    // f.render_widget(block, top_chunks[1]);

    // let bottom_chunks = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
    //     .split(chunks[1]);
    // let block = Block::default().title("With borders").borders(Borders::ALL);
    // f.render_widget(block, bottom_chunks[0]);
    // let block = Block::default()
    //     .title("With styled borders and doubled borders")
    //     .border_style(Style::default().fg(Color::Cyan))
    //     .borders(Borders::LEFT | Borders::RIGHT)
    //     .border_type(BorderType::Double);
    // f.render_widget(block, bottom_chunks[1]);
    // })?;

    // if let key = rx.recv()? {
    //     if key == Key::Char('q') {
    //         break;
    //     }
    //     info!("I GOT A KEY {:?}", key);
    // }
    // }

    loop {
        print!("wash> ");
        use std::io::Write;
        io::stdout().flush()?;
        let input: String = read!("{}\n");
        // This works but doesn't correctly interpret strings, like "{"x": 10}" will make two tokens: '{"x":' and '10}'
        // For now, leaving this as a Vec of strings and `join`ing the vec by a space character will work.
        let iter = input.split_ascii_whitespace();
        let cli = ReplCli::from_iter(iter);

        if let ReplCliCommand::Exit = cli.cmd {
            println!("Have a good day!");
            break;
        }
        println!("Command parsed! {:?}\n", cli.cmd);
    }
    Ok(())
}
