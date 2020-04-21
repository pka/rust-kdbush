## kdbush

[![crates.io](https://meritbadge.herokuapp.com/kdbush)](https://crates.io/crates/kdbush)
[![Documentation](https://docs.rs/kdbush/badge.svg)](https://docs.rs/kdbush)

A Rust port of [kdbush](https://github.com/mourner/kdbush), a very fast static spatial
index for 2D points based on a flat KD-tree.

Introduction: [A dive into spatial search algorithms](https://medium.com/@agafonkin/a-dive-into-spatial-search-algorithms-ebd0c5e39d2a)

[Comparison](https://gist.github.com/andrewharvey/de265c3e5315cbb674488999d866d4c6)
of point and box spatial index libraries.

### Usage

```rust
let points = vec![(54.0, 1.0), (97.0, 21.0), (65.0, 35.0)];
let index = KDBush::create(points, kdbush::DEFAULT_NODE_SIZE); // make an index
index.range(20.0, 30.0, 50.0, 70.0, |id| print!("{} ", id));   // bbox search - minX, minY, maxX, maxY
index.within(50.0, 50.0, 20.0, |id| print!("{} ", id));        // radius search - x, y, radius
```
