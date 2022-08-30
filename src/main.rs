mod config;
mod verbose;
mod views;
use actix_web::{web, App, HttpResponse, HttpServer};
use anyhow::Result;
use clap::Parser;
use config::Config;
use std::env;
use std::path::PathBuf;
use verbose::Verbose;

const DEFAULT_CONFIG_FILE: &str = "orehome.toml";

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long, value_parser, value_name = "FILE")]
    file: Option<PathBuf>,

    #[clap(short, long, action=clap::ArgAction::Count)]
    verbose: u8,
}

impl Cli {
    pub fn verbose(&self) -> bool {
        self.verbose > 0
    }
}

fn get_config_path(cli: &Cli) -> Result<PathBuf> {
    // 1. from environment variables
    // 2. from arguments
    // 3. location of executable
    // 4. TODO: from xdg config path
    let path = if let Ok(file) = env::var("RCON_CONFIG") {
        PathBuf::from(file)
    } else if let Some(file) = cli.file.as_deref() {
        file.to_path_buf()
    } else {
        let mut bin_path = env::current_exe()?;
        bin_path.pop();
        bin_path.push(DEFAULT_CONFIG_FILE);
        bin_path
    };

    let absolute_path = if path.is_absolute() {
        path
    } else {
        env::current_dir()?.join(path)
    };

    Ok(absolute_path)
}

fn get_config(path: PathBuf) -> Result<Config> {
    println!("Loading {}", path.display());
    let config = match Config::from_file(path) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{}\nDefault config will be used.", e);
            Config::default()
        }
    };
    Ok(config)
}

#[actix_web::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config_path = get_config_path(&cli)?;
    let config = get_config(config_path)?;
    let verbose = cli.verbose();
    if verbose {
        println!("{}", config);
    }

    HttpServer::new(move || {
        let app = App::new().default_service(web::route().to(HttpResponse::NotFound));
        let app = app.app_data(web::Data::new(Verbose::new(verbose)));
        views::register(app)
    })
    .workers(1)
    .bind(config.addr())?
    .run()
    .await?;
    Ok(())
}
