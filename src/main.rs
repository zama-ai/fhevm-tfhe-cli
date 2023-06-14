use std::{fs::File, io::Read};

use clap::{Parser, Subcommand};

use std::fs::write;

use tfhe::{
    generate_keys,
    prelude::{FheDecrypt, FheEncrypt},
    shortint::parameters::PARAM_SMALL_MESSAGE_2_CARRY_2_COMPACT_PK,
    ClientKey, CompactFheUint128List, CompactFheUint16List, CompactFheUint256List,
    CompactFheUint32List, CompactFheUint64List, CompactFheUint8List, CompactPublicKey,
    ConfigBuilder,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct PublicEncryptConf {
    /// Save the ciphertext in the given output file.
    #[clap(required = true)]
    ciphertext_output_file: String,

    /// Path to the FHE public key.
    #[clap(required = true)]
    public_key_file: String,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate FHE key triple (sks, cks, pks).
    #[clap(arg_required_else_help = true)]
    GenerateKeys {
        /// A directory to save the keys in.
        #[clap(required = true)]
        destination_dir: String,
    },

    /// Encrypts an 8-bit integer to an 8-bit FHE ciphertext.
    #[clap(arg_required_else_help = true)]
    PublicEncryptInteger8 {
        /// The integer to encrypt.
        #[clap(required = true)]
        plaintext: u8,

        #[clap(flatten)]
        conf: PublicEncryptConf,
    },

    /// Encrypts a 16-bit integer to a 16-bit FHE ciphertext.
    #[clap(arg_required_else_help = true)]
    PublicEncryptInteger16 {
        /// The integer to encrypt.
        #[clap(required = true)]
        plaintext: u16,

        #[clap(flatten)]
        conf: PublicEncryptConf,
    },

    /// Encrypts a 32-bit integer to a 32-bit FHE ciphertext.
    #[clap(arg_required_else_help = true)]
    PublicEncryptInteger32 {
        /// The integer to encrypt.
        #[clap(required = true)]
        plaintext: u32,

        #[clap(flatten)]
        conf: PublicEncryptConf,
    },

    /// Encrypts a 64-bit integer to a 64-bit FHE ciphertext.
    #[clap(arg_required_else_help = true)]
    PublicEncryptInteger64 {
        /// The integer to encrypt.
        #[clap(required = true)]
        plaintext: u64,

        #[clap(flatten)]
        conf: PublicEncryptConf,
    },

    /// Encrypts a 64-bit integer to a 128-bit FHE ciphertext.
    #[clap(arg_required_else_help = true)]
    PublicEncryptInteger128 {
        /// The integer to encrypt.
        #[clap(required = true)]
        plaintext: u64,

        #[clap(flatten)]
        conf: PublicEncryptConf,
    },

    /// Encrypts a 64-bit integer to a 256-bit FHE ciphertext.
    #[clap(arg_required_else_help = true)]
    PublicEncryptInteger256 {
        /// The integer to encrypt.
        #[clap(required = true)]
        plaintext: u64,

        #[clap(flatten)]
        conf: PublicEncryptConf,
    },

    /// Decrypts an 8-bit ciphertext.
    #[clap(arg_required_else_help = true)]
    DecryptInteger8 {
        /// The ciphertext to decrypt.
        #[clap(required = true)]
        ciphertext_file: String,

        /// Path to the FHE secret key.
        #[clap(required = true)]
        secret_key_file: String,
    },

    /// Decrypts a 16-bit ciphertext.
    #[clap(arg_required_else_help = true)]
    DecryptInteger16 {
        /// The ciphertext to decrypt.
        #[clap(required = true)]
        ciphertext_file: String,

        /// Path to the FHE secret key.
        #[clap(required = true)]
        secret_key_file: String,
    },

    /// Decrypts a 32-bit ciphertext.
    #[clap(arg_required_else_help = true)]
    DecryptInteger32 {
        /// The ciphertext to decrypt.
        #[clap(required = true)]
        ciphertext_file: String,

        /// Path to the FHE secret key.
        #[clap(required = true)]
        secret_key_file: String,
    },

    /// Decrypts a 64-bit ciphertext.
    #[clap(arg_required_else_help = true)]
    DecryptInteger64 {
        /// The ciphertext to decrypt.
        #[clap(required = true)]
        ciphertext_file: String,

        /// Path to the FHE secret key.
        #[clap(required = true)]
        secret_key_file: String,
    },

    /// Decrypts a 128-bit ciphertext.
    #[clap(arg_required_else_help = true)]
    DecryptInteger128 {
        /// The ciphertext to decrypt.
        #[clap(required = true)]
        ciphertext_file: String,

        /// Path to the FHE secret key.
        #[clap(required = true)]
        secret_key_file: String,
    },

    /// Decrypts a 256-bit ciphertext.
    #[clap(arg_required_else_help = true)]
    DecryptInteger256 {
        /// The ciphertext to decrypt.
        #[clap(required = true)]
        ciphertext_file: String,

        /// Path to the FHE secret key.
        #[clap(required = true)]
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

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::GenerateKeys { destination_dir } => {
            println!("Generating FHE keys in {destination_dir}");

            let config = ConfigBuilder::all_disabled()
                .enable_custom_integers(PARAM_SMALL_MESSAGE_2_CARRY_2_COMPACT_PK, None)
                .build();
            let (cks, sks) = generate_keys(config);
            let pks = CompactPublicKey::new(&cks);

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

        Commands::PublicEncryptInteger8 { plaintext, conf } => {
            println!("Encrypting {plaintext}");
            let pks = read_pks(&conf.public_key_file);
            let bytes = bincode::serialize(&CompactFheUint8List::encrypt(&vec![plaintext], &pks))
                .expect("ciphertext serialization");
            write(conf.ciphertext_output_file, &bytes).expect("ciphertext write to disk");
        }

        Commands::PublicEncryptInteger16 { plaintext, conf } => {
            println!("Encrypting {plaintext}");
            let pks = read_pks(&conf.public_key_file);
            let bytes = bincode::serialize(&CompactFheUint16List::encrypt(&vec![plaintext], &pks))
                .expect("ciphertext serialization");
            write(conf.ciphertext_output_file, &bytes).expect("ciphertext write to disk");
        }

        Commands::PublicEncryptInteger32 { plaintext, conf } => {
            println!("Encrypting {plaintext}");
            let pks = read_pks(&conf.public_key_file);
            let bytes = bincode::serialize(&CompactFheUint32List::encrypt(&vec![plaintext], &pks))
                .expect("ciphertext serialization");
            write(conf.ciphertext_output_file, &bytes).expect("ciphertext write to disk");
        }

        Commands::PublicEncryptInteger64 { plaintext, conf } => {
            println!("Encrypting {plaintext}");
            let pks = read_pks(&conf.public_key_file);
            let bytes = bincode::serialize(&CompactFheUint64List::encrypt(&vec![plaintext], &pks))
                .expect("ciphertext serialization");
            write(conf.ciphertext_output_file, &bytes).expect("ciphertext write to disk");
        }

        Commands::PublicEncryptInteger128 { plaintext, conf } => {
            println!("Encrypting {plaintext}");
            let pks = read_pks(&conf.public_key_file);
            let bytes = bincode::serialize(&CompactFheUint128List::encrypt(&vec![plaintext], &pks))
                .expect("ciphertext serialization");
            write(conf.ciphertext_output_file, &bytes).expect("ciphertext write to disk");
        }

        Commands::PublicEncryptInteger256 { plaintext, conf } => {
            println!("Encrypting {plaintext}");
            let pks = read_pks(&conf.public_key_file);
            let bytes = bincode::serialize(&CompactFheUint256List::encrypt(&vec![plaintext], &pks))
                .expect("ciphertext serialization");
            write(conf.ciphertext_output_file, &bytes).expect("ciphertext write to disk");
        }

        Commands::DecryptInteger8 {
            ciphertext_file,
            secret_key_file,
        } => {
            let (cks, bytes) = read_cks_and_ciphertext(&secret_key_file, &ciphertext_file);
            let ct: CompactFheUint8List =
                bincode::deserialize(&bytes).expect("ciphertext deserialization");
            let expanded_ct = ct.expand();
            let plaintext: u8 = expanded_ct[0].decrypt(&cks);
            println!("Decryption result: {plaintext}");
        }

        Commands::DecryptInteger16 {
            ciphertext_file,
            secret_key_file,
        } => {
            let (cks, bytes) = read_cks_and_ciphertext(&secret_key_file, &ciphertext_file);
            let ct: CompactFheUint16List =
                bincode::deserialize(&bytes).expect("ciphertext deserialization");
            let expanded_ct = ct.expand();
            let plaintext: u16 = expanded_ct[0].decrypt(&cks);
            println!("Decryption result: {plaintext}");
        }

        Commands::DecryptInteger32 {
            ciphertext_file,
            secret_key_file,
        } => {
            let (cks, bytes) = read_cks_and_ciphertext(&secret_key_file, &ciphertext_file);
            let ct: CompactFheUint32List =
                bincode::deserialize(&bytes).expect("ciphertext deserialization");
            let expanded_ct = ct.expand();
            let plaintext: u32 = expanded_ct[0].decrypt(&cks);
            println!("Decryption result: {plaintext}");
        }

        Commands::DecryptInteger64 {
            ciphertext_file,
            secret_key_file,
        } => {
            let (cks, bytes) = read_cks_and_ciphertext(&secret_key_file, &ciphertext_file);
            let ct: CompactFheUint64List =
                bincode::deserialize(&bytes).expect("ciphertext deserialization");
            let expanded_ct = ct.expand();
            let plaintext: u64 = expanded_ct[0].decrypt(&cks);
            println!("Decryption result: {plaintext}");
        }

        Commands::DecryptInteger128 {
            ciphertext_file,
            secret_key_file,
        } => {
            let (cks, bytes) = read_cks_and_ciphertext(&secret_key_file, &ciphertext_file);
            let ct: CompactFheUint128List =
                bincode::deserialize(&bytes).expect("ciphertext deserialization");
            let expanded_ct = ct.expand();
            let plaintext: u64 = expanded_ct[0].decrypt(&cks);
            println!("Decryption result: {plaintext}");
        }

        Commands::DecryptInteger256 {
            ciphertext_file,
            secret_key_file,
        } => {
            let (cks, bytes) = read_cks_and_ciphertext(&secret_key_file, &ciphertext_file);
            let ct: CompactFheUint256List =
                bincode::deserialize(&bytes).expect("ciphertext deserialization");
            let expanded_ct = ct.expand();
            let plaintext: u64 = expanded_ct[0].decrypt(&cks);
            println!("Decryption result: {plaintext}");
        }
    }
}
