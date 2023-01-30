# Start Cluster

```shell
docker-compose up -d
```

# Redis UI (RedisInsight)

- Go to localhost:8001
- Add Redis Database in the UI

  - Host: localhost
  - Port: 6370
  - Name: test-cluster

- Check in Cluster Management that Cluster Health is fine

# Run Rust App

```shell
cargo test
```
