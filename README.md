# Tux

A docker compose uploading tool.

```text
services
└── picsou
    └── compose.yaml

2 directories, 1 file
```

## The config

```toml
# tux.toml

[lab] # The server id
username = "root" # The server connection username
ip = "lab.ji" # The server ip address
services = ["picsou"] # All services to deploy on the server  
```

## Login

```bash
tux login # Login to docker hub
```

## Logout

```bash
tux logout # Logout to docker hub
```

## Running

```bash
tux running # List all running docker container in all servers
```

## Deploy

```bash
tux deploy # Send all services in the dedicated servers
```

## Build

```bash
tux build <tag> # Create the image form the Dockerfile on the current directory
```

## Build

```bash
tux build <tag> # Create the image form the Dockerfile on the current directory
```

## Build

```bash
tux push <images> # Send images on docker hub
```