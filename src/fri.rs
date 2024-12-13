use crate::merkle::{merkelize, hash, verify_branch, get_branch};
use crate::fft::{fft, inv_fft, get_initial_domain_of_size, log2, halve_domain, get_single_domain_value, halve_single_domain_value};

const BASE_CASE_SIZE = 128
const FOLDS_PER_ROUND = 3
const FOLD_SIZE_RATIO = 2**FOLDS_PER_ROUND
const NUM_CHALLENGES = 80

fn extend_trace(field: u32, trace: &[CirclePoint]) -> vec<CirclePoint>{
    let small_domain = get_initial_domain_of_size(field, len(trace));
    let coeffs = fft(trace, small_domain);
    let big_domain = get_initial_domain_of_size(field, len(trace)*2);
    let res = inv_fft(trace, big_domain);
    res
}

fn line_function(P1: CirclePoint, P2: CirclePoint, domain: &[CirclePoint]) -> vec<CirclePoint>{
    let x1 = P1.x;
    let x2 = P2.x;
    let y1 = P1.y;
    let y2 = P2.y;
    let denominator = x1*y2 - x2*y1;
    let a = (y2-y1) / denominator;
    let b = (x1-x2) / denominator;
    let c = -1 * (a*x1 + b*y1);
    domain.iter().map(|&(x,y)| a*x + b*y + c).collect();
}

fn rbo(values: &[CirclePoint]) -> vec<CirclePoint> {
    if values.len() == 1{
        return values;
    }
    let L = rbo(values.iter().step_by(2).collect());
    let R = rbo(values.iter().skip(1).step_by(2).collect());
    let res = L;
    res.extend(R);
    return res;
}

fn fold_reverse_bit_order(values: &[CirclePoint]) -> vec<CirclePoint>{
    let L = rbo(values.iter().step_by(2).collect());
    let R = rbo(values.iter().skip(1).step_by(2).rev().cloned().collect());
    let o = vec![None; values.len()];
    let mut i = 0;
    for x in L{
        o[i] = x;
        i+=2;
    }
    let mut j = 1;
    for x in R{
        o[j] = x;
        i+=2;
    }
    o
}
