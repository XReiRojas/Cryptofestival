use anchor_lang::prelude::*;

declare_id!("BiyZfCwRd3TzMeNXfMfLMm6zqS174j3NZrKZgCCTUVd7");

#[program]
pub mod event_manager {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
