use clap::{Parser, Subcommand};
use icy_engine::{Buffer, SaveOptions, TextPane};
use icy_engine_egui::animations::Animator;
use std::{fs, path::PathBuf, time::Duration};

use crate::com::Com;

mod com;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Terminal {
    IcyTerm,
    SyncTerm,
    Unknown,
    Name(String),
}

impl Terminal {
    pub fn use_dcs(&self) -> bool {
        match self {
            Terminal::IcyTerm | Terminal::SyncTerm => true,
            _ => false,
        }
    }
}

#[derive(Parser)]
pub struct Cli {
    #[arg(
        help = "If true modern terminal output (UTF8) is used.",
        long,
        default_value_t = false
    )]
    utf8: bool,

    #[arg(help = "File to play/show.", required = true)]
    path: Option<PathBuf>,

    #[arg(help = "Socket port address for i/o", long)]
    port: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Plays the animation (default)")]
    Play,

    #[command(about = "Show a specific frame of the animation")]
    ShowFrame { frame: usize },
}

fn main() {
    let args = Cli::parse();

    let mut io: Box<dyn Com> = if let Some(port) = args.port {
        println!("connect to : {}", "127.0.0.1:".to_string() + port.as_str());
        Box::new(com::SocketCom::connect("127.0.0.1:".to_string() + port.as_str()).unwrap())
    } else {
        Box::new(com::StdioCom::start().unwrap())
    };

    io.write(b"\x1B[0c").unwrap();
    if let Some(path) = args.path {
        let parent = Some(path.parent().unwrap().to_path_buf());

        let Some(ext) = path.extension() else {
            println!("Error: File extension not found.");
            return;
        };
        let ext = ext.to_string_lossy().to_ascii_lowercase();
        let mut term = Terminal::Unknown;
        match io.read() {
            Ok(Some(data)) => {
                let txt = String::from_utf8_lossy(&data).to_string();
                term = if txt.contains("73;99;121;84;101;114;109") {
                    Terminal::IcyTerm
                } else if txt.contains("67;84;101;114") {
                    Terminal::SyncTerm
                } else {
                    Terminal::Name(txt)
                }
            } // 67;84;101;114;109;1;316
            Ok(_) => {}
            Err(_) => {
                eprintln!("Connection aborted.");
                return;
            }
        }

        match ext.as_str() {
            "icyanim" => match fs::read_to_string(path) {
                Ok(txt) => {
                    let animator = Animator::run(&parent, &txt);

                    if let Ok(animator) = animator {
                        let animator = animator.lock().unwrap();
                        let mut opt: SaveOptions = SaveOptions::default();
                        if args.utf8 {
                            opt.modern_terminal_output = true;
                        }
                        match args.command.unwrap_or(Commands::Play) {
                            Commands::Play => {
                                for (buffer, _, delay) in animator.frames.iter() {
                                    show_buffer(&mut io, buffer, false, args.utf8, &term).unwrap();
                                    std::thread::sleep(Duration::from_millis(*delay as u64));
                                }
                            }
                            Commands::ShowFrame { frame } => {
                                show_buffer(
                                    &mut io,
                                    &animator.frames[frame].0,
                                    true,
                                    args.utf8,
                                    &term,
                                )
                                .unwrap();
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Error opening file: {e}");
                }
            },
            _ => {
                let buffer = Buffer::load_buffer(&path, true);
                if let Ok(buffer) = &buffer {
                    show_buffer(&mut io, buffer, true, args.utf8, &Terminal::Unknown).unwrap();
                }
            }
        }
    }
}

fn show_buffer(
    io: &mut Box<dyn Com>,
    buffer: &Buffer,
    single_frame: bool,
    use_utf8: bool,
    terminal: &Terminal,
) -> anyhow::Result<()> {
    let mut opt: SaveOptions = SaveOptions::default();
    if use_utf8 {
        opt.modern_terminal_output = true;
    }
    opt.control_char_handling = icy_engine::ControlCharHandling::FilterOut;
    opt.longer_terminal_output = true;
    opt.compress = false;
    let bytes = buffer.to_bytes("ans", &opt)?;
    if !single_frame && terminal.use_dcs() {
        io.write(b"\x1BP0;1;0!z")?;
    }
    io.write(b"\x1b[0m")?;
    io.write(&bytes)?;
    /*
    for i in 0..buffer.get_height() {
        io.write(format!("\x1b[{};1H{}:", i + 1, i).as_bytes())?;
    }*/

    //io.write(format!("\x1b[0;0HTerminal:{:?}", terminal).as_bytes())?;
    if !single_frame && terminal.use_dcs() {
        io.write(b"\x1b\\\x1b[0*z")?;
    }
    io.write(b"\n\r")?;

    Ok(())
}