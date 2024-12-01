use anyhow::{anyhow, Ok, Result};
use bytemuck::bytes_of;

use std::num::NonZero;

use message_program::{send_instruction, Instructions, Message};

const SENDER_ID: [u8; 32] = [0; 32];

fn main() -> Result<()> {
    initialize()?;
    update()?;
    close()?;

    Ok(())
}

fn initialize() -> Result<()> {
    // initialize a message by sending an instruction to the message program

    let message = Message {
        sender_id: SENDER_ID,
        size: u64::try_from(Message::LEN)?,
        priority_fee: None,
        data: [0; 1024],
    };

    let data = [&[Instructions::Initialize.into()], bytes_of(&message)].concat();
    send_instruction(data.as_slice())?;

    // Expected output: "Allocating..."

    Ok(())
}

fn update() -> Result<()> {
    let message = Message {
        sender_id: SENDER_ID,
        size: u64::try_from(Message::LEN)?,
        priority_fee: Some(NonZero::new(1u64)).ok_or_else(|| anyhow!("wtf"))?,
        data: [1; 1024],
    };

    let data = [&[Instructions::Update.into()], bytes_of(&message)].concat();
    send_instruction(data.as_slice())?;

    // Expected output: "collected [priority_fee] lamports from [sender_id]"

    Ok(())
}

fn close() -> Result<()> {
    let message = Message {
        sender_id: SENDER_ID,
        size: u64::try_from(Message::LEN)?,
        priority_fee: Some(NonZero::new(2u64)).ok_or_else(|| anyhow!("wtf"))?,
        data: [2; 1024],
    };

    let data = [&[Instructions::Close.into()], bytes_of(&message)].concat();
    send_instruction(data.as_slice())?;

    // Expected output: "[size] deleted."

    Ok(())
}
