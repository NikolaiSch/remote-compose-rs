use crate::context::ExpressionContext;
use crate::utils::cubic_easing;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Debug, PartialEq, Clone)]
pub enum FloatExpression {
    Value(f32),
    Variable(u32),
    Var1,
    Var2,
    Var3,
    Add(Box<FloatExpression>, Box<FloatExpression>),
    Subtract(Box<FloatExpression>, Box<FloatExpression>),
    Multiply(Box<FloatExpression>, Box<FloatExpression>),
    Divide(Box<FloatExpression>, Box<FloatExpression>),
    Modulus(Box<FloatExpression>, Box<FloatExpression>),
    Min(Box<FloatExpression>, Box<FloatExpression>),
    Max(Box<FloatExpression>, Box<FloatExpression>),
    Pow(Box<FloatExpression>, Box<FloatExpression>),
    Sqrt(Box<FloatExpression>),
    Abs(Box<FloatExpression>),
    Sign(Box<FloatExpression>),
    Exp(Box<FloatExpression>),
    Floor(Box<FloatExpression>),
    Log(Box<FloatExpression>),
    Ln(Box<FloatExpression>),
    Round(Box<FloatExpression>),
    Sin(Box<FloatExpression>),
    Cos(Box<FloatExpression>),
    Tan(Box<FloatExpression>),
    Asin(Box<FloatExpression>),
    Acos(Box<FloatExpression>),
    Atan(Box<FloatExpression>),
    Atan2(Box<FloatExpression>, Box<FloatExpression>),
    Mad(
        Box<FloatExpression>,
        Box<FloatExpression>,
        Box<FloatExpression>,
    ),
    IfElse(
        Box<FloatExpression>,
        Box<FloatExpression>,
        Box<FloatExpression>,
    ),
    Clamp(
        Box<FloatExpression>,
        Box<FloatExpression>,
        Box<FloatExpression>,
    ),
    Cbrt(Box<FloatExpression>),
    Deg(Box<FloatExpression>),
    Rad(Box<FloatExpression>),
    Ceil(Box<FloatExpression>),
    Rand,
    RandSeed(Box<FloatExpression>),
    NoiseFrom(Box<FloatExpression>),
    RandInRange(Box<FloatExpression>, Box<FloatExpression>),
    SquareSum(Box<FloatExpression>, Box<FloatExpression>),
    Step(Box<FloatExpression>, Box<FloatExpression>),
    Square(Box<FloatExpression>),
    Hypot(Box<FloatExpression>, Box<FloatExpression>),
    Lerp(
        Box<FloatExpression>,
        Box<FloatExpression>,
        Box<FloatExpression>,
    ),
    SmoothStep(
        Box<FloatExpression>,
        Box<FloatExpression>,
        Box<FloatExpression>,
    ),
    Log2(Box<FloatExpression>),
    Inv(Box<FloatExpression>),
    Fract(Box<FloatExpression>),
    PingPong(Box<FloatExpression>, Box<FloatExpression>),
    Nop,
    StoreR0(Box<FloatExpression>),
    StoreR1(Box<FloatExpression>),
    StoreR2(Box<FloatExpression>),
    StoreR3(Box<FloatExpression>),
    LoadR0,
    LoadR1,
    LoadR2,
    LoadR3,
    ChangeSign(Box<FloatExpression>),
    Cubic(
        Box<FloatExpression>,
        Box<FloatExpression>,
        Box<FloatExpression>,
        Box<FloatExpression>,
        Box<FloatExpression>,
    ),
    Sequence(Vec<FloatExpression>),
}

impl FloatExpression {
    pub fn evaluate(&self, ctx: &mut ExpressionContext) -> f32 {
        match self {
            FloatExpression::Value(v) => *v,
            FloatExpression::Variable(id) => *ctx.vars.get(id).unwrap_or(&0.0),
            FloatExpression::Var1 => panic!("Var1 not implemented"),
            FloatExpression::Var2 => panic!("Var2 not implemented"),
            FloatExpression::Var3 => panic!("Var3 not implemented"),
            FloatExpression::Add(a, b) => a.evaluate(ctx) + b.evaluate(ctx),
            FloatExpression::Subtract(a, b) => a.evaluate(ctx) - b.evaluate(ctx),
            FloatExpression::Multiply(a, b) => a.evaluate(ctx) * b.evaluate(ctx),
            FloatExpression::Divide(a, b) => a.evaluate(ctx) / b.evaluate(ctx),
            FloatExpression::Modulus(a, b) => a.evaluate(ctx) % b.evaluate(ctx),
            FloatExpression::Min(a, b) => a.evaluate(ctx).min(b.evaluate(ctx)),
            FloatExpression::Max(a, b) => a.evaluate(ctx).max(b.evaluate(ctx)),
            FloatExpression::Pow(a, b) => a.evaluate(ctx).powf(b.evaluate(ctx)),
            FloatExpression::Sqrt(a) => a.evaluate(ctx).sqrt(),
            FloatExpression::Abs(a) => a.evaluate(ctx).abs(),
            FloatExpression::Sign(a) => a.evaluate(ctx).signum(),
            FloatExpression::Exp(a) => a.evaluate(ctx).exp(),
            FloatExpression::Floor(a) => a.evaluate(ctx).floor(),
            FloatExpression::Log(a) => a.evaluate(ctx).log10(),
            FloatExpression::Ln(a) => a.evaluate(ctx).ln(),
            FloatExpression::Round(a) => a.evaluate(ctx).round(),
            FloatExpression::Sin(a) => a.evaluate(ctx).sin(),
            FloatExpression::Cos(a) => a.evaluate(ctx).cos(),
            FloatExpression::Tan(a) => a.evaluate(ctx).tan(),
            FloatExpression::Asin(a) => a.evaluate(ctx).asin(),
            FloatExpression::Acos(a) => a.evaluate(ctx).acos(),
            FloatExpression::Atan(a) => a.evaluate(ctx).atan(),
            FloatExpression::Atan2(a, b) => a.evaluate(ctx).atan2(b.evaluate(ctx)),
            FloatExpression::Mad(a, b, c) => a.evaluate(ctx) * b.evaluate(ctx) + c.evaluate(ctx),
            FloatExpression::IfElse(a, b, cond) => {
                if cond.evaluate(ctx) > 0.0 {
                    b.evaluate(ctx)
                } else {
                    a.evaluate(ctx)
                }
            }
            FloatExpression::Clamp(val, min, max) => {
                let v = val.evaluate(ctx);
                let mi = min.evaluate(ctx);
                let ma = max.evaluate(ctx);
                v.max(mi).min(ma)
            }
            FloatExpression::Cbrt(a) => a.evaluate(ctx).powf(1.0 / 3.0),
            FloatExpression::Deg(a) => a.evaluate(ctx) * 180.0 / std::f32::consts::PI,
            FloatExpression::Rad(a) => a.evaluate(ctx) * std::f32::consts::PI / 180.0,
            FloatExpression::Ceil(a) => a.evaluate(ctx).ceil(),
            FloatExpression::Rand => ctx.rng.random(),
            FloatExpression::RandSeed(a) => {
                let seed = a.evaluate(ctx);
                ctx.rng = ChaCha8Rng::seed_from_u64(seed.to_bits() as u64);
                0.0
            }
            FloatExpression::NoiseFrom(a) => {
                let val = a.evaluate(ctx);
                let mut x = val.to_bits() as i32;
                x = (x << 13) ^ x;
                let res = 1.0
                    - ((x
                        .wrapping_mul(x.wrapping_mul(x).wrapping_mul(15731).wrapping_add(789221))
                        .wrapping_add(1376312589))
                        & 0x7fffffff) as f32
                        / 1.0737418E+9;
                res
            }
            FloatExpression::RandInRange(a, b) => {
                let va = a.evaluate(ctx);
                let vb = b.evaluate(ctx);
                ctx.rng.random_range(va..vb)
            }
            FloatExpression::SquareSum(a, b) => {
                let va = a.evaluate(ctx);
                let vb = b.evaluate(ctx);
                va * va + vb * vb
            }
            FloatExpression::Step(val, edge) => {
                if val.evaluate(ctx) > edge.evaluate(ctx) {
                    1.0
                } else {
                    0.0
                }
            }
            FloatExpression::Square(a) => {
                let va = a.evaluate(ctx);
                va * va
            }
            FloatExpression::Hypot(a, b) => a.evaluate(ctx).hypot(b.evaluate(ctx)),
            FloatExpression::Lerp(a, b, t) => {
                let va = a.evaluate(ctx);
                let vb = b.evaluate(ctx);
                let vt = t.evaluate(ctx);
                va + (vb - va) * vt
            }
            FloatExpression::SmoothStep(val, edge0, edge1) => {
                let v = val.evaluate(ctx);
                let e0 = edge0.evaluate(ctx);
                let e1 = edge1.evaluate(ctx);
                let t = ((v - e0) / (e1 - e0)).clamp(0.0, 1.0);
                t * t * (3.0 - 2.0 * t)
            }
            FloatExpression::Log2(a) => a.evaluate(ctx).log2(),
            FloatExpression::Inv(a) => 1.0 / a.evaluate(ctx),
            FloatExpression::Fract(a) => {
                let va = a.evaluate(ctx);
                va - va.trunc()
            }
            FloatExpression::PingPong(a, b) => {
                let va = a.evaluate(ctx);
                let vb = b.evaluate(ctx);
                let max_2 = vb * 2.0;
                let tmp = va % max_2;
                if tmp < vb {
                    tmp
                } else {
                    max_2 - tmp
                }
            }
            FloatExpression::Nop => 0.0,
            FloatExpression::StoreR0(a) => {
                let val = a.evaluate(ctx);
                ctx.registers[0] = val;
                val
            }
            FloatExpression::StoreR1(a) => {
                let val = a.evaluate(ctx);
                ctx.registers[1] = val;
                val
            }
            FloatExpression::StoreR2(a) => {
                let val = a.evaluate(ctx);
                ctx.registers[2] = val;
                val
            }
            FloatExpression::StoreR3(a) => {
                let val = a.evaluate(ctx);
                ctx.registers[3] = val;
                val
            }
            FloatExpression::LoadR0 => ctx.registers[0],
            FloatExpression::LoadR1 => ctx.registers[1],
            FloatExpression::LoadR2 => ctx.registers[2],
            FloatExpression::LoadR3 => ctx.registers[3],
            FloatExpression::ChangeSign(a) => -a.evaluate(ctx),
            FloatExpression::Cubic(x1, y1, x2, y2, t) => {
                let vx1 = x1.evaluate(ctx);
                let vy1 = y1.evaluate(ctx);
                let vx2 = x2.evaluate(ctx);
                let vy2 = y2.evaluate(ctx);
                let vt = t.evaluate(ctx);
                cubic_easing(vx1, vy1, vx2, vy2, vt)
            }
            FloatExpression::Sequence(ops) => {
                let mut last = 0.0;
                for op in ops {
                    last = op.evaluate(ctx);
                }
                last
            }
        }
    }
}
