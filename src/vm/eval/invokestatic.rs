use classfile::Classfile;
use classfile::constants::Constant;
use vm::Vm;
use vm::utils;

pub fn eval(vm: &mut Vm, class: &Classfile, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);
    match class.constants.get(index as usize).unwrap() {
        &Constant::Methodref(ref class_path, ref method_name, ref method_signature) => {
            debug!("invokestatic: {}.{}{}", class_path, method_name, method_signature);
            utils::invoke_method(vm, class_path, method_name, method_signature, false);
        },
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc+3)
}