//! # Multidimensional Point
//! A crate that provides a simple multidimensional point struct, base on a vector.

#![allow(dead_code)]
use num::traits::Signed;
use std::ops::{Add, Div, Mul, Sub};
/// multidimensional point type.
pub struct Point<T> {
    values: Vec<T>,
    dim: usize,
}

impl<T> Point<T>
where
    T: Clone + Default,
{
    /// Creates a new point with default values. The size of the point base on the argument.
    /// # Examples
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new(3);
    /// assert_eq!(p1.get_size(), 3);
    /// ```
    ///
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new(3);
    /// assert_eq!(p1.get_vector(), &[0,0,0]);
    /// ```
    pub fn new(dimension: usize) -> Point<T> {
        Point {
            values: vec![T::default(); dimension],
            dim: dimension,
        }
    }
}
impl<T> Point<T>
where
    T: Clone,
{
    /// Creates a new point from a vector.
    /// # Examples
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![1,2,3]);
    /// assert_eq!(p1.get_vector(), &vec![1,2,3]);
    /// ```
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![1,2,3]);
    /// assert_eq!(p1.get_size(), 3);
    pub fn new_from_vec(values_vec: &Vec<T>) -> Point<T> {
        Point {
            values: values_vec.clone(),
            dim: values_vec.len(),
        }
    }

    /// Return a vector with the point values.
    /// # Example
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![1,2,3]);
    /// assert_eq!(p1.get_vector(), &vec![1,2,3]);
    /// ```
    pub fn get_vector(&self) -> &Vec<T> {
        &self.values
    }

    /// Return a value in a specific dimension.
    /// # Examples
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![1,10,3]);
    /// assert_eq!(p1.get_value(2), &10);
    /// ```
    ///
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<f32> = Point::new_from_vec(&vec![1.2, 3.2, 1.6]);
    /// assert_eq!(p1.get_value(1), &1.2);
    /// ```
    /// # Panic
    /// This function will panic when the dimension index is smaller than 1, or bigger than the
    /// point's dimension.
    ///
    /// ```should_panic
    /// use multi_dim_point::Point;
    /// let p1: Point<f32> = Point::new_from_vec(&vec![1.2, 3.2, 1.6]);
    /// let _ = p1.get_value(0);
    /// ```
    /// ```should_panic
    /// use multi_dim_point::Point;
    /// let p1: Point<f32> = Point::new_from_vec(&vec![1.2, 3.2, 1.6]);
    /// let _ = p1.get_value(10);
    /// ```
    pub fn get_value(&self, dim_index: usize) -> &T {
        self.check_valid_dim(dim_index);
        self.get_vector().get(dim_index - 1).unwrap() // dim start from 1, vector index from 0.
    }
    /// Change the value of the point in a specific dimension.
    /// # Example
    /// ```
    /// use multi_dim_point::Point;
    /// let mut p1: Point<i32> = Point::new_from_vec(&vec![1,2]);
    /// assert_eq!(p1.get_vector(), &vec![1,2]);
    /// p1.set_value(1,&5);
    /// assert_eq!(p1.get_vector(), &vec![5,2]);
    /// ```
    ///
    /// # Panic
    /// This function will panic when the dimension is invalid.
    /// ```should_panic
    /// use multi_dim_point::Point;
    /// let mut p1: Point<i32> = Point::new_from_vec(&vec![1,2]);
    /// p1.set_value(3,&3);
    /// ```
    ///
    pub fn set_value(&mut self, dim: usize, new_val: &T) {
        self.check_valid_dim(dim);
        self.values[dim - 1] = new_val.clone(); // dim start from 1, index from 0.
    }
    fn check_valid_dim(&self, dim: usize) {
        if dim < 1 {
            panic!("dimenstion start from 1 ({} < 1)", dim);
        }
        if dim > self.dim {
            panic!("{} is bigger than {} (point max dimension)", dim, self.dim);
        }
    }
    /// This function will apply a function on every pair of values in the same dimension, and
    /// return a vector of the result.
    /// # Example
    /// ```
    /// use multi_dim_point::Point;
    ///
    /// fn add_f(num1: &i32, num2: &i32) -> i32 {
    ///     num1 + num2
    /// }
    /// let p1: Point<i32> = Point::new_from_vec(&vec![2, 8, 64, 256, 0]);
    /// let p2: Point<i32> = Point::new_from_vec(&vec![2, 8, 14, 6, 0]);
    /// assert_eq!((p1.apply_func(&p2, &add_f)), &[4, 16, 78, 262, 0])
    /// ```
    /// # Panic
    /// The function will panic when the points are without equal dimensions
    ///
    /// ```should_panic
    /// use multi_dim_point::Point;
    /// fn add_f(num1: &i32, num2: &i32) -> i32 {
    ///     num1 + num2
    /// }
    /// let p1: Point<i32> = Point::new_from_vec(&vec![2, 8, 64, 256, 0]);
    /// let p2: Point<i32> = Point::new_from_vec(&vec![2, 14, 6, 0]);
    /// p1.apply_func(&p2, &add_f);
    ///
    ///```
    pub fn apply_func<S>(&self, other: &Point<T>, f: &dyn Fn(&T, &T) -> S) -> Vec<S> {
        if self.dim != other.dim {
            panic!("dimensions are not equal");
        }
        self.get_vector()
            .iter()
            .zip(other.get_vector().iter())
            .map(|(a, b)| f(a, b))
            .collect()
    }

    /// The function will return the number of dimensions of the point.
    /// # Exampl
    /// ```
    /// use multi_dim_point::Point;
    /// let p1 : Point<bool> = Point::new(3);
    /// assert_eq!(p1.get_size(), 3);
    /// ```
    pub fn get_size(&self) -> usize {
        self.dim.clone()
    }
}

impl<T> Add for &Point<T>
where
    T: Clone + Copy + Default + Add<Output = T>,
{
    type Output = Point<T>;
    /// \+ operator. Adding values in each dimension.
    /// # Example
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![1,2,3]);
    /// let p2: Point<i32> = Point::new_from_vec(&vec![4,5,6]);
    /// assert_eq!((&p1+&p2).get_vector(), &vec![5,7,9]);
    /// ```
    /// # Panic
    /// This function will panic if the dimensions of the points are not equal.
    ///
    /// ```should_panic
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![1,2,3]);
    /// let p2: Point<i32> = Point::new_from_vec(&vec![5,6]);
    /// &p1+&p2;
    /// ```
    fn add(self, other: Self) -> Point<T> {
        if self.dim != other.dim {
            panic!("dimensions are not equal, can't add");
        }
        Point::new_from_vec(
            &self
                .get_vector()
                .iter()
                .zip(other.get_vector().iter())
                .map(|(a, b)| *a + *b)
                .collect(),
        )
    }
}
impl<T> Add for Point<T>
where
    T: Clone + Copy + Default + Add<Output = T>,
{
    type Output = Point<T>;
    /// \+ operator. Adding values in each dimension.
    /// # Example
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![1,2,3]);
    /// let p2: Point<i32> = Point::new_from_vec(&vec![4,5,6]);
    /// assert_eq!((p1+p2).get_vector(), &vec![5,7,9]);
    /// ```
    /// # Panic
    /// This function will panic if the dimensions of the points are not equal.
    ///
    /// ```should_panic
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![1,2,3]);
    /// let p2: Point<i32> = Point::new_from_vec(&vec![5,6]);
    /// p1+p2;
    /// ```
    fn add(self, other: Self) -> Point<T> {
        &self + &other
    }
}

impl<T> Sub for &Point<T>
where
    T: Clone + Copy + Default + Sub<Output = T>,
{
    type Output = Point<T>;
    /// \- operator. Subtraction values in each dimension.
    /// # Example
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![1,2,3]);
    /// let p2: Point<i32> = Point::new_from_vec(&vec![4,5,6]);
    /// assert_eq!((&p1-&p2).get_vector(), &vec![-3,-3,-3]);
    /// ```
    /// # Panic
    /// This function will panic if the dimensions of the points are not equal.
    ///
    /// ```should_panic
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![1,2,3]);
    /// let p2: Point<i32> = Point::new_from_vec(&vec![5,6]);
    /// &p1-&p2;
    /// ```
    fn sub(self, other: Self) -> Point<T> {
        if self.dim != other.dim {
            panic!("dimensions are not equle, can't sub");
        }
        Point::new_from_vec(
            &self
                .get_vector()
                .iter()
                .zip(other.get_vector().iter())
                .map(|(a, b)| *a - *b)
                .collect(),
        )
    }
}
impl<T> Sub for Point<T>
where
    T: Clone + Copy + Default + Sub<Output = T>,
{
    type Output = Point<T>;
    /// \- operator. Subtraction values in each dimension.
    /// # Examples
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![1,2,3]);
    /// let p2: Point<i32> = Point::new_from_vec(&vec![4,5,6]);
    /// assert_eq!((p1-p2).get_vector(), &vec![-3,-3,-3]);
    /// ```
    /// # Panic
    ///
    /// ```should_panic
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![1,2,3]);
    /// let p2: Point<i32> = Point::new_from_vec(&vec![5,6]);
    /// p1-p2;
    /// ```
    fn sub(self, other: Self) -> Point<T> {
        &self - &other
    }
}
impl<T, S> Mul<&S> for &Point<T>
where
    T: Default + Copy + Clone + Mul<S, Output = T>,
    S: Copy,
{
    type Output = Point<T>;
    /// \* operator. Multiply each value in the point.
    /// # Example
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![1,2,3]);
    /// assert_eq!((&p1 * &5).get_vector(), &vec![5,10,15]);
    /// ```
    fn mul(self, scalar: &S) -> Point<T> {
        Point::new_from_vec(&self.get_vector().iter().map(|a| *a * *scalar).collect())
    }
}

impl<T, S> Mul<S> for Point<T>
where
    T: Default + Copy + Clone + Mul<S, Output = T>,
    S: Copy,
{
    type Output = Point<T>;
    /// \* operator. Multiply each value in the point.
    /// # Example
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![1,2,3]);
    /// assert_eq!((p1 * 5).get_vector(), &vec![5,10,15]);
    /// ```
    fn mul(self, scalar: S) -> Point<T> {
        &self * &scalar
    }
}

impl<T, S> Div<&S> for &Point<T>
where
    T: Default + Copy + Clone + Div<S, Output = T>,
    S: Copy,
{
    type Output = Point<T>;
    /// / operator. Divide each value in the point.
    /// # Example
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![5,10,15]);
    /// assert_eq!((&p1 / &5).get_vector(), &vec![1,2,3]);
    /// ```
    fn div(self, scalar: &S) -> Point<T> {
        Point::new_from_vec(&self.get_vector().iter().map(|a| *a / *scalar).collect())
    }
}

impl<T, S> Div<S> for Point<T>
where
    T: Default + Copy + Clone + Div<S, Output = T>,
    S: Copy,
{
    type Output = Point<T>;
    /// / operator. Divide each value in the point.
    /// # Example
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![5,10,15]);
    /// assert_eq!((p1 / 5).get_vector(), &vec![1,2,3]);
    /// ```
    fn div(self, scalar: S) -> Point<T> {
        &self / &scalar
    }
}

impl<T> PartialEq for Point<T>
where
    T: PartialEq + Clone,
{
    /// == operator. Check if 2 points are the same (in all dimensions).
    /// # Examples
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![5,10,15]);
    /// let p2: Point<i32> = Point::new_from_vec(&vec![5,10,15]);
    /// assert_eq!(&p1 == &p2, true);
    /// ```
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![10,15]);
    /// let p2: Point<i32> = Point::new_from_vec(&vec![5,10,15]);
    /// assert_eq!(&p1 == &p2, false);
    /// ```
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![20,10,15]);
    /// let p2: Point<i32> = Point::new_from_vec(&vec![5,10,15]);
    /// assert_eq!(&p1 == &p2, false);
    /// ```
    ///
    fn eq(&self, other: &Self) -> bool {
        self.get_size() == other.get_size()
            && self
                .get_vector()
                .iter()
                .zip(other.get_vector().iter())
                .all(|(a, b)| a == b)
    }
}

impl<T> Point<T>
where
    T: Sub<Output = T> + PartialOrd + Clone + Copy + Signed,
{
    /// Check if the points are close to each other, in each dimension, up to epsilon.
    ///
    /// # Examples:
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![5,10,15]);
    /// let p2: Point<i32> = Point::new_from_vec(&vec![7,8,14]);
    /// assert_eq!(p1.close(&p2,3), true);
    /// ```
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![11,10,15]);
    /// let p2: Point<i32> = Point::new_from_vec(&vec![7,8,14]);
    /// assert_eq!(p1.close(&p2,3), false);
    /// ```
    /// ```
    /// use multi_dim_point::Point;
    /// let p1: Point<i32> = Point::new_from_vec(&vec![7,8,14]);
    /// let p2: Point<i32> = Point::new_from_vec(&vec![7,8,14]);
    /// assert_eq!(p1.close(&p2,0), true);
    /// ```
    pub fn close(&self, other: &Self, eps: T) -> bool {
        self.get_size() == other.get_size()
            && self
                .get_vector()
                .iter()
                .zip(other.get_vector().iter())
                .all(|(a, b)| (*a - *b).abs() <= eps)
    }
}

#[cfg(test)]
mod test;
