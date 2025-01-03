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

fn math_variables() -> Vec<String> {
    let inputs = [
        r#"$ A = pi r^2 $"#,
        r#"$ "area" = pi dot "radius"^2 $"#,
        r#"$ cal(A) :=
        { x in RR | x "is natural" } $"#,
        r#"#let x = 5
    $ #x < 17 $"#,
    ];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

fn math_symbols() -> Vec<String> {
    let inputs = [r#"$ x < y => x gt.eq.not y $"#];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

fn math_linebreaks() -> Vec<String> {
    let inputs = [r#"$ sum_(k=0)^n k
    &= 1 + ... + n \
    &= (n(n+1)) / 2 $
"#];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

fn math_function_calls() -> Vec<String> {
    let inputs = [
        r#"$ frac(a^2, 2) $"#,
        r#"$ vec(1, 2, delim: "[") $"#,
        r#"$ mat(1, 2; 3, 4) $"#,
        r#"$ lim_x = op("lim", limits: #true)_x $"#,
    ];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

fn math_alignment() -> Vec<String> {
    let inputs = [r#"$ (3x + y) / 7 &= 9 && "given" \
  3x + y &= 63 & "multiply by 7" \
  3x &= 63 - y && "subtract y" \
  x &= 21 - y/3 & "divide by 3" $
"#];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

test_math_function!(
    math_variables, test_math_variables;
    math_symbols, test_math_symbols;
    math_linebreaks, test_math_linebreaks;
    math_function_calls, test_math_function_calls;
    math_alignment, test_math_alignment;
);
