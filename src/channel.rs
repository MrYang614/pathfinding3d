use crate::impls::{Portal3, Vec3};
use crate::math::{distance_sq_segment_to_segment, judge_dir, vequal};

pub(crate) fn funnel3d_into(start: Vec3, end: Vec3, portals: &[Portal3], path: &mut Vec<Vec3>) {
    path.clear();
    if portals.is_empty() {
        path.push(start);
        path.push(end);
        return;
    }

    path.push(start);
    let mut portal_apex = start;
    let mut portal_left = portals[0].left;
    let mut portal_right = portals[0].right;
    let mut apex_index = 0usize;
    let mut left_index = 0usize;
    let mut right_index = 0usize;
    let mut i = 1usize;

    while i < portals.len() {
        let left = portals[i].left;
        let right = portals[i].right;

        if judge_dir(&portal_apex, &portal_right, &right) >= 0.0 {
            if vequal(&portal_apex, &portal_right)
                || judge_dir(&portal_apex, &portal_left, &right) < 0.0
            {
                portal_right = right;
                right_index = i;
            } else {
                insert(
                    portal_apex,
                    portal_left,
                    apex_index,
                    left_index,
                    portals,
                    path,
                );
                path.push(portal_left);
                portal_apex = portal_left;
                apex_index = left_index;
                portal_right = portal_apex;
                right_index = apex_index;
                i = apex_index + 1;
                continue;
            }
        }

        if judge_dir(&portal_apex, &portal_left, &left) <= 0.0 {
            if vequal(&portal_apex, &portal_left)
                || judge_dir(&portal_apex, &portal_right, &left) > 0.0
            {
                portal_left = left;
                left_index = i;
            } else {
                insert(
                    portal_apex,
                    portal_right,
                    apex_index,
                    right_index,
                    portals,
                    path,
                );
                path.push(portal_right);
                portal_apex = portal_right;
                apex_index = right_index;
                portal_left = portal_apex;
                left_index = apex_index;
                i = apex_index + 1;
                continue;
            }
        }

        i += 1;
    }

    if path
        .last()
        .map(|p| !vequal(p, &portals[portals.len() - 1].left))
        .unwrap_or(true)
    {
        path.push(portals[portals.len() - 1].left);
    }
}

fn insert(
    p1: Vec3,
    p2: Vec3,
    pre_idx: usize,
    end_idx: usize,
    portals: &[Portal3],
    path: &mut Vec<Vec3>,
) {
    if end_idx <= pre_idx + 1 {
        return;
    }

    for portal in portals.iter().take(end_idx - 1).skip(pre_idx) {
        let l = portal.left;
        let r = portal.right;
        let (dist, _, _) = distance_sq_segment_to_segment(p1, p2, l, r);
        if dist > 0.01 {
            let (_, _, intersect_xz) = distance_sq_segment_to_segment_xz(
                (p1.x, p1.z),
                (p2.x, p2.z),
                (l.x, l.z),
                (r.x, r.z),
            );
            let delta = segment_fraction_xz((l.x, l.z), (r.x, r.z), intersect_xz);
            if (0.0..=1.0).contains(&delta) {
                path.push(l.lerp(r, delta));
            }
        }
    }
}

fn clamp(value: f64, min_v: f64, max_v: f64) -> f64 {
    value.max(min_v).min(max_v)
}

fn distance_sq_segment_to_segment_xz(
    p1: (f64, f64),
    q1: (f64, f64),
    p2: (f64, f64),
    q2: (f64, f64),
) -> (f64, (f64, f64), (f64, f64)) {
    let epsilon = 1e-8 * 1e-8;
    let d1 = (q1.0 - p1.0, q1.1 - p1.1);
    let d2 = (q2.0 - p2.0, q2.1 - p2.1);
    let r = (p1.0 - p2.0, p1.1 - p2.1);
    let a = d1.0 * d1.0 + d1.1 * d1.1;
    let e = d2.0 * d2.0 + d2.1 * d2.1;
    let f = d2.0 * r.0 + d2.1 * r.1;
    let mut s;
    let mut t;

    if a <= epsilon && e <= epsilon {
        let c1 = p1;
        let c2 = p2;
        let dx = c1.0 - c2.0;
        let dz = c1.1 - c2.1;
        return (dx * dx + dz * dz, c1, c2);
    }

    if a <= epsilon {
        s = 0.0;
        t = clamp(f / e, 0.0, 1.0);
    } else {
        let c = d1.0 * r.0 + d1.1 * r.1;
        if e <= epsilon {
            t = 0.0;
            s = clamp(-c / a, 0.0, 1.0);
        } else {
            let b = d1.0 * d2.0 + d1.1 * d2.1;
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

    let c1 = (p1.0 + d1.0 * s, p1.1 + d1.1 * s);
    let c2 = (p2.0 + d2.0 * t, p2.1 + d2.1 * t);
    let dx = c1.0 - c2.0;
    let dz = c1.1 - c2.1;
    (dx * dx + dz * dz, c1, c2)
}

fn segment_fraction_xz(start: (f64, f64), end: (f64, f64), point: (f64, f64)) -> f64 {
    let dx = end.0 - start.0;
    let dz = end.1 - start.1;
    let len_sq = dx * dx + dz * dz;
    if len_sq <= f64::EPSILON {
        return 0.0;
    }
    let px = point.0 - start.0;
    let pz = point.1 - start.1;
    clamp((px * dx + pz * dz) / len_sq, 0.0, 1.0)
}
