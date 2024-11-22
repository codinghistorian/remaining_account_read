use anchor_lang::prelude::*;
use crate::error::ErrorCode;
#[account]
#[derive(Default, Debug, InitSpace)]
pub struct InvestTracker {
    pub amount_invested: u64,
}   

impl InvestTracker {
    pub fn save_changes(&self, writer: &mut dyn std::io::Write) -> Result<()> {
        let data = self.try_to_vec()
            .map_err(|_| ErrorCode::SerializationError)?;
        writer.write_all(&data)
            .map_err(|_| ErrorCode::SerializationError)?;
        Ok(())
    }
}
