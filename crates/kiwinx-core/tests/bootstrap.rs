use kiwinx_common::GuestAddress;
use kiwinx_core::{System, SystemError};
use kiwinx_cpu::CpuError;
use kiwinx_testkit::{add_x0_x0, exit_after_add, movz_x0, words_to_bytes};

#[test]
fn executes_movz_add_and_svc_exit() {
    let mut system = System::new(4096);
    system
        .load(GuestAddress::new(0), &exit_after_add(40, 2))
        .unwrap();

    let exit_code = system.run(GuestAddress::new(0), 16).unwrap();

    assert_eq!(exit_code, 42);
}

#[test]
fn rejects_an_unsupported_instruction() {
    let mut system = System::new(4096);
    system
        .load(GuestAddress::new(0), &0xFFFF_FFFFu32.to_le_bytes())
        .unwrap();

    let error = system.run(GuestAddress::new(0), 1).unwrap_err();

    assert_eq!(
        error,
        SystemError::Cpu(CpuError::UnsupportedInstruction {
            pc: GuestAddress::new(0),
            opcode: 0xFFFF_FFFF,
        })
    );
}

#[test]
fn stops_at_the_explicit_step_limit() {
    let mut system = System::new(4096);
    let program = words_to_bytes(&[movz_x0(40), add_x0_x0(2)]);
    system.load(GuestAddress::new(0), &program).unwrap();

    let error = system.run(GuestAddress::new(0), 2).unwrap_err();

    assert_eq!(error, SystemError::Cpu(CpuError::StepLimit { steps: 2 }));
}
