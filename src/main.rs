use std::{fs::File, io::Read};

use clap::{Parser, Subcommand};

use tfhe::{FheUint128, FheUint16, FheUint256, FheUint32, FheUint64, FheUint8};
use zbc_fhe_tool::ciphertext_types::{CiphertextTypeRepo, Format, Precision};
use zbc_fhe_tool::gen_keys::gen_keys;

use std::fs::write;

use tfhe::{
    prelude::{FheDecrypt, FheEncrypt},
    ClientKey, CompactFheUint128List, CompactFheUint16List, CompactFheUint256List,
    CompactFheUint32List, CompactFheUint64List, CompactFheUint8List, CompactPublicKey,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct PublicEncryptCommon {
    /// Save the ciphertext in the given output file.
    #[clap(short, long)]
    ciphertext_output_file: String,

    /// Path to the FHE public key.
    #[clap(short, long)]
    public_key_file: String,

    /// Whether to encrypt to an expanded FHE ciphertext (compact is used if not expanded).
    #[clap(short, long)]
    expanded: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate FHE key triple (sks, cks, pks).
    #[clap(arg_required_else_help = true)]
    GenerateKeys {
        /// A directory to save the keys in.
        #[clap(short, long)]
        destination_dir: String,
    },

    /// Encrypts an 8-bit integer to an 8-bit FHE ciphertext.
    #[clap(arg_required_else_help = true)]
    PublicEncryptInteger8 {
        /// The integer to encrypt.
        #[clap(short, long)]
        value: u8,

        #[clap(flatten)]
        common: PublicEncryptCommon,
    },

    /// Encrypts a 16-bit integer to a 16-bit FHE ciphertext.
    #[clap(arg_required_else_help = true)]
    PublicEncryptInteger16 {
        /// The integer to encrypt.
        #[clap(short, long)]
        value: u16,

        #[clap(flatten)]
        common: PublicEncryptCommon,
    },

    /// Encrypts a 32-bit integer to a 32-bit FHE ciphertext.
    #[clap(arg_required_else_help = true)]
    PublicEncryptInteger32 {
        /// The integer to encrypt.
        #[clap(short, long)]
        value: u32,

        #[clap(flatten)]
        common: PublicEncryptCommon,
    },

    /// Encrypts a 64-bit integer to a 64-bit FHE ciphertext.
    #[clap(arg_required_else_help = true)]
    PublicEncryptInteger64 {
        /// The integer to encrypt.
        #[clap(short, long)]
        value: u64,

        #[clap(flatten)]
        common: PublicEncryptCommon,
    },

    /// Encrypts a 64-bit integer to a 128-bit FHE ciphertext.
    #[clap(arg_required_else_help = true)]
    PublicEncryptInteger128 {
        /// The integer to encrypt.
        #[clap(short, long)]
        value: u64,

        #[clap(flatten)]
        common: PublicEncryptCommon,
    },

    /// Encrypts a 64-bit integer to a 256-bit FHE ciphertext.
    #[clap(arg_required_else_help = true)]
    PublicEncryptInteger256 {
        /// The integer to encrypt.
        #[clap(short, long)]
        value: u64,

        #[clap(flatten)]
        common: PublicEncryptCommon,
    },

    /// Decrypts ciphertext.
    #[clap(arg_required_else_help = true)]
    DecryptCiphertext {
        /// The ciphertext to decrypt.
        #[clap(short, long)]
        ciphertext_file: String,

        /// Path to the FHE secret key.
        #[clap(short, long)]
        secret_key_file: String,
    },
}

fn read_pks(file: &str) -> CompactPublicKey {
    let mut f = File::open(file).expect("pks file open");
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).expect("pks file read from disk");
    bincode::deserialize(&buf).expect("pks deserialization")
}

fn read_cks(file: &str) -> ClientKey {
    let mut f = File::open(file).expect("cks file open");
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).expect("cks file read from disk");
    bincode::deserialize(&buf).expect("cks deserialization")
}

fn read_ciphertext(file: &str) -> Vec<u8> {
    let mut f = File::open(file).expect("ciphertext file open");
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)
        .expect("ciphertext file read from disk");
    buf
}

fn read_cks_and_ciphertext(cks_file: &str, ciphertext_file: &str) -> (ClientKey, Vec<u8>) {
    (read_cks(cks_file), read_ciphertext(ciphertext_file))
}

fn encrypt_impl<T, Compact, Expanded>(value: T, common: &PublicEncryptCommon)
where
    T: std::fmt::Display,
    Compact: for<'a> FheEncrypt<&'a [T], CompactPublicKey>,
    Compact: serde::Serialize,
    Expanded: FheEncrypt<T, CompactPublicKey>,
    Expanded: serde::Serialize,
{
    println!("Encrypting {value}");
    let pks = read_pks(&common.public_key_file);
    let bytes;
    if common.expanded {
        bytes =
            bincode::serialize(&Expanded::encrypt(value, &pks)).expect("ciphertext serialization");
    } else {
        let value: [T; 1] = [value];
        bytes =
            bincode::serialize(&Compact::encrypt(&value, &pks)).expect("ciphertext serialization");
    }
    write(&common.ciphertext_output_file, &bytes).expect("ciphertext write to disk");
}

fn decrypt_expanded<Expanded>(ciphertext: &[u8], cks: &ClientKey) -> u64
where
    Expanded: for<'a> serde::Deserialize<'a>,
    Expanded: FheDecrypt<u64>,
{
    let ct: Expanded = bincode::deserialize(&ciphertext).expect("ciphertext deserialization");
    ct.decrypt(&cks)
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::GenerateKeys { destination_dir } => {
            println!("Generating FHE keys in {destination_dir}");

            let (cks, sks, pks) = gen_keys();

            {
                let cks = bincode::serialize(&cks).expect("cks serialization");
                println!("Generated cks size: {} bytes", cks.len());
                write(destination_dir.clone() + "/cks", cks).expect("cks write to disk");
            }

            {
                let sks = bincode::serialize(&sks).expect("sks serialization");
                println!("Generated sks size: {} bytes", sks.len());
                write(destination_dir.clone() + "/sks", sks).expect("sks write to disk");
            }

            {
                let pks = bincode::serialize(&pks).unwrap();
                println!("Generated pks size: {} bytes", pks.len());
                write(destination_dir.clone() + "/pks", pks).expect("pks write to disk");
            }
        }

        Commands::PublicEncryptInteger8 { value, common } => {
            encrypt_impl::<_, CompactFheUint8List, FheUint8>(value, &common);
        }

        Commands::PublicEncryptInteger16 { value, common } => {
            encrypt_impl::<_, CompactFheUint16List, FheUint16>(value, &common);
        }

        Commands::PublicEncryptInteger32 { value, common } => {
            encrypt_impl::<_, CompactFheUint32List, FheUint32>(value, &common);
        }

        Commands::PublicEncryptInteger64 { value, common } => {
            encrypt_impl::<_, CompactFheUint64List, FheUint64>(value, &common);
        }

        Commands::PublicEncryptInteger128 { value, common } => {
            encrypt_impl::<_, CompactFheUint128List, FheUint128>(value, &common);
        }

        Commands::PublicEncryptInteger256 { value, common } => {
            encrypt_impl::<_, CompactFheUint256List, FheUint256>(value, &common);
        }

        Commands::DecryptCiphertext {
            ciphertext_file,
            secret_key_file,
        } => {
            let (cks, ct_bytes) = read_cks_and_ciphertext(&secret_key_file, &ciphertext_file);
            let type_repo = CiphertextTypeRepo::new();
            let ct_type = type_repo
                .get_type(&ct_bytes)
                .expect("known ciphertext type");
            let plaintext: u64;
            match ct_type.format {
                Format::Compact => match ct_type.precision {
                    Precision::FheUint8 => {
                        let ct: CompactFheUint8List =
                            bincode::deserialize(&ct_bytes).expect("ciphertext deserialization");
                        let expanded_ct = ct.expand();
                        plaintext = expanded_ct[0].decrypt(&cks);
                    }
                    Precision::FheUint16 => {
                        let ct: CompactFheUint16List =
                            bincode::deserialize(&ct_bytes).expect("ciphertext deserialization");
                        let expanded_ct = ct.expand();
                        plaintext = expanded_ct[0].decrypt(&cks);
                    }
                    Precision::FheUint32 => {
                        let ct: CompactFheUint32List =
                            bincode::deserialize(&ct_bytes).expect("ciphertext deserialization");
                        let expanded_ct = ct.expand();
                        plaintext = expanded_ct[0].decrypt(&cks);
                    }
                    Precision::FheUint64 => {
                        let ct: CompactFheUint64List =
                            bincode::deserialize(&ct_bytes).expect("ciphertext deserialization");
                        let expanded_ct = ct.expand();
                        plaintext = expanded_ct[0].decrypt(&cks);
                    }
                    Precision::FheUint128 => {
                        let ct: CompactFheUint128List =
                            bincode::deserialize(&ct_bytes).expect("ciphertext deserialization");
                        let expanded_ct = ct.expand();
                        plaintext = expanded_ct[0].decrypt(&cks);
                    }
                    Precision::FheUint256 => {
                        let ct: CompactFheUint256List =
                            bincode::deserialize(&ct_bytes).expect("ciphertext deserialization");
                        let expanded_ct = ct.expand();
                        plaintext = expanded_ct[0].decrypt(&cks);
                    }
                },
                Format::Expanded => match ct_type.precision {
                    Precision::FheUint8 => {
                        plaintext = decrypt_expanded::<FheUint8>(&ct_bytes, &cks);
                    }
                    Precision::FheUint16 => {
                        plaintext = decrypt_expanded::<FheUint16>(&ct_bytes, &cks);
                    }
                    Precision::FheUint32 => {
                        plaintext = decrypt_expanded::<FheUint32>(&ct_bytes, &cks);
                    }
                    Precision::FheUint64 => {
                        plaintext = decrypt_expanded::<FheUint64>(&ct_bytes, &cks);
                    }
                    Precision::FheUint128 => {
                        plaintext = decrypt_expanded::<FheUint128>(&ct_bytes, &cks);
                    }
                    Precision::FheUint256 => {
                        plaintext = decrypt_expanded::<FheUint256>(&ct_bytes, &cks);
                    }
                },
            }
            println!("Decryption result: {plaintext}");
        }
    }
}
