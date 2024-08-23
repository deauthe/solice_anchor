use anchor_lang::prelude::*;

declare_id!("EaBetnnDL2p6HBNKaE5ZkE2a3qZDbAvdaN2ZD4BZBxE2");

#[program]
pub mod solice_smart_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
