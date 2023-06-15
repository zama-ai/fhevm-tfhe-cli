use tfhe::{
    generate_keys, shortint::parameters::PARAM_SMALL_MESSAGE_2_CARRY_2_COMPACT_PK, ClientKey,
    CompactPublicKey, ConfigBuilder, ServerKey,
};

pub fn gen_keys() -> (ClientKey, ServerKey, CompactPublicKey) {
    let config = ConfigBuilder::all_disabled()
        .enable_custom_integers(PARAM_SMALL_MESSAGE_2_CARRY_2_COMPACT_PK, None)
        .build();
    let (cks, sks) = generate_keys(config);
    let pks = CompactPublicKey::new(&cks);
    (cks, sks, pks)
}
