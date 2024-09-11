# Docks

> A docker compose uploading tool.

## Requirements

- docker
- docker-compose
- docker-buildx
- rsync
- ssh
- nmap

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
# docks.toml

[lab] # The server id
username = "root" # The server connection username
ip = "lab.ji" # The server ip address
services = ["picsou"] # All services to deploy on the server  
```

## A config example

```toml
# docks.toml

[homelab] # The server id
username = "root" # The server connection username
ip = "home.lan" # The server ip address
services = ["nextcloud", "adminer"] # All services to deploy on the server

[lab] # The server id
username = "git" # The server connection username
ip = "git.otechdo.org" # The server ip address
services = ["gitlab"] # All services to deploy on the server  
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
