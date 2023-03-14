# Zbc FHE tool

This repository allows developers to use __tfhe-rs__ features through a user friendly CLI tool.

This tool could be used locally or through a docker image. 


Examples are available in examples folder and a CLI tool called __zbc-fhe__ is ready-to-use (need to be completed)


## Using zbc-fhe tool locally

During the development, one need to generate public and secret FHE keys, or encrypt and decrypt an integer using a convenient encoding format as base64, hex or bin, this is the purpose of this tool. 

To use the src version export the following env variable/create an alias/compile in as a binary release:
```bash
export ZBC_FHE_TOOL="cargo run --release --bin zbc-fhe"
```

## Using the local docker version (with docker-compose to mount current folder)

Build the docker image from the local src:

```bash
docker compose  run app zbc-fhe help
export ZBC_FHE_TOOL="docker compose  run app zbc-fhe"
```


## List of example commands


### Print the global help menu

```bash
$ZBC_FHE_TOOL help
```

### Print menu for a specific command

```bash
$ZBC_FHE_TOOL  generate-secret-key
```

### Generate a new secret key cks


