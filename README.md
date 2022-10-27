# filloabot-rs
A multipurpose (including music) Discord bot. This is a full rewrite using Rust of the original [FilloaBot](https://github.com/FilloaBot/filloabot/). Some features that were considered useless haven't been ported.
So, right now it simply plays music and answers to messages with appropriate memes, the latter is meant to be configurable by the guild administrators using a web but I'm still working on it.

## 🔧 Deploy it

### 🐳 Docker (Recommended)

As commented before this needs access to an API to fetch the message memes (a.k.a. references) and thus it needs a dumb server serving them until I finish the final one.
It's all included in the `docker-compose.yml` file:

```bash
mkdir filloabot-rs
cd filloabot-rs

wget https://github.com/FilloaBot/filloabot-rs/raw/master/docker-compose.yml
wget https://github.com/FilloaBot/dumb-api/raw/main/references.json

vim references.json # Edit as you wish
docker-compose up -d
```

#### Environment Variables

| Name                  | Description                                                                                                                       |
|-----------------------|-----------------------------------------------------------------------------------------------------------------------------------|
| `DISCORD_TOKEN`       | Token used to connect to Discord's services.                                                                                      |
| `FILLOABOT_API_URL`   | Base URL to the api to fetch the references.                                                                                      |
| `FILLOABOT_API_TOKEN` | Token to the API. Not useful until the actual web-ui is finished.                                                                 |
| `RUST_LOG`            | Log level. Defaults to "error". Check [tracing's docs](https://docs.rs/tracing/latest/tracing/struct.Level.html#implementations). |

#### Build the image

```bash
git clone https://github.com/FilloaBot/filloabot-rs.git
cd filloabot-rs
docker build -t filloabot-rs .
```

### 💪🏻 Without Docker

You probably knew how to do this already.
	
```bash
git clone https://github.com/FilloaBot/filloabot-rs.git
cd filloabot-rs
vim .env # Edit with the env vars specified before
cargo run
```
