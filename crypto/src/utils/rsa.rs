use md4::Md4;
use md5::Md5;
use rand::rngs::OsRng;
use ripemd160::Ripemd160;
use ripemd256::Ripemd256;
use ripemd320::Ripemd320;
use rsa::{
    errors,
    pkcs1::{FromRsaPrivateKey, FromRsaPublicKey},
    pkcs8::{FromPrivateKey, FromPublicKey, ToPublicKey},
    BigUint, Hash, PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey,
};
use sha1::Sha1;
use sha2::*;
use sha3::*;
use thiserror::Error;

use super::hash::{make_hash, HashError};

#[derive(Error, Debug)]
pub enum RsaError {
    #[error("invalid public key")]
    InvalidPublicKey,
    #[error("invalid private key")]
    InvalidPrivateKey,
    #[error("sign mode unsupported yet: `{0}`")]
    InvalidSignMode(String),
    #[error(transparent)]
    InvalidHashType(#[from] HashError),
    #[error(transparent)]
    Error(#[from] errors::Error),
}

pub fn modulus_to_pem(n: Vec<u8>, e: Vec<u8>) -> Result<String, errors::Error> {
    let n = BigUint::from_bytes_be(n.as_slice());
    let e = BigUint::from_bytes_be(e.as_slice());

    let pub_key = RsaPublicKey::new(n, e)?;
    let pub_key = RsaPublicKey::to_public_key_pem(&pub_key)?;
    Ok(pub_key)
}

pub fn rsa_encrypt(data: Vec<u8>, key: String, hash_type: String) -> Result<Vec<u8>, RsaError> {
    let pub_key: RsaPublicKey;
    if key.contains("--BEGIN RSA") {
        pub_key = match RsaPublicKey::from_pkcs1_pem(key.as_str()) {
            Ok(pub_key) => pub_key,
            Err(_) => return Err(RsaError::InvalidPublicKey),
        };
    } else {
        pub_key = match RsaPublicKey::from_public_key_pem(key.as_str()) {
            Ok(pub_key) => pub_key,
            Err(_) => return Err(RsaError::InvalidPublicKey),
        };
    }

    let padding = padding_from_str(hash_type)?;
    let mut rng = OsRng;
    let encrypted = pub_key.encrypt(&mut rng, padding, &data)?;
    Ok(encrypted)
}

pub fn rsa_decrypt(data: Vec<u8>, key: String, hash_type: String) -> Result<Vec<u8>, RsaError> {
    let priv_key: RsaPrivateKey;
    if key.contains("--BEGIN RSA") {
        priv_key = match RsaPrivateKey::from_pkcs1_pem(key.as_str()) {
            Ok(priv_key) => priv_key,
            Err(_) => return Err(RsaError::InvalidPrivateKey),
        };
    } else {
        priv_key = match RsaPrivateKey::from_pkcs8_pem(key.as_str()) {
            Ok(priv_key) => priv_key,
            Err(_) => return Err(RsaError::InvalidPrivateKey),
        };
    }

    let padding = padding_from_str(hash_type)?;
    let decrypted = priv_key.decrypt(padding, &data)?;
    Ok(decrypted)
}

pub fn rsa_sign(
    data: Vec<u8>,
    key: String,
    hash_type: String,
    mode: String,
) -> Result<Vec<u8>, RsaError> {
    let priv_key: RsaPrivateKey;
    if key.contains("--BEGIN RSA") {
        priv_key = match RsaPrivateKey::from_pkcs1_pem(key.as_str()) {
            Ok(priv_key) => priv_key,
            Err(_) => return Err(RsaError::InvalidPrivateKey),
        };
    } else {
        priv_key = match RsaPrivateKey::from_pkcs8_pem(key.as_str()) {
            Ok(priv_key) => priv_key,
            Err(_) => return Err(RsaError::InvalidPrivateKey),
        };
    }

    let hashed_data = make_hash(data, &hash_type)?;

    let padding = sign_padding_from_str(hash_type, mode)?;
    let signed = priv_key.sign(padding, &hashed_data)?;
    Ok(signed)
}

fn padding_from_str(hash_type: String) -> Result<PaddingScheme, RsaError> {
    match hash_type.len() {
        0 => Ok(PaddingScheme::new_pkcs1v15_encrypt()),
        _ => {
            let padding = match hash_type.as_str() {
                "md5" => PaddingScheme::new_oaep::<Md5>(),
                "md4" => PaddingScheme::new_oaep::<Md4>(),
                "sha1" => PaddingScheme::new_oaep::<Sha1>(),
                "sha224" => PaddingScheme::new_oaep::<Sha224>(),
                "sha256" => PaddingScheme::new_oaep::<Sha256>(),
                "sha384" => PaddingScheme::new_oaep::<Sha384>(),
                "sha512" => PaddingScheme::new_oaep::<Sha512>(),
                "sha3-224" => PaddingScheme::new_oaep::<Sha3_224>(),
                "sha3-256" => PaddingScheme::new_oaep::<Sha3_256>(),
                "sha3-384" => PaddingScheme::new_oaep::<Sha3_384>(),
                "sha3-512" => PaddingScheme::new_oaep::<Sha3_512>(),
                "keccak224" => PaddingScheme::new_oaep::<Keccak224>(),
                "keccak256" => PaddingScheme::new_oaep::<Keccak256>(),
                "keccak384" => PaddingScheme::new_oaep::<Keccak384>(),
                "keccak512" => PaddingScheme::new_oaep::<Keccak512>(),
                "ripemd160" => PaddingScheme::new_oaep::<Ripemd160>(),
                "ripemd256" => PaddingScheme::new_oaep::<Ripemd256>(),
                "ripemd320" => PaddingScheme::new_oaep::<Ripemd320>(),
                _ => {
                    return Err(RsaError::InvalidHashType(HashError::InvalidHashType(
                        hash_type,
                    )))
                }
            };
            Ok(padding)
        }
    }
}

fn sign_padding_from_str(hash_type: String, mode: String) -> Result<PaddingScheme, RsaError> {
    let padding = match mode.as_str() {
        "pkcs1" => {
            let hash = match hash_type.as_str() {
                "md5" => Hash::MD5,
                "sha1" => Hash::SHA1,
                "sha224" => Hash::SHA2_224,
                "sha256" => Hash::SHA2_256,
                "sha384" => Hash::SHA2_384,
                "sha512" => Hash::SHA2_512,
                "sha3-256" => Hash::SHA3_256,
                "sha3-384" => Hash::SHA3_384,
                "sha3-512" => Hash::SHA3_512,
                "ripemd160" => Hash::RIPEMD160,
                "md5sha1" => Hash::MD5SHA1,
                _ => {
                    return Err(RsaError::InvalidHashType(HashError::InvalidHashType(
                        hash_type,
                    )))
                }
            };
            PaddingScheme::new_pkcs1v15_sign(Some(hash))
        }

        "pss" => {
            let rng = OsRng;
            // don't know how to implement with switch_hash_trait
            match hash_type.as_str() {
                "md5" => PaddingScheme::new_pss::<Md5, OsRng>(rng),
                "md4" => PaddingScheme::new_pss::<Md4, OsRng>(rng),
                "sha1" => PaddingScheme::new_pss::<Sha1, OsRng>(rng),
                "sha224" => PaddingScheme::new_pss::<Sha224, OsRng>(rng),
                "sha256" => PaddingScheme::new_pss::<Sha256, OsRng>(rng),
                "sha384" => PaddingScheme::new_pss::<Sha384, OsRng>(rng),
                "sha512" => PaddingScheme::new_pss::<Sha512, OsRng>(rng),
                "sha3-224" => PaddingScheme::new_pss::<Sha3_224, OsRng>(rng),
                "sha3-256" => PaddingScheme::new_pss::<Sha3_256, OsRng>(rng),
                "sha3-384" => PaddingScheme::new_pss::<Sha3_384, OsRng>(rng),
                "sha3-512" => PaddingScheme::new_pss::<Sha3_512, OsRng>(rng),
                "keccak224" => PaddingScheme::new_pss::<Keccak224, OsRng>(rng),
                "keccak256" => PaddingScheme::new_pss::<Keccak256, OsRng>(rng),
                "keccak384" => PaddingScheme::new_pss::<Keccak384, OsRng>(rng),
                "keccak512" => PaddingScheme::new_pss::<Keccak512, OsRng>(rng),
                "ripemd160" => PaddingScheme::new_pss::<Ripemd160, OsRng>(rng),
                "ripemd256" => PaddingScheme::new_pss::<Ripemd256, OsRng>(rng),
                "ripemd320" => PaddingScheme::new_pss::<Ripemd320, OsRng>(rng),
                _ => {
                    return Err(RsaError::InvalidHashType(HashError::InvalidHashType(
                        hash_type,
                    )))
                }
            }
        }

        _ => return Err(RsaError::InvalidSignMode(mode)),
    };

    Ok(padding)
}
