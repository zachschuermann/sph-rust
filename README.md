SPH
=====
Smoothed-particle hydrodynamics in Rust. 

## Tests
Release target: ~4s for single threaded N = 1200, I = 1000 (particles/iterations, respectively).

## Ref
https://wiki.alopex.li/AGuideToRustGraphicsLibraries2019

## Todo
- [x] Debug implementation
- [ ] Add parallelism (investigate Rayon) + test
- [ ] Switch to GFX
- [ ] Benchmarks (Look into Criteron/hyperfine)
- [ ] Keyboard input
- [ ] Haskell comparison / writeup
- [ ] Target WASM and make browser demo
