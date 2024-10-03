use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

#[wasm_bindgen]
pub struct Transform {
    pub s: f32,
    pub t: f32,
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
pub fn agarwal(p: Vec<Point>, q: Vec<Point>) -> Transform {
    let m = p.len();
    let n = q.len();
    let mut q_boundaries = vec![];
    for j in 0..n {
        let Point { x: x1, y: y1 } = q[j];
        let Point { x: x2, y: y2 } = q[(j + 1) % n];
        let d = x1 * y2 - x2 * y1;
        q_boundaries.push(((y2 - y1) / d, (x1 - x2) / d))
    }
    let mut g = vec![];
    for j in 0..n {
        let mut gj = vec![];
        let (aj, bj) = q_boundaries[j];
        for i in 0..m {
            let Point { x: xi, y: yi } = p[i];
            gj.push((aj * xi + bj * yi, -aj * yi + bj * xi))
        }
        g.push((gj, (aj, bj)));
    }

    let mut vertices = vec![];
    for j1 in 0..n {
        let (a1, b1) = g[j1].1;
        for j2 in 0..n {
            let (a2, b2) = g[j2].1;
            if a1 * a2 < 0. {
                let v = (b1 * a2 - b2 * a1) / (a2 - a1);
                for i1 in 0..m {
                    let (s1, t1) = g[j1].0[i1];
                    for i2 in 0..m {
                        let (s2, t2) = g[j1].0[i2];
                        vertices.push(nalgebra::Point3::new(s1 + s2, t1 + t2, v))
                    }
                }
            }
        }
    }
    let (points, edges) = parry3d::transformation::convex_hull(&vertices);
    let mut c2_vertices = vec![];
    for indices in edges.iter() {
        let mut d2_vertices = vec![];
        for k in 0..3 {
            let i = indices[k] as usize;
            let j = indices[(k + 1) % 3] as usize;
            let x1 = points[i].x;
            let y1 = points[i].y;
            let z1 = points[i].z;
            let x2 = points[j].x;
            let y2 = points[j].y;
            let z2 = points[j].z;
            if z1 * z2 < 0. {
                let r = z1 / (z1 - z2);
                d2_vertices.push((x1 * r + x2 * (1. - r), y1 * r + y2 * (1. - r)));
            }
        }
        if d2_vertices.len() == 2 {
            let (x1, y1) = d2_vertices[0];
            let (x2, y2) = d2_vertices[1];
            let d = x1 * y2 - x2 * y1;
            c2_vertices.push(((y2 - y1) / d, (x1 - x2) / d))
        }
    }
    Transform {
        s: 1.,
        t: 0.,
        x: 0.,
        y: 0.,
    }
}
