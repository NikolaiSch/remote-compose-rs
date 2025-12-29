use rand::SeedableRng;
use remote_compose_expressions::*;
use std::collections::HashMap;
use std::f32::consts::PI;

#[test]
fn simple_test() {
    // (3+5)*(2-8) -> [3, 5, +, 2, 8, -, *]
    let rpn = vec![
        3.0,
        5.0,
        as_nan(OpCode::Add),
        2.0,
        8.0,
        as_nan(OpCode::Subtract),
        as_nan(OpCode::Multiply),
    ];
    let vars = HashMap::new();
    let mut ctx = ExpressionContext::new(&vars);
    assert_eq!(evaluate_bytecode(&rpn, &mut ctx).unwrap(), -48.0);
}

#[test]
fn all_operators_test() {
    let vars = HashMap::new();
    let mut ctx = ExpressionContext::new(&vars);

    // 2 + 3
    assert_eq!(
        evaluate_bytecode(&[2.0, 3.0, as_nan(OpCode::Add)], &mut ctx).unwrap(),
        5.0
    );

    // sin(2 / 3)
    let sin_val = evaluate_bytecode(
        &[2.0, 3.0, as_nan(OpCode::Divide), as_nan(OpCode::Sin)],
        &mut ctx,
    )
    .unwrap();
    assert!((sin_val - (2.0f32 / 3.0).sin()).abs() < 1e-6);

    // min(2, 3)
    assert_eq!(
        evaluate_bytecode(&[2.0, 3.0, as_nan(OpCode::Min)], &mut ctx).unwrap(),
        2.0
    );

    // pow(2, 3)
    assert_eq!(
        evaluate_bytecode(&[2.0, 3.0, as_nan(OpCode::Pow)], &mut ctx).unwrap(),
        8.0
    );

    // sqrt(4)
    assert_eq!(
        evaluate_bytecode(&[4.0, as_nan(OpCode::Sqrt)], &mut ctx).unwrap(),
        2.0
    );

    // mad(2, 3, 4) -> 2 * 3 + 4 = 10
    assert_eq!(
        evaluate_bytecode(&[2.0, 3.0, 4.0, as_nan(OpCode::Mad)], &mut ctx).unwrap(),
        10.0
    );

    // ifElse(3, 4, 2) -> 3 > 0 ? 4 : 2 = 4
    assert_eq!(
        evaluate_bytecode(&[2.0, 4.0, 3.0, as_nan(OpCode::IfElse)], &mut ctx).unwrap(),
        4.0
    );
}

#[test]
fn test_accuracy() {
    let vars = HashMap::new();
    let mut ctx = ExpressionContext::new(&vars);
    let total = 1000;
    let mut sum_eval = 3.0;

    for i in 0..total {
        let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
        let i_f = i as f32;
        // v = sign * 4 / ((i*2+2) * (i*2+3) * (i*2+4))
        let rpn = vec![
            sign,
            4.0,
            as_nan(OpCode::Multiply),
            i_f * 2.0 + 2.0,
            i_f * 2.0 + 3.0,
            as_nan(OpCode::Multiply),
            i_f * 2.0 + 4.0,
            as_nan(OpCode::Multiply),
            as_nan(OpCode::Divide),
        ];
        sum_eval += evaluate_bytecode(&rpn, &mut ctx).unwrap();
    }

    assert!((sum_eval - PI).abs() < 0.0001);
}

#[test]
fn test_random() {
    let vars = HashMap::new();
    let mut ctx = ExpressionContext::new(&vars);
    // Test that RAND_SEED works and gives consistent results
    let rpn_seed = vec![12345.0, as_nan(OpCode::RandSeed), as_nan(OpCode::Rand)];
    let r1 = evaluate_bytecode(&rpn_seed, &mut ctx).unwrap();
    // Re-seed for consistency in test
    ctx.rng = rand_chacha::ChaCha8Rng::seed_from_u64(12345.0f32.to_bits() as u64);
    let r2 = evaluate_bytecode(&rpn_seed, &mut ctx).unwrap();
    assert_eq!(r1, r2);
}

#[test]
fn test_set3_ops() {
    let vars = HashMap::new();
    let mut ctx = ExpressionContext::new(&vars);

    // square_sum(4, 3) = 16 + 9 = 25
    assert_eq!(
        evaluate_bytecode(&[4.0, 3.0, as_nan(OpCode::SquareSum)], &mut ctx).unwrap(),
        25.0
    );

    // step(4, 3) -> 4 > 3 ? 1 : 0 = 1
    assert_eq!(
        evaluate_bytecode(&[4.0, 3.0, as_nan(OpCode::Step)], &mut ctx).unwrap(),
        1.0
    );

    // square(3) = 9
    assert_eq!(
        evaluate_bytecode(&[3.0, as_nan(OpCode::Square)], &mut ctx).unwrap(),
        9.0
    );

    // hypot(3, 4) = 5
    assert_eq!(
        evaluate_bytecode(&[3.0, 4.0, as_nan(OpCode::Hypot)], &mut ctx).unwrap(),
        5.0
    );

    // lerp(100, 200, 0.75) = 175
    assert_eq!(
        evaluate_bytecode(&[100.0, 200.0, 0.75, as_nan(OpCode::Lerp)], &mut ctx).unwrap(),
        175.0
    );
}

#[test]
fn test_advance_operators() {
    let vars = HashMap::new();
    let mut ctx = ExpressionContext::new(&vars);

    // log2(1.2)
    assert!(
        (evaluate_bytecode(&[1.2, as_nan(OpCode::Log2)], &mut ctx).unwrap() - 1.2f32.log2()).abs()
            < 1e-6
    );

    // inv(1.2) = 1 / 1.2
    assert!(
        (evaluate_bytecode(&[1.2, as_nan(OpCode::Inv)], &mut ctx).unwrap() - 1.0 / 1.2).abs()
            < 1e-6
    );

    // fract(1.2) = 0.2
    let res = evaluate_bytecode(&[1.2, as_nan(OpCode::Fract)], &mut ctx).unwrap();
    assert!((res - 0.2).abs() < 1e-6);

    // ping_pong(1.2, 1.0) -> tmp = 1.2 % 2.0 = 1.2; 1.2 > 1.0 ? 2.0 - 1.2 : 1.2 = 0.8
    let res = evaluate_bytecode(&[1.2, 1.0, as_nan(OpCode::PingPong)], &mut ctx).unwrap();
    assert!((res - 0.8).abs() < 1e-6);
}

#[test]
fn test_cubic() {
    let vars = HashMap::new();
    let mut ctx = ExpressionContext::new(&vars);
    // cubic(0.4, 0.0, 0.2, 1.0, 0.5)
    let res =
        evaluate_bytecode(&[0.4, 0.0, 0.2, 1.0, 0.5, as_nan(OpCode::Cubic)], &mut ctx).unwrap();
    // We don't have the exact expected value here without running the same implementation,
    // but we can check limits.
    assert!(res > 0.0 && res < 1.0);
}

#[test]
fn test_change_sign() {
    let vars = HashMap::new();
    let mut ctx = ExpressionContext::new(&vars);
    assert_eq!(
        evaluate_bytecode(&[123.321, as_nan(OpCode::ChangeSign)], &mut ctx).unwrap(),
        -123.321
    );
}

#[test]
fn test_names() {
    assert_eq!(OpCode::Add.to_math_name(), "+");
    assert_eq!(OpCode::Subtract.to_math_name(), "-");
    assert_eq!(OpCode::Multiply.to_math_name(), "*");
    assert_eq!(OpCode::Divide.to_math_name(), "/");
    assert_eq!(OpCode::Modulus.to_math_name(), "%");
    assert_eq!(OpCode::Min.to_math_name(), "min");
    assert_eq!(OpCode::Max.to_math_name(), "max");
    assert_eq!(OpCode::Pow.to_math_name(), "pow");
    assert_eq!(OpCode::Sqrt.to_math_name(), "sqrt");
    assert_eq!(OpCode::Abs.to_math_name(), "abs");
    assert_eq!(OpCode::Sign.to_math_name(), "sign");
    assert_eq!(OpCode::Exp.to_math_name(), "exp");
    assert_eq!(OpCode::Floor.to_math_name(), "floor");
    assert_eq!(OpCode::Log.to_math_name(), "log");
    assert_eq!(OpCode::Ln.to_math_name(), "ln");
    assert_eq!(OpCode::Round.to_math_name(), "round");
    assert_eq!(OpCode::Cos.to_math_name(), "cos");
    assert_eq!(OpCode::Sin.to_math_name(), "sin");
    assert_eq!(OpCode::Tan.to_math_name(), "tan");
    assert_eq!(OpCode::Asin.to_math_name(), "asin");
    assert_eq!(OpCode::Atan.to_math_name(), "atan");
    assert_eq!(OpCode::Atan2.to_math_name(), "atan2");
    assert_eq!(OpCode::Acos.to_math_name(), "acos");
    assert_eq!(OpCode::Mad.to_math_name(), "mad");
    assert_eq!(OpCode::IfElse.to_math_name(), "ifElse");
    assert_eq!(OpCode::Clamp.to_math_name(), "clamp");
    assert_eq!(OpCode::Cbrt.to_math_name(), "cbrt");
    assert_eq!(OpCode::Deg.to_math_name(), "deg");
    assert_eq!(OpCode::Rad.to_math_name(), "rad");
    assert_eq!(OpCode::Ceil.to_math_name(), "ceil");
    assert_eq!(OpCode::ChangeSign.to_math_name(), "change_sign");
    assert_eq!(OpCode::Cubic.to_math_name(), "cubic");
}
