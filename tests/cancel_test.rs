use convert_typst_math_to_mathml::mathml::convert_to_mathml;

macro_rules! test_math_function {
    ($( $func_name:ident, $test_func_name:ident );* $(;)?) => {
        $(
            #[test]
            fn $test_func_name() {
                let result = $func_name();
                for item in result {
                    println!("{}", item);
                }
            }
        )*
    };
}

fn math_cancel() -> Vec<String> {
    let inputs = [r#"$ (a dot b dot cancel(x)) / cancel(x) $"#];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

fn cancel_length() -> Vec<String> {
    let inputs = [r#"$ a + cancel(x, length: #200%)
     - cancel(x, length: #200%) $"#];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

fn cancel_inverted() -> Vec<String> {
    let inputs = [r#"$ (a cancel((b + c), inverted: #true)) /
    cancel(b + c, inverted: #true) $"#];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

fn cancel_cross() -> Vec<String> {
    let inputs = [r#"$ cancel(Pi, cross: #true) $"#];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

fn cancel_angle() -> Vec<String> {
    let inputs = [r#"$ cancel(Pi)
  cancel(Pi, angle: #0deg)
  cancel(Pi, angle: #45deg)
  cancel(Pi, angle: #90deg)
  cancel(1/(1+x), angle: #(a => a + 45deg))
  cancel(1/(1+x), angle: #(a => a + 90deg)) $"#];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

fn cancel_stroke() -> Vec<String> {
    let inputs = [r#"$ cancel(
  sum x,
  stroke: #(
    paint: red,
    thickness: 1.5pt,
    dash: "dashed",
  ),
) $"#];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

test_math_function!(
    math_cancel, test_math_cancel;
    cancel_length, test_cancel_length;
    cancel_inverted, test_cancel_inverted;
    cancel_cross, test_cancel_cross;
    cancel_angle, test_cancel_angle;
    cancel_stroke, test_cancel_stroke;
);
