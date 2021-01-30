Random Bru
----------

# Quick Start

```bash
# Start conduit
docker-compose up -d

# Create random_bru user
curl -XPOST -d '{"username":"random_bru_bot", "password":"wordpass", "auth": {"type":"m.login.dummy"}}' "http://localhost:8448/_matrix/client/r0/register"

# Start service
cargo run http://localhost:8448 random_bru_bot wordpass
```

Connect with a client (I use `Spectral` for local testing) and join the channel called `Random Bru`.
