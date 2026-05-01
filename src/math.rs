use crate::impls::{PolygonInput, Vec3};

pub(crate) fn clamp(value: f64, min_v: f64, max_v: f64) -> f64 {
    value.max(min_v).min(max_v)
}

pub(crate) fn vequal(a: &Vec3, b: &Vec3) -> bool {
    a.distance_squared(*b) < 0.0001
}

pub(crate) fn judge_dir(a: &Vec3, b: &Vec3, c: &Vec3) -> f64 {
    (*b - *a).cross(*c - *a).y
}

pub(crate) fn is_point_in_triangle(a: Vec3, b: Vec3, c: Vec3, pt: Vec3) -> bool {
    let ab = b - a;
    let bc = c - b;
    let ca = a - c;
    let ap = pt - a;
    let bp = pt - b;
    let cp = pt - c;
    let cross_ab = ab.cross(ap);
    let cross_bc = bc.cross(bp);
    let cross_ca = ca.cross(cp);
    (cross_ab.y >= 0.0 && cross_bc.y >= 0.0 && cross_ca.y >= 0.0)
        || (cross_ab.y <= 0.0 && cross_bc.y <= 0.0 && cross_ca.y <= 0.0)
}

pub(crate) fn point_to_plane_distance(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3) -> f64 {
    let n = (*b - *a).cross(*c - *a);
    let len = n.length_squared().sqrt();
    if len == 0.0 {
        return 0.0;
    }
    n.dot(*p - *a) / len
}

pub(crate) fn is_vector_in_polygon(
    vector: &Vec3,
    polygon: &PolygonInput,
    vertices: &[Vec3],
) -> bool {
    let mut lowest = f64::INFINITY;
    let mut highest = f64::NEG_INFINITY;
    let mut poly_vertices = [Vec3::ZERO; 3];

    for (i, vid) in polygon.vertex_indices.iter().enumerate() {
        let v = vertices[*vid];
        lowest = lowest.min(v.y);
        highest = highest.max(v.y);
        poly_vertices[i] = v;
    }

    vector.y < highest + 0.5
        && vector.y > lowest - 0.5
        && is_point_in_triangle(
            poly_vertices[0],
            poly_vertices[1],
            poly_vertices[2],
            *vector,
        )
}

pub(crate) fn distance_sq_segment_to_segment(
    p1: Vec3,
    q1: Vec3,
    p2: Vec3,
    q2: Vec3,
) -> (f64, Vec3, Vec3) {
    let epsilon = 1e-8 * 1e-8;
    let d1 = q1 - p1;
    let d2 = q2 - p2;
    let r = p1 - p2;
    let a = d1.dot(d1);
    let e = d2.dot(d2);
    let f = d2.dot(r);
    let mut s;
    let mut t;

    if a <= epsilon && e <= epsilon {
        let c1 = p1;
        let c2 = p2;
        return (c1.distance_squared(c2), c1, c2);
    }

    if a <= epsilon {
        s = 0.0;
        t = clamp(f / e, 0.0, 1.0);
    } else {
        let c = d1.dot(r);
        if e <= epsilon {
            t = 0.0;
            s = clamp(-c / a, 0.0, 1.0);
        } else {
            let b = d1.dot(d2);
            let denom = a * e - b * b;
            if denom != 0.0 {
                s = clamp((b * f - c * e) / denom, 0.0, 1.0);
            } else {
                s = 0.0;
            }
            t = (b * s + f) / e;
            if t < 0.0 {
                t = 0.0;
                s = clamp(-c / a, 0.0, 1.0);
            } else if t > 1.0 {
                t = 1.0;
                s = clamp((b - c) / a, 0.0, 1.0);
            }
        }
    }

    let c1 = p1 + d1 * s;
    let c2 = p2 + d2 * t;
    (c1.distance_squared(c2), c1, c2)
}
