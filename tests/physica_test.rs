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

const BASIC: &str = include_str!("./physica.typ");

fn physica_transpose() -> Vec<String> {
    let base = BASIC.to_string();

    let inputs = [&(base + r#"$ A^T $"#)];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

fn physica_fxydxdy() -> Vec<String> {
    let base = BASIC.to_string();

    let inputs = [&(base + r#"$ f(x,y) dd(x,y), $"#)];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

test_math_function!(
    physica_transpose, test_physica_transpose;
    physica_fxydxdy, test_physica_fxydxdy;
);
