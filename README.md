# Lil'Bits

## Purpose
In some instances, even a simple `HashMap<T>` is too much for your application.
The `LilBitsSet` is inspired by works such as `smallset`, `sparseset` and
`bitmap`, but is incredibly simplistic with very few bells and whistles by
excelling at really only one thing: `HashSet<u8>` semantics when your values are
never `> 63`.

Personally, I plan to use this in my server-client library, where users can 
optionally choose to broadcast to a `LilBitSet` of clients. As no more than ~10
are expected to be online at once, a `LilBitSet` is perfect for reasoning over 
objects relating to these clients and their corresponding IDs.

## Usefulness
This data structure has two severe limitations in comparison to other sets:
1. Only `u8` can be stored (or any integer `as` u8, of course)
1. No value `>= 64` can be stored. This is in constrast to `smallset`, where the
   limitation is on the _number_ of elements, rather than the _value_ of the elements.
Chances are, if these restrictions are dealbreakers for your purpose rather use a `smallset` or something like it. Otherwise, `LilBitSet` has some excellent properties:
* Super-fast basic operations such as `insert`
* super fast `Clone` and `Copy`
* trivially benefits from obvious `Sized`,`Sync`, `Serialize`, `Deserialize` etc.
* set-logic leaning on super fast bitwise operations (eg `union` to `|`)

Additionally, there exists a convenience macro `lilbits!` for constructing a `LilBitSet`,
as well as some potentially useful `From` implementations to convert back and forth to other common sets 
`HashSet`, `BTreeSet`.

## Using It Yourself
The semantics of `LilBitSet` are similar to that of `HashSet<u8>`minus some missing functions which woudln't be useful, such as `new_with_capacity`.

Most noteworthy is that the thread will `Panic!` if there is _ever_ an attempt to `insert` some u8 with a value greater than 63. The function `try_insert` also exists in the event a silent failure is preferable.


## Examples
See `tests.rs` for annotated examples.