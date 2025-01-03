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

fn math_class() -> Vec<String> {
    let inputs = [r#"#let loves = math.class(
  "relation",
  sym.suit.heart,
)

$ x loves y and y loves 5 $"#];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

test_math_function!(
    math_class, test_math_class;
);
