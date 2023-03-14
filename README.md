# Zbc FHE tool

This repository allows developers to use __tfhe-rs__ features through a user friendly CLI tool.

This tool could be used locally or through a docker image. 



## Using the published image (easiest way)

One need to docker login to ghcr.io to download the published image.

<br />
<details>
  <summary>How to login into Zama github packages</summary>
<br />
The first step is to create a PAT (Personnal Access token) in github developer settings with 
a read access to Zama github registry. 

Then execute docker login ghcr.io with your github account name and the newly created PAT.
</details>
<br />

The following command mount the current directory in order to persist the created files as keys or ciphertext.
Keys are generated in res/keys and ciphertexts in res/ct.

```bash
docker run -v $PWD:/usr/local/app/ ghcr.io/zama-ai/zbc-fhe-tool:0.1.1 zbc-fhe
export ZBC_FHE_TOOL="docker run ghcr.io/zama-ai/zbc-fhe-tool zbc-fhe"
```

Example:

```bash
docker run -v $PWD:/usr/local/app/ ghcr.io/zama-ai/zbc-fhe-tool:0.1.1 zbc-fhe generate-secret-key base64 2 cks_2_2
$ZBC_FHE_TOOL generate-secret-key base64 2 cks_2_2
```

One can use the docker compose where the mount is already handled. 

```bash
docker compose -f docker-compose.override.yml run app zbc-fhe
```


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


