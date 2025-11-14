use anchor_lang::prelude::*;

declare_id!("4PHJdddkzpuKSu3G9ooC1VxvvE9TT7Hrma7LrkEUgoaD");

#[program]
pub mod test_hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
