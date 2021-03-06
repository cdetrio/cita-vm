//! Package hash_sha3 is a function set for implementing Ethereum's state
//! which use sha3 instead of keccak.
use super::err;
pub use cita_trie::codec::RLPNodeCodec;
use ethereum_types::H256;
use sha3::Digest;

pub const NIL_DATA: H256 = H256([
    0xa7, 0xff, 0xc6, 0xf8, 0xbf, 0x1e, 0xd7, 0x66, 0x51, 0xc1, 0x47, 0x56, 0xa0, 0x61, 0xd6, 0x62, 0xf5, 0x80, 0xff,
    0x4d, 0xe4, 0x3b, 0x49, 0xfa, 0x82, 0xd8, 0x0a, 0x4b, 0x80, 0xf8, 0x43, 0x4a,
]);
pub const RLP_NULL: H256 = H256([
    0xbc, 0x20, 0x71, 0xa4, 0xde, 0x84, 0x6f, 0x28, 0x57, 0x02, 0x44, 0x7f, 0x25, 0x89, 0xdd, 0x16, 0x36, 0x78, 0xe0,
    0x97, 0x2a, 0x8a, 0x1b, 0x0d, 0x28, 0xb0, 0x4e, 0xd5, 0xc0, 0x94, 0x54, 0x7f,
]);

pub fn summary(data: &[u8]) -> Vec<u8> {
    sha3::Sha3_256::digest(data).to_vec()
}

pub fn encodek(data: &[u8]) -> Vec<u8> {
    data.to_vec()
}

pub fn encodev<E: rlp::Encodable>(object: &E) -> Vec<u8> {
    rlp::encode(object)
}

pub fn decodev(data: &[u8]) -> Result<Vec<u8>, err::Error> {
    Ok(rlp::decode(&data)?)
}
