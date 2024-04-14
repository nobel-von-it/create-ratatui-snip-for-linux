use std::{
    io::{self, Write},
    process::Command,
};
const FILE: &str = r#"
use std::io::stdout;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let mut t = Terminal::new(CrosstermBackend::new(stdout()))?;

    let res = run(&mut t);

    disable_raw_mode()?;
    execute!(t.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    t.show_cursor()?;

    res?;
    Ok(())
}
fn run<B: Backend>(t: &mut Terminal<B>) -> anyhow::Result<()> {
    loop {
        t.draw(|f| ui(f))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            match key.code {
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }
    Ok(())
}
fn ui(f: &mut Frame) {}
"#;

fn main() {
    println!("Enter project name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("read line error");
    let crate_name = name.trim();

    let dir = "/home/nerd/dev/rusty/";

    let output = Command::new("cargo")
        .args(["new", "--bin", crate_name])
        .current_dir(dir)
        .output()
        .expect("crate create error");

    if output.status.success() {
        println!("crate created");
    }

    let crate_dir = format!("{dir}{crate_name}/");
    let addlib = Command::new("cargo")
        .args(["add", "crossterm", "ratatui", "anyhow"])
        .current_dir(crate_dir.clone())
        .output()
        .expect("add crates error");

    if addlib.status.success() {
        println!("add crates success");
    }

    let build = Command::new("cargo")
        .args(["build"])
        .current_dir(crate_dir.clone())
        .output()
        .expect("build error");

    if build.status.success() {
        println!("build success");
    }

    write_file(&crate_dir);
    println!("file is rewrote");
}
fn write_file(crate_path: &str) {
    let full_path_to_file = format!("{crate_path}src/main.rs");

    println!("{}", &full_path_to_file);
    let mut path = std::fs::File::options()
        .write(true)
        .open(full_path_to_file)
        .expect("main file open error");

    let _ = writeln!(path, "{FILE}");
}
