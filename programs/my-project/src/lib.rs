use anchor_lang::prelude::*;

declare_id!("4hYGSqSLxxwrUfKr9qFRAUUg6mp6ZipBgSV1jHUS8aXA");

#[program]
pub mod my_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
