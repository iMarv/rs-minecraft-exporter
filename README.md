# rs-minecraft-exporter

This is a shameless rust-plug of [joshi425/minecraft-exporter](https://github.com/Joshi425/minecraft-exporter) for minecraft 1.15.2+.

## Usage

### Docker

```
docker run -p 8000:8000 \
           -v /opt/server/world:/world
           imarv/rs_minecraft_exporter
```

### Binary

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
# TYPE mc_food_level counter

# HELP mc_health current player health
# TYPE mc_health counter

# HELP mc_killed collected stats for category `killed`
# TYPE mc_killed counter

# HELP mc_picked_up collected stats for category `picked_up`
# TYPE mc_picked_up counter

# HELP mc_score current player score
# TYPE mc_score counter

# HELP mc_used collected stats for category `used`
# TYPE mc_used counter

# HELP mc_xp_level current player level
# TYPE mc_xp_level counter

# HELP mc_xp_total total collected xp
# TYPE mc_xp_total counter
```
