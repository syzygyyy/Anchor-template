use anchor_lang::prelude::*;

declare_id!("47EwzEbrAx2nE7PekrV4Jf8B7Ro3qodVEFc4H6AZ2TmZ");

#[program]
pub mod my_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
