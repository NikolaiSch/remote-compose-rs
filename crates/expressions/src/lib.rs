pub mod context;
pub mod expression;
pub mod opcode;
pub mod parser;
pub mod remote_path;
pub mod utils;

pub use context::ExpressionContext;
pub use expression::FloatExpression;
pub use opcode::OpCode;
pub use parser::{evaluate_bytecode, parse_to_tree};
pub use utils::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::f32::consts;

    #[test]
    fn test_eval_simple() {
        let input = vec![1.0, 2.0, as_nan(OpCode::Add)];
        let vars = HashMap::new();
        let mut ctx = ExpressionContext::new(&vars);
        assert_eq!(evaluate_bytecode(&input, &mut ctx).unwrap(), 3.0);
    }

    #[test]
    fn test_eval_sub() {
        let input = vec![10.0, 2.0, as_nan(OpCode::Subtract)];
        let vars = HashMap::new();
        let mut ctx = ExpressionContext::new(&vars);
        assert_eq!(evaluate_bytecode(&input, &mut ctx).unwrap(), 8.0);
    }

    #[test]
    fn test_eval_mul() {
        let input = vec![3.0, 4.0, as_nan(OpCode::Multiply)];
        let vars = HashMap::new();
        let mut ctx = ExpressionContext::new(&vars);
        assert_eq!(evaluate_bytecode(&input, &mut ctx).unwrap(), 12.0);
    }

    #[test]
    fn test_eval_div() {
        let input = vec![10.0, 2.0, as_nan(OpCode::Divide)];
        let vars = HashMap::new();
        let mut ctx = ExpressionContext::new(&vars);
        assert_eq!(evaluate_bytecode(&input, &mut ctx).unwrap(), 5.0);
    }

    #[test]
    fn test_eval_mod() {
        let input = vec![10.0, 3.0, as_nan(OpCode::Modulus)];
        let vars = HashMap::new();
        let mut ctx = ExpressionContext::new(&vars);
        assert_eq!(evaluate_bytecode(&input, &mut ctx).unwrap(), 1.0);
    }

    #[test]
    fn test_eval_pow() {
        let input = vec![2.0, 3.0, as_nan(OpCode::Pow)];
        let vars = HashMap::new();
        let mut ctx = ExpressionContext::new(&vars);
        assert_eq!(evaluate_bytecode(&input, &mut ctx).unwrap(), 8.0);
    }

    #[test]
    fn test_eval_complex() {
        // (10 - 2) * 3 = 24
        let input = vec![
            10.0,
            2.0,
            as_nan(OpCode::Subtract),
            3.0,
            as_nan(OpCode::Multiply),
        ];
        let vars = HashMap::new();
        let mut ctx = ExpressionContext::new(&vars);
        assert_eq!(evaluate_bytecode(&input, &mut ctx).unwrap(), 24.0);
    }

    #[test]
    fn test_eval_clamp() {
        // clamp(15, 0, 10) = 10
        let input = vec![15.0, 0.0, 10.0, as_nan(OpCode::Clamp)];
        let vars = HashMap::new();
        let mut ctx = ExpressionContext::new(&vars);
        assert_eq!(evaluate_bytecode(&input, &mut ctx).unwrap(), 10.0);
    }

    #[test]
    fn test_eval_lerp() {
        let input = vec![10.0, 20.0, 0.5, as_nan(OpCode::Lerp)];
        let vars = HashMap::new();
        let mut ctx = ExpressionContext::new(&vars);
        assert_eq!(evaluate_bytecode(&input, &mut ctx).unwrap(), 15.0);
    }

    #[test]
    fn test_eval_registers() {
        // store 5.0 in R0, then load it and add 2.0
        let input = vec![
            5.0,
            as_nan(OpCode::StoreR0),
            as_nan(OpCode::LoadR0),
            2.0,
            as_nan(OpCode::Add),
        ];
        let vars = HashMap::new();
        let mut ctx = ExpressionContext::new(&vars);
        assert_eq!(evaluate_bytecode(&input, &mut ctx).unwrap(), 7.0);
    }

    #[test]
    fn test_parse_to_tree() {
        let input = vec![1.0, 2.0, as_nan(OpCode::Add)];
        let tree = parse_to_tree(&input).unwrap();
        assert_eq!(
            tree,
            FloatExpression::Add(
                Box::new(FloatExpression::Value(1.0)),
                Box::new(FloatExpression::Value(2.0))
            )
        );
        let vars = HashMap::new();
        let mut ctx = ExpressionContext::new(&vars);
        assert_eq!(tree.evaluate(&mut ctx), 3.0);
    }

    #[test]
    fn test_parse_to_tree_complex() {
        // (10 - 2) * 3 = 24
        let input = vec![
            10.0,
            2.0,
            as_nan(OpCode::Subtract),
            3.0,
            as_nan(OpCode::Multiply),
        ];
        let tree = parse_to_tree(&input).unwrap();
        assert_eq!(
            tree,
            FloatExpression::Multiply(
                Box::new(FloatExpression::Subtract(
                    Box::new(FloatExpression::Value(10.0)),
                    Box::new(FloatExpression::Value(2.0))
                )),
                Box::new(FloatExpression::Value(3.0))
            )
        );
        let vars = HashMap::new();
        let mut ctx = ExpressionContext::new(&vars);
        assert_eq!(tree.evaluate(&mut ctx), 24.0);
    }

    #[test]
    fn test_parse_to_tree_unary_op() {
        let input = vec![9.0, as_nan(OpCode::Sqrt)];
        let tree = parse_to_tree(&input).unwrap();
        assert_eq!(
            tree,
            FloatExpression::Sqrt(Box::new(FloatExpression::Value(9.0)))
        );
        let vars = HashMap::new();
        let mut ctx = ExpressionContext::new(&vars);
        assert_eq!(tree.evaluate(&mut ctx), 3.0);
    }

    #[test]
    fn test_parse_to_tree_clamp() {
        // clamp(15, 0, 10) = 10
        let input = vec![15.0, 0.0, 10.0, as_nan(OpCode::Clamp)];
        let tree = parse_to_tree(&input).unwrap();
        assert_eq!(
            tree,
            FloatExpression::Clamp(
                Box::new(FloatExpression::Value(15.0)),
                Box::new(FloatExpression::Value(0.0)),
                Box::new(FloatExpression::Value(10.0))
            )
        );
        let vars = HashMap::new();
        let mut ctx = ExpressionContext::new(&vars);
        assert_eq!(tree.evaluate(&mut ctx), 10.0);
    }

    #[test]
    fn test_parse_to_tree_lerp() {
        // lerp(10, 20, 0.5) = 15
        let input = vec![10.0, 20.0, 0.5, as_nan(OpCode::Lerp)];
        let tree = parse_to_tree(&input).unwrap();
        assert_eq!(
            tree,
            FloatExpression::Lerp(
                Box::new(FloatExpression::Value(10.0)),
                Box::new(FloatExpression::Value(20.0)),
                Box::new(FloatExpression::Value(0.5))
            )
        );
        let vars = HashMap::new();
        let mut ctx = ExpressionContext::new(&vars);
        assert_eq!(tree.evaluate(&mut ctx), 15.0);
    }

    #[test]
    fn test_parse_to_tree_log() {
        // log10(100) = 2
        let input = vec![100.0, as_nan(OpCode::Log)];
        let tree = parse_to_tree(&input).unwrap();
        assert_eq!(
            tree,
            FloatExpression::Log(Box::new(FloatExpression::Value(100.0)))
        );
        let vars = HashMap::new();
        let mut ctx = ExpressionContext::new(&vars);
        assert_eq!(tree.evaluate(&mut ctx), 2.0);
    }

    #[test]
    fn test_parse_to_tree_atan2() {
        // atan2(1, 1) = PI/4
        let input = vec![1.0, 1.0, as_nan(OpCode::Atan2)];
        let tree = parse_to_tree(&input).unwrap();
        assert_eq!(
            tree,
            FloatExpression::Atan2(
                Box::new(FloatExpression::Value(1.0)),
                Box::new(FloatExpression::Value(1.0))
            )
        );
        let vars = HashMap::new();
        let mut ctx = ExpressionContext::new(&vars);
        assert_eq!(tree.evaluate(&mut ctx), consts::FRAC_PI_4);
    }

    #[test]
    fn test_parse_to_tree_registers() {
        // store 5.0 in R0, then LoadR0 + 2.0
        let input = vec![
            5.0,
            as_nan(OpCode::StoreR0),
            as_nan(OpCode::LoadR0),
            2.0,
            as_nan(OpCode::Add),
        ];
        let tree = parse_to_tree(&input).unwrap();
        assert_eq!(
            tree,
            FloatExpression::Sequence(vec![
                FloatExpression::StoreR0(Box::new(FloatExpression::Value(5.0))),
                FloatExpression::Add(
                    Box::new(FloatExpression::LoadR0),
                    Box::new(FloatExpression::Value(2.0))
                )
            ])
        );

        let vars = HashMap::new();
        let mut ctx = ExpressionContext::new(&vars);
        assert_eq!(tree.evaluate(&mut ctx), 7.0);
        assert_eq!(ctx.registers[0], 5.0);
    }

    #[test]
    fn test_stack_underflow_error() {
        let input = vec![5.0, as_nan(OpCode::Add)];
        let err = parse_to_tree(&input).unwrap_err();
        assert_eq!(err, "Add: missing first argument");

        let input = vec![as_nan(OpCode::Sqrt)];
        let err = parse_to_tree(&input).unwrap_err();
        assert_eq!(err, "Sqrt: missing argument");

        let input = vec![1.0, 2.0, as_nan(OpCode::Clamp)];
        let err = parse_to_tree(&input).unwrap_err();
        assert_eq!(err, "Clamp: missing 'value' argument");
    }

    #[test]
    fn test_eval_non_contiguous_vars() {
        let mut vars = HashMap::new();
        vars.insert(42, 100.0);
        vars.insert(7, 50.0);

        let tree = FloatExpression::Add(
            Box::new(FloatExpression::Variable(42)),
            Box::new(FloatExpression::Variable(7)),
        );

        let mut ctx = ExpressionContext::new(&vars);
        assert_eq!(tree.evaluate(&mut ctx), 150.0);
    }
}
