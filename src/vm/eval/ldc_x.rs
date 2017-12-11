use classfile::Classfile;
use classfile::constants::Constant;
use vm::Vm;
use vm::Frame;
use vm::utils;

/// Can handle instructions ldc (decimal 18) and ldc_2 (decimal 19).
pub fn eval(vm: &mut Vm, class: &Classfile, code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    // Check which instruction triggered this call, if it was ldc, then only one byte should be read,
    // when it was ldc_w, two bytes must be read
    let (index, pc_inc) = match *code.get(pc as usize).unwrap() {
        18 => (*code.get((pc+1) as usize).unwrap() as u16, 2),
        19 => (utils::read_u16_code(code, pc), 3),
        i => panic!("Unexpected invocation of this instruction, found: {}", i),
    };

    match class.constants.get(index as usize).unwrap() {
        &Constant::String(ref value) => {
            trace!("ldc: pushing String {} to stack", value);
//            frame.stack_push(value.clone());
        },
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc + pc_inc)
}