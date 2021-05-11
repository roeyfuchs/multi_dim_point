[![Rust](https://github.com/RoeyFuchs/multi_dim_point/actions/workflows/rust.yml/badge.svg)](https://github.com/RoeyFuchs/multi_dim_point/actions/workflows/rust.yml)
# Multi-Dimensional Point 
A simple multi-dimensional point structer, base on a vector.

## Examples
```
use multi_dim_point::Point;
let p1: Point<i32> = Point::new(3); // create a 3-dim point, with default  values (i32 default = 0).
```

```
use multi_dim_point::Point;
let p1: Point<i32> = Point::new_from_vec(&vec![1,2,3]); // create a 3-dim point with values from a vector.
p1.get_value(1) // return a reference to the value in the first dimension.
p1.get_size() // return how many dimensions the point has.
p1.get_vector() // return a reference to the vector that represents the point.

let p2: Point<i32> = Point::new_from_vec(&vec![4,5,6]); 
let p3: Point<i32> = &p1+&p2; // create a new point, adding values in the same dimension (the new point will be 5,7,9).
let p4: Point<i32> = &p3-&p2;

let p5: Point<i32> = &p4 * &5 // multiply each value in the point.
let p6: Point<i32> = &p5 / &5;
```

```
fn add_f(num1: &i32, num2: &i32) -> i32 {
	num1 + num2
}
let p1: Point<i32> = Point::new_from_vec(&vec![2, 8, 64, 256, 0]);
let p2: Point<i32> = Point::new_from_vec(&vec![2, 8, 14, 6, 0]);

p1.apply_func(&p2, &add_f); // return a vector ([4, 16, 78, 262, 0])
```

See more examples in the documentation.
