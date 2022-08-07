use mathml_core::{MathML, MathRoot};

/// Cast isize matrix to f64 matrix
pub fn cast_isize_to_f64(v: Vec<Vec<isize>>) -> Vec<Vec<f64>> {
    v.iter().map(|v| v.iter().map(|&x| x as f64).collect()).collect()
}

/// Display MathML with namespace and display style
pub fn display_mathml<M>(math: M) -> MathML
where
    M: Into<MathML>,
{
    MathML::from(MathRoot::new(vec![math.into()]).with_namespace().with_display_style(true))
}
