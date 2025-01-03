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

fn math_cases() -> Vec<String> {
    let inputs = [r#"$ f(x, y) := cases(
  1 "if" (x dot y)/2 <= 0,
  2 "if" x "is even",
  3 "if" x in NN,
  4 "else",
) $"#];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

fn cases_delim() -> Vec<String> {
    let inputs = [r#"#set math.cases(delim: "[")
$ x = cases(1, 2) $"#];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

fn cases_reverse() -> Vec<String> {
    let inputs = [r#"#set math.cases(reverse: true)
$ cases(1, 2) = x $"#];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

fn cases_gap() -> Vec<String> {
    let inputs = [r#"#set math.cases(gap: 1em)
$ x = cases(1, 2) $"#];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

test_math_function!(
    math_cases, test_math_cases;
    cases_delim, test_cases_delim;
    cases_reverse, test_cases_reverse;
    cases_gap, test_cases_gap;
);
