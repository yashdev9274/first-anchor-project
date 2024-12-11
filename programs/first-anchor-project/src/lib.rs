use anchor_lang::prelude::*;

declare_id!("FTkYjrEgs7VosoTAAryJZfgkkN2PkmcCBFBuQEz5WXLU");

#[program]
pub mod first_anchor_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
