use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use std::str;

declare_id!("DsqmUY3txpP3RNLxjHUWEepFTHiZBK6BMqaucqJD1Jzh");

#[program]
pub mod zero_copy {

    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_hit_stack_size(_ctx: Context<InitializeHitStackSize>) -> Result<()> {
        Ok(())
    }

    pub fn set_data(ctx: Context<SetData>, string_to_set: String, index: u64) -> Result<()> {
        let text_to_add_to_the_account = str::from_utf8(string_to_set.as_bytes()).unwrap();
        msg!(text_to_add_to_the_account);

        // Since the account is bigger that the heap space as soon as we access the whole account we will get a out of memory error        
        // let string = &ctx.accounts.data_holder.load_mut()?.greet_string;
        // let complete_string = str::from_utf8(string).unwrap(); 
        // msg!("DataLength: {}", string.len());
        // msg!("CompleteString: {}", complete_string);

        // So the solution is use copy_from_slice and mem copy when we want to access data in the big account
        ctx.accounts
            .data_holder
            .load_mut()?
            .greet_string[((index) as usize)..((index +912) as usize)]
            .copy_from_slice(string_to_set.as_bytes());

        Ok(())
    }

    pub fn increase_account_data_zero_copy(_ctx: Context<IncreaseZeroCopy>, _len: u16) -> Result<()> {
        Ok(())
    }

    pub fn increase_account_data(_ctx: Context<IncreaseAccoutSize>, _len: u16) -> Result<()> {
        Ok(())
    }

    pub fn set_data_no_zero_copy(ctx: Context<SetDataNoZeroCopy>, string_to_set: String) -> Result<()> {
        // This will work up to the limit of head space
        ctx.accounts.data_holder.greet_string.push_str(&string_to_set);
        //msg!(&ctx.accounts.data_holder.greet_string.len().to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, seeds = [b"data_holder_zero_copy_v0", author.key().as_ref()], bump, payer=author, space= 10 * 1024 as usize)]
    pub data_holder: AccountLoader<'info, DataHolder>,
    #[account(init, seeds = [b"data_holder_no_zero_copy_v0", author.key().as_ref()], bump, payer=author, space= 10 * 1024 as usize)]
    pub data_holder_no_zero_copy: Account<'info, DataHolderNoZeroCopy>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub data_holder: AccountLoader<'info, DataHolder>,
    #[account(mut)]
    pub writer: Signer<'info>,
}

#[account(zero_copy)]
#[repr(packed)]
pub struct DataHolder {
    // 40952 = 40960 - 8 (account desciminator)
    pub greet_string: [u8; 40952],
}

#[derive(Accounts)]
pub struct SetDataNoZeroCopy<'info> {
    #[account(mut)]
    pub data_holder: Account<'info, DataHolderNoZeroCopy>,
    #[account(mut)]
    pub writer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(len: u16)]
pub struct IncreaseZeroCopy<'info> {
    #[account(mut, 
        realloc = len as usize, 
        realloc::zero = true, 
        realloc::payer=writer)]
    pub data_holder: AccountLoader<'info, DataHolder>,
    #[account(mut)]
    pub writer: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(len: u16)]
pub struct IncreaseAccoutSize<'info> {
    #[account(mut, 
        realloc = len as usize, 
        realloc::zero = true, 
        realloc::payer=writer)]
    pub data_holder: Account<'info, DataHolderNoZeroCopy>,
    #[account(mut)]
    pub writer: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataHolderNoZeroCopy {
    pub greet_string: String,
}

#[derive(Accounts)]
pub struct InitializeHitStackSize<'info> {
    #[account(init, 
        seeds = [b"hit_stack_size", author.key().as_ref()], 
        bump, 
        payer=author, 
        space= 10 * 1024 as usize)]
    pub data_holder: Account<'info, HitStackSize>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[account]
// 9 * 128 = 1152 bytes -> With the way anchor deserialized the account in the init functio this will already hit the stack limit
pub struct HitStackSize {
    board: [Option<BigStruct>; 9], 
}

#[derive(
    AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq,
)]
// Size of this struct is 32 bytes * 4 = 128 bytes
pub struct BigStruct {
    pub public_key_1: Pubkey,
    pub public_key_2: Pubkey,
    pub public_key_3: Pubkey,
    pub public_key_4: Pubkey,
}
