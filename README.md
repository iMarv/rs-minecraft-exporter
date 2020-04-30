# rs-minecraft-exporter

[![Crates.io](https://img.shields.io/crates/v/rs-minecraft-exporter)](https://crates.io/crates/rs-minecraft-exporter)

This is a shameless rust-plug of [joshi425/minecraft-exporter](https://github.com/Joshi425/minecraft-exporter) for minecraft 1.15.2+.

## Usage

### Docker

```
docker run -p 8000:8000 \
           -v /opt/server/world:/world
           imarv/rs_minecraft_exporter
```

### Binary

Either clone the repository and build the project yourself with cargo or install it through crates.io with

```
cargo install rs-minecraft-exporter
```

then simply run


```
rs-minecraft-exporter /opt/server/world
```

### Changing IP to listen to

If you only want to expose metrics inside a private network or similar, you can change the ip the webserver is listening on.
This can be done by setting the environment variable `HOST_IP` to the desired ip.

If not set, the server will default to `0.0.0.0`.

### Log Level

You can adjust the log level by appending any of the following strings as an argument to either the docker command or the binary.

- error
- warn
- info
- debug
- trace

docker example
```
docker run -p 8000:8000 \
           -v /opt/server/world:/world
           imarv/rs_minecraft_exporter debug
```

binary example
```
rs-minecraft-exporter /opt/server/world debug
```

## Metrics

```
# HELP mc_broken collected stats for category `broken`
# TYPE mc_broken counter

# HELP mc_crafted collected stats for category `crafted`
# TYPE mc_crafted counter

# HELP mc_custom collected stats for category `custom`
# TYPE mc_custom counter

# HELP mc_dropped collected stats for category `dropped`
# TYPE mc_dropped counter

# HELP mc_food_level current player food level
# TYPE mc_food_level gauge

# HELP mc_health current player health
# TYPE mc_health gauge

# HELP mc_killed collected stats for category `killed`
# TYPE mc_killed counter

# HELP mc_killed_by collected stats for category `killed_by`
# TYPE mc_killed_by counter

# HELP mc_mined collected stats for category `mined`
# TYPE mc_mined counter

# HELP mc_picked_up collected stats for category `picked_up`
# TYPE mc_picked_up counter

# HELP mc_score current player score
# TYPE mc_score gauge

# HELP mc_used collected stats for category `used`
# TYPE mc_used counter

# HELP mc_xp_level current player level
# TYPE mc_xp_level gauge

# HELP mc_xp_total total collceted xp
# TYPE mc_xp_total gauge

# HELP process_cpu_seconds_total Total user and system CPU time spent in seconds.
# TYPE process_cpu_seconds_total counter

# HELP process_max_fds Maximum number of open file descriptors.
# TYPE process_max_fds gauge

# HELP process_open_fds Number of open file descriptors.
# TYPE process_open_fds gauge

# HELP process_resident_memory_bytes Resident memory size in bytes.
# TYPE process_resident_memory_bytes gauge

# HELP process_start_time_seconds Start time of the process since unix epoch in seconds.
# TYPE process_start_time_seconds gauge

# HELP process_virtual_memory_bytes Virtual memory size in bytes.
# TYPE process_virtual_memory_bytes gauge
```
