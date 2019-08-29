//! ChaCha20Poly1305 and XChaCha20Poly1305 tests

macro_rules! impl_tests {
    ($cipher:ty, $key:expr, $nonce:expr, $aad:expr, $plaintext:expr, $ciphertext:expr, $tag:expr) => {
        #[test]
        fn encrypt() {
            let key = GenericArray::from_slice($key);
            let nonce = GenericArray::from_slice($nonce);
            let payload = chacha20poly1305::aead::Payload {
                msg: $plaintext,
                aad: $aad,
            };

            let cipher = <$cipher>::new(*key);
            let ciphertext = cipher.encrypt(nonce, payload).unwrap();

            let tag_begins = ciphertext.len() - 16;
            assert_eq!($ciphertext, &ciphertext[..tag_begins]);
            assert_eq!($tag, &ciphertext[tag_begins..]);
        }

        #[test]
        fn decrypt() {
            let key = GenericArray::from_slice($key);
            let nonce = GenericArray::from_slice($nonce);

            let mut ciphertext = Vec::from($ciphertext);
            ciphertext.extend_from_slice($tag);
            let payload = chacha20poly1305::aead::Payload {
                msg: &ciphertext,
                aad: $aad,
            };

            let cipher = <$cipher>::new(*key);
            let plaintext = cipher.decrypt(nonce, payload).unwrap();

            assert_eq!($plaintext, plaintext.as_slice());
        }

        #[test]
        fn decrypt_modified() {
            let key = GenericArray::from_slice($key);
            let nonce = GenericArray::from_slice($nonce);

            let mut ciphertext = Vec::from($ciphertext);
            ciphertext.extend_from_slice($tag);

            // Tweak the first byte
            ciphertext[0] ^= 0xaa;

            let payload = chacha20poly1305::aead::Payload {
                msg: &ciphertext,
                aad: $aad,
            };

            let cipher = <$cipher>::new(*key);
            assert!(cipher.decrypt(nonce, payload).is_err());
        }
    };
}

//
// Test vectors common to RFC 8439 and `draft-arciszewski-xchacha`
//

const KEY: &[u8; 32] = &[
    0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f,
    0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f,
];

const AAD: &[u8; 12] = &[
    0x50, 0x51, 0x52, 0x53, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7,
];

const PLAINTEXT: &[u8] = b"Ladies and Gentlemen of the class of '99: \
    If I could offer you only one tip for the future, sunscreen would be it.";

/// ChaCha20Poly1305 test vectors.
///
/// From RFC 8439 Section 2.8.2:
/// <https://tools.ietf.org/html/rfc8439#section-2.8.2>
mod chacha20 {
    use super::{AAD, KEY, PLAINTEXT};
    use chacha20poly1305::aead::generic_array::GenericArray;
    use chacha20poly1305::aead::{Aead, NewAead};
    use chacha20poly1305::ChaCha20Poly1305;

    const NONCE: &[u8; 12] = &[
        0x07, 0x00, 0x00, 0x00, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47,
    ];

    const CIPHERTEXT: &[u8] = &[
        0xd3, 0x1a, 0x8d, 0x34, 0x64, 0x8e, 0x60, 0xdb, 0x7b, 0x86, 0xaf, 0xbc, 0x53, 0xef, 0x7e,
        0xc2, 0xa4, 0xad, 0xed, 0x51, 0x29, 0x6e, 0x08, 0xfe, 0xa9, 0xe2, 0xb5, 0xa7, 0x36, 0xee,
        0x62, 0xd6, 0x3d, 0xbe, 0xa4, 0x5e, 0x8c, 0xa9, 0x67, 0x12, 0x82, 0xfa, 0xfb, 0x69, 0xda,
        0x92, 0x72, 0x8b, 0x1a, 0x71, 0xde, 0x0a, 0x9e, 0x06, 0x0b, 0x29, 0x05, 0xd6, 0xa5, 0xb6,
        0x7e, 0xcd, 0x3b, 0x36, 0x92, 0xdd, 0xbd, 0x7f, 0x2d, 0x77, 0x8b, 0x8c, 0x98, 0x03, 0xae,
        0xe3, 0x28, 0x09, 0x1b, 0x58, 0xfa, 0xb3, 0x24, 0xe4, 0xfa, 0xd6, 0x75, 0x94, 0x55, 0x85,
        0x80, 0x8b, 0x48, 0x31, 0xd7, 0xbc, 0x3f, 0xf4, 0xde, 0xf0, 0x8e, 0x4b, 0x7a, 0x9d, 0xe5,
        0x76, 0xd2, 0x65, 0x86, 0xce, 0xc6, 0x4b, 0x61, 0x16,
    ];

    const TAG: &[u8] = &[
        0x1a, 0xe1, 0x0b, 0x59, 0x4f, 0x09, 0xe2, 0x6a, 0x7e, 0x90, 0x2e, 0xcb, 0xd0, 0x60, 0x06,
        0x91,
    ];

    impl_tests!(
        ChaCha20Poly1305,
        KEY,
        NONCE,
        AAD,
        PLAINTEXT,
        CIPHERTEXT,
        TAG
    );
}

/// XChaCha20Poly1305 test vectors.
///
/// From <https://tools.ietf.org/html/draft-arciszewski-xchacha-03#appendix-A.1>
mod xchacha20 {
    use super::{AAD, KEY, PLAINTEXT};
    use chacha20poly1305::aead::generic_array::GenericArray;
    use chacha20poly1305::aead::{Aead, NewAead};
    use chacha20poly1305::XChaCha20Poly1305;

    const NONCE: &[u8; 24] = &[
        0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e,
        0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57,
    ];

    const CIPHERTEXT: &[u8] = &[
        0xbd, 0x6d, 0x17, 0x9d, 0x3e, 0x83, 0xd4, 0x3b, 0x95, 0x76, 0x57, 0x94, 0x93, 0xc0, 0xe9,
        0x39, 0x57, 0x2a, 0x17, 0x00, 0x25, 0x2b, 0xfa, 0xcc, 0xbe, 0xd2, 0x90, 0x2c, 0x21, 0x39,
        0x6c, 0xbb, 0x73, 0x1c, 0x7f, 0x1b, 0x0b, 0x4a, 0xa6, 0x44, 0x0b, 0xf3, 0xa8, 0x2f, 0x4e,
        0xda, 0x7e, 0x39, 0xae, 0x64, 0xc6, 0x70, 0x8c, 0x54, 0xc2, 0x16, 0xcb, 0x96, 0xb7, 0x2e,
        0x12, 0x13, 0xb4, 0x52, 0x2f, 0x8c, 0x9b, 0xa4, 0x0d, 0xb5, 0xd9, 0x45, 0xb1, 0x1b, 0x69,
        0xb9, 0x82, 0xc1, 0xbb, 0x9e, 0x3f, 0x3f, 0xac, 0x2b, 0xc3, 0x69, 0x48, 0x8f, 0x76, 0xb2,
        0x38, 0x35, 0x65, 0xd3, 0xff, 0xf9, 0x21, 0xf9, 0x66, 0x4c, 0x97, 0x63, 0x7d, 0xa9, 0x76,
        0x88, 0x12, 0xf6, 0x15, 0xc6, 0x8b, 0x13, 0xb5, 0x2e,
    ];

    const TAG: &[u8] = &[
        0xc0, 0x87, 0x59, 0x24, 0xc1, 0xc7, 0x98, 0x79, 0x47, 0xde, 0xaf, 0xd8, 0x78, 0x0a, 0xcf,
        0x49,
    ];

    impl_tests!(
        XChaCha20Poly1305,
        KEY,
        NONCE,
        AAD,
        PLAINTEXT,
        CIPHERTEXT,
        TAG
    );
}
