const NAN_MASK: u32 = 0xFF800000;

const PATH_MOVE: u32 = 10;
const PATH_LINE: u32 = 11;
const PATH_QUAD: u32 = 12;
const PATH_CONIC: u32 = 13;
const PATH_CUBIC: u32 = 14;
const PATH_CLOSE: u32 = 15;
const PATH_DONE: u32 = 16;

#[derive(Debug, Clone, PartialEq)]
pub enum PathOp {
    Move(f32, f32),                      // PATH_MOVE
    Line(f32, f32),                      // PATH_LINE
    Quad(f32, f32, f32, f32),            // PATH_QUAD
    Conic(f32, f32, f32, f32, f32),      // PATH_CONIC
    Cubic(f32, f32, f32, f32, f32, f32), // PATH_CUBIC
    Close,                               // PATH_CLOSE
    Done,                                // PATH_DONE
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemotePath {
    pub ops: Vec<PathOp>,
}

impl RemotePath {
    pub fn read(data: &[u8]) -> Result<(Self, usize), ()> {
        let mut ops = Vec::new();
        let mut offset = 0;

        while offset + 4 <= data.len() {
            let bits = u32::from_be_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]);
            offset += 4;

            // Check if it's a command (NaN)
            if (bits & NAN_MASK) == NAN_MASK {
                let id = bits & !NAN_MASK;
                match id {
                    PATH_MOVE => {
                        if offset + 8 > data.len() {
                            return Err(());
                        }
                        let x = f32::from_be_bytes(data[offset..offset + 4].try_into().unwrap());
                        let y =
                            f32::from_be_bytes(data[offset + 4..offset + 8].try_into().unwrap());
                        offset += 8;
                        ops.push(PathOp::Move(x, y));
                    }
                    PATH_LINE => {
                        if offset + 16 > data.len() {
                            return Err(());
                        }
                        // Skip start point (x0, y0)
                        let x =
                            f32::from_be_bytes(data[offset + 8..offset + 12].try_into().unwrap());
                        let y =
                            f32::from_be_bytes(data[offset + 12..offset + 16].try_into().unwrap());
                        offset += 16;
                        ops.push(PathOp::Line(x, y));
                    }
                    PATH_QUAD => {
                        if offset + 24 > data.len() {
                            return Err(());
                        }
                        // Skip start point (x0, y0)
                        let x1 =
                            f32::from_be_bytes(data[offset + 8..offset + 12].try_into().unwrap());
                        let y1 =
                            f32::from_be_bytes(data[offset + 12..offset + 16].try_into().unwrap());
                        let x2 =
                            f32::from_be_bytes(data[offset + 16..offset + 20].try_into().unwrap());
                        let y2 =
                            f32::from_be_bytes(data[offset + 20..offset + 24].try_into().unwrap());
                        offset += 24;
                        ops.push(PathOp::Quad(x1, y1, x2, y2));
                    }
                    PATH_CONIC => {
                        if offset + 28 > data.len() {
                            return Err(());
                        }
                        // Skip start point (x0, y0)
                        let x1 =
                            f32::from_be_bytes(data[offset + 8..offset + 12].try_into().unwrap());
                        let y1 =
                            f32::from_be_bytes(data[offset + 12..offset + 16].try_into().unwrap());
                        let x2 =
                            f32::from_be_bytes(data[offset + 16..offset + 20].try_into().unwrap());
                        let y2 =
                            f32::from_be_bytes(data[offset + 20..offset + 24].try_into().unwrap());
                        let w =
                            f32::from_be_bytes(data[offset + 24..offset + 28].try_into().unwrap());
                        offset += 28;
                        ops.push(PathOp::Conic(x1, y1, x2, y2, w));
                    }
                    PATH_CUBIC => {
                        if offset + 32 > data.len() {
                            return Err(());
                        }
                        // Skip start point (x0, y0)
                        let x1 =
                            f32::from_be_bytes(data[offset + 8..offset + 12].try_into().unwrap());
                        let y1 =
                            f32::from_be_bytes(data[offset + 12..offset + 16].try_into().unwrap());
                        let x2 =
                            f32::from_be_bytes(data[offset + 16..offset + 20].try_into().unwrap());
                        let y2 =
                            f32::from_be_bytes(data[offset + 20..offset + 24].try_into().unwrap());
                        let x3 =
                            f32::from_be_bytes(data[offset + 24..offset + 28].try_into().unwrap());
                        let y3 =
                            f32::from_be_bytes(data[offset + 28..offset + 32].try_into().unwrap());
                        offset += 32;
                        ops.push(PathOp::Cubic(x1, y1, x2, y2, x3, y3));
                    }
                    PATH_CLOSE => {
                        ops.push(PathOp::Close);
                    }
                    PATH_DONE => {
                        ops.push(PathOp::Done);
                        break;
                    }
                    _ => {
                        return Err(());
                    }
                }
            } else {
                return Err(());
            }
        }

        Ok((RemotePath { ops }, offset))
    }
}
