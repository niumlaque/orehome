mod config;
mod handlers;

use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer};
use anyhow::Result;
use clap::Parser;
use config::Config;
use std::env;
use std::path::PathBuf;
use tera::Tera;

const DEFAULT_CONFIG_FILE: &str = "orehome.toml";

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long, value_parser, value_name = "FILE")]
    file: Option<PathBuf>,
}

fn get_config_path(cli: &Cli) -> Result<PathBuf> {
    // 1. from environment variables
    // 2. from arguments
    // 3. location of executable
    // 4. TODO: from xdg config path
    let path = if let Ok(file) = env::var("OREHOME_CONFIG") {
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

pub fn get_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    if let Some(hash) = option_env!("GIT_HASH") {
        format!("{}({})", version, hash)
    } else {
        version.into()
    }
}

fn get_config(path: PathBuf) -> Result<Config> {
    println!("Loading {}", path.display());
    let config = match Config::from_file(path) {
        Ok(config) => config,
        Err(_) => {
            eprintln!("Config file not found. Default config will be used.");
            Default::default()
        }
    };
    Ok(config)
}

#[actix_web::main]
async fn main() -> Result<()> {
    let name = env!("CARGO_PKG_NAME");
    let version = get_version();
    println!("{}: {}", name, version);

    let cli = Cli::parse();
    let config_path = get_config_path(&cli)?;
    let config = get_config(config_path)?;
    let log_level = config.level()?;

    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(false)
        .with_thread_ids(true)
        .compact();
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(log_level)
        .event_format(format)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("logging failed");
    tracing::info!("{}: {}", name, version);
    tracing::info!("{}", config);

    if let Err(e) = exec(config).await {
        tracing::error!("{:?}", e);
        std::process::exit(1);
    }

    Ok(())
}

async fn exec(config: Config) -> Result<()> {
    let templates = Tera::new("templates/**/*")?;
    HttpServer::new(move || {
        use handlers::*;
        App::new()
            .default_service(web::route().to(HttpResponse::NotFound))
            .service(Files::new("/static", "./assets").show_files_listing())
            .app_data(web::Data::new(templates.clone()))
            .route("/", web::get().to(search))
    })
    .workers(1)
    .bind(config.addr())?
    .run()
    .await?;
    Ok(())
}
