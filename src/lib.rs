//! Fast cryptographic accumulator and vector commitment library, originally written by Cambrian
//! Technologies [\[GitHub\]](https://github.com/cambrian/accumulator).
//!
//! **Disclaimer**: This library is intended to be production-quality code, but it has not been
//! independently-audited for correctness or tested to a critical degree. As such, please treat this
//! library as **research-grade** for the time being.
//!
//! # Important Note
//!
//! To ensure correspondence between accumulator methods and logical set operations in your
//! application, you must ensure that **no element is accumulated twice**. In particular, deleting
//! a doubly-accumulated element will remove only one "copy" of it from the accumulator, meaning
//! that its membership can still be verified. Hence, an accumulator without this invariant can be
//! viewed as a multiset.
//!
//! # What is an accumulator?
//!
//! An accumulator is a cryptographic primitive which functions essentially as a secure
//! decentralized set. It allows parties to maintain consensus on a set of values via a
//! _succinct binding commitment_ as well as to issue _efficiently verifiable (non)membership
//! proofs_ for elements of interest, all without requiring any party to store the entire set.
//!
//! Similarly to a Merkle tree, the accumulator stores its state commitment in constant space. A
//! notable difference, however, is that its inclusion and exclusion proofs also take up constant
//! space, and can be verified in constant time. For a far more detailed discussion of accumulators
//! as implemented here, see _Batching Techniques for Accumulators with Applications to IOPs and
//! Stateless Blockchains_ (Boneh, Bünz, and Fisch 2018)
//! [\[Link\]](https://eprint.iacr.org/2018/1188.pdf).
//!
//! Throughout our code, we refer to this paper as `BBF`. We also refer to another paper, _Universal
//! Accumulators with Efficient Nonmembership Proofs_ (Li, Li, Xue 2007)
//! [\[Link\]](https://link.springer.com/content/pdf/10.1007/978-3-540-72738-5_17.pdf), abbreviated
//! henceforth as `LLX`.
//!
//! # What is a vector commitment?
//!
//! A vector commitment (VC) is a closely-related primitive, distinguished from an accumulator in
//! that it provides a _position-binding_ commitment to state. That is, a VC allows parties to
//! prove or disprove that a certain element exists at a certain position.
//!
//! (Think VC : Vector :: Accumulator : Set.)
//!
//! Our vector commitment implementation is a work-in-progress (WIP), and should be treated with
//! even more skepticism than our accumulators.
//!
//! # Usage
//! ```
//! // A very basic example.
//! use accumulator::Accumulator;
//! use accumulator::group::Rsa2048;
//!
//! let acc = Accumulator::<Rsa2048, &'static str>::empty();
//!
//! // Accumulate "dog" and "cat". The `add_with_proof` method returns the new accumulator state
//! // and a proof that you accumulated "dog" and "cat".
//! let (acc, proof) = acc.add_with_proof(&["dog", "cat"]);
//!
//! // A network participant who sees (acc, proof, and ["dog", "cat"]) can verify that the update
//! // was formed correctly ...
//! assert!(acc.verify_membership_batch(&["dog", "cat"], &proof));
//!
//! // ... and trying to verify something that has not been accumulated will fail.
//! assert!(!acc.verify_membership(&"cow", &proof));
//! ```
//!
//! Typical users of this library will access public-facing routines on `accumulator` and
//! `vector_commitment`. However, we also export internal modules for useful traits, types (such as
//! the `Rsa2048` group), and specialized procedures. **Use internal components at your own risk**.
//!
//! You can find a more interesting application of our library
//! [here](https://github.com/cambrian/accumulator-demo), where we create a proof-of-concept for
//! stateless Bitcoin nodes!
//!
//! # Groups
//!
//! Accumulator and vector commitment operations take place over algebraic groups with certain
//! cryptographic properties. We provide implementations for two suitable groups:
//! (1) an RSA group with the [RSA-2048 modulus](https://en.wikipedia.org/wiki/RSA_numbers#RSA-2048)
//! and (2) an ideal class group with a fixed discriminant generated by OpenSSL.
//!
//! The RSA group is fast but relies on the security of the RSA-2048 modulus and needs trusted
//! setup if using a different modulus. The class group is slower but eliminates the need for a
//! trusted setup. For more on class groups, please visit this
//! [thorough explainer](https://www.michaelstraka.com/posts/classgroups/) by contributor Michael
//! Straka.
//!
//! # Performance
//!
//! Most accumulator or vector commitment functions will bottleneck in hashing to large primes. To
//! alleviate this, we created a zero-allocation `U256` type that uses the low-level `mpn_`
//! functions in [GMP](https://gmplib.org). Our `hash_to_prime` uses this type internally.
//!
//! Class groups are currently not performant for any meaningful use case. A pull request is in the
//! works to drastically improve their performance using techniques learned from the
//! [Chia VDF competition](https://github.com/Chia-Network/vdf-competition).
#![allow(clippy::unknown_clippy_lints)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::empty_enum)]
#![warn(missing_docs)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate arrayref;

mod accumulator;
pub use crate::accumulator::*;
mod vector_commitment;
pub use vector_commitment::*;

pub mod group;
pub mod hash;
pub mod proof;
#[allow(missing_docs)]
pub mod uint;
pub mod util;
