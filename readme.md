Random Bru
----------

# Quick Start

```bash
# Start conduit
docker-compose up -d

# Create random_bru user
curl -XPOST -d '{"username":"random_bru_bot", "password":"wordpass", "auth": {"type":"m.login.dummy"}}' "http://localhost:8448/_matrix/client/r0/register"

# Start service
cargo run
```

Connect with a client (I use `Spectral` for local testing) and join the channel called `Random Bru`.

# Build Docker Image

```
docker build -t random_bru_bot .
```

Currently uses ubuntu as a base image but I want to migrate it to alpine at somepoint.
