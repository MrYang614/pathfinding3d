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

/// Returns the nearest point on the closed triangle in three dimensions.
/// Collinear or coincident vertices describe a segment or a single point.
pub(crate) fn closest_point_on_triangle(point: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3) -> Vec3 {
    let ab = *b - *a;
    let ac = *c - *a;
    let normal = ab.cross(ac);
    let normal_squared = normal.length_squared();
    if normal_squared == 0.0 {
        let ab_squared = ab.length_squared();
        let ac_squared = ac.length_squared();
        let bc_squared = b.distance_squared(*c);
        // The longest edge spans all three vertices of a degenerate triangle.
        let (start, end) = if ab_squared >= ac_squared && ab_squared >= bc_squared {
            (a, b)
        } else if ac_squared >= bc_squared {
            (a, c)
        } else {
            (b, c)
        };
        return closest_point_on_segment(point, start, end);
    }

    // These two axes lie in the triangle plane. A large perpendicular component
    // of the query therefore does not enter the barycentric coordinates.
    let weight_b = projected_offset(point, a, ac.cross(normal) / normal_squared);
    let weight_c = projected_offset(point, a, normal.cross(ab) / normal_squared);
    if weight_b >= 0.0 && weight_c >= 0.0 && weight_b + weight_c <= 1.0 {
        return *a + ab * weight_b + ac * weight_c;
    }

    let mut nearest = closest_point_on_segment(point, a, b);
    for candidate in [
        closest_point_on_segment(point, b, c),
        closest_point_on_segment(point, a, c),
    ] {
        if is_closer_surface_point(point, candidate, nearest) {
            nearest = candidate;
        }
    }
    nearest
}

fn closest_point_on_segment(point: &Vec3, start: &Vec3, end: &Vec3) -> Vec3 {
    let edge = *end - *start;
    let along = projected_offset(point, start, edge);
    if along <= 0.0 {
        return *start;
    }
    if projected_offset(point, end, edge) >= 0.0 {
        return *end;
    }
    *start + edge * (along / edge.length_squared())
}

/// Strictly compares distances without squaring the potentially large query offset.
pub(crate) fn is_closer_surface_point(query: &Vec3, candidate: Vec3, current: Vec3) -> bool {
    let delta = candidate - current;
    // Expand |query - candidate|² around current and cancel |query - current|².
    projected_offset(query, &current, delta) > delta.length_squared() * 0.5
}

fn projected_offset(point: &Vec3, origin: &Vec3, axis: Vec3) -> f64 {
    let terms = (*point - *origin) * axis;
    let projected = terms.element_sum();
    let largest = terms.abs().max_element();
    if largest == 0.0 || (projected.is_finite() && projected.abs() > largest * (8.0 * f64::EPSILON))
    {
        return projected;
    }

    // Severe cancellation can erase the origin's contribution when a query is
    // far away. Subtract that projection afterwards. Scaling keeps all products
    // and sums finite, and adding the smallest term last preserves it.
    let scale = axis.abs().max_element().max(1.0) * 4.0;
    let scaled_axis = axis / scale;
    let terms = *point * scaled_axis;
    let abs = terms.abs();
    let projected = if abs.x <= abs.y && abs.x <= abs.z {
        (terms.y + terms.z) + terms.x
    } else if abs.y <= abs.z {
        (terms.x + terms.z) + terms.y
    } else {
        (terms.x + terms.y) + terms.z
    };
    (projected - origin.dot(scaled_axis)) * scale
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projects_onto_face_edge_and_vertex_in_three_dimensions() {
        let a = Vec3::new(2.0, 0.0, 0.0);
        let b = Vec3::new(2.0, 4.0, 0.0);
        let c = Vec3::new(2.0, 0.0, 4.0);
        for (query, expected) in [
            (Vec3::new(-6.0, 1.0, 2.0), Vec3::new(2.0, 1.0, 2.0)),
            (Vec3::new(1e15, 3.0, 3.0), Vec3::new(2.0, 2.0, 2.0)),
            (Vec3::new(2.0, -1.0, 6.0), c),
        ] {
            assert_eq!(closest_point_on_triangle(&query, &a, &b, &c), expected);
            assert_eq!(closest_point_on_triangle(&query, &a, &c, &b), expected);
        }
    }

    #[test]
    fn projects_onto_collapsed_triangles() {
        let a = Vec3::ZERO;
        let b = Vec3::new(1.0, 1.0, 1.0);
        let c = Vec3::new(4.0, 4.0, 4.0);
        let query = Vec3::new(2.0, 3.0, 1.0);
        let expected = Vec3::splat(2.0);
        assert_eq!(closest_point_on_triangle(&query, &b, &c, &a), expected);
        assert_eq!(closest_point_on_triangle(&query, &a, &a, &c), expected);
        assert_eq!(closest_point_on_triangle(&query, &b, &b, &b), b);
    }

    #[test]
    fn retains_in_plane_coordinates_for_distant_queries() {
        let query = Vec3::new(2.0, 3.0, f64::MAX);
        let a = Vec3::ZERO;
        let b = Vec3::X * 10.0;
        let c = Vec3::Y * 10.0;
        let closest = closest_point_on_triangle(&query, &a, &b, &c);
        assert!(closest.distance(Vec3::new(2.0, 3.0, 0.0)) < 1e-14);
        assert!(is_closer_surface_point(&query, closest, a));
        assert!(!is_closer_surface_point(&query, a, closest));
    }

    #[test]
    fn compares_distant_surface_points_across_a_diagonal() {
        let a = Vec3::ZERO;
        let b = Vec3::new(2.0, 2.0, -2.0);
        for distance in [1e20, f64::MAX] {
            let query = Vec3::new(distance, 3.0, distance);
            // The query is in the bisector plane. Moving one unit in y breaks the tie.
            assert!(!is_closer_surface_point(&query, a, b));
            assert!(!is_closer_surface_point(&query, b, a));
            let query = Vec3::new(query.x, 2.0, query.z);
            assert!(is_closer_surface_point(&query, a, b));
            assert!(!is_closer_surface_point(&query, b, a));
        }
    }
}
