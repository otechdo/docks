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

# Example config


```toml
# Docker user information (optional)
[docker]
username = "otechdo"
email = "otechdo@otechdo.com"

# Private registry information (if you are using one)
[registry]
# url = "your_registry_url"  # Uncomment and fill in if necessary
# username = "your_username"  # Uncomment and fill in if necessary
# password = "your_password"  # Uncomment and fill in if necessary

# SSH settings for remote deployment
[ssh]
port = 22
user = "root"

# Docker image tag configuration
[hub]
tags = [
    ["version", ["stable", "beta", "nightly", "latest"]],
    ["env", ["staging", "dev", "prod"]],
    ["schedule", ["hourly", "daily", "weekly", "monthly"]],
]

# List of Docker images to build
[[hub.images]]
name = "rlang"
tags = ["version", "schedule"]
path = "./rlang"  # Path relative to the configuration file directory

[[hub.images]]
name = "zuu"
tags = ["version", "schedule", "env"]
path = "./zuu"

[[hub.images]]
name = "teams"
tags = ["version", "schedule", "env"]
path = "./teams"

[[hub.images]]
name = "picsou"
tags = ["version", "schedule", "env"]
path = "./picsou"

# Deployment configuration
[deploy]
local = ["zuu:dev", "teams:dev", "picsou:dev"]

[deploy.remotes]
"lab.ji" = ["zuu:stable", "teams:stable", "picsou:stable"]
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

