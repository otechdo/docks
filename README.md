# Docks

> A docker compose uploading tool.

## Requirements

- docker
- docker-compose
- docker-buildx
- rsync
- ssh
- nmap
- ranger

## Installation

```bash
cargo install docks
```

## Uninstall

```bash
cargo uninstall docks
```

## Usage

```bash
docks
```

# Structure

```text
services # The root directory contenting all services
└── picsou  # A service (can contains folders and files) 
    └── compose.yaml # The service main file

2 directories, 1 file
```

## Compose.yaml

```yaml
services:
  picsou:
    image: otechdo/picsou:latest
    restart: always
    ports:
      - "3000:3000"
```

## The config

```toml
[local]
containers = ["adminer", "teams"]

[lab]
username = "root"
ip = "lab.ji"
port = "22"
containers = ["gitlab"]
```

## Min structure

```text
services
├── adminer
│   └── compose.yaml
├── gitlab
│   └── compose.yaml
└── nexcloud
    └── compose.yaml

4 directories, 3 files
```

