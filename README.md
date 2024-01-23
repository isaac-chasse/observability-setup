# observability-setup

Set-up for system monitoring and observability via [supervisor](http://supervisord.org/), [loki](https://github.com/grafana/loki), [promtail](https://grafana.com/docs/loki/latest/send-data/promtail/installation/), [grafana](https://github.com/grafana/grafana), etc.

Some notes:
- `emissions/` is where the code, which auto-generates a random signal lives. Logs are also produced here.
- Log output is batched and saved to a `timescaledb` instance.
- `grafana` is spun up to connect to & query the `timescaledb` instance. 
- processes can be stopped/started using `supervisor`.

Todos:
- [ ] Loki implemenation
- [ ] Improve docs on set up
- [x] Remove legacy python code
