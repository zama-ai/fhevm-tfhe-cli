use tfhe::{
    generate_keys,
    prelude::{FheDecrypt, FheEncrypt},
    CompressedPublicKey, ConfigBuilder, FheUint8, PublicKey,
};

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("******************************************************************************");
        println!("Please give one argument:");
        println!("cargo run --release --example gen_keys <user_name> ");
        println!("example: cargo run --release --example gen_keys alice ");
        println!("******************************************************************************");
        panic!();
    }
    let user_name = &args[1];

    let conf = ConfigBuilder::all_disabled()
        .enable_default_uint8_small()
        .enable_default_uint16_small()
        .enable_default_uint32_small()
        .build();

    let cks_filename = format!("{}_cks.bin", user_name);
    let sks_filename = format!("{}_sks.bin", user_name);
    let pks_compressed_filename = format!("{}_compressed_pks.bin", user_name);
    let pks_uncompressed_filename = format!("{}_uncompressed_pks.bin", user_name);

    let (cks, sks) = generate_keys(conf);
    let pks_compressed: CompressedPublicKey = CompressedPublicKey::new(&cks);
    let pks: PublicKey = PublicKey::new(&cks);

    let clear_a = 27u8;
    let clear_b = 3u8;

    let ciphertext = FheUint8::encrypt(clear_a, &cks);
    let ciphertext2 = FheUint8::encrypt(clear_b, &cks);

    //Client-side
    let decrypted_result: u8 = FheUint8::decrypt(&ciphertext, &cks);

    assert_eq!(decrypted_result, clear_a);

    let mut serialized_pks_compressed = Vec::new();
    let mut serialized_pks = Vec::new();
    let mut serialized_cks = Vec::new();
    let mut serialized_sks = Vec::new();
    let mut serialized_ciphertext = Vec::new();
    let mut serialized_ciphertext2 = Vec::new();

    bincode::serialize_into(&mut serialized_pks_compressed, &pks_compressed).unwrap();
    bincode::serialize_into(&mut serialized_pks, &pks).unwrap();
    bincode::serialize_into(&mut serialized_sks, &sks).unwrap();
    bincode::serialize_into(&mut serialized_cks, &cks).unwrap();
    bincode::serialize_into(&mut serialized_ciphertext, &ciphertext).unwrap();
    bincode::serialize_into(&mut serialized_ciphertext2, &ciphertext2).unwrap();

    fs::write(
        format!("./res/keys/{}", pks_uncompressed_filename),
        serialized_pks,
    )
    .unwrap();
    fs::write(
        format!("./res/keys/{}", pks_compressed_filename),
        serialized_pks_compressed,
    )
    .unwrap();
    fs::write(format!("./res/keys/{}", sks_filename), serialized_sks).unwrap();
    fs::write(format!("./res/keys/{}", cks_filename), serialized_cks).unwrap();
    fs::write("./res/ct/enc_of_27.bin", serialized_ciphertext).unwrap();
    fs::write("./res/ct/enc_of_3.bin", serialized_ciphertext2).unwrap();
}
