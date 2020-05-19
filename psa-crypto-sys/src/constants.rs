// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Constants used by the Mbed Provider for interaction with the Mbed Crypto C library.

#![allow(missing_docs)]

use super::psa_crypto_binding::*;

// xx There are not part of the public PSA API and should be removed:
pub const PSA_KEY_SLOT_COUNT: isize = 32;
pub const PSA_MAX_PERSISTENT_KEY_IDENTIFIER: psa_key_id_t = 0x3fff_ffff;

// PSA error codes
pub const PSA_SUCCESS: psa_status_t = shim_PSA_SUCCESS;
pub const PSA_ERROR_GENERIC_ERROR: psa_status_t = shim_PSA_ERROR_GENERIC_ERROR;
pub const PSA_ERROR_NOT_SUPPORTED: psa_status_t = shim_PSA_ERROR_NOT_SUPPORTED;
pub const PSA_ERROR_NOT_PERMITTED: psa_status_t = shim_PSA_ERROR_NOT_PERMITTED;
pub const PSA_ERROR_BUFFER_TOO_SMALL: psa_status_t = shim_PSA_ERROR_BUFFER_TOO_SMALL;
pub const PSA_ERROR_ALREADY_EXISTS: psa_status_t = shim_PSA_ERROR_ALREADY_EXISTS;
pub const PSA_ERROR_DOES_NOT_EXIST: psa_status_t = shim_PSA_ERROR_DOES_NOT_EXIST;
pub const PSA_ERROR_BAD_STATE: psa_status_t = shim_PSA_ERROR_BAD_STATE;
pub const PSA_ERROR_INVALID_ARGUMENT: psa_status_t = shim_PSA_ERROR_INVALID_ARGUMENT;
pub const PSA_ERROR_INSUFFICIENT_MEMORY: psa_status_t = shim_PSA_ERROR_INSUFFICIENT_MEMORY;
pub const PSA_ERROR_INSUFFICIENT_STORAGE: psa_status_t = shim_PSA_ERROR_INSUFFICIENT_STORAGE;
pub const PSA_ERROR_COMMUNICATION_FAILURE: psa_status_t = shim_PSA_ERROR_COMMUNICATION_FAILURE;
pub const PSA_ERROR_STORAGE_FAILURE: psa_status_t = shim_PSA_ERROR_STORAGE_FAILURE;
pub const PSA_ERROR_HARDWARE_FAILURE: psa_status_t = shim_PSA_ERROR_HARDWARE_FAILURE;
pub const PSA_ERROR_INSUFFICIENT_ENTROPY: psa_status_t = shim_PSA_ERROR_INSUFFICIENT_ENTROPY;
pub const PSA_ERROR_INVALID_SIGNATURE: psa_status_t = shim_PSA_ERROR_INVALID_SIGNATURE;
pub const PSA_ERROR_INVALID_PADDING: psa_status_t = shim_PSA_ERROR_INVALID_PADDING;
pub const PSA_ERROR_INSUFFICIENT_DATA: psa_status_t = shim_PSA_ERROR_INSUFFICIENT_DATA;
pub const PSA_ERROR_INVALID_HANDLE: psa_status_t = shim_PSA_ERROR_INVALID_HANDLE;

pub const PSA_MAX_KEY_BITS: usize = shim_PSA_MAX_KEY_BITS;
pub const PSA_KEY_BITS_TOO_LARGE: psa_key_bits_t = shim_PSA_KEY_BITS_TOO_LARGE;
pub const PSA_KEY_TYPE_NONE: psa_key_type_t = shim_PSA_KEY_TYPE_NONE;
pub const PSA_KEY_TYPE_VENDOR_FLAG: psa_key_type_t = shim_PSA_KEY_TYPE_VENDOR_FLAG;
pub const PSA_KEY_TYPE_CATEGORY_MASK: psa_key_type_t = shim_PSA_KEY_TYPE_CATEGORY_MASK;
pub const PSA_KEY_TYPE_CATEGORY_SYMMETRIC: psa_key_type_t = shim_PSA_KEY_TYPE_CATEGORY_SYMMETRIC;
pub const PSA_KEY_TYPE_CATEGORY_RAW: psa_key_type_t = shim_PSA_KEY_TYPE_CATEGORY_RAW;
pub const PSA_KEY_TYPE_CATEGORY_PUBLIC_KEY: psa_key_type_t = shim_PSA_KEY_TYPE_CATEGORY_PUBLIC_KEY;
pub const PSA_KEY_TYPE_CATEGORY_KEY_PAIR: psa_key_type_t = shim_PSA_KEY_TYPE_CATEGORY_KEY_PAIR;
pub const PSA_KEY_TYPE_CATEGORY_FLAG_PAIR: psa_key_type_t = shim_PSA_KEY_TYPE_CATEGORY_FLAG_PAIR;
pub const PSA_KEY_TYPE_RAW_DATA: psa_key_type_t = shim_PSA_KEY_TYPE_RAW_DATA;
pub const PSA_KEY_TYPE_HMAC: psa_key_type_t = shim_PSA_KEY_TYPE_HMAC;
pub const PSA_KEY_TYPE_DERIVE: psa_key_type_t = shim_PSA_KEY_TYPE_DERIVE;
pub const PSA_KEY_TYPE_AES: psa_key_type_t = shim_PSA_KEY_TYPE_AES;
pub const PSA_KEY_TYPE_DES: psa_key_type_t = shim_PSA_KEY_TYPE_DES;
pub const PSA_KEY_TYPE_CAMELLIA: psa_key_type_t = shim_PSA_KEY_TYPE_CAMELLIA;
pub const PSA_KEY_TYPE_ARC4: psa_key_type_t = shim_PSA_KEY_TYPE_ARC4;
pub const PSA_KEY_TYPE_CHACHA20: psa_key_type_t = shim_PSA_KEY_TYPE_CHACHA20;
pub const PSA_KEY_TYPE_RSA_PUBLIC_KEY: psa_key_type_t = shim_PSA_KEY_TYPE_RSA_PUBLIC_KEY;
pub const PSA_KEY_TYPE_RSA_KEY_PAIR: psa_key_type_t = shim_PSA_KEY_TYPE_RSA_KEY_PAIR;
pub const PSA_KEY_TYPE_DSA_PUBLIC_KEY: psa_key_type_t = shim_PSA_KEY_TYPE_DSA_PUBLIC_KEY;
pub const PSA_KEY_TYPE_ECC_PUBLIC_KEY_BASE: psa_key_type_t = shim_PSA_KEY_TYPE_ECC_PUBLIC_KEY_BASE;
pub const PSA_KEY_TYPE_ECC_KEY_PAIR_BASE: psa_key_type_t = shim_PSA_KEY_TYPE_ECC_KEY_PAIR_BASE;
pub const PSA_KEY_TYPE_ECC_CURVE_MASK: psa_key_type_t = shim_PSA_KEY_TYPE_ECC_CURVE_MASK;
pub const PSA_ECC_CURVE_SECP_K1: psa_ecc_curve_t = shim_PSA_ECC_CURVE_SECP_K1;
pub const PSA_ECC_CURVE_SECP_R1: psa_ecc_curve_t = shim_PSA_ECC_CURVE_SECP_R1;
pub const PSA_ECC_CURVE_SECP_R2: psa_ecc_curve_t = shim_PSA_ECC_CURVE_SECP_R2;
pub const PSA_ECC_CURVE_SECT_K1: psa_ecc_curve_t = shim_PSA_ECC_CURVE_SECT_K1;
pub const PSA_ECC_CURVE_SECT_R1: psa_ecc_curve_t = shim_PSA_ECC_CURVE_SECT_R1;
pub const PSA_ECC_CURVE_SECT_R2: psa_ecc_curve_t = shim_PSA_ECC_CURVE_SECT_R2;
pub const PSA_ECC_CURVE_BRAINPOOL_P_R1: psa_ecc_curve_t = shim_PSA_ECC_CURVE_BRAINPOOL_P_R1;
pub const PSA_ECC_CURVE_MONTGOMERY: psa_ecc_curve_t = shim_PSA_ECC_CURVE_MONTGOMERY;
pub const PSA_DH_GROUP_RFC7919: psa_dh_group_t = shim_PSA_DH_GROUP_RFC7919;
pub const PSA_ALG_VENDOR_FLAG: psa_algorithm_t = shim_PSA_ALG_VENDOR_FLAG;
pub const PSA_ALG_CATEGORY_MASK: psa_algorithm_t = shim_PSA_ALG_CATEGORY_MASK;
pub const PSA_ALG_CATEGORY_HASH: psa_algorithm_t = shim_PSA_ALG_CATEGORY_HASH;
pub const PSA_ALG_CATEGORY_MAC: psa_algorithm_t = shim_PSA_ALG_CATEGORY_MAC;
pub const PSA_ALG_CATEGORY_CIPHER: psa_algorithm_t = shim_PSA_ALG_CATEGORY_CIPHER;
pub const PSA_ALG_CATEGORY_AEAD: psa_algorithm_t = shim_PSA_ALG_CATEGORY_AEAD;
pub const PSA_ALG_CATEGORY_SIGN: psa_algorithm_t = shim_PSA_ALG_CATEGORY_SIGN;
pub const PSA_ALG_CATEGORY_ASYMMETRIC_ENCRYPTION: psa_algorithm_t =
    shim_PSA_ALG_CATEGORY_ASYMMETRIC_ENCRYPTION;
pub const PSA_ALG_CATEGORY_KEY_AGREEMENT: psa_algorithm_t = shim_PSA_ALG_CATEGORY_KEY_AGREEMENT;
pub const PSA_ALG_CATEGORY_KEY_DERIVATION: psa_algorithm_t = shim_PSA_ALG_CATEGORY_KEY_DERIVATION;
pub const PSA_ALG_HASH_MASK: psa_algorithm_t = shim_PSA_ALG_HASH_MASK;
pub const PSA_ALG_MD2: psa_algorithm_t = shim_PSA_ALG_MD2;
pub const PSA_ALG_MD4: psa_algorithm_t = shim_PSA_ALG_MD4;
pub const PSA_ALG_MD5: psa_algorithm_t = shim_PSA_ALG_MD5;
pub const PSA_ALG_RIPEMD160: psa_algorithm_t = shim_PSA_ALG_RIPEMD160;
pub const PSA_ALG_SHA_1: psa_algorithm_t = shim_PSA_ALG_SHA_1;
pub const PSA_ALG_SHA_224: psa_algorithm_t = shim_PSA_ALG_SHA_224;
pub const PSA_ALG_SHA_256: psa_algorithm_t = shim_PSA_ALG_SHA_256;
pub const PSA_ALG_SHA_384: psa_algorithm_t = shim_PSA_ALG_SHA_384;
pub const PSA_ALG_SHA_512: psa_algorithm_t = shim_PSA_ALG_SHA_512;
pub const PSA_ALG_SHA_512_224: psa_algorithm_t = shim_PSA_ALG_SHA_512_224;
pub const PSA_ALG_SHA_512_256: psa_algorithm_t = shim_PSA_ALG_SHA_512_256;
pub const PSA_ALG_SHA3_224: psa_algorithm_t = shim_PSA_ALG_SHA3_224;
pub const PSA_ALG_SHA3_256: psa_algorithm_t = shim_PSA_ALG_SHA3_256;
pub const PSA_ALG_SHA3_384: psa_algorithm_t = shim_PSA_ALG_SHA3_384;
pub const PSA_ALG_SHA3_512: psa_algorithm_t = shim_PSA_ALG_SHA3_512;
pub const PSA_ALG_ANY_HASH: psa_algorithm_t = shim_PSA_ALG_ANY_HASH;
pub const PSA_ALG_MAC_SUBCATEGORY_MASK: psa_algorithm_t = shim_PSA_ALG_MAC_SUBCATEGORY_MASK;
pub const PSA_ALG_HMAC_BASE: psa_algorithm_t = shim_PSA_ALG_HMAC_BASE;
pub const PSA_ALG_MAC_TRUNCATION_MASK: psa_algorithm_t = shim_PSA_ALG_MAC_TRUNCATION_MASK;
pub const PSA_ALG_CIPHER_MAC_BASE: psa_algorithm_t = shim_PSA_ALG_CIPHER_MAC_BASE;
pub const PSA_ALG_CBC_MAC: psa_algorithm_t = shim_PSA_ALG_CBC_MAC;
pub const PSA_ALG_CMAC: psa_algorithm_t = shim_PSA_ALG_CMAC;
pub const PSA_ALG_CIPHER_STREAM_FLAG: psa_algorithm_t = shim_PSA_ALG_CIPHER_STREAM_FLAG;
pub const PSA_ALG_CIPHER_FROM_BLOCK_FLAG: psa_algorithm_t = shim_PSA_ALG_CIPHER_FROM_BLOCK_FLAG;
pub const PSA_ALG_ARC4: psa_algorithm_t = shim_PSA_ALG_ARC4;
pub const PSA_ALG_CTR: psa_algorithm_t = shim_PSA_ALG_CTR;
pub const PSA_ALG_CFB: psa_algorithm_t = shim_PSA_ALG_CFB;
pub const PSA_ALG_OFB: psa_algorithm_t = shim_PSA_ALG_OFB;
pub const PSA_ALG_XTS: psa_algorithm_t = shim_PSA_ALG_XTS;
pub const PSA_ALG_CBC_NO_PADDING: psa_algorithm_t = shim_PSA_ALG_CBC_NO_PADDING;
pub const PSA_ALG_CBC_PKCS7: psa_algorithm_t = shim_PSA_ALG_CBC_PKCS7;
pub const PSA_ALG_CCM: psa_algorithm_t = shim_PSA_ALG_CCM;
pub const PSA_ALG_GCM: psa_algorithm_t = shim_PSA_ALG_GCM;
pub const PSA_ALG_AEAD_TAG_LENGTH_MASK: psa_algorithm_t = shim_PSA_ALG_AEAD_TAG_LENGTH_MASK;
pub const PSA_ALG_RSA_PKCS1V15_SIGN_BASE: psa_algorithm_t = shim_PSA_ALG_RSA_PKCS1V15_SIGN_BASE;
pub const PSA_ALG_RSA_PKCS1V15_SIGN_RAW: psa_algorithm_t = shim_PSA_ALG_RSA_PKCS1V15_SIGN_RAW;
pub const PSA_ALG_RSA_PSS_BASE: psa_algorithm_t = shim_PSA_ALG_RSA_PSS_BASE;
pub const PSA_ALG_DSA_BASE: psa_algorithm_t = shim_PSA_ALG_DSA_BASE;
pub const PSA_ALG_DETERMINISTIC_DSA_BASE: psa_algorithm_t = shim_PSA_ALG_DETERMINISTIC_DSA_BASE;
pub const PSA_ALG_DSA_DETERMINISTIC_FLAG: psa_algorithm_t = shim_PSA_ALG_DSA_DETERMINISTIC_FLAG;
pub const PSA_ALG_ECDSA_BASE: psa_algorithm_t = shim_PSA_ALG_ECDSA_BASE;
pub const PSA_ALG_ECDSA_ANY: psa_algorithm_t = shim_PSA_ALG_ECDSA_ANY;
pub const PSA_ALG_DETERMINISTIC_ECDSA_BASE: psa_algorithm_t = shim_PSA_ALG_DETERMINISTIC_ECDSA_BASE;
pub const PSA_ALG_RSA_PKCS1V15_CRYPT: psa_algorithm_t = shim_PSA_ALG_RSA_PKCS1V15_CRYPT;
pub const PSA_ALG_RSA_OAEP_BASE: psa_algorithm_t = shim_PSA_ALG_RSA_OAEP_BASE;
pub const PSA_ALG_HKDF_BASE: psa_algorithm_t = shim_PSA_ALG_HKDF_BASE;
pub const PSA_ALG_TLS12_PRF_BASE: psa_algorithm_t = shim_PSA_ALG_TLS12_PRF_BASE;
pub const PSA_ALG_TLS12_PSK_TO_MS_BASE: psa_algorithm_t = shim_PSA_ALG_TLS12_PSK_TO_MS_BASE;
pub const PSA_ALG_KEY_DERIVATION_MASK: psa_algorithm_t = shim_PSA_ALG_KEY_DERIVATION_MASK;
pub const PSA_KEY_LIFETIME_VOLATILE: psa_key_lifetime_t = shim_PSA_KEY_LIFETIME_VOLATILE;
pub const PSA_KEY_LIFETIME_PERSISTENT: psa_key_lifetime_t = shim_PSA_KEY_LIFETIME_PERSISTENT;
pub const PSA_KEY_USAGE_EXPORT: psa_key_usage_t = shim_PSA_KEY_USAGE_EXPORT;
pub const PSA_KEY_USAGE_ENCRYPT: psa_key_usage_t = shim_PSA_KEY_USAGE_ENCRYPT;
pub const PSA_KEY_USAGE_DECRYPT: psa_key_usage_t = shim_PSA_KEY_USAGE_DECRYPT;
pub const PSA_KEY_USAGE_SIGN: psa_key_usage_t = shim_PSA_KEY_USAGE_SIGN;
pub const PSA_KEY_USAGE_VERIFY: psa_key_usage_t = shim_PSA_KEY_USAGE_VERIFY;
pub const PSA_KEY_USAGE_DERIVE: psa_key_usage_t = shim_PSA_KEY_USAGE_DERIVE;
