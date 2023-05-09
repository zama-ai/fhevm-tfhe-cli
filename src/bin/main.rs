use std::{fmt, fs};

use clap::{ArgEnum, Parser, Subcommand};

use tfhe::{
    generate_keys,
    prelude::{FheDecrypt, FheEncrypt, FheTryEncrypt},
    ClientKey, CompressedPublicKey, ConfigBuilder, FheUint8, PublicKey,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
enum Format {
    Hex,
    Base64,
    Bin,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Format::Hex => write!(f, "Hex"),
            Format::Base64 => write!(f, "Base64"),
            Format::Bin => write!(f, "Bin"),
        }
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate an FHE secret key
    #[clap(arg_required_else_help = true)]
    GenerateSecretKey {
        /// The format of the keys
        #[clap(arg_enum)]
        key_format: Format,

        /// A file to save the FHE secret key to
        #[clap(required = true)]
        secret_key_file: String,
    },

    /// Generate ful FHE keys aka cks, sks and pks
    #[clap(arg_required_else_help = true)]
    GenerateFullKeys {
        /// The format of the keys
        #[clap(arg_enum)]
        keys_format: Format,

        /// A file to save the FHE secret key to
        #[clap(required = true)]
        prefix_keys: String,
    },

    /// Encrypts an integer with the given FHE secret key
    #[clap(arg_required_else_help = true)]
    EncryptInteger {
        /// The integer to encrypt
        #[clap(required = true)]
        to_encrypt: u64,

        /// The format of the output ciphertext
        #[clap(arg_enum)]
        output_format: Format,

        /// Ciphertext prefix
        #[clap(required = true)]
        ciphertext_prefix: String,

        /// Path to the FHE secret key
        #[clap(required = true)]
        secret_key_file: String,

        /// The format of the secret key
        #[clap(arg_enum)]
        key_format: Format,
    },

    /// Encrypts an integer with the compressed public key
    #[clap(arg_required_else_help = true)]
    EncryptCompressedPublicInteger {
        /// The integer to encrypt
        #[clap(required = true)]
        to_encrypt: u64,

        /// The format of the output ciphertext
        #[clap(arg_enum)]
        output_format: Format,

        /// Ciphertext prefix
        #[clap(required = true)]
        ciphertext_prefix: String,

        /// Path to the FHE compressed public key
        #[clap(required = true)]
        compressed_public_key_file: String,

        /// The format of the  compressed public key
        #[clap(arg_enum)]
        key_format: Format,
    },

    /// Encrypts an integer with the public key
    #[clap(arg_required_else_help = true)]
    EncryptPublicInteger {
        /// The integer to encrypt
        #[clap(required = true)]
        to_encrypt: u64,

        /// The format of the output ciphertext
        #[clap(arg_enum)]
        output_format: Format,

        /// Ciphertext prefix
        #[clap(required = true)]
        ciphertext_prefix: String,

        /// Path to the FHE public key
        #[clap(required = true)]
        public_key_file: String,

        /// The format of the public key
        #[clap(arg_enum)]
        key_format: Format,
    },

    /// Decrypts an integer with the given FHE secret key
    #[clap(arg_required_else_help = true)]
    DecryptInteger {
        /// The ciphertext to decrypt
        #[clap(required = true)]
        ciphertext_file: String,

        /// The format of the input ciphertext
        #[clap(arg_enum)]
        ciphertext_format: Format,

        /// Path to the FHE secret key
        #[clap(required = true)]
        secret_key_file: String,

        /// The format of the secret key
        #[clap(arg_enum)]
        key_format: Format,
    },

    /// Decrypts an integer with the given FHE secret key and compare against an expected value
    #[clap(arg_required_else_help = true)]
    DecryptAndCheckInteger {
        /// The ciphertext to decrypt
        #[clap(required = true)]
        ciphertext_file: String,

        /// The format of the input ciphertext
        #[clap(arg_enum)]
        ciphertext_format: Format,

        /// Path to the FHE secret key
        #[clap(required = true)]
        secret_key_file: String,

        /// The format of the secret key
        #[clap(arg_enum)]
        key_format: Format,

        /// The expected integer
        #[clap(required = true)]
        expected_result: u64,
    },
}

fn main() {
    let args = Args::parse();
    let keys_path = "./res/keys";
    let ct_path = "./res/ct";

    fs::create_dir_all(keys_path).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    fs::create_dir_all(ct_path).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    match args.command {
        Commands::GenerateSecretKey {
            key_format,
            secret_key_file,
        } => {
            println!("Generating secret key: {} ", secret_key_file,);

            let config = ConfigBuilder::all_disabled()
                .enable_default_uint8_small()
                .build();

            // Client-side
            let (secret_key, _) = generate_keys(config);

            // let mut serialized_secret_key = Vec::new();
            // bincode::serialize_into(&mut serialized_secret_key, &secret_key).unwrap();
            let serialized_secret_key = bincode::serialize(&secret_key).unwrap();

            match key_format {
                Format::Base64 => {
                    let base64_cks = base64::encode(&serialized_secret_key);
                    std::fs::write(format!("{}/{}.b64", keys_path, secret_key_file), base64_cks)
                        .unwrap();
                }
                Format::Hex => {
                    let hex_cks = hex::encode(&serialized_secret_key);
                    std::fs::write(format!("{}/{}.hex", keys_path, secret_key_file), hex_cks)
                        .unwrap();
                }
                Format::Bin => {
                    std::fs::write(
                        format!("{}/{}.bin", keys_path, secret_key_file),
                        serialized_secret_key,
                    )
                    .unwrap();
                }
            }
        }
        // Step_1
        Commands::GenerateFullKeys {
            keys_format,
            prefix_keys,
        } => {
            println!("Generating {}_cks key", prefix_keys);
            println!("Generating {}_pks key", prefix_keys);
            println!("Generating {}_sks key", prefix_keys);

            let config = ConfigBuilder::all_disabled().enable_default_uint8().build();

            // Client-side
            let (cks, sks) = generate_keys(config);
            let pks: PublicKey = PublicKey::new(&cks);

            let serialized_secret_key = bincode::serialize(&cks).unwrap();
            let serialized_server_key = bincode::serialize(&sks).unwrap();
            let serialized_public_key = bincode::serialize(&pks).unwrap();

            match keys_format {
                Format::Base64 => {
                    let base64_cks = base64::encode(&serialized_secret_key);
                    let base64_sks = base64::encode(&serialized_server_key);
                    let base64_pks = base64::encode(&serialized_public_key);
                    std::fs::write(format!("{}/{}_cks.b64", keys_path, prefix_keys), base64_cks)
                        .unwrap();
                    std::fs::write(format!("{}/{}_sks.b64", keys_path, prefix_keys), base64_sks)
                        .unwrap();
                    std::fs::write(format!("{}/{}_pks.b64", keys_path, prefix_keys), base64_pks)
                        .unwrap();
                }
                Format::Hex => {
                    let hex_cks = hex::encode(&serialized_secret_key);
                    let hex_sks = hex::encode(&serialized_server_key);
                    let hex_pks = hex::encode(&serialized_public_key);
                    std::fs::write(format!("{}/{}_cks.hex", keys_path, prefix_keys), hex_cks)
                        .unwrap();
                    std::fs::write(format!("{}/{}_sks.hex", keys_path, prefix_keys), hex_sks)
                        .unwrap();
                    std::fs::write(format!("{}/{}_pks.hex", keys_path, prefix_keys), hex_pks)
                        .unwrap();
                }
                Format::Bin => {
                    std::fs::write(
                        format!("{}/{}_cks.bin", keys_path, prefix_keys),
                        serialized_secret_key,
                    )
                    .unwrap();
                    std::fs::write(
                        format!("{}/{}_sks.bin", keys_path, prefix_keys),
                        serialized_server_key,
                    )
                    .unwrap();
                    std::fs::write(
                        format!("{}/{}_pks.bin", keys_path, prefix_keys),
                        serialized_public_key,
                    )
                    .unwrap();
                }
            }
        }

        Commands::EncryptInteger {
            to_encrypt,
            output_format,
            ciphertext_prefix,
            secret_key_file,
            key_format,
        } => {
            println!(
                "Encrypting {} with secret key: {}",
                to_encrypt, secret_key_file
            );
            println!("Key format: {}", key_format.to_string());
            println!("Ciphertext format: {}", output_format.to_string());

            let bytes = std::fs::read(&secret_key_file).unwrap();
            let cks_encoded = match key_format {
                Format::Base64 => base64::decode(&bytes).unwrap(),
                Format::Hex => hex::decode(&bytes).unwrap(),
                Format::Bin => bytes,
            };
            let cks = bincode::deserialize_from(cks_encoded.as_slice()).unwrap();

            let ciphertext = FheUint8::encrypt(to_encrypt.try_into().unwrap(), &cks);

            let mut serialized_ct = Vec::new();
            bincode::serialize_into(&mut serialized_ct, &ciphertext).unwrap();

            match output_format {
                Format::Hex => {
                    let hex_ct = hex::encode(&serialized_ct);
                    std::fs::write(format!("{}/{}.hex", ct_path, ciphertext_prefix), hex_ct)
                        .unwrap();
                    println!(
                        "Ciphertext: {}",
                        format!("{}/{}.hex", ct_path, ciphertext_prefix)
                    );
                }
                Format::Base64 => {
                    let base64_ct = base64::encode(&serialized_ct);
                    std::fs::write(format!("{}/{}.b64", ct_path, ciphertext_prefix), base64_ct)
                        .unwrap();
                    println!(
                        "Ciphertext: {}",
                        format!("{}/{}.b64", ct_path, ciphertext_prefix)
                    );
                }
                Format::Bin => {
                    std::fs::write(
                        format!("{}/{}.bin", ct_path, ciphertext_prefix),
                        serialized_ct,
                    )
                    .unwrap();
                    println!(
                        "Ciphertext: {}",
                        format!("{}/{}.bin", ct_path, ciphertext_prefix)
                    );
                }
            }
        }

        Commands::EncryptCompressedPublicInteger {
            to_encrypt,
            output_format,
            ciphertext_prefix,
            compressed_public_key_file,
            key_format,
        } => {
            println!(
                "Encrypting {} with compressed public key: {}",
                to_encrypt, compressed_public_key_file
            );
            println!("Key format: {}", key_format.to_string());
            println!("Ciphertext format: {}", output_format.to_string());

            let bytes = std::fs::read(&compressed_public_key_file).unwrap();
            let pks_encoded = match key_format {
                Format::Base64 => base64::decode(&bytes).unwrap(),
                Format::Hex => hex::decode(&bytes).unwrap(),
                Format::Bin => bytes,
            };

            let pks_compressed: CompressedPublicKey = bincode::deserialize(&pks_encoded).unwrap();
            let ciphertext = FheUint8::try_encrypt(to_encrypt, &pks_compressed).unwrap();

            let mut serialized_ct = Vec::new();
            bincode::serialize_into(&mut serialized_ct, &ciphertext).unwrap();

            match output_format {
                Format::Hex => {
                    let hex_ct = hex::encode(&serialized_ct);
                    std::fs::write(format!("{}/{}.hex", ct_path, ciphertext_prefix), hex_ct)
                        .unwrap();
                    println!(
                        "Ciphertext: {}",
                        format!("{}/{}.hex", ct_path, ciphertext_prefix)
                    );
                }
                Format::Base64 => {
                    let base64_ct = base64::encode(&serialized_ct);
                    std::fs::write(format!("{}/{}.b64", ct_path, ciphertext_prefix), base64_ct)
                        .unwrap();
                    println!(
                        "Ciphertext: {}",
                        format!("{}/{}.b64", ct_path, ciphertext_prefix)
                    );
                }
                Format::Bin => {
                    std::fs::write(
                        format!("{}/{}.bin", ct_path, ciphertext_prefix),
                        serialized_ct,
                    )
                    .unwrap();
                    println!(
                        "Ciphertext: {}",
                        format!("{}/{}.bin", ct_path, ciphertext_prefix)
                    );
                }
            }
        }

        Commands::EncryptPublicInteger {
            to_encrypt,
            output_format,
            ciphertext_prefix,
            public_key_file,
            key_format,
        } => {
            println!(
                "Encrypting {} with public key: {}",
                to_encrypt, public_key_file
            );
            println!("Key format: {}", key_format.to_string());
            println!("Ciphertext format: {}", output_format.to_string());

            let bytes = std::fs::read(&public_key_file).unwrap();
            let pks_encoded = match key_format {
                Format::Base64 => base64::decode(&bytes).unwrap(),
                Format::Hex => hex::decode(&bytes).unwrap(),
                Format::Bin => bytes,
            };
            let pks: PublicKey = bincode::deserialize(&pks_encoded).unwrap();
            let ciphertext = FheUint8::try_encrypt(to_encrypt, &pks).unwrap();

            let mut serialized_ct = Vec::new();
            bincode::serialize_into(&mut serialized_ct, &ciphertext).unwrap();

            match output_format {
                Format::Hex => {
                    let hex_ct = hex::encode(&serialized_ct);
                    std::fs::write(format!("{}/{}.hex", ct_path, ciphertext_prefix), hex_ct)
                        .unwrap();
                    println!(
                        "Ciphertext: {}",
                        format!("{}/{}.hex", ct_path, ciphertext_prefix)
                    );
                }
                Format::Base64 => {
                    let base64_ct = base64::encode(&serialized_ct);
                    std::fs::write(format!("{}/{}.b64", ct_path, ciphertext_prefix), base64_ct)
                        .unwrap();
                    println!(
                        "Ciphertext: {}",
                        format!("{}/{}.b64", ct_path, ciphertext_prefix)
                    );
                }
                Format::Bin => {
                    std::fs::write(
                        format!("{}/{}.bin", ct_path, ciphertext_prefix),
                        serialized_ct,
                    )
                    .unwrap();
                    println!(
                        "Ciphertext: {}",
                        format!("{}/{}.bin", ct_path, ciphertext_prefix)
                    );
                }
            }
        }

        Commands::DecryptInteger {
            ciphertext_file,
            ciphertext_format,
            secret_key_file,
            key_format,
        } => {
            println!("Decrypting with secret key: {}", secret_key_file);
            println!("Key format: {}", key_format.to_string());
            println!("Ciphertext format: {}", ciphertext_format.to_string());
            println!("Ciphertext: {}", ciphertext_file);
            let bytes = std::fs::read(&secret_key_file).unwrap();
            let cks_encoded = match key_format {
                Format::Base64 => base64::decode(&bytes).unwrap(),
                Format::Hex => hex::decode(&bytes).unwrap(),
                Format::Bin => bytes,
            };
            let cks: ClientKey = bincode::deserialize_from(cks_encoded.as_slice()).unwrap();

            let bytes = std::fs::read(&ciphertext_file).unwrap();

            match ciphertext_format {
                Format::Base64 => {
                    let base64_ct = base64::decode(&bytes).unwrap();
                    let ct: FheUint8 = bincode::deserialize_from(base64_ct.as_slice()).unwrap();
                    let plaintext: u64 = FheUint8::decrypt(&ct, &cks);
                    println!("Decrypted integer: {}", plaintext);
                }
                Format::Hex => {
                    let hex_ct = hex::decode(&bytes).unwrap();
                    let ct: FheUint8 = bincode::deserialize_from(hex_ct.as_slice()).unwrap();
                    let plaintext: u64 = FheUint8::decrypt(&ct, &cks);
                    println!("Decrypted integer: {}", plaintext);
                }

                Format::Bin => {
                    let ct: FheUint8 = bincode::deserialize_from(bytes.as_slice()).unwrap();
                    let plaintext: u64 = FheUint8::decrypt(&ct, &cks);
                    println!("Decrypted integer: {}", plaintext);
                }
            }
        }

        Commands::DecryptAndCheckInteger {
            ciphertext_file,
            ciphertext_format,
            secret_key_file,
            key_format,
            expected_result,
        } => {
            println!("Decrypting with secret key: {}", secret_key_file);
            println!("Key format: {}", key_format.to_string());
            println!("Ciphertext format: {}", ciphertext_format.to_string());
            println!("Ciphertext: {}", ciphertext_file);
            let bytes = std::fs::read(&secret_key_file).unwrap();
            let cks_encoded = match key_format {
                Format::Base64 => base64::decode(&bytes).unwrap(),
                Format::Hex => hex::decode(&bytes).unwrap(),
                Format::Bin => bytes,
            };
            let cks: ClientKey = bincode::deserialize(&cks_encoded).unwrap();

            let bytes = std::fs::read(&ciphertext_file).unwrap();

            match ciphertext_format {
                Format::Base64 => {
                    let base64_ct = base64::decode(&bytes).unwrap();
                    let ct: FheUint8 = bincode::deserialize_from(base64_ct.as_slice()).unwrap();
                    let plaintext: u64 = FheUint8::decrypt(&ct, &cks);
                    println!("Decrypted integer: {}", plaintext);
                    assert_eq!(plaintext, expected_result);
                }
                Format::Hex => {
                    let hex_ct = hex::decode(&bytes).unwrap();
                    let ct: FheUint8 = bincode::deserialize_from(hex_ct.as_slice()).unwrap();
                    let plaintext: u64 = FheUint8::decrypt(&ct, &cks);
                    println!("Decrypted integer: {}", plaintext);
                    assert_eq!(plaintext, expected_result);
                }

                Format::Bin => {
                    let ct: FheUint8 = bincode::deserialize_from(bytes.as_slice()).unwrap();
                    let plaintext: u64 = FheUint8::decrypt(&ct, &cks);
                    println!("Decrypted integer: {}", plaintext);
                    assert_eq!(plaintext, expected_result);
                }
            }
        }
    }
}
