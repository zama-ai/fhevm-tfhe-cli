// BSD 3-Clause Clear License
//
// Copyright © 2023 ZAMA.
// All rights reserved.

use tfhe::{
    generate_keys,
    shortint::parameters::parameters_compact_pk::PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS,
    ClientKey, CompactPublicKey, ConfigBuilder, ServerKey,
};

pub fn gen_keys() -> (ClientKey, ServerKey, CompactPublicKey) {
    let config = ConfigBuilder::all_disabled()
        .enable_custom_integers(PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_PBS_KS, None)
        .build();
    let (cks, sks) = generate_keys(config);
    let pks = CompactPublicKey::new(&cks);
    (cks, sks, pks)
}
