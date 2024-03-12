# TFHE-CLI

The TFHE-CLI tool allows developers to use __tfhe-rs__ features through a user-friendly CLI.

This tool can be used locally or through a docker image.


# Docker images versioning

Registry and image name: docker pull ghcr.io/zama-ai/fhevm-tfhe-cli

Tags:

* v0.2.1: based on tfhe-rs 0.3.1
* v0.2.2: based on tfhe-rs 0.4.0
* v0.2.3: based on tfhe-rs 0.4.1
* v0.2.4: based on tfhe-rs 0.5.1

# Build

## Local
```
# for x86 CPUs
cargo build --features tfhe/x86_64-unix --release

# for ARM64
cargo build --features tfhe/aarch64-unix --release
```

## Docker

Either download latest image:

```
docker pull ghcr.io/zama-ai/fhevm-tfhe-cli:latest
```

or build it locally:

```
docker build -t fhevm-tfhe-cli:latest .
```

# Running operations

Make sure you either have Docker or the Rust toolchain installed on your host machine.

Please replace `FHEVM_TFHE_CLI` with either:
 * "`cargo run --features tfhe/aarch64-unix --release -- `" - for ARM CPUs when running locally on the host
 * "`cargo run --features tfhe/x86_64-unix --release -- `" - for x86 CPUs when running locally on the host
 * "`docker run -v $LOCAL_DIR:/usr/local/app ghcr.io/zama-ai/fhevm-tfhe-cli:latest fhevm-tfhe-cli`"
    * replace LOCAL_DIR with a local directory of choice in order to persist output from the tool when using Docker

For more information on Docker, see below.

For more information on supported operations and their variations, please see the built-in help:
```bash
FHEVM_TFHE_CLI help
```

## Key generation

```bash
mkdir -p /path/to/keys/directory
FHEVM_TFHE_CLI generate-keys -d /path/to/keys/directory
```

It will generate 3 keys in `/path/to/keys/directory`:

* `cks`: the private key, used for data decryption, that should never be shared publicly.
* `pks`: the public key, used for data encryption, that could be shared publicly.
* `sks`: the evaluation key, used for performing operations, that could be shared publicly.

## Public encryption

```bash
# Encryption requires the public key `pks`.
FHEVM_TFHE_CLI public-encrypt-integer32 -c ./ciphertext -p /path/to/keys/directory/pks -v 42
```

## Decryption

```bash
# Decryption requires the secret key `cks`.
FHEVM_TFHE_CLI decrypt-ciphertext -c ./ciphertext -s /path/to/keys/directory/cks
```

<!--
# Using published Docker images

One needs to login to ghcr.io to download the published image.

<br />
<details>
  <summary>How to login into Zama github packages</summary>
<br />

1. Create a PAT (Personal Access token) in github **developer settings** with a read (write if necessary) access to Zama github registry.
2. Execute docker login ghcr.io with your **github account name** and the **newly created PAT**.

![PAT](./resources/PAT_github_packages.png)
</details>
<br />
-->
