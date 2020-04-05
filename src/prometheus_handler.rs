use crate::player_stats::{Player, StatCategory};
use log::warn;
use prometheus::{
    default_registry, labels, opts, register_counter, Counter, Encoder, Registry, TextEncoder,
};

pub fn track_metric(player: &Player, stat: StatCategory) {
    let stat_str = remove_prefix(&stat.to_string());
    let name = format!("mc_{}", stat_str);
    let help = format!("collected stats for category `{}`", stat_str);

    if let Some(stats) = player.stats.get_stat(stat) {
        for (key, value) in stats.iter() {
            let key = remove_prefix(key);
            let value = value.as_f64().expect("Property value not a number");

            let labels = labels!(
                "player" => &player.name,
                "type" => &key
            );

            let opts = opts!(&name, &help, labels);

            let counter = register_counter!(opts).unwrap();
            counter.inc_by(value);
        }
    } else {
        warn!(
            "Missing category `{}` for player `{}`",
            stat_str, player.name
        );
    }
}

fn remove_prefix(property: &String) -> String {
    property[10..].to_string()
}

pub fn print_metrics() {
    // Gather the metrics.
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = default_registry().gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    // Output to the standard output.
    println!("{}", String::from_utf8(buffer).unwrap());
}
