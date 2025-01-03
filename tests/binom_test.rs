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

fn math_binom() -> Vec<String> {
    let inputs = [
        r#"$ binom(n, k) $"#,
        r#"$ binom(n, k_1, k_2, k_3, ..., k_m) $"#,
    ];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

test_math_function!(
    math_binom, test_math_binom;
);
