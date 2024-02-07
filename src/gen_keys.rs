// BSD 3-Clause Clear License
//
// Copyright © 2023 ZAMA.
// All rights reserved.

use tfhe::{
    generate_keys, shortint::parameters::PARAM_MESSAGE_2_CARRY_2_KS_PBS, ClientKey, CompactPublicKey, ConfigBuilder, ServerKey
};

pub fn gen_keys() -> (ClientKey, ServerKey, CompactPublicKey) {
    let config = ConfigBuilder::with_custom_parameters(PARAM_MESSAGE_2_CARRY_2_KS_PBS, None)
         .build();
    let (cks, sks) = generate_keys(config);
    let pks = CompactPublicKey::new(&cks);
    (cks, sks, pks)
}
