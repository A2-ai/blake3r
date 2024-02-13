# blake3r

`blake3r` is an implementation of [BLAKE3](https://github.com/BLAKE3-team/BLAKE3) in R, 
by wrapping the rust implementation. This was an experiment
to try out the [extendr](https://github.com/extendr/extendr) package and learn a bit about rust. 

We've found hashing in blake3r to be up to 25x faster
than getting an md5sum via `digest`. This has been key to keep [dvs](https://github.com/a2-ai/devious)
performant when dealing with repositories with large (by size or quantity) files. 

By providing an R interface, we can also interact with the metadata files `dvs` generates, or hash files
independent to `dvs` entirely. At the moment, some testing has been done to confirm hashes come out
consistent to the [b3sum cli](https://github.com/michaelforney/b3sum).

<!-- badges: start -->
  [![R-CMD-check](https://github.com/A2-ai/blake3r/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/A2-ai/blake3r/actions/workflows/R-CMD-check.yaml)
  <!-- badges: end -->

## more about BLAKE3

The BLAKE3 site notes:

> BLAKE3 is a cryptographic hash function that is:

> - **Much faster** than MD5, SHA-1, SHA-2, SHA-3, and BLAKE2.
> - **Secure**, unlike MD5 and SHA-1. And secure against length extension,
  unlike SHA-2.
> - **Highly parallelizable** across any number of threads and SIMD lanes,
  because it's a Merkle tree on the inside.
> - Capable of **verified streaming** and **incremental updates**, again
  because it's a Merkle tree.
> - A **PRF**, **MAC**, **KDF**, and **XOF**, as well as a regular hash.
> - **One algorithm with no variants**, which is fast on x86-64 and also
  on smaller architectures.


