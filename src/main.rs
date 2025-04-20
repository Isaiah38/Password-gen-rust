use clap::Parser;
use rand::Rng;
use clipboard::{ClipboardContext, ClipboardProvider};
use dialoguer::{Input, Confirm};

/// Simple password generator
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Use interactive mode
    #[arg(short, long)]
    interactive: bool,

    /// Length of the password
    #[arg(short, long, default_value_t = 12)]
    length: usize,

    /// Include numbers
    #[arg(long)]
    numbers: bool,

    /// Include symbols
    #[arg(long)]
    symbols: bool,

    /// Include lowercase letters
    #[arg(long, default_value_t = true)]
    lowercase: bool,

    /// Include uppercase letters
    #[arg(long, default_value_t = true)]
    uppercase: bool,

    /// Copy the password to clipboard
    #[arg(long)]
    copy: bool,
}

fn main() {
    let mut args = Args::parse();

    if args.interactive {
        println!("🔧 Interactive Password Generator");

        args.length = Input::new()
            .with_prompt("Password length")
            .default(12)
            .interact_text()
            .unwrap();

        args.numbers = Confirm::new()
            .with_prompt("Include numbers?")
            .default(true)
            .interact()
            .unwrap();

        args.symbols = Confirm::new()
            .with_prompt("Include symbols?")
            .default(true)
            .interact()
            .unwrap();

        args.lowercase = Confirm::new()
            .with_prompt("Include lowercase letters?")
            .default(true)
            .interact()
            .unwrap();

        args.uppercase = Confirm::new()
            .with_prompt("Include uppercase letters?")
            .default(true)
            .interact()
            .unwrap();

        args.copy = Confirm::new()
            .with_prompt("Copy password to clipboard?")
            .default(false)
            .interact()
            .unwrap();
    }

    let mut charset = String::new();
    if args.lowercase {
        charset += "abcdefghijklmnopqrstuvwxyz";
    }
    if args.uppercase {
        charset += "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    }
    if args.numbers {
        charset += "0123456789";
    }
    if args.symbols {
        charset += "!@#$%^&*()_+-=[]{}|;:,.<>?";
    }

    if charset.is_empty() {
        eprintln!("⚠️ No character sets selected! Use flags or interactive mode.");
        std::process::exit(1);
    }

    let mut rng = rand::thread_rng();
    let password: String = (0..args.length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();

    println!("\n🔐 Your generated password:\n{}\n", password);

    if args.copy {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(password.clone()).unwrap();
        println!("📋 Password copied to clipboard!");
    }
}
