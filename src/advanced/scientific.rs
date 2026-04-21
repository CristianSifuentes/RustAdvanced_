//! Scientific domain module showing advanced generics, const generics, traits,
//! and iterator-driven numeric modeling.

use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, Mul};

/// Step 1: zero-cost, strongly typed units via `PhantomData`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quantity<T, Unit> {
    value: T,
    _unit: PhantomData<Unit>,
}

impl<T, Unit> Quantity<T, Unit> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            _unit: PhantomData,
        }
    }

    pub fn raw(&self) -> &T {
        &self.value
    }
}

/// Marker units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Meter;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Second;

/// Step 2: trait-based scientific behavior (static dispatch + generic algebra).
pub trait Mean<T> {
    fn mean(&self) -> Option<T>;
}

impl<T> Mean<T> for [T]
where
    T: Copy + Default + Add<Output = T> + Mul<f64, Output = T>,
{
    fn mean(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let sum = self.iter().copied().fold(T::default(), |acc, n| acc + n);
        Some(sum * (1.0 / self.len() as f64))
    }
}

impl Mul<f64> for Quantity<f64, Meter> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.value * rhs)
    }
}

impl Add for Quantity<f64, Meter> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

impl Default for Quantity<f64, Meter> {
    fn default() -> Self {
        Self::new(0.0)
    }
}

/// Step 3: const generics for fixed-size scientific vectors.
#[derive(Clone, Debug, PartialEq)]
pub struct Vector<const N: usize> {
    data: [f64; N],
}

impl<const N: usize> Vector<N> {
    pub fn new(data: [f64; N]) -> Self {
        Self { data }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

/// Step 4: Higher-ranked trait bound (HRTB) for borrow-flexible reporting.
pub fn report_with<'a, T, F>(dataset: &'a [T], formatter: F) -> Vec<String>
where
    T: Debug + 'a,
    F: for<'b> Fn(&'b T) -> String,
{
    dataset.iter().map(formatter).collect()
}

#[cfg(test)]
mod tests {
    use super::{Mean, Meter, Quantity, Vector};

    #[test]
    fn mean_quantity_works() {
        let values = [
            Quantity::<f64, Meter>::new(2.0),
            Quantity::<f64, Meter>::new(4.0),
        ];
        let mean = values.as_slice().mean().expect("mean must exist");
        assert_eq!(*mean.raw(), 3.0);
    }

    #[test]
    fn dot_product_works() {
        let a = Vector::<3>::new([1.0, 2.0, 3.0]);
        let b = Vector::<3>::new([4.0, 5.0, 6.0]);
        assert_eq!(a.dot(&b), 32.0);
    }
}
