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
- eza
- vim
- screen

## Environment

### Fish

```shell
set -x DOCKS_WORKING_DIR "$HOME/Docks"
set -x DOCKS_PUBLIC_DIR "$HOME/Docks/Public"
```

### Bash

```bash
export DOCKS_WORKING_DIR="$HOME/Docks"
export DOCKS_PUBLIC_DIR="$HOME/Docks/Public"

```

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

