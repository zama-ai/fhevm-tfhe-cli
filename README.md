# Zbc FHE tool

This repository allows developers to use __tfhe-rs__ features through a user friendly CLI tool.

This tool could be used locally or through a docker image. 

**WARNING: The CLI tool is limited is allowing by default operation on 8,16 and 32 bits.**

## Using the published image (easiest way)

One need to docker login to ghcr.io to download the published image.

<br />
<details>
  <summary>How to login into Zama github packages</summary>
<br />

1. Create a PAT (Personnal Access token) in github **developer settings** with a read (write if necessary) access to Zama github registry. 
2. Execute docker login ghcr.io with your **github account name** and the **newly created PAT**.


![PAT](./ressources/PAT_github_packages.png)
</details>
<br />

The following command **mount the current directory** in order to persist the created files as keys or ciphertext.
Keys are generated in res/keys and ciphertexts in res/ct.

```bash
docker run -v $PWD:/usr/local/app/ ghcr.io/zama-ai/zbc-fhe-tool:$TAG zbc-fhe
export ZBC_FHE_TOOL="docker run ghcr.io/zama-ai/zbc-fhe-tool zbc-fhe"
```

Example:

```bash
docker run -v $PWD:/usr/local/app/ ghcr.io/zama-ai/zbc-fhe-tool:$TAG zbc-fhe generate-secret-key bin cks
$ZBC_FHE_TOOL generate-secret-key bin cks
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
docker compose build app
export ZBC_FHE_TOOL="docker compose run app zbc-fhe"
```

## Generating keys for evmos node

```bash
cargo run --release --example gen_keys carol 
# ls -ll res/keys
# .rw-r--r--@ 121k user 22 mai   15:14 -I carol_cks.bin
# .rw-r--r--@ 1,3M user 22 mai   15:14 -I carol_compressed_pks.bin
# .rw-r--r--@ 2,3G user 22 mai   15:14 -I carol_sks.bin
# .rw-r--r--@ 1,2G user 22 mai   15:14 -I carol_uncompressed_pks.bin
```


## List of example commands

```bash
export ZBC_FHE_TOOL="docker run -v $PWD:/usr/local/app/ ghcr.io/zama-ai/zbc-fhe-tool:$TAG zbc-fhe"
```

### Print the global help menu

```bash
$ZBC_FHE_TOOL help
zbc-fhe-tool 0.1.0
Zama <hello@zama.ai>

USAGE:
    zbc-fhe <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    decrypt-integer
            Decrypts an integer with the given FHE secret key
    encrypt-compressed-public-integer
            Encrypts an integer with the compressed public key
    encrypt-integer
            Encrypts an integer with the given FHE secret key
    encrypt-public-integer
            Encrypts an integer with the public key
    generate-full-keys
            Generate ful FHE keys aka cks, sks and pks
    generate-secret-key
            Generate an FHE secret key
    help
            Print this message or the help of the given subcommand(s)
```

### Print menu for a specific command

```bash
$ZBC_FHE_TOOL  generate-secret-key
# zbc-fhe-generate-secret-key 
# Generate an FHE secret key

# USAGE:
#     zbc-fhe generate-secret-key <KEY_FORMAT> <SECRET_KEY_FILE>

# ARGS:
#     <KEY_FORMAT>         The format of the keys [possible values: hex, base64, bin]
#     <SECRET_KEY_FILE>    A file to save the FHE secret key to

# OPTIONS:
#     -h, --help    Print help information
```

### Generate a new secret key cks

```bash
$ZBC_FHE_TOOL generate-secret-key bin cks_test 
# Generating secret key: cks_test 
ls res/keys/
# cks_test.bin
```

### Encrypt an integer with the newly created secret key

```bash
$ZBC_FHE_TOOL encrypt-integer 11 bin ct_11 res/keys/cks_test.bin bin
# Encrypting 11 with secret key: res/keys/cks_test.bin
# Key format: Bin
# Ciphertext format: Bin
# Ciphertext: ./res/ct/ct_11.bin
```

## Decrypt a ciphertext using the private key

```bash
$ZBC_FHE_TOOL decrypt-integer ./res/ct/ct_11.bin bin ./res/keys/cks_test.bin bin 
# Decrypting with secret key: ./res/keys/cks_test.bin
# Key format: Bin
# Ciphertext format: Bin
# Ciphertext: ./res/ct/ct_11.bin
# Decrypted integer: 11
```

### Generate a full set of keys

```bash
$ZBC_FHE_TOOL  generate-full-keys bin test_8_bits 
# Generating test_8_bits_cks key
# Generating test_8_bits_pks key
# Generating test_8_bits_sks key
ls res/keys/
# test_8_bits_cks.bin
# test_8_bits_pks.bin
# test_8_bits_sks.bin
```

### Encrypt an integer with the newly created public key

```bash
$ZBC_FHE_TOOL encrypt-public-integer 46 hex enc_of_46 ./res/keys/test_8_bits_pks.bin bin 
# Encrypting 46 with public key: ./res/keys/test_8_bits_pks.bin
# Key format: Bin
# Ciphertext format: Hex
# Ciphertext: ./res/ct/enc_of_46.hex
```


### Check everything is ok 


```bash
$ZBC_FHE_TOOL decrypt-and-check-integer ./res/ct/enc_of_46.hex hex ./res/keys/test_8_bits_cks.bin bin 46
# Decrypting with secret key: ./res/keys/test_8_bits_cks.bin
# Key format: Bin
# Ciphertext format: Hex
# Ciphertext: ./res/ct/enc_of_46.hex
# Decrypted integer: 46
```
