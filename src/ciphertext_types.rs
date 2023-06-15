use std::collections::HashMap;

use tfhe::{
    prelude::FheEncrypt, CompactFheUint128List, CompactFheUint16List, CompactFheUint256List,
    CompactFheUint32List, CompactFheUint64List, CompactFheUint8List, CompactPublicKey, FheUint128,
    FheUint16, FheUint256, FheUint32, FheUint64, FheUint8,
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
    pks: CompactPublicKey,
}

impl CiphertextTypeRepo {
    pub fn new() -> CiphertextTypeRepo {
        let (_, _, pks) = gen_keys();

        let mut repo = CiphertextTypeRepo {
            types: Default::default(),
            pks,
        };

        repo.insert_compact::<u8, CompactFheUint8List>(Precision::FheUint8);
        repo.insert_compact::<u16, CompactFheUint16List>(Precision::FheUint16);
        repo.insert_compact::<u32, CompactFheUint32List>(Precision::FheUint32);
        repo.insert_compact::<u64, CompactFheUint64List>(Precision::FheUint64);
        repo.insert_compact::<u64, CompactFheUint128List>(Precision::FheUint128);
        repo.insert_compact::<u64, CompactFheUint256List>(Precision::FheUint256);

        repo.insert_expanded::<u8, FheUint8>(Precision::FheUint8);
        repo.insert_expanded::<u16, FheUint16>(Precision::FheUint16);
        repo.insert_expanded::<u32, FheUint32>(Precision::FheUint32);
        repo.insert_expanded::<u64, FheUint64>(Precision::FheUint64);
        repo.insert_expanded::<u64, FheUint128>(Precision::FheUint128);
        repo.insert_expanded::<u64, FheUint256>(Precision::FheUint256);

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

    fn insert_compact<T, Compact>(&mut self, precision: Precision)
    where
        T: Default,
        Compact: for<'a> FheEncrypt<&'a [T], CompactPublicKey>,
        Compact: serde::Serialize,
    {
        let value: [T; 1] = [T::default()];
        let list = bincode::serialize(&Compact::encrypt(&value, &self.pks))
            .expect("ciphertext serialization");
        self.insert(
            &list,
            CiphertextType {
                precision,
                format: Format::Compact,
            },
        );
    }

    fn insert_expanded<T, Expanded>(&mut self, precision: Precision)
    where
        T: Default,
        Expanded: FheEncrypt<T, CompactPublicKey>,
        Expanded: serde::Serialize,
    {
        let list = bincode::serialize(&Expanded::encrypt(T::default(), &self.pks))
            .expect("ciphertext serialization");
        self.insert(
            &list,
            CiphertextType {
                precision,
                format: Format::Expanded,
            },
        );
    }
}
