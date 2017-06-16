/// A Rust port of [kdbush](https://github.com/mourner/kdbush), a fast static spatial index for 2D points.
///
/// [A dive into spatial search algorithms](https://medium.com/@agafonkin/a-dive-into-spatial-search-algorithms-ebd0c5e39d2a)

use std::f64;
use std::cmp;

type TIndex = usize;
type TNumber = f64;
type Point = [TNumber; 2];

pub struct KDBush {
    ids: Vec<TIndex>,
    points: Vec<Point>,
    node_size: u8,
}

impl KDBush {
    pub fn fill<'a, I>(points: I, size: usize, node_size: u8) -> KDBush
        where I: Iterator<Item = &'a Point>
    {
        let mut kdbush = KDBush {
            ids: Vec::with_capacity(size),
            points: Vec::with_capacity(size),
            node_size: node_size,
        };
        for (i, point) in points.enumerate() {
            kdbush.points.push([point[0], point[1]]);
            kdbush.ids.push(i);
        }
        kdbush.sort_kd(0, size - 1, 0);
        kdbush
    }

    /// Finds all items within the given bounding box.
    pub fn range<F>(&self,
                    minx: TNumber,
                    miny: TNumber,
                    maxx: TNumber,
                    maxy: TNumber,
                    mut visitor: F)
        where F: FnMut(TIndex)
    {
        self.range_idx(minx,
                       miny,
                       maxx,
                       maxy,
                       &mut visitor,
                       0,
                       self.ids.len() - 1,
                       0);
    }

    /// Finds all items within a given radius from the query point.
    pub fn within<F>(&self, qx: TNumber, qy: TNumber, r: TNumber, mut visitor: F)
        where F: FnMut(TIndex)
    {
        self.within_idx(qx, qy, r, &mut visitor, 0, self.ids.len() - 1, 0);
    }

    fn range_idx<F>(&self,
                    minx: TNumber,
                    miny: TNumber,
                    maxx: TNumber,
                    maxy: TNumber,
                    visitor: &mut F,
                    left: TIndex,
                    right: TIndex,
                    axis: usize)
        where F: FnMut(TIndex)
    {
        if right - left <= self.node_size as usize {
            for i in left..right + 1 {
                let x = self.points[i][0];
                let y = self.points[i][1];
                if x >= minx && x <= maxx && y >= miny && y <= maxy {
                    visitor(self.ids[i]);
                }
            }
            return;
        }

        let m = (left + right) >> 1;
        let x = self.points[m][0];
        let y = self.points[m][1];

        if x >= minx && x <= maxx && y >= miny && y <= maxy {
            visitor(self.ids[m]);
        }

        let lte = if axis == 0 { minx <= x } else { miny <= y };
        if lte {
            self.range_idx(minx, miny, maxx, maxy, visitor, left, m - 1, (axis + 1) % 2);
        }

        let gte = if axis == 0 { maxx >= x } else { maxy >= y };
        if gte {
            self.range_idx(minx,
                           miny,
                           maxx,
                           maxy,
                           visitor,
                           m + 1,
                           right,
                           (axis + 1) % 2);
        }
    }

    pub fn within_idx<F>(&self,
                         qx: TNumber,
                         qy: TNumber,
                         r: TNumber,
                         visitor: &mut F,
                         left: TIndex,
                         right: TIndex,
                         axis: usize)
        where F: FnMut(TIndex)
    {
        let r2 = r * r;

        if right - left <= self.node_size as usize {
            for i in left..right + 1 {
                let x = self.points[i][0];
                let y = self.points[i][1];
                if KDBush::sq_dist(x, y, qx, qy) <= r2 {
                    visitor(self.ids[i]);
                }
            }
            return;
        }

        let m = (left + right) >> 1;
        let x = self.points[m][0];
        let y = self.points[m][1];

        if KDBush::sq_dist(x, y, qx, qy) <= r2 {
            visitor(self.ids[m]);
        }

        let lte = if axis == 0 { qx - r <= x } else { qy - r <= y };
        if lte {
            self.within_idx(qx, qy, r, visitor, left, m - 1, (axis + 1) % 2);
        }

        let gte = if axis == 0 { qx + r >= x } else { qy + r >= y };
        if gte {
            self.within_idx(qx, qy, r, visitor, m + 1, right, (axis + 1) % 2);
        }
    }

    fn sort_kd(&mut self, left: TIndex, right: TIndex, axis: u8) {
        if right - left <= self.node_size as usize {
            return;
        }
        let m: TIndex = (left + right) >> 1;
        if axis == 0 {
            self.select(m, left, right, 0);
        } else {
            self.select(m, left, right, 1);
        }
        self.sort_kd(left, m - 1, (axis + 1) % 2);
        self.sort_kd(m + 1, right, (axis + 1) % 2);
    }

    fn select(&mut self, k: TIndex, mut left: TIndex, mut right: TIndex, axis: usize) {
        while right > left {
            if right - left > 600 {
                let n = (right - left + 1) as f64;
                let m = (k - left + 1) as f64;
                let z = f64::ln(n);
                let s = 0.5 * f64::exp(2.0 * z / 3.0);
                let r = k as f64 - m * s / n +
                        0.5 * f64::sqrt(z * s * (1.0 - s / n)) *
                        (if 2.0 * m < n { -1.0 } else { 1.0 });
                self.select(k,
                            cmp::max(left, r as usize),
                            cmp::min(right, (r + s) as usize),
                            axis);
            }

            let t = self.points[k][axis];
            let mut i = left;
            let mut j = right;

            self.swap_item(left, k);
            if self.points[right][axis] > t {
                self.swap_item(left, right);
            }

            while i < j {
                self.swap_item(i, j);
                i += 1;
                j -= 1;
                while self.points[i][axis] < t {
                    i += 1;
                }
                while self.points[j][axis] > t {
                    j -= 1;
                }
            }

            if self.points[left][axis] == t {
                self.swap_item(left, j);
            } else {
                j += 1;
                self.swap_item(j, right);
            }

            if j <= k {
                left = j + 1;
            }
            if k <= j {
                right = j - 1;
            }
        }
    }

    fn swap_item(&mut self, i: TIndex, j: TIndex) {
        self.ids.swap(i, j);
        self.points.swap(i, j);
    }

    fn sq_dist(ax: TNumber, ay: TNumber, bx: TNumber, by: TNumber) -> TNumber {
        (ax - bx).powi(2) + (ay - by).powi(2)
    }
}


#[cfg(test)]
#[cfg_attr(rustfmt, rustfmt_skip)]
const POINTS: [Point; 100] = [
    [ 54.0, 1.0 ],  [ 97.0, 21.0 ], [ 65.0, 35.0 ], [ 33.0, 54.0 ], [ 95.0, 39.0 ], [ 54.0, 3.0 ],  [ 53.0, 54.0 ], [ 84.0, 72.0 ],
    [ 33.0, 34.0 ], [ 43.0, 15.0 ], [ 52.0, 83.0 ], [ 81.0, 23.0 ], [ 1.0, 61.0 ],  [ 38.0, 74.0 ], [ 11.0, 91.0 ], [ 24.0, 56.0 ],
    [ 90.0, 31.0 ], [ 25.0, 57.0 ], [ 46.0, 61.0 ], [ 29.0, 69.0 ], [ 49.0, 60.0 ], [ 4.0, 98.0 ],  [ 71.0, 15.0 ], [ 60.0, 25.0 ],
    [ 38.0, 84.0 ], [ 52.0, 38.0 ], [ 94.0, 51.0 ], [ 13.0, 25.0 ], [ 77.0, 73.0 ], [ 88.0, 87.0 ], [ 6.0, 27.0 ],  [ 58.0, 22.0 ],
    [ 53.0, 28.0 ], [ 27.0, 91.0 ], [ 96.0, 98.0 ], [ 93.0, 14.0 ], [ 22.0, 93.0 ], [ 45.0, 94.0 ], [ 18.0, 28.0 ], [ 35.0, 15.0 ],
    [ 19.0, 81.0 ], [ 20.0, 81.0 ], [ 67.0, 53.0 ], [ 43.0, 3.0 ],  [ 47.0, 66.0 ], [ 48.0, 34.0 ], [ 46.0, 12.0 ], [ 32.0, 38.0 ],
    [ 43.0, 12.0 ], [ 39.0, 94.0 ], [ 88.0, 62.0 ], [ 66.0, 14.0 ], [ 84.0, 30.0 ], [ 72.0, 81.0 ], [ 41.0, 92.0 ], [ 26.0, 4.0 ],
    [ 6.0, 76.0 ],  [ 47.0, 21.0 ], [ 57.0, 70.0 ], [ 71.0, 82.0 ], [ 50.0, 68.0 ], [ 96.0, 18.0 ], [ 40.0, 31.0 ], [ 78.0, 53.0 ],
    [ 71.0, 90.0 ], [ 32.0, 14.0 ], [ 55.0, 6.0 ],  [ 32.0, 88.0 ], [ 62.0, 32.0 ], [ 21.0, 67.0 ], [ 73.0, 81.0 ], [ 44.0, 64.0 ],
    [ 29.0, 50.0 ], [ 70.0, 5.0 ],  [ 6.0, 22.0 ],  [ 68.0, 3.0 ],  [ 11.0, 23.0 ], [ 20.0, 42.0 ], [ 21.0, 73.0 ], [ 63.0, 86.0 ],
    [ 9.0, 40.0 ],  [ 99.0, 2.0 ],  [ 99.0, 76.0 ], [ 56.0, 77.0 ], [ 83.0, 6.0 ],  [ 21.0, 72.0 ], [ 78.0, 30.0 ], [ 75.0, 53.0 ],
    [ 41.0, 11.0 ], [ 95.0, 20.0 ], [ 30.0, 38.0 ], [ 96.0, 82.0 ], [ 65.0, 48.0 ], [ 33.0, 18.0 ], [ 87.0, 28.0 ], [ 10.0, 10.0 ],
    [ 40.0, 34.0 ], [ 10.0, 20.0 ], [ 47.0, 29.0 ], [ 46.0, 78.0 ]
];

#[test]
fn test_range() {
    let index = KDBush::fill(POINTS.iter(), POINTS.len(), 10);
    let expected_ids = vec![3, 90, 77, 72, 62, 96, 47, 8, 17, 15, 69, 71, 44, 19, 18, 45, 60, 20];
    let mut result = Vec::<TIndex>::new();
    {
        let visitor = |idx: TIndex| result.push(idx);
        index.range(20.0, 30.0, 50.0, 70.0, visitor);
    }
    assert_eq!(expected_ids, result);
}

#[test]
fn test_radius() {
    let index = KDBush::fill(POINTS.iter(), POINTS.len(), 10);
    let expected_ids = vec![3, 96, 71, 44, 18, 45, 60, 6, 25, 92, 42, 20];
    let mut result = Vec::<TIndex>::new();
    {
        let visitor = |idx: TIndex| result.push(idx);
        index.within(50.0, 50.0, 20.0, visitor);
    }
    assert_eq!(expected_ids, result);
}