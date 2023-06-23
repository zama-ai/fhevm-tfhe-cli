# ZBC FHE tool

The ZBC FHE tool allows developers to use __tfhe-rs__ features through a user-friendly CLI.

This tool could be used locally or through a docker image.

# Quick start

If you want to generate Fhe keys for your evmos node:

On x86:
```bash
$> mkdir -p res && docker run -v $PWD:/usr/local/app ghcr.io/zama-ai/zbc-fhe-tool:v0.1.0 zbc-fhe-tool  generate-keys -d res
```

On arm64:
```bash
$> mkdir -p res && docker run -v $PWD:/usr/local/app ghcr.io/zama-ai/zbc-fhe-tool:v0.1.0-arm64 zbc-fhe-tool  generate-keys -d res
```

Typical output:
```bash
# Generating FHE keys in res
# Generated cks size: 41130 bytes
# Generated sks size: 100729006 bytes
# Generated pks size: 16553 bytes
$> ls res
# cks  pks  sks
```

# Running operations

Make sure you either have Docker or the Rust toolchain installed on your host machine.

Please replace `ZBC_FHE_TOOL` with either:
 * "`cargo run --features tfhe/aarch64-unix --release -- `" - for ARM CPUs when running locally on the host
 * "`cargo run --features tfhe/x86_64-unix --release -- `" - for x86 CPUs when running locally on the host
 * "`docker run -v $LOCAL_DIR:/usr/local/app ghcr.io/zama-ai/zbc-fhe-tool:$TAG zbc-fhe-tool`"
    * replace LOCAL_DIR with a local directory of choice in order to persist output from the tool when using Docker
    * replace TAG with a known tagged version

For more information on Docker, see below.

For more information on supported operations and their variations, please see the built-in help:
```bash
ZBC_FHE_TOOL help
```

## Key generation

```bash
mkdir -p /path/to/keys/directory
ZBC_FHE_TOOL generate-keys -d /path/to/keys/directory
```

## Public encryption

```bash
# Encryption requires the public key `pks`.
ZBC_FHE_TOOL public-encrypt-integer32 -c ./ciphertext -p /path/to/keys/directory/pks -v 42
```

## Decryption

```bash
# Decryption requires the secret key `cks`.
ZBC_FHE_TOOL decrypt-ciphertext -c ./ciphertext -s /path/to/keys/directory/cks
```

# Using published Docker images

One needs to docker login to ghcr.io to download the published image.

<br />
<details>
  <summary>How to login into Zama github packages</summary>
<br />

1. Create a PAT (Personnal Access token) in github **developer settings** with a read (write if necessary) access to Zama github registry. 
2. Execute docker login ghcr.io with your **github account name** and the **newly created PAT**.

![PAT](./resources/PAT_github_packages.png)
</details>
<br />
