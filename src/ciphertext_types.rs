use std::collections::HashMap;

use tfhe::{
    prelude::FheEncrypt, CompactFheUint128List, CompactFheUint16List, CompactFheUint256List,
    CompactFheUint32List, CompactFheUint64List, CompactFheUint8List, FheUint128, FheUint16,
    FheUint256, FheUint32, FheUint64, FheUint8,
};

use crate::gen_keys::gen_keys;

pub enum Precision {
    FheUint8,
    FheUint16,
    FheUint32,
    FheUint64,
    FheUint128,
    FheUint256,
}

impl Default for Precision {
    fn default() -> Self {
        Precision::FheUint8
    }
}

pub enum Format {
    Compact,
    Expanded,
}

impl Default for Format {
    fn default() -> Self {
        Format::Compact
    }
}

#[derive(Default)]
pub struct CiphertextType {
    pub precision: Precision,
    pub format: Format,
}

// A repository of ciphertext types. Given a ciphertext, the repository returns its type.
// TODO: Currently, we generate keys and encrypt on new() to determine ciphertext sizes
// per type. This is inefficient and error-prone.
pub struct CiphertextTypeRepo {
    types: HashMap<usize, CiphertextType>,
}

impl CiphertextTypeRepo {
    pub fn new() -> CiphertextTypeRepo {
        let mut repo = CiphertextTypeRepo {
            types: Default::default(),
        };

        let (_cks, _, pks) = gen_keys();

        let list = bincode::serialize(&CompactFheUint8List::encrypt(&vec![0], &pks))
            .expect("ciphertext serialization");
        repo.insert(
            &list,
            CiphertextType {
                precision: Precision::FheUint8,
                format: Format::Compact,
            },
        );

        let list = bincode::serialize(&CompactFheUint16List::encrypt(&vec![0], &pks))
            .expect("ciphertext serialization");
        repo.insert(
            &list,
            CiphertextType {
                precision: Precision::FheUint16,
                format: Format::Compact,
            },
        );

        let list = bincode::serialize(&CompactFheUint32List::encrypt(&vec![0], &pks))
            .expect("ciphertext serialization");
        repo.insert(
            &list,
            CiphertextType {
                precision: Precision::FheUint32,
                format: Format::Compact,
            },
        );

        let list = bincode::serialize(&CompactFheUint64List::encrypt(&vec![0], &pks))
            .expect("ciphertext serialization");
        repo.insert(
            &list,
            CiphertextType {
                precision: Precision::FheUint64,
                format: Format::Compact,
            },
        );

        let list = bincode::serialize(&CompactFheUint128List::encrypt(&vec![0], &pks))
            .expect("ciphertext serialization");
        repo.insert(
            &list,
            CiphertextType {
                precision: Precision::FheUint128,
                format: Format::Compact,
            },
        );

        let list = bincode::serialize(&CompactFheUint256List::encrypt(&vec![0], &pks))
            .expect("ciphertext serialization");
        repo.insert(
            &list,
            CiphertextType {
                precision: Precision::FheUint256,
                format: Format::Compact,
            },
        );

        let ct =
            bincode::serialize(&FheUint8::encrypt(0u8, &pks)).expect("ciphertext serialization");
        repo.insert(
            &ct,
            CiphertextType {
                precision: Precision::FheUint8,
                format: Format::Expanded,
            },
        );

        let ct =
            bincode::serialize(&FheUint16::encrypt(0u16, &pks)).expect("ciphertext serialization");
        repo.insert(
            &ct,
            CiphertextType {
                precision: Precision::FheUint16,
                format: Format::Expanded,
            },
        );

        let ct =
            bincode::serialize(&FheUint32::encrypt(0u32, &pks)).expect("ciphertext serialization");
        repo.insert(
            &ct,
            CiphertextType {
                precision: Precision::FheUint32,
                format: Format::Expanded,
            },
        );

        let ct =
            bincode::serialize(&FheUint64::encrypt(0u64, &pks)).expect("ciphertext serialization");
        repo.insert(
            &ct,
            CiphertextType {
                precision: Precision::FheUint64,
                format: Format::Expanded,
            },
        );

        let ct =
            bincode::serialize(&FheUint128::encrypt(0u64, &pks)).expect("ciphertext serialization");
        repo.insert(
            &ct,
            CiphertextType {
                precision: Precision::FheUint128,
                format: Format::Expanded,
            },
        );

        let ct =
            bincode::serialize(&FheUint256::encrypt(0u64, &pks)).expect("ciphertext serialization");
        repo.insert(
            &ct,
            CiphertextType {
                precision: Precision::FheUint256,
                format: Format::Expanded,
            },
        );
        repo
    }

    pub fn get_type(&self, ciphertext: &[u8]) -> Option<&CiphertextType> {
        self.types.get(&ciphertext.len())
    }

    fn insert(&mut self, ct: &[u8], ct_type: CiphertextType) {
        if self.types.insert(ct.len(), ct_type).is_some() {
            panic!("type size already existing");
        }
    }
}
