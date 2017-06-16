## kdbush

A Rust port of [kdbush](https://github.com/mourner/kdbush), a fast static spatial index for 2D points.

### Usage

```rust
let index = KDBush::fill(points, 10);            // make an index
index.range(20.0, 30.0, 50.0, 70.0, visitor);    // bbox search - minX, minY, maxX, maxY
index.within(50.0, 50.0, 20.0, visitor);         // radius search - x, y, radius
```
