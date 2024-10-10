extern crate agarwal;

use agarwal::Point;

#[test]
fn agarwal() {
    let p = vec![
        Point::new(-1., -1.),
        Point::new(-1., 0.),
        Point::new(0., 1.),
        Point::new(1., 0.),
        Point::new(0., -1.),
    ];
    let q = vec![
        Point::new(-1., -1.),
        Point::new(-1., 1.),
        Point::new(0., 2.),
        Point::new(1., 0.),
        Point::new(0., -2.),
    ];
    agarwal::agarwal(p, q);
}
