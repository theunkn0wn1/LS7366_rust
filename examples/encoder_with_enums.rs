use std::error::Error;

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::system::DeviceInfo;

use ls7366::ir;
use ls7366::mdr0;
use ls7366::mdr1;
use ls7366::traits::Encodable;

fn main() -> Result<(), Box<dyn Error>> {
    let device = DeviceInfo::new()?.model();
    println!("device model := {}", device);
//    let spi_0 = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 14000000, Mode::Mode0)?;
    let spi_1 = Spi::new(Bus::Spi0, SlaveSelect::Ss1, 14000000, Mode::Mode0)?;
    let mut spi_driver = ls7366::Ls7366::new(spi_1);

    let mdr0_payload = mdr0::Mdr0 {
        quad_count_mode: mdr0::QuadCountMode::Quad4x,
        cycle_count_mode: mdr0::CycleCountMode::FreeRunning,
        index_mode: mdr0::IndexMode::DisableIndex,
        is_index_inverted: false,
        filter_clock: mdr0::FilterClockDivisionFactor::One,
    };
    let mdr1_payload = mdr1::Mdr1{
        counter_mode: mdr1::CounterMode::Byte4,
        enable_counting: true,
        flag_on_idx: false,
        flag_on_cmp: false,
        flag_on_bw: false,
        flag_on_cy: false
    };

    // primary configuration
    spi_driver.act(ir::InstructionRegister {
        action: ir::Action::Write,
        target: ir::Target::Mdr0,
    },
                   vec![mdr0_payload.encode()],
    )?;
    // secondary configuration
    spi_driver.act(ir::InstructionRegister {
        action: ir::Action::Write,
        target: ir::Target::Mdr1,
    },
                   vec![mdr1_payload.encode()],
    )?;
    // zero Dtr
    spi_driver.act(ir::InstructionRegister {
        action: ir::Action::Write,
        target: ir::Target::Dtr,
    },
                   vec![0x00, 0x00, 0x00, 0x00],
    )?;
    // move Dtr to Cntr to clear Cntr to zero.
    spi_driver.act(ir::InstructionRegister {
        action: ir::Action::Load,
        target: ir::Target::Cntr,
    },
                   vec![],
    )?;

    loop {
        let result = spi_driver.act(
            ir::InstructionRegister{
                action: ir::Action::Read,
                target: ir::Target::Cntr
            },
            vec![]
        )?;
        println!("read data:= {:?}", result);
    }
}

#[allow(dead_code)]
fn init_command(configuration: mdr0::Mdr0) -> Vec<u8> {
    let ir_cmd = ir::InstructionRegister {
        target: ir::Target::Mdr0,
        action: ir::Action::Write,
    };
    return vec![ir_cmd.encode(), configuration.encode(), 0x00, 0x00];
}

#[allow(dead_code)]
fn zero_dtr_command() -> Vec<u8> {
    let ir_cmd = ir::InstructionRegister {
        target: ir::Target::Dtr,
        action: ir::Action::Write,
    };
    return vec![ir_cmd.encode(), 0x00, 0x00, 0x00, 0x00];
}

#[allow(dead_code)]
fn transfer_dtr_to_cntr_command() -> Vec<u8> {
    let irc_cmd = ir::InstructionRegister {
        target: ir::Target::Cntr,
        action: ir::Action::Load,
    };
    return vec![irc_cmd.encode()];
}

#[allow(dead_code)]
fn read_cntr_command() -> Vec<u8> {
    let ir_cmd = ir::InstructionRegister {
        target: ir::Target::Cntr,
        action: ir::Action::Read,
    };
    return vec![ir_cmd.encode(), 0x00, 0x00, 0x00, 0x00];
}

#[allow(dead_code)]
fn clear_cntr_command() -> Vec<u8> {
    let ir_cmd = ir::InstructionRegister {
        target: ir::Target::Cntr,
        action: ir::Action::Load,
    };
    return vec![ir_cmd.encode()];
}