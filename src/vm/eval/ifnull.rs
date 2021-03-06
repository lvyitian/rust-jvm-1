use vm::Vm;
use vm::primitive::Primitive;
use vm::utils;

pub fn eval(vm: &mut Vm, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_reference();
    match value {
        Primitive::Null => {
            trace!("ifnull: Popped Null from stack -> branching");

            let branchoffset = utils::read_i16_code(code, pc);
            let target_pc: u16 = (pc as i16 + branchoffset) as u16;

            Some(target_pc)
        },
        _ => {
            trace!("ifnull: Popped Reference from stack -> not branching");

            Some(pc + 3)
        }
    }
}
