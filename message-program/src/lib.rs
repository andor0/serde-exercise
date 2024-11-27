use std::{num::NonZeroU64, u64};

use anyhow::{Ok, Result};
use bytemuck::{Pod, Zeroable};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, PartialEq, Eq)]
pub struct Message {
    // sender pubkey
    pub sender_id: [u8; 32],
    // size of the Message
    pub size: u64,
    // validator priority fees
    pub priority_fee: Option<NonZeroU64>,
    // content of the message
    pub data: [u8; 1024],
}
impl Message {
    pub const LEN: usize = size_of::<Self>();
}

#[derive(TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
enum Instructions {
    Initialize = 3,
    Close = 6,
    Update = 9,
}

pub fn send_instruction(data: &[u8]) -> Result<()> {
    let (discriminator, data) = data.split_first().unwrap();
    process_instruction(*discriminator, data)?;
    Ok(())
}

fn process_instruction(discriminator: u8, data: &[u8]) -> Result<()> {
    match Instructions::try_from(discriminator)? {
        Instructions::Initialize => initialize_message(data),
        Instructions::Close => close_message(data),
        Instructions::Update => update_message(data),
    }
}

fn initialize_message(data: &[u8]) -> Result<()> {
    let msg = bytemuck::try_from_bytes::<Message>(data).unwrap();
    println!("Allocatin {:?} bytes for {:?}...", msg.size, msg.sender_id);
    Ok(())
}
fn close_message(data: &[u8]) -> Result<()> {
    let msg = bytemuck::try_from_bytes::<Message>(data).unwrap();
    assert!(msg.priority_fee.is_some());
    println!("{:?} deleted.", msg.size);
    Ok(())
}
fn update_message(data: &[u8]) -> Result<()> {
    let msg = bytemuck::try_from_bytes::<Message>(data).unwrap();
    println!(
        "collected {:?} lamports from {:?}",
        msg.priority_fee.unwrap(),
        msg.sender_id
    );
    Ok(())
}
