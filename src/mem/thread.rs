use super::frame::Frame;

struct Thread {
    pc: usize,
    stack: Stack,
}

pub struct Stack {
    max_stack: usize,
    frames: Vec<Frame>,
}
