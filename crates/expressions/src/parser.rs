use crate::context::ExpressionContext;
use crate::expression::FloatExpression;
use crate::opcode::OpCode;
use crate::utils::{
    id_from_nan, is_normal_variable, is_operation_variable, is_system_variable, NAN_OFFSET,
};

/// Evaluates a float array-based expression using the AndroidX Remote Core logic.
pub fn evaluate_bytecode(exp: &[f32], ctx: &mut ExpressionContext) -> Result<f32, String> {
    let tree = parse_to_tree(exp)?;
    Ok(tree.evaluate(ctx))
}

/// Parses a float array into a FloatExpression tree.
pub fn parse_to_tree(input: &[f32]) -> Result<FloatExpression, String> {
    let mut stack: Vec<FloatExpression> = Vec::new();

    for &v in input {
        if v.is_nan() {
            let bits = v.to_bits();
            if is_normal_variable(v) || is_system_variable(v) {
                let id = id_from_nan(v);
                stack.push(FloatExpression::Variable(id));
                continue;
            }

            if is_operation_variable(v) {
                let opcode_val = (bits & 0x7FFFFF).saturating_sub(NAN_OFFSET);
                let op = OpCode::from_u32(opcode_val)
                    .ok_or_else(|| format!("Unknown opcode: {:#x}", opcode_val))?;

                let res = match op {
                    OpCode::Var1 => FloatExpression::Var1,
                    OpCode::Var2 => FloatExpression::Var2,
                    OpCode::Var3 => FloatExpression::Var3,
                    OpCode::Add => {
                        let b = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing second argument", op))?;
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing first argument", op))?;
                        FloatExpression::Add(Box::new(a), Box::new(b))
                    }
                    OpCode::Subtract => {
                        let b = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing second argument", op))?;
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing first argument", op))?;
                        FloatExpression::Subtract(Box::new(a), Box::new(b))
                    }
                    OpCode::Multiply => {
                        let b = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing second argument", op))?;
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing first argument", op))?;
                        FloatExpression::Multiply(Box::new(a), Box::new(b))
                    }
                    OpCode::Divide => {
                        let b = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing second argument", op))?;
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing first argument", op))?;
                        FloatExpression::Divide(Box::new(a), Box::new(b))
                    }
                    OpCode::Modulus => {
                        let b = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing second argument", op))?;
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing first argument", op))?;
                        FloatExpression::Modulus(Box::new(a), Box::new(b))
                    }
                    OpCode::Min => {
                        let b = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing second argument", op))?;
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing first argument", op))?;
                        FloatExpression::Min(Box::new(a), Box::new(b))
                    }
                    OpCode::Max => {
                        let b = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing second argument", op))?;
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing first argument", op))?;
                        FloatExpression::Max(Box::new(a), Box::new(b))
                    }
                    OpCode::Pow => {
                        let b = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing second argument", op))?;
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing first argument", op))?;
                        FloatExpression::Pow(Box::new(a), Box::new(b))
                    }
                    OpCode::Atan2 => {
                        let b = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing second argument", op))?;
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing first argument", op))?;
                        FloatExpression::Atan2(Box::new(a), Box::new(b))
                    }
                    OpCode::Sqrt => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Sqrt(Box::new(a))
                    }
                    OpCode::Abs => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Abs(Box::new(a))
                    }
                    OpCode::Sign => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Sign(Box::new(a))
                    }
                    OpCode::Exp => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Exp(Box::new(a))
                    }
                    OpCode::Floor => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Floor(Box::new(a))
                    }
                    OpCode::Log => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Log(Box::new(a))
                    }
                    OpCode::Ln => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Ln(Box::new(a))
                    }
                    OpCode::Round => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Round(Box::new(a))
                    }
                    OpCode::Sin => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Sin(Box::new(a))
                    }
                    OpCode::Cos => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Cos(Box::new(a))
                    }
                    OpCode::Tan => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Tan(Box::new(a))
                    }
                    OpCode::Asin => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Asin(Box::new(a))
                    }
                    OpCode::Acos => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Acos(Box::new(a))
                    }
                    OpCode::Atan => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Atan(Box::new(a))
                    }
                    OpCode::Mad => {
                        let c = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing 'c' argument", op))?;
                        let b = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing 'b' argument", op))?;
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing 'a' argument", op))?;
                        FloatExpression::Mad(Box::new(a), Box::new(b), Box::new(c))
                    }
                    OpCode::IfElse => {
                        let cond = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing condition argument", op))?;
                        let b = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing 'true' argument", op))?;
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing 'false' argument", op))?;
                        FloatExpression::IfElse(Box::new(a), Box::new(b), Box::new(cond))
                    }
                    OpCode::Clamp => {
                        let max = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing 'max' argument", op))?;
                        let min = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing 'min' argument", op))?;
                        let val = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing 'value' argument", op))?;
                        FloatExpression::Clamp(Box::new(val), Box::new(min), Box::new(max))
                    }
                    OpCode::Cbrt => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Cbrt(Box::new(a))
                    }
                    OpCode::Deg => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Deg(Box::new(a))
                    }
                    OpCode::Rad => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Rad(Box::new(a))
                    }
                    OpCode::Ceil => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Ceil(Box::new(a))
                    }
                    OpCode::Rand => FloatExpression::Rand,
                    OpCode::RandSeed => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing seed argument", op))?;
                        FloatExpression::RandSeed(Box::new(a))
                    }
                    OpCode::NoiseFrom => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::NoiseFrom(Box::new(a))
                    }
                    OpCode::RandInRange => {
                        let b = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing second argument", op))?;
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing first argument", op))?;
                        FloatExpression::RandInRange(Box::new(a), Box::new(b))
                    }
                    OpCode::SquareSum => {
                        let b = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing second argument", op))?;
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing first argument", op))?;
                        FloatExpression::SquareSum(Box::new(a), Box::new(b))
                    }
                    OpCode::Step => {
                        let edge = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing edge argument", op))?;
                        let value = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing value argument", op))?;
                        FloatExpression::Step(Box::new(value), Box::new(edge))
                    }
                    OpCode::Square => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Square(Box::new(a))
                    }
                    OpCode::Dup => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        stack.push(a.clone());
                        stack.push(a);
                        continue;
                    }
                    OpCode::Hypot => {
                        let b = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing second argument", op))?;
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing first argument", op))?;
                        FloatExpression::Hypot(Box::new(a), Box::new(b))
                    }
                    OpCode::Swap => {
                        let b = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing second argument", op))?;
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing first argument", op))?;
                        stack.push(b);
                        stack.push(a);
                        continue;
                    }
                    OpCode::Lerp => {
                        let t = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing 't' argument", op))?;
                        let b = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing 'b' argument", op))?;
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing 'a' argument", op))?;
                        FloatExpression::Lerp(Box::new(a), Box::new(b), Box::new(t))
                    }
                    OpCode::StoreR0 => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::StoreR0(Box::new(a))
                    }
                    OpCode::StoreR1 => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::StoreR1(Box::new(a))
                    }
                    OpCode::StoreR2 => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::StoreR2(Box::new(a))
                    }
                    OpCode::StoreR3 => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::StoreR3(Box::new(a))
                    }
                    OpCode::LoadR0 => FloatExpression::LoadR0,
                    OpCode::LoadR1 => FloatExpression::LoadR1,
                    OpCode::LoadR2 => FloatExpression::LoadR2,
                    OpCode::LoadR3 => FloatExpression::LoadR3,
                    OpCode::SmoothStep => {
                        let edge1 = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing edge1 argument", op))?;
                        let edge0 = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing edge0 argument", op))?;
                        let value = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing value argument", op))?;
                        FloatExpression::SmoothStep(
                            Box::new(value),
                            Box::new(edge0),
                            Box::new(edge1),
                        )
                    }
                    OpCode::Log2 => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Log2(Box::new(a))
                    }
                    OpCode::Inv => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Inv(Box::new(a))
                    }
                    OpCode::Fract => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::Fract(Box::new(a))
                    }
                    OpCode::PingPong => {
                        let b = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing second argument", op))?;
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing first argument", op))?;
                        FloatExpression::PingPong(Box::new(a), Box::new(b))
                    }
                    OpCode::Nop => FloatExpression::Nop,
                    OpCode::ChangeSign => {
                        let a = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing argument", op))?;
                        FloatExpression::ChangeSign(Box::new(a))
                    }
                    OpCode::Cubic => {
                        let t = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing 't' argument", op))?;
                        let y2 = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing 'y2' argument", op))?;
                        let x2 = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing 'x2' argument", op))?;
                        let y1 = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing 'y1' argument", op))?;
                        let x1 = stack
                            .pop()
                            .ok_or_else(|| format!("{:?}: missing 'x1' argument", op))?;
                        FloatExpression::Cubic(
                            Box::new(x1),
                            Box::new(y1),
                            Box::new(x2),
                            Box::new(y2),
                            Box::new(t),
                        )
                    }
                };
                stack.push(res);
            }
        } else {
            stack.push(FloatExpression::Value(v));
        }
    }

    if stack.is_empty() {
        return Err("Empty expression".to_string());
    }
    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Ok(FloatExpression::Sequence(stack))
    }
}
