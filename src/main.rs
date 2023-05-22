use clap::{Args, Parser, Subcommand};
use cli::commands::describe::call_describe;
use cli::commands::list::call_list;
use cli::commands::run::call_run;
use cli::utilities::args;
use dotenv::dotenv;
use std::env;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long, default_value = "playbooks/")]
    playbook_dir: PathBuf,
    #[arg(short, long, default_value = "inventory.yaml")]
    inventory: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "list", about = "This will list all the available playbooks")]
    List(ListArgs),
    #[command(name = "run", about = "This will run a specified playbook")]
    Run(PlaybookArgs),
    #[command(name = "describe", about = "This will describe a specified playbook")]
    Describe(PlaybookArgs),
}

#[derive(Args)]
struct ListArgs {}

#[derive(Args)]
struct PlaybookArgs {
    books: Vec<String>,
}

fn main() {
    dotenv().ok();

    let mut cli = Cli::parse();
    if let Ok(playbook_dir) = env::var("PLAYBOOK_DIR") {
        cli.playbook_dir = PathBuf::from(playbook_dir);
    }
    if let Ok(inventory_dir) = env::var("INVENTORY_DIR") {
        cli.inventory = PathBuf::from(inventory_dir);
    }
    if let Ok(verbose) = env::var("VERBOSE") {
        cli.verbose = verbose.parse::<bool>().unwrap_or(false);
    }

    match &cli.command {
        Commands::List(_) => {
            let mapped_files = args::map_files(&cli.playbook_dir).expect("Failed to map files");
            print!(
                "{}",
                call_list(&cli.verbose, &mapped_files, &cli.playbook_dir).expect("Failed to list")
            )
        }
        Commands::Run(args) => {
            let mapped_files =
                args::arg_parse_env(&args.books, &cli.playbook_dir).expect("Failed to map files");
            let mut appended_mapped_files = mapped_files.clone();
            for (i, book_map) in mapped_files.iter() {
                let book = book_map.0.clone();
                let mut envs_map = book_map.1.clone();
                if let Ok(envs) = env::var(book.clone()) {
                    for env in envs.split(",") {
                        let env_var: Vec<&str> = env.split("=").collect();
                        if env_var.len() == 2 {
                            envs_map.insert(env_var[0].to_string(), env_var[1].to_string());
                        } else {
                            panic!("Misformatted env var: {}", env);
                        }
                    }
                }

                appended_mapped_files.insert(*i, (book, envs_map));
            }
            print!(
                "{}",
                call_run(
                    &appended_mapped_files,
                    &cli.verbose,
                    &cli.playbook_dir,
                    &cli.inventory
                )
                .expect("Failed to run")
            )
        }
        Commands::Describe(args) => {
            let mapped_files =
                args::arg_parse(&args.books, &cli.playbook_dir).expect("Failed to map files");
            print!(
                "{}",
                call_describe(&mapped_files, &cli.verbose, &cli.playbook_dir)
                    .expect("Failed to describe")
            )
        }
    }
}
