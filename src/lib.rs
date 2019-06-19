#[cfg(not(target_arch = "wasm32"))]
pub use self::regular::*;

#[cfg(not(target_arch = "wasm32"))]
mod regular {
    pub use secp256k1_c::*;
}

#[cfg(target_arch = "wasm32")]
pub use self::pure_rust::*;

#[cfg(target_arch = "wasm32")]
mod pure_rust {
    pub mod constants {
        /// The size (in bytes) of a message
        pub const MESSAGE_SIZE: usize = secp256k1_r::util::MESSAGE_SIZE;

        /// The size (in bytes) of a secret key
        pub const SECRET_KEY_SIZE: usize = secp256k1_r::util::SECRET_KEY_SIZE;

        /// The size (in bytes) of a serialized public key.
        pub const PUBLIC_KEY_SIZE: usize = secp256k1_r::util::COMPRESSED_PUBLIC_KEY_SIZE;

        /// The size (in bytes) of an serialized uncompressed public key
        pub const UNCOMPRESSED_PUBLIC_KEY_SIZE: usize = secp256k1_r::util::FULL_PUBLIC_KEY_SIZE;

        /// The maximum size of a signature
        pub const MAX_SIGNATURE_SIZE: usize = secp256k1_r::util::DER_MAX_SIGNATURE_SIZE;

        /// The maximum size of a compact signature
        pub const COMPACT_SIGNATURE_SIZE: usize = secp256k1_r::util::SIGNATURE_SIZE;

        /// The order of the secp256k1 curve
        pub const CURVE_ORDER: [u8; 32] = [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe,
            0xba, 0xae, 0xdc, 0xe6, 0xaf, 0x48, 0xa0, 0x3b,
            0xbf, 0xd2, 0x5e, 0x8c, 0xd0, 0x36, 0x41, 0x41
        ];

        /// The X coordinate of the generator
        pub const GENERATOR_X: [u8; 32] = [
            0x79, 0xbe, 0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac,
            0x55, 0xa0, 0x62, 0x95, 0xce, 0x87, 0x0b, 0x07,
            0x02, 0x9b, 0xfc, 0xdb, 0x2d, 0xce, 0x28, 0xd9,
            0x59, 0xf2, 0x81, 0x5b, 0x16, 0xf8, 0x17, 0x98
        ];

        /// The Y coordinate of the generator
        pub const GENERATOR_Y: [u8; 32] = [
            0x48, 0x3a, 0xda, 0x77, 0x26, 0xa3, 0xc4, 0x65,
            0x5d, 0xa4, 0xfb, 0xfc, 0x0e, 0x11, 0x08, 0xa8,
            0xfd, 0x17, 0xb4, 0x48, 0xa6, 0x85, 0x54, 0x19,
            0x9c, 0x47, 0xd0, 0x8f, 0xfb, 0x10, 0xd4, 0xb8
        ];
    }

    pub mod ecdh {
        use std::ops;
        use super::key::{SecretKey, PublicKey};

        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct SharedSecret(secp256k1_r::SharedSecret);

        impl SharedSecret {
            pub fn new(point: &PublicKey, scalar: &SecretKey) -> SharedSecret {
                SharedSecret(secp256k1_r::SharedSecret::new(&point.0, &scalar.0).unwrap())
            }
        }

        impl ops::Index<usize> for SharedSecret {
            type Output = u8;

            #[inline]
            fn index(&self, index: usize) -> &u8 {
                &self.0.as_ref()[index]
            }
        }

        impl ops::Index<ops::Range<usize>> for SharedSecret {
            type Output = [u8];

            #[inline]
            fn index(&self, index: ops::Range<usize>) -> &[u8] {
                &self.0.as_ref()[index]
            }
        }

        impl ops::Index<ops::RangeFrom<usize>> for SharedSecret {
            type Output = [u8];

            #[inline]
            fn index(&self, index: ops::RangeFrom<usize>) -> &[u8] {
                &self.0.as_ref()[index.start..]
            }
        }

        impl ops::Index<ops::RangeFull> for SharedSecret {
            type Output = [u8];

            #[inline]
            fn index(&self, _: ops::RangeFull) -> &[u8] {
                &self.0.as_ref()[..]
            }
        }

    }

    pub mod key {
        use std::{fmt, ops};

        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct SecretKey(pub(crate) secp256k1_r::SecretKey);

        impl fmt::Display for SecretKey {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                for ch in &self.0.serialize()[..] {
                    write!(f, "{:02x}", *ch)?;
                }
                Ok(())
            }
        }

        impl ops::Index<usize> for SecretKey {
            type Output = u8;

            #[inline]
            fn index(&self, index: usize) -> &u8 {
                let _ = index;
                unimplemented!()
            }
        }

        impl ops::Index<ops::Range<usize>> for SecretKey {
            type Output = [u8];

            #[inline]
            fn index(&self, index: ops::Range<usize>) -> &[u8] {
                let _ = index;
                unimplemented!()
            }
        }

        impl ops::Index<ops::RangeFrom<usize>> for SecretKey {
            type Output = [u8];

            #[inline]
            fn index(&self, index: ops::RangeFrom<usize>) -> &[u8] {
                let _ = index;
                unimplemented!()
            }
        }

        impl ops::Index<ops::RangeTo<usize>> for SecretKey {
            type Output = [u8];

            #[inline]
            fn index(&self, index: ops::RangeTo<usize>) -> &[u8] {
                let _ = index;
                unimplemented!()
            }
        }

        impl ops::Index<ops::RangeFull> for SecretKey {
            type Output = [u8];

            #[inline]
            fn index(&self, _: ops::RangeFull) -> &[u8] {
                unimplemented!()
            }
        }

        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct PublicKey(pub(crate) secp256k1_r::PublicKey);

        impl fmt::Display for PublicKey {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                for ch in &self.0.serialize()[..] {
                    write!(f, "{:02x}", *ch)?;
                }
                Ok(())
            }
        }
    }
}
