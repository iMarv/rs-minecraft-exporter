use hyper::{
    header::CONTENT_TYPE,
    service::{make_service_fn, service_fn},
    Body, Response, Server,
};
use player::gather_players;
use prometheus::{gather, Encoder, TextEncoder};
use prometheus_handler::track_for_player;
use std::env;
use std::{
    error,
    net::{IpAddr, SocketAddr},
    path::{Path, PathBuf},
    str::FromStr,
    time::Duration,
};
use tokio::time;

#[macro_use]
extern crate log;
#[macro_use]
extern crate prometheus;
#[macro_use]
extern crate lazy_static;
extern crate hyper;
extern crate simple_logger;

mod player;
mod prometheus_handler;
mod stats;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let (path, log_level) = handle_args(args)?;
    simple_logger::init_with_level(log_level)?;

    let ip = env::var("HOST_IP").unwrap_or(String::from("0.0.0.0"));
    let addr = (ip.parse::<IpAddr>()?, 8000).into();

    tokio::spawn(async move {
        loop {
            trace!("Scraping player Metrics ...");
            if let Err(e) = gather_metrics(&path).await {
                error!("Scraping error: {}", e);
            }

            time::delay_for(Duration::from_secs(5)).await;
        }
    });

    run_server(addr).await
}

async fn run_server(addr: SocketAddr) -> Result<()> {
    info!("Listening on http://{}", addr);

    let make_svc = make_service_fn(move |_| async move {
        Ok::<_, hyper::Error>(service_fn(move |_req| async move { serve_req().await }))
    });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(err) = server.await {
        error!("server error: {}", err);
    }

    Ok(())
}

fn handle_args(args: Vec<String>) -> Result<(PathBuf, log::Level)> {
    match args.len() {
        0 => Err("No arguments given")?,
        1 => {
            let path = check_path(args.get(0).unwrap().clone())?;
            Ok((path, log::Level::Info))
        }
        2 | _ => {
            let path = check_path(args.get(0).unwrap().clone())?;
            let level = args.get(1).unwrap();
            let level = log::Level::from_str(level).map_err(|_| "Could not parse log level")?;

            Ok((path, level))
        }
    }
}

fn check_path(path: String) -> Result<PathBuf> {
    let path = PathBuf::from(path);

    if path.is_dir() {
        Ok(path)
    } else {
        Err("Given path is not a directory")?
    }
}

async fn serve_req() -> std::result::Result<Response<Body>, hyper::http::Error> {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    Response::builder()
        .status(200)
        .header(CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
}

async fn gather_metrics(path: &Path) -> Result<()> {
    let players = gather_players(path).await?;

    for player in &players {
        track_for_player(&player).await?;
    }

    Ok(())
}
