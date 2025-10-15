# garbage collector in rust

a mark-and-sweep garbage collector written in rust because why not

## what it does

- allocates objects on a custom heap
- tracks root references (like stack variables)
- automatically frees unreachable objects
- reuses freed memory slots to avoid fragmentation

## how it works

**mark phase**: starting from root objects, recursively mark everything that's reachable

**sweep phase**: walk the heap and free anything that wasn't marked

basically the same algorithm that runs in java and python but we built it from scratch

## why

rust has ownership and doesn't need garbage collection. that's literally the whole point of the language.

but you can still build one if you want to understand how GC works or just prove it's possible

## running it

```bash
cargo run
```

output shows:
- initial allocation of 5 objects
- collection freeing 2 unreachable objects
- new allocation reusing a freed slot

## the code

~no external dependencies, just stdlib

implements a simple heap with optional slots, root tracking, and mark-and-sweep collection

## performance

good enough for a demo. real GCs have generational collection, compaction, and way more optimizations but this gets the concept across

## license

do whatever you want with it