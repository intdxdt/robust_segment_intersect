extern crate robust_orientation;

use robust_orientation::orientation_2d;

//Checks if two Segments Intersect
pub fn segment_intersects(a0: &[f64], a1: &[f64], b0: &[f64], b1: &[f64]) -> bool {
    let x0 = orientation_2d(a0, b0, b1);
    let y0 = orientation_2d(a1, b0, b1);
    if (x0 > 0. && y0 > 0.) || (x0 < 0. && y0 < 0.) {
        return false;
    }

    let x1 = orientation_2d(b0, a0, a1);
    let y1 = orientation_2d(b1, a0, a1);
    if (x1 > 0. && y1 > 0.) || (x1 < 0. && y1 < 0.) {
        return false;
    }

    //Check for degenerate collinear case
    if x0 == 0. && y0 == 0. && x1 == 0. && y1 == 0. {
        return check_collinear(a0, a1, b0, b1);
    }
    return true;
}

fn check_collinear(a0: &[f64], a1: &[f64], b0: &[f64], b1: &[f64]) -> bool {
    for d in 0..2 {
        let x0 = a0[d];
        let y0 = a1[d];
        let l0 = x0.min(y0);
        let h0 = x0.max(y0);

        let x1 = b0[d];
        let y1 = b1[d];
        let l1 = x1.min(y1);
        let h1 = x1.max(y1);

        if h1 < l0 || h0 < l1 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod test_seg_intersects {
    #[test]
    fn seg_intersects() {}
}
