use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2D<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Add for &Vector2D<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector2D<T>;
    fn add(self, other: Self) -> Self::Output {
        Self::Output { x: self.x + other.x, y: self.y + other.y }
    }
}

// TODO: Accept reference in RHS
impl<T> AddAssign for Vector2D<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T> Sub for &Vector2D<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vector2D<T>;
    fn sub(self, other: Self) -> Self::Output {
        Self::Output { x: self.x - other.x, y: self.y - other.y }
    }
}

impl<Scalar> Mul<Scalar> for &Vector2D<Scalar>
where
    Scalar: Mul<Output = Scalar> + Copy,
{
    type Output = Vector2D<Scalar>;
    fn mul(self, rhs: Scalar) -> Self::Output {
        Self::Output { x: self.x * rhs, y: self.y * rhs }
    }
}

impl<Scalar> Div<Scalar> for &Vector2D<Scalar>
where
    Scalar: Div<Output = Scalar> + Copy,
{
    type Output = Vector2D<Scalar>;
    fn div(self, rhs: Scalar) -> Self::Output {
        Self::Output { x: self.x / rhs, y: self.y / rhs }
    }
}

impl<T> Neg for &Vector2D<T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Vector2D<T>;
    fn neg(self) -> Self::Output {
        Self::Output { x: -self.x, y: -self.y }
    }
}

// Vec2I

pub type Vec2I = Vector2D<i32>;

pub const ZERO_VEC: Vec2I = Vec2I::new(0, 0);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2i_ops() {
        assert_eq!(Vec2I::new(4, 6), &Vec2I::new(1, 2) + &Vec2I::new(3, 4));
        assert_eq!(Vec2I::new(1, 12), &Vec2I::new(10, 20) - &Vec2I::new(9, 8));
        assert_eq!(Vec2I::new(33, 69), &Vec2I::new(11, 23) * 3);
        assert_eq!(Vec2I::new(1, 45), &Vec2I::new(123, 4567) / 100);
    }
}
