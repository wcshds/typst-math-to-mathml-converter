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

fn math_enonomics() -> Vec<String> {
    let inputs = [
        r#"$
  upright(d) y = (partial F) / (partial t) upright(d) t + (partial F) / (partial k) upright(d) k + (partial F) / (partial cal(l)) upright(d) cal(l) = (partial F) / (partial t) upright(d) t + (partial F) / (partial k) k (upright(d) k) / k + (partial F) / (partial cal(l)) cal(l) (upright(d) cal(l)) / cal(l),
$"#,
        r#"$
  (upright(d) y) / y = (partial F) / (partial t) (upright(d) t) / y + (r k) / y (upright(d) k) / k + (w cal(l)) / y (upright(d) cal(l)) / cal(l).
$"#,
        r#"$
  max_(k, cal(l)) F_t (k, cal(l)) - r k - w cal(l).
$"#,
        r#"$
  (upright(d) y) / y = (partial F) / (partial t) (upright(d) t) / F + alpha (upright(d) k) / k + (1 - alpha) (upright(d) cal(l)) / cal(l).
$"#,
        r#"$
  (upright(d) y) / y = (upright(d) z) / z + alpha (upright(d) k) / k + (1 - alpha) (upright(d) cal(l)) / cal(l)
$"#,
        r#"$
  (upright(d) y) / y = (1 - alpha) (upright(d) z) / z + alpha (upright(d) k) / k + (1 - alpha) (upright(d) cal(l)) / cal(l)
$"#,
        r#"$
  k_(t + 1) = (1 - delta) k_t + i_t.
$"#,
        r#"$
  k_(t + 1) / y_(t + 1) y_(t + 1) / y_t = (1 - delta) k_t / y_t + i_t / y_t
$"#,
        r#"$
  k_(t + 1) / y_(t + 1) (1 + gamma_t) = (1 - delta) k_t / y_t + i_t / y_t.
$"#,
        r#"$
  k_t / y_t = s / (gamma + delta)
$"#,
        r#"$
  k_(t + 1) = s F(k_t, (1 + gamma)^t cal(l)) + (1 - delta) k_t
$"#,
        r#"$
  (1 + gamma) dash(tilde(k)) = s F(dash(tilde(k)), cal(l)) + (1 - delta) dash(tilde(k)).
$"#,
        r#"$ 
    (1 + gamma) tilde(k)_(t + 1) = s F(tilde(k)_t, cal(l)) + (1 - delta) tilde(k)_t;
$"#,
    ];

    inputs.map(|input| convert_to_mathml(input, false)).to_vec()
}

test_math_function!(
    math_enonomics, test_math_enonomics;
);
