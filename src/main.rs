use hyper::{
    header::CONTENT_TYPE,
    service::{make_service_fn, service_fn},
    Body, Response, Server,
};
use player::gather_players;
use prometheus::{Encoder, Registry, TextEncoder};
use prometheus_handler::track_for_player;
use std::env;
use std::{
    error,
    path::{Path, PathBuf},
    str::FromStr,
};

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

    let addr = ([127, 0, 0, 1], 9898).into();
    info!("Listening on http://{}", addr);

    let make_svc = make_service_fn(move |_| {
        let path = path.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |_req| {
                let path = path.clone();
                async move { serve_req(path.as_path()).await }
            }))
        }
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
            let path = check_path(args.get(0).unwrap().clone());
            Ok((path, log::Level::Info))
        }
        2 | _ => {
            let path = check_path(args.get(0).unwrap().clone());
            let level = log::Level::from_str(args.get(1).unwrap());

            Ok((path, level.unwrap_or(log::Level::Trace)))
        }
    }
}

fn check_path(path: String) -> PathBuf {
    let path = PathBuf::from(path);

    if path.is_dir() {
        path
    } else {
        panic!("Given path is not a directory");
    }
}

async fn serve_req(path: &Path) -> std::result::Result<Response<Body>, hyper::Error> {
    let registry = Registry::new();

    let response = match gather_metrics(&registry, path).await {
        Ok(_) => {
            // Gather the metrics.
            let mut buffer = vec![];
            let encoder = TextEncoder::new();
            let metric_families = registry.gather();
            encoder.encode(&metric_families, &mut buffer).unwrap();

            Response::builder()
                .status(200)
                .header(CONTENT_TYPE, encoder.format_type())
                .body(Body::from(buffer))
                .unwrap()
        }
        Err(e) => {
            error!("{}", e);

            Response::builder()
                .status(500)
                .header(CONTENT_TYPE, "text/html")
                .body(Body::from(format!("{}", e)))
                .unwrap()
        }
    };

    Ok(response)
}

async fn gather_metrics(registry: &Registry, path: &Path) -> Result<()> {
    let players = gather_players(path).await?;
    // Each run needs its own registry as we just map the stats rather
    // than actually upping any counters

    for player in &players {
        track_for_player(&player, registry);
    }

    Ok(())
}
