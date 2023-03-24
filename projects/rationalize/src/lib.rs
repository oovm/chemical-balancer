use num::rational::Ratio;

/// Expand a float to a continued fraction expansion
///
/// # Arguments
///
/// * `n`:  The float to expand
/// * `min`:  Numbers less than this value are treated as zero
///
/// returns: [usize; N] where N is the number of terms in the expansion
///
/// # Examples
///
/// ```
/// # use rationalize::continued_fraction_expansion;
/// let cfe = continued_fraction_expansion::<10>(std::f64::consts::PI, 1e-10);
/// assert_eq!(cfe, [3, 7, 15, 1, 292, 1, 1, 1, 2, 1]);
/// ```
pub fn continued_fraction_expansion<const N: usize>(n: f64, min: f64) -> [usize; N] {
    let mut out = [0; N];
    let mut n = n;
    for i in 0..N {
        let integer = n.floor() as usize;
        out[i] = integer;
        n -= integer as f64;
        if n <= min {
            break;
        }
        n = 1.0 / n;
    }
    out
}

/// Convert a float to a rational number
///
/// # Arguments
///
/// * `n`:  The float to convert
/// * `min`:  Numbers less than this value are treated as zero
///
/// returns: Ratio<usize>
///
/// # Examples
///
/// ```
/// use num::rational::Ratio;
/// use rationalize::float2ratio;
/// let cfe = float2ratio::<4>(std::f64::consts::PI, 1e-10);
/// assert_eq!(cfe, Ratio::new(355, 113));
/// ```
pub fn float2ratio<const N: usize>(n: f64, min: f64) -> Ratio<isize> {
    let cfe = continued_fraction_expansion::<N>(n, min);
    let (numer, denom) = build_ratio(&cfe).into();
    match n.is_sign_negative() {
        true => Ratio::new(-(numer as isize), denom as isize),
        false => Ratio::new(numer as isize, denom as isize),
    }
}

// build ratio from continued fraction expansion
pub fn build_ratio(cfe: &[usize]) -> Ratio<usize> {
    let seq = trim_tail_zeros(cfe);
    let mut out = Ratio::new(seq[seq.len() - 1], 1);
    for i in (0..seq.len() - 1).rev() {
        out = Ratio::new(seq[i], 1) + out.recip();
    }
    out
}

pub fn trim_tail_zeros(cfe: &[usize]) -> &[usize] {
    let mut i = cfe.len() - 1;
    while i > 0 && cfe[i] == 0 {
        i -= 1;
    }
    &cfe[0..=i]
}
