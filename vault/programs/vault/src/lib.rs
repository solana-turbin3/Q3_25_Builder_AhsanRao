#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};

declare_id!("HfvWmQnm1KD2nw72L3MxEEP4Vh4DhTjbmtBeQEcSRPeC");

#[program]
pub mod programs_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(ctx.bumps)?;
        
        Ok(())
    }

    pub fn deposit(ctx: Context<VaultOperation>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        
        Ok(())
    }

    pub fn withdraw(ctx: Context<VaultOperation>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()?;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"programs_vault", user.key().as_ref()],
        bump,
        space = 8 + ProgramsVault::INIT_SPACE,
    )]
    pub programs_vault: Account<'info, ProgramsVault>,
    #[account(
        mut,
        seeds = [b"programs_vault_authority", programs_vault.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize (&mut self, bumps: InitializeBumps) -> Result<()> {

        // Transfer the rent-exempt balance to the vault account
        // let rent_exempt: u64 = Rent::get()?.minimum_balance(self.programs_vault.to_account_info().data_len());
        // let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();
        
        // let cpi_accounts = Transfer {
        //     from: self.user.to_account_info(),
        //     to: self.vault.to_account_info(),
        // };

        // let cpi_context: CpiContext<'_, '_, '_, '_, Transfer<'_>> = CpiContext::new(cpi_program, cpi_accounts);

        // transfer(cpi_context, rent_exempt)?;

        // Rent is automatically deducted from the vault account, so we don't need to transfer it manually

        self.programs_vault.vault_bump = bumps.vault;
        self.programs_vault.state_bump = bumps.programs_vault;
        self.programs_vault.authority = self.user.key();
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct VaultOperation<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"programs_vault", user.key().as_ref()],
        bump = programs_vault.state_bump,
        constraint = programs_vault.authority == user.key() @ ErrorCode::Unauthorized,
    )]
    pub programs_vault: Account<'info, ProgramsVault>,
    #[account(
        mut,
        seeds = [b"programs_vault_authority", programs_vault.key().as_ref()],
        bump = programs_vault.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> VaultOperation<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_context, amount)?;

        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);
        
        // Check if vault has enough funds
        let rent_exempt = Rent::get()?.minimum_balance(0);
        require!(
            self.vault.lamports() >= amount + rent_exempt,
            ErrorCode::InsufficientFunds
        );

        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = [
            b"programs_vault_authority",
            self.programs_vault.to_account_info().key.as_ref(),
            &[self.programs_vault.vault_bump],
        ];

        let signer_seeds: &[&[&[u8]]] = &[&seeds[..]];
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        
        transfer(cpi_context, amount)?;

        Ok(())
    }
}

// Challenges:
// Check that the withdraw leaves the vault with a rent-exempt balance
// Check the account has enough funds for the user to withdraw
// Implement a context to close the account
// Tip: Look for a close constraint
// Don’t forget to manually close the vault account (how do you do that? In doubt ask Ayodeji)
// Don’t the withdraw and deposit context have the same accounts? Can’t we just use the same context in different instructions?

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        close = user, // Automatically closes and transfers lamports to user
        seeds = [b"programs_vault", user.key().as_ref()],
        bump = programs_vault.state_bump,
        constraint = programs_vault.authority == user.key() @ ErrorCode::Unauthorized, // Ensure only owner can close
    )]
    pub programs_vault: Account<'info, ProgramsVault>,
    #[account(
        mut,
        seeds = [b"programs_vault_authority", programs_vault.key().as_ref()],
        bump = programs_vault.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        // Transfer all remaining lamports from vault to user
        let vault_balance = self.vault.lamports();
        
        if vault_balance > 0 {
            let cpi_program = self.system_program.to_account_info();
            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.user.to_account_info(),
            };

            let seeds = [
                b"programs_vault_authority",
                self.programs_vault.to_account_info().key.as_ref(),
                &[self.programs_vault.vault_bump],
            ];

            let signer_seeds: &[&[&[u8]]] = &[&seeds[..]];
            let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

            transfer(cpi_context, self.vault.lamports())?;
        }
        // The programs_vault account is automatically closed due to the 'close = user' constraint
        // This happens after this function completes
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct ProgramsVault {
    pub vault_bump: u8,
    pub state_bump: u8,
    pub authority: Pubkey, // Owner
}

// impl Space for ProgramsVault {
//     fn space() -> usize {
//         8 + // discriminator
//         1 + // bump
//         32 + // authority
//         8 // created_at
//     }
// }

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds in vault")]
    InsufficientFunds,
    #[msg("Unauthorized access - you don't own this vault")]
    Unauthorized,
    #[msg("Invalid amount - must be greater than 0")]
    InvalidAmount,
}