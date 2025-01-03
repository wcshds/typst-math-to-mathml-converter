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

fn math_accent() -> Vec<String> {
    let inputs = [
        r#"$ grave(a) = accent(a, `) $"#,
        r#"$ arrow(a) = accent(a, arrow) $"#,
        r#"$ tilde(a) = accent(a, \u{0303}) $"#,
    ];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

fn accent_base() -> Vec<String> {
    let inputs = [r#"$ arrow(A B C) $"#];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

test_math_function!(
    math_accent, test_math_accent;
    accent_base, test_accent_base;
);
