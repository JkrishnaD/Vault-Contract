use anchor_lang::{ prelude::*, system_program::{ transfer, Transfer } };

declare_id!("CBk5WRtN2Zhm8BGUSdH2WHwDaUJ8JNtCdbNGsD34a9oQ");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposite(ctx: Context<Payment>) -> Result<()> {
        ctx.accounts.deposite(1000)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Payment>) -> Result<()> {
        ctx.accounts.withdraw(1000)?;
        Ok(())
    }
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;
        Ok(())
    }
}

impl<'info> Payment<'info> {
    pub fn deposite(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let key_ref = self.user.key();
        let vault_bump = self.vault_state.vault_bump;
        let seeds = &[b"vault", key_ref.as_ref(), &[vault_bump]];
        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

impl VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1;
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // this is the account that will hold the
    #[account(
        init,
        payer = user,
        space = 8 + VaultState::INIT_SPACE,
        seeds = [b"state", user.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    // this is the pda account where the funds will be held
    #[account(seeds = [b"vault", user.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(seeds = [b"vault", user.key().as_ref()], bump)]
    pub vault_state: Account<'info, VaultState>,

    #[account(seeds = [b"vault", user.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}
