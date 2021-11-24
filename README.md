# attorand

> from 'atto', meaning smaller than small, and 'rand', short for random.

A small random number generator hacked on top of Rust's standard library.

### Origins
Rust, as you may well know, has support for `HashMap`s.
These maps, as per their name, require hashing.
Hashing, of course, takes some data, and produces a random representation of that data.
Where we hash, random numbers follow!

### Why `attorand`?
Other crates, like `rand`, can be hard to use cross-platform,
Because they rely on the system's source of randomness.
`attorand`, however, does not have this limitation,
and provides a firehose of random bytes to drink from.

### How does it work?
Internally, `attorand` works by repeatedly hashing a seed,
using the output of the last round as the input to the next.
The algorithm used is SipHash 1-3, as per Rust's
[standard library docs](https://doc.rust-lang.org/std/collections/struct.HashMap.html),
but note that this is "subject to change at any point in the future".

### Addendum
Please don't use this for anything that needs cryptographic randomness.
(You knew that.)

I hold no further responsibility for this abomination.

### LICENSE

MIT.
