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

fn math_attach() -> Vec<String> {
    let inputs = [r#"$ sum_(i=0)^n a_i = 2^(1+i) $"#];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

fn attach_script() -> Vec<String> {
    let inputs = [r#"$ scripts(sum)_1^2 != sum_1^2 $"#];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

fn attach_limits() -> Vec<String> {
    let inputs = [r#"$ limits(A)_1^2 != A_1^2 $"#];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

test_math_function!(
    math_attach, test_math_attach;
    attach_script, test_attach_script;
    attach_limits, test_attach_limits;
);
