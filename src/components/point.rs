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

    /// exclusive limit
    pub fn limit(&self, value: usize) -> Option<Point> {
        if self.x >= value {
            None
        } else if self.y >= value {
            None
        } else {
            Some(*self)
        }
    }

    pub fn offset_and_limit(&self, offset: &Point<i32>, limit: usize) -> Option<Point> {
        self.offset(&Point {
            x: offset.x,
            y: offset.y,
        })
        .and_then(|o| o.limit(limit))
    }

    pub fn neighboring_points(&self, limit: usize) -> Vec<Point> {
        let mut result: Vec<Point> = vec![];
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let neighbor = self.offset_and_limit(&Point { x: i, y: j }, limit);

                if let Some(point) = neighbor {
                    result.push(point);
                }
            }
        }

        result
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

    #[test]
    fn neighboring_points_works() {
        let point: Point<usize> = Point { x: 1, y: 1 };
        assert_eq!(
            point.neighboring_points(3),
            vec![
                Point { x: 0, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 0, y: 2 },
                Point { x: 1, y: 0 },
                Point { x: 1, y: 2 },
                Point { x: 2, y: 0 },
                Point { x: 2, y: 1 },
                Point { x: 2, y: 2 },
            ]
        );
    }

    #[test]
    fn neighboring_points_edge() {
        let point: Point<usize> = Point { x: 1, y: 1 };
        assert_eq!(
            point.neighboring_points(2),
            vec![
                Point { x: 0, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 0 },
            ]
        );
    }

    #[test]
    fn neighboring_points_0_0() {
        let point: Point<usize> = Point { x: 0, y: 0 };
        assert_eq!(
            point.neighboring_points(3),
            vec![
                Point { x: 0, y: 1 },
                Point { x: 1, y: 0 },
                Point { x: 1, y: 1 },
            ]
        );
    }
}
