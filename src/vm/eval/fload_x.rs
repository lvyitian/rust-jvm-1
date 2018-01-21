use vm::Frame;
use vm::primitive::Primitive;

/// Can handle instructions fload and fload_<n>.
pub fn eval(code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    // Check which instruction triggered this call, if it was fload, then one byte should be read,
    // when it was fload_<n>, the index is implicit
    let (index, pc_inc) = match *code.get(pc as usize).unwrap() {
        // fload
        23 => (*code.get((pc+1) as usize).unwrap(), 2),
        // fload_<n>
        i @ 34...37 => (i - 34, 1),
        i => panic!("Unexpected invocation of this instruction, found: {}", i),
    };

    let value = frame.locals_get_float(index as usize);
    trace!("fload_{}: Reading Float {} from locals and push it to the stack", index, value);

    frame.stack_push(Primitive::Float(value));

    Some(pc + pc_inc)
}
