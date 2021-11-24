//! A small random number generator hacked on top of Rust's standard library.
//!
//! ### Origins
//! Rust, as you may well know, has support for `HashMap`s.
//! These maps, as per their name, require hashing.
//! Hashing, of course, takes some data, and produces a random representation of that data.
//! Where we hash, random numbers follow!
//!
//! > "Well, it seemed like a good idea at the time..."
//! >
//! > — You explaining `attorand` to your smart contract shareholders, probably.
//!
//! ### Why `attorand`?
//! Other crates, like `rand`, can be hard to use cross-platform,
//! Because they rely on the system's source of randomness.
//! `attorand`, however, does not have this limitation,
//! and provides a firehose of random bytes to drink from.
//!
//! ### How does it work?
//! Internally, `attorand` works by repeatedly hashing a seed,
//! using the output of the last round as the input to the next.
//! The algorithm used is SipHash 1-3, as per Rust's
//! [standard library docs](https://doc.rust-lang.org/std/collections/struct.HashMap.html),
//! but note that this is "subject to change at any point in the future".
//!
//! ### Addendum
//! Please don't use this for anything that needs cryptographic randomness.
//! (You knew that.)
//!
//! I hold no further responsibility for this abomination.
//!
//! Welp, That's it from me. What are you waiting for?
//! Allocate a fresh [`Rng`] to get started!

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

/// The default seed for the [`Rng`] if no other seed is specified.
/// There is nothing special about this number.
/// Please don't try to figure out why this number is special.
pub const RNG_SEED: u64 = 0x_6865_636B_7965_6168;

/// A portable random number generator built on Rust's default `HashMap` hasher.
/// Has no dependencies other than the standard library (doh).
/// This random number generator is not cryptographically secure, not the fastest, yada yada yada,
/// so don't use it unless you have a really good reason to.
pub struct Rng {
    data:   u64,
    hasher: DefaultHasher,
}

impl Rng {
    /// Creates a new pseudorandom number, using the default seed.
    /// Will produce the same series of numbers every time.
    /// Use [`Rng::new_with_seed`] to spice things up a little!
    pub fn new_default() -> Rng { Rng::new_with_seed(RNG_SEED) }

    /// Create a new random number generator for a given seed.
    /// Don't overthink it, you can just use the current iteration of a loop as a seed
    /// or something to get a new random number generator on each iteration.
    pub fn new_with_seed(seed: u64) -> Rng {
        Rng {
            data:   seed,
            hasher: DefaultHasher::new(),
        }
    }

    /// Return the next unsigned 64-bit natural number
    /// less than or equal to a given number.
    /// uses rejection sampling, so timing attacks are a thing.
    ///
    /// But you're not using this for crypto, right?
    ///
    /// But you're not using this for crypto... right!?
    pub fn next_u64_max(&mut self, max: u64) -> u64 {
        let mask = 2_u64.pow(64 - (max as u64).leading_zeros()) - 1;
        let mut out = self.next_u64() & mask;
        while out > max { out = self.next_u64() & mask }
        out
    }

    /// Return the next 64-bit unsigned integer.
    /// If you want to constrict the range a bit (say, before casting to a `u32`),
    /// try using [`Rng::next_u64_max`].
    pub fn next_u64(&mut self) -> u64 {
        let mut out = 0;
        for i in self.take(8) {
            out = out << 8 | i as u64;
        }
        out
    }

    /// Return a random byte.
    /// If you need more than a byte, use [`Rng::next_u64`].
    /// If you're looking for an infinite byte generator,
    /// try using this bad boy as an iterator.
    pub fn next_byte(&mut self) -> u8 {
        self.next().unwrap()
    }

    /// Return a random boolean.
    /// Super handy for choosing between a couple of possibilities.
    /// Oh, you need more than a `bool`?
    /// why not try [`Rng::next_byte`]?
    pub fn next_bool(&mut self) -> bool {
        self.next().unwrap() % 2 == 0
    }
}

impl Iterator for Rng {
    type Item = u8;

    /// Generates an infinite stream of random bytes.
    /// Used as a source for all other functions.
    ///
    /// Note how hashing gives us 8 fresh bytes,
    /// but we waste a lot of time XOR-in' them all together
    /// to produce a single byte at a time.
    /// Easy 8x speedup for version 2 of the library.
    fn next(&mut self) -> Option<u8> {
        self.hasher.write_u64(self.data);
        self.data = self.hasher.finish();
        Some(self.data.to_be_bytes().iter().fold(0, |a, b| a^b))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Just a quick sanity check for the 'less than or equal to' statement.
    /// These are all the tests a cryptographically-secure random number generator needs.
    #[test]
    fn it_works() {
        let mut rng = Rng::new_default();
        for _ in 0..100 {
            let randers = rng.next_u64_max(2);
            println!("{}", randers);
            assert!(randers <= 2);
        }
    }
}
