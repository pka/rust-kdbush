## kdbush

A Rust port of [kdbush](https://github.com/mourner/kdbush), a fast static spatial index for 2D points.

Introduction: [A dive into spatial search algorithms](https://medium.com/@agafonkin/a-dive-into-spatial-search-algorithms-ebd0c5e39d2a)

### Usage

```rust
let index = KDBush::create(points, kdbush::DEFAULT_NODE_SIZE); // make an index
index.range(20.0, 30.0, 50.0, 70.0, |id| print!("{} ", id));   // bbox search - minX, minY, maxX, maxY
index.within(50.0, 50.0, 20.0, |id| print!("{} ", id));        // radius search - x, y, radius
```
