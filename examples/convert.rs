use convert_typst_math_to_mathml::mathml::convert_to_mathml;

fn main() {
    let input = r#"
$
&mu_0 gradient f(x^*) + sum_(i = 1)^m mu_i gradient g_i (x^*) + sum_(j = 1) ^ cal(l) lambda_j gradient h_j (x^*) = 0, \

&mu_j g_i (x^*) = 0, quad i = 1, dots, m,
$
"#;

    println!("{}", convert_to_mathml(input, false));
}
