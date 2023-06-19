#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point<T = usize> {
    pub x: T,
    pub y: T,
}

impl Point {
    pub fn offset(&self, value: &Point<i32>) -> Option<Point> {
        let x = if value.x.is_negative() {
            self.x.checked_sub(value.x.wrapping_abs() as u32 as usize)
        } else {
            self.x.checked_add(value.x as usize)
        };

        let y = if value.y.is_negative() {
            self.y.checked_sub(value.y.wrapping_abs() as u32 as usize)
        } else {
            self.y.checked_add(value.y as usize)
        };

        match (x, y) {
            (Some(x), Some(y)) => Some(Point { x, y }),
            (_, _) => None,
        }
    }

    pub fn limit(&self, value: usize) -> Option<Point> {
        if self.x >= value {
            None
        } else if self.y >= value {
            None
        } else {
            Some(*self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset_works() {
        let point: Point<usize> = Point { x: 10, y: 10 };
        let point = point.offset(&Point { x: 1, y: -3 });
        assert_eq!(point, Some(Point { x: 11, y: 7 }));
    }

    #[test]
    fn offset_returns_none_if_below_0() {
        let point: Point<usize> = Point { x: 10, y: 10 };
        let point = point.offset(&Point { x: -11, y: -13 });
        assert_eq!(point, None);
    }

    #[test]
    fn limit_works() {
        let point: Point<usize> = Point { x: 10, y: 10 };
        let point = point.limit(7);
        assert_eq!(point, None)
    }
}
