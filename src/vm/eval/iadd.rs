use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let value2 = frame.stack_pop_int();
    let value1 = frame.stack_pop_int();
    let result = value1 + value2;

    trace!("iadd: Adding {} and {} -> pushing {} to stack", value1, value2, result);
    frame.stack_push(Primitive::Int(result));

    Some(pc + 1)
}