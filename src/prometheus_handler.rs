use crate::{player::Player, stats::StatCategory};
use prometheus::{labels, opts, Counter, Encoder, Registry, TextEncoder};

pub fn track_for_player(player: &Player, registry: &Registry) {
    for metric in [
        StatCategory::Broken,
        StatCategory::Crafted,
        StatCategory::Custom,
        StatCategory::Dropped,
        StatCategory::Killed,
        StatCategory::KilledBy,
        StatCategory::Mined,
        StatCategory::PickedUp,
        StatCategory::Used,
    ]
    .iter()
    {
        track_metric(player, *metric, registry);
    }
}

fn track_metric(player: &Player, stat: StatCategory, registry: &Registry) {
    let stat_str = remove_prefix(&stat.to_string());
    let name = format!("mc_{}", stat_str);
    let help = format!("collected stats for category `{}`", stat_str);

    if let Some(stats) = player.stats.get_stat(stat) {
        for (key, value) in stats.iter() {
            let value = value.as_f64().expect("Property value not a number");

            let labels = labels!(
                "player" => &player.name,
                "type" => &key
            );
            let counter = Counter::with_opts(opts!(&name, &help, labels)).unwrap();
            counter.inc_by(value);

            registry.register(Box::new(counter.clone())).unwrap();
        }
    } else {
        info!(
            "Missing category `{}` for player `{}`",
            stat_str, player.name
        );
    }
}

fn remove_prefix(property: &String) -> String {
    property[10..].to_string()
}

pub fn print_metrics(registry: Registry) {
    // Gather the metrics.
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    // Output to the standard output.
    println!("{}", String::from_utf8(buffer).unwrap());
}
