use anchor_lang::prelude::*;

declare_id!("GQ27gS3sM95CWmwbN4V9kPwKTeHQBfqWQBQn4rGNNrci");

#[program]
pub mod cef {
    use super::*;

    /// Initialize a Fund PDA owned by the program.
    /// `name` limited to 64 bytes; `description` optional (max 256).
    pub fn initialize_fund(
        ctx: Context<InitializeFund>,
        name: String,
        description: String,
    ) -> Result<()> {
        let fund = &mut ctx.accounts.fund;

        require!(name.len() <= Fund::MAX_NAME_LEN, ErrorCode::NameTooLong);
        require!(
            description.len() <= Fund::MAX_DESCRIPTION_LEN,
            ErrorCode::DescriptionTooLong
        );

        fund.creator = *ctx.accounts.creator.key;
        fund.name = name;
        fund.description = description;
        fund.total_donations = 0;
        fund.donation_count = 0;
        fund.request_count = 0;
        fund.bump = *ctx.bumps.get("fund").ok_or(ErrorCode::BumpNotFound)?;
        emit!(FundInitialized {
            fund: fund.key(),
            creator: fund.creator,
            name: fund.name.clone(),
        });
        Ok(())
    }

    /// Donate lamports to the Fund PDA.
    /// The donor signs and the transaction includes lamports moved to the Fund PDA address.
    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let fund = &mut ctx.accounts.fund;
        let donor = &ctx.accounts.donor;
        let system_program = &ctx.accounts.system_program;

        // Transfer lamports from donor to fund PDA using system_instruction::transfer via CPI
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            donor.key,
            fund.key().as_ref(),
            amount,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                donor.to_account_info(),
                fund.to_account_info(),
                system_program.to_account_info(),
            ],
        )?;

        // Update accounting
        fund.total_donations = fund
            .total_donations
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;
        fund.donation_count = fund
            .donation_count
            .checked_add(1)
            .ok_or(ErrorCode::Overflow)?;

        // Emit event for indexing
        emit!(DonationMade {
            fund: fund.key(),
            donor: *donor.key,
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /// Create a withdrawal request PDA (metadata only).
    /// Maintainer (fund creator) should call this.
    pub fn create_withdrawal_request(
        ctx: Context<CreateWithdrawalRequest>,
        amount: u64,
        recipient: Pubkey,
        reason: String,
    ) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);
        require!(reason.len() <= Request::MAX_REASON_LEN, ErrorCode::ReasonTooLong);

        let fund = &mut ctx.accounts.fund;
        let request = &mut ctx.accounts.request;

        // Only fund creator (maintainer) can create requests
        require!(
            fund.creator == *ctx.accounts.maintainer.key,
            ErrorCode::Unauthorized
        );

        let id = fund.request_count;
        request.id = id;
        request.fund = fund.key();
        request.amount = amount;
        request.recipient = recipient;
        request.reason = reason;
        request.created_by = *ctx.accounts.maintainer.key;
        request.executed = false;
        request.bump = *ctx.bumps.get("request").ok_or(ErrorCode::BumpNotFound)?;

        fund.request_count = fund
            .request_count
            .checked_add(1)
            .ok_or(ErrorCode::Overflow)?;

        emit!(WithdrawalRequested {
            fund: fund.key(),
            request: request.key(),
            id,
            amount,
            recipient,
            created_by: request.created_by,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /// Execute a previously created withdrawal request.
    /// Only the fund creator (maintainer) may execute.
    pub fn execute_withdrawal(ctx: Context<ExecuteWithdrawal>) -> Result<()> {
        let fund = &mut ctx.accounts.fund;
        let request = &mut ctx.accounts.request;
        let maintainer = &ctx.accounts.maintainer;
        let recipient = &mut ctx.accounts.recipient;

        // Access control: only fund creator can execute
        require!(fund.creator == *maintainer.key, ErrorCode::Unauthorized);
        require!(!request.executed, ErrorCode::AlreadyExecuted);
        require!(request.amount > 0, ErrorCode::InvalidAmount);

        // Check enough lamports in Fund PDA
        let fund_lamports = ctx.accounts.fund.to_account_info().lamports();
        require!(fund_lamports >= request.amount, ErrorCode::InsufficientFunds);

        // Move lamports from fund PDA to recipient by adjusting lamports on accounts
        **ctx.accounts.fund.to_account_info().try_borrow_mut_lamports()? -= request.amount;
        **recipient.to_account_info().try_borrow_mut_lamports()? += request.amount;

        // Mark executed and update fund accounting if desired (this example only records request executed)
        request.executed = true;

        emit!(WithdrawalExecuted {
            fund: fund.key(),
            request: request.key(),
            id: request.id,
            amount: request.amount,
            recipient: request.recipient,
            executed_by: *maintainer.key,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}

//
// Accounts / Data Structures
//

#[derive(Accounts)]
#[instruction(name: String, description: String)]
pub struct InitializeFund<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    /// The fund PDA account which stores fund metadata and receives lamports.
    /// We init with space sized by Fund::MAX_SIZE and payer = creator.
    #[account(
        init,
        payer = creator,
        seeds = [b"fund", creator.key().as_ref()],
        bump,
        space = Fund::MAX_SIZE
    )]
    pub fund: Account<'info, Fund>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub donor: Signer<'info>,

    /// Fund PDA must exist. We mark it mut because its lamports and data can change.
    #[account(mut, seeds = [b"fund", fund.creator.as_ref()], bump = fund.bump)]
    pub fund: Account<'info, Fund>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u64, recipient: Pubkey)]
pub struct CreateWithdrawalRequest<'info> {
    #[account(mut)]
    pub maintainer: Signer<'info>,

    /// Fund PDA
    #[account(mut, seeds = [b"fund", fund.creator.as_ref()], bump = fund.bump)]
    pub fund: Account<'info, Fund>,

    /// Request PDA storing request metadata
    #[account(
        init,
        payer = maintainer,
        seeds = [b"request", fund.key().as_ref(), &fund.request_count.to_le_bytes()],
        bump,
        space = Request::MAX_SIZE
    )]
    pub request: Account<'info, Request>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteWithdrawal<'info> {
    #[account(mut)]
    pub maintainer: Signer<'info>,

    /// Fund PDA - source of lamports
    #[account(mut, seeds = [b"fund", fund.creator.as_ref()], bump = fund.bump)]
    pub fund: Account<'info, Fund>,

    /// The recipient of the withdrawal (any system account)
    /// This is unchecked because we only transfer lamports to it.
    #[account(mut)]
    pub recipient: UncheckedAccount<'info>,

    /// The request PDA that authorizes this transfer
    #[account(mut, seeds = [b"request", fund.key().as_ref(), &request.id.to_le_bytes()], bump = request.bump)]
    pub request: Account<'info, Request>,

    pub system_program: Program<'info, System>,
}

//
// Data accounts
//

#[account]
pub struct Fund {
    pub creator: Pubkey,       // 32
    pub name: String,          // up to 64 bytes
    pub description: String,   // up to 256 bytes
    pub total_donations: u64,  // 8
    pub donation_count: u64,   // 8
    pub request_count: u64,    // 8
    pub bump: u8,              // 1
}

// size calculation: discriminator (8) + fields
impl Fund {
    pub const MAX_NAME_LEN: usize = 64;
    pub const MAX_DESCRIPTION_LEN: usize = 256;
    pub const MAX_SIZE: usize = 8 + // discriminator
        32 + // creator
        4 + Self::MAX_NAME_LEN + // name string prefix + bytes
        4 + Self::MAX_DESCRIPTION_LEN + // description string prefix + bytes
        8 + // total_donations
        8 + // donation_count
        8 + // request_count
        1; // bump
}

#[account]
pub struct Request {
    pub id: u64,           // 8
    pub fund: Pubkey,      // 32
    pub amount: u64,       // 8
    pub recipient: Pubkey, // 32
    pub reason: String,    // up to 256
    pub created_by: Pubkey,// 32
    pub executed: bool,    // 1
    pub bump: u8,          // 1
}

impl Request {
    pub const MAX_REASON_LEN: usize = 256;
    pub const MAX_SIZE: usize = 8 + // discriminator
        8 + // id
        32 + // fund
        8 + // amount
        32 + // recipient
        4 + Self::MAX_REASON_LEN + // reason
        32 + // created_by
        1 + // executed
        1; // bump
}

//
// Events
//

#[event]
pub struct FundInitialized {
    pub fund: Pubkey,
    pub creator: Pubkey,
    pub name: String,
}

#[event]
pub struct DonationMade {
    pub fund: Pubkey,
    pub donor: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct WithdrawalRequested {
    pub fund: Pubkey,
    pub request: Pubkey,
    pub id: u64,
    pub amount: u64,
    pub recipient: Pubkey,
    pub created_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct WithdrawalExecuted {
    pub fund: Pubkey,
    pub request: Pubkey,
    pub id: u64,
    pub amount: u64,
    pub recipient: Pubkey,
    pub executed_by: Pubkey,
    pub timestamp: i64,
}

//
// Errors
//

#[error_code]
pub enum ErrorCode {
    #[msg("Provided name is too long")]
    NameTooLong,
    #[msg("Provided description is too long")]
    DescriptionTooLong,
    #[msg("Provided reason is too long")]
    ReasonTooLong,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Insufficient funds in fund PDA")]
    InsufficientFunds,
    #[msg("Unauthorized action")]
    Unauthorized,
    #[msg("Request already executed")]
    AlreadyExecuted,
    #[msg("Overflow occurred")]
    Overflow,
    #[msg("Bump seed not found")]
    BumpNotFound,
}
