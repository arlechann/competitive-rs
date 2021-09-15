use std::fmt::Debug;

pub trait Vector2D: Sized {
    fn new(x: f64, y: f64) -> Self;
    fn x(&self) -> f64;
    fn y(&self) -> f64;

    fn origin() -> Self {
        Self::new(0.0, 0.0)
    }

    fn dot(&self, rhs: &Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y()
    }

    fn cross(&self, rhs: &Self) -> f64 {
        self.x() * rhs.y() - self.y() * rhs.x()
    }

    fn length(&self) -> f64 {
        self.distance(&<Self as Vector2D>::origin())
    }

    fn len(&self) -> f64 {
        self.length()
    }

    fn distance(&self, rhs: &Self) -> f64 {
        (self.x() - rhs.x()).hypot(self.y() - rhs.y()).abs()
    }

    fn argument(&self) -> f64 {
        self.y().atan2(self.x())
    }

    fn rotate(&self, rad: f64) -> Self {
        Self::new(
            self.x() * rad.cos() - self.y() * rad.sin(),
            self.x() * rad.sin() + self.y() * rad.cos(),
        )
    }

    fn unit(&self) -> Self {
        let len = self.len();
        Self::new(self.x() / len, self.y() / len)
    }

    fn normal(&self) -> Self {
        let len = self.len();
        Self::new(-self.y() / len, self.x() / len)
    }

    fn add(&self, rhs: &Self) -> Self {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y())
    }

    fn sub(&self, rhs: &Self) -> Self {
        Self::new(self.x() - rhs.x(), self.y() - rhs.y())
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec2(pub f64, pub f64);

impl Vector2D for Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Vec2(x, y)
    }

    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        <Self as Vector2D>::add(&self, &rhs)
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        <Self as Vector2D>::sub(&self, &rhs)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CCW {
    Clockwise,
    CounterClockwise,
    ABC,
    ACB,
    CAB,
}

impl CCW {
    pub fn ccw<T: Vector2D>(a: T, b: T, c: T) -> Self {
        let ab = b.sub(&a);
        let ac = c.sub(&a);
        let det = ab.cross(&ac);
        if det > 0.0 {
            CCW::CounterClockwise
        } else if det < 0.0 {
            CCW::Clockwise
        } else if ab.dot(&ac) < 0.0 {
            CCW::CAB
        } else if ab.len() < ac.len() {
            CCW::ABC
        } else {
            CCW::ACB
        }
    }
}

pub struct LineSegment<T: Vector2D + PartialEq + Debug>(T, T);

impl<T> LineSegment<T>
where
    T: Vector2D + PartialEq + Debug,
{
    pub fn new(a: T, b: T) -> Self {
        Self(a, b)
    }

    pub fn length(&self) -> f64 {
        self.1.sub(&self.0).length()
    }

    pub fn is_crossing(&self, rhs: &Self) -> bool {
        let a = &self.0;
        let b = &self.1;
        let c = &rhs.0;
        let d = &rhs.1;

        let ab = b.sub(a);
        let cd = d.sub(c);
        let ac = c.sub(a);
        let ad = d.sub(a);
        let ca = a.sub(c);
        let cb = b.sub(c);

        let det_ab_ac = ab.cross(&ac);
        let det_ab_ad = ab.cross(&ad);
        let det_cd_ca = cd.cross(&ca);
        let det_cd_cb = cd.cross(&cb);

        det_ab_ac * det_ab_ad < 0.0 && det_cd_ca * det_cd_cb < 0.0
    }
}

#[cfg(test)]
mod test {
    mod vec2 {
        use super::super::{Vec2, Vector2D};
        use std::f64::consts::PI;

        fn vec2_delta_eq(a: Vec2, b: Vec2) -> bool {
            const DELTA: f64 = 1e-10;
            (a.x() - b.x()).abs() <= DELTA && (a.y() - b.y()).abs() <= DELTA
        }

        #[test]
        fn test_new() {
            assert_eq!(Vec2(0.0, 0.0), Vec2::new(0.0, 0.0));
            assert_eq!(Vec2(1.0, 1.0), Vec2::new(1.0, 1.0));
            assert_eq!(Vec2(1.0, -1.0), Vec2::new(1.0, -1.0));
            assert_eq!(Vec2(-1.0, 1.0), Vec2::new(-1.0, 1.0));
            assert_eq!(Vec2(-1.0, -1.0), Vec2::new(-1.0, -1.0));
        }

        #[test]
        fn test_x() {
            assert_eq!(0.0, Vec2::new(0.0, 0.0).x());
            assert_eq!(1.0, Vec2::new(1.0, -1.0).x());
            assert_eq!(-1.0, Vec2::new(-1.0, 1.0).x());
        }

        #[test]
        fn test_y() {
            assert_eq!(0.0, Vec2::new(0.0, 0.0).y());
            assert_eq!(1.0, Vec2::new(-1.0, 1.0).y());
            assert_eq!(-1.0, Vec2::new(1.0, -1.0).y());
        }

        #[test]
        fn test_origin() {
            assert_eq!(Vec2(0.0, 0.0), Vec2::origin());
        }

        #[test]
        fn test_dot() {
            assert_eq!(0.0, Vec2::new(1.0, 0.0).dot(&Vec2::new(0.0, 1.0)));
            assert_eq!(0.0, Vec2::new(0.0, 1.0).dot(&Vec2::new(1.0, 0.0)));
            assert_eq!(0.0, Vec2::new(1.0, 1.0).dot(&Vec2::new(-1.0, 1.0)));
            assert_eq!(-2.0, Vec2::new(1.0, 1.0).dot(&Vec2::new(-1.0, -1.0)));
        }

        #[test]
        fn test_cross() {
            assert_eq!(1.0, Vec2::new(1.0, 0.0).cross(&Vec2::new(0.0, 1.0)));
            assert_eq!(-1.0, Vec2::new(0.0, 1.0).cross(&Vec2::new(1.0, 0.0)));
            assert_eq!(2.0, Vec2::new(1.0, 1.0).cross(&Vec2::new(-1.0, 1.0)));
            assert_eq!(0.0, Vec2::new(1.0, 1.0).cross(&Vec2::new(-1.0, -1.0)));
        }

        #[test]
        fn test_length() {
            assert_eq!(1.0, Vec2::new(1.0, 0.0).length());
            assert_eq!(1.0, Vec2::new(0.0, 1.0).length());
            assert_eq!(2.0f64.sqrt(), Vec2::new(1.0, 1.0).length());
        }

        #[test]
        fn test_len() {
            assert_eq!(1.0, Vec2::new(1.0, 0.0).len());
            assert_eq!(1.0, Vec2::new(0.0, 1.0).len());
            assert_eq!(2.0f64.sqrt(), Vec2::new(1.0, 1.0).len());
        }

        #[test]
        fn test_distance() {
            assert_eq!(1.0, Vec2::new(1.0, 0.0).distance(&Vec2::origin()));
            assert_eq!(2.0, Vec2::new(1.0, 1.0).distance(&Vec2::new(-1.0, 1.0)));
            assert_eq!(
                2.0f64.sqrt() * 2.0,
                Vec2::new(1.0, 1.0).distance(&Vec2::new(-1.0, -1.0))
            );
        }

        #[test]
        fn test_argument() {
            assert_eq!(0.0, Vec2::new(1.0, 0.0).argument());
            assert_eq!(PI / 4.0, Vec2::new(1.0, 1.0).argument());
            assert_eq!(PI / 2.0, Vec2::new(0.0, 1.0).argument());
            assert_eq!(PI, Vec2::new(-1.0, 0.0).argument());
            assert_eq!(-PI / 2.0, Vec2::new(0.0, -1.0).argument());
        }

        #[test]
        fn test_rotate() {
            assert!(vec2_delta_eq(
                Vec2(0.0, 1.0),
                Vec2::new(1.0, 0.0).rotate(-PI * 3.0 / 2.0)
            ));
            assert!(vec2_delta_eq(
                Vec2(-1.0, 0.0),
                Vec2::new(1.0, 0.0).rotate(-PI)
            ));
            assert!(vec2_delta_eq(
                Vec2(0.0, -1.0),
                Vec2::new(1.0, 0.0).rotate(-PI / 2.0)
            ));
            assert!(vec2_delta_eq(
                Vec2(1.0, 0.0),
                Vec2::new(1.0, 0.0).rotate(0.0)
            ));
            assert!(vec2_delta_eq(
                Vec2(0.0, 1.0),
                Vec2::new(1.0, 0.0).rotate(PI / 2.0)
            ));
            assert!(vec2_delta_eq(
                Vec2(-1.0, 0.0),
                Vec2::new(1.0, 0.0).rotate(PI)
            ));
            assert!(vec2_delta_eq(
                Vec2(0.0, -1.0),
                Vec2::new(1.0, 0.0).rotate(PI * 3.0 / 2.0)
            ));
            assert!(vec2_delta_eq(
                Vec2(1.0, 0.0),
                Vec2::new(1.0, 0.0).rotate(PI * 2.0)
            ));
        }

        #[test]
        fn test_unit() {
            assert_eq!(Vec2(1.0, 0.0), Vec2::new(1.0, 0.0).unit());
            assert_eq!(Vec2(0.0, 1.0), Vec2::new(0.0, 1.0).unit());
            assert_eq!(
                Vec2(1.0 / 2.0f64.sqrt(), 1.0 / 2.0f64.sqrt()),
                Vec2::new(1.0, 1.0).unit()
            );
        }

        #[test]
        fn test_normal() {
            assert_eq!(Vec2(0.0, 1.0), Vec2::new(1.0, 0.0).normal());
            assert_eq!(Vec2(-1.0, 0.0), Vec2::new(0.0, 1.0).normal());
            assert_eq!(
                Vec2(-1.0 / 2.0f64.sqrt(), 1.0 / 2.0f64.sqrt()),
                Vec2::new(1.0, 1.0).normal()
            );
        }

        #[test]
        fn test_add() {
            assert_eq!(
                Vec2(1.0, 1.0),
                Vec2::new(1.0, 0.0).add(&Vec2::new(0.0, 1.0))
            );
            assert_eq!(
                Vec2(2.0, 0.0),
                Vec2::new(1.0, 0.0).add(&Vec2::new(1.0, 0.0))
            );
            assert_eq!(
                Vec2(0.0, 2.0),
                Vec2::new(0.0, 1.0).add(&Vec2::new(0.0, 1.0))
            );
            assert_eq!(
                Vec2(2.0, 2.0),
                Vec2::new(1.0, 1.0).add(&Vec2::new(1.0, 1.0))
            );
            assert_eq!(
                Vec2(0.0, 0.0),
                Vec2::new(1.0, -1.0).add(&Vec2::new(-1.0, 1.0))
            );
        }

        #[test]
        fn test_sub() {
            assert_eq!(
                Vec2(1.0, -1.0),
                Vec2::new(1.0, 0.0).sub(&Vec2::new(0.0, 1.0))
            );
            assert_eq!(
                Vec2(0.0, 0.0),
                Vec2::new(1.0, 0.0).sub(&Vec2::new(1.0, 0.0))
            );
            assert_eq!(
                Vec2(0.0, 0.0),
                Vec2::new(0.0, 1.0).sub(&Vec2::new(0.0, 1.0))
            );
            assert_eq!(
                Vec2(0.0, 0.0),
                Vec2::new(1.0, 1.0).sub(&Vec2::new(1.0, 1.0))
            );
            assert_eq!(
                Vec2(2.0, -2.0),
                Vec2::new(1.0, -1.0).sub(&Vec2::new(-1.0, 1.0))
            );
        }
    }
}
