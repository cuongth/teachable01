use borsh::{BorshSerialize, BorshDeserialize, from_slice, to_vec};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    msg,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Rust program entrypoint");
    // Iterating accounts is safer than indexing
    let accounts_iter =&mut accounts.iter();
    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;
    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account {} does not have the correct program id", account.owner);
        return Err(ProgramError::IncorrectProgramId);
    } else {
        msg!("Greeted account {} have correct program id", account.owner);
    }
    let mut greeting_array = account.data.try_borrow_mut().unwrap();
    let mut greeting_data: GreetingAccount = from_slice::<GreetingAccount>(&greeting_array[..]).unwrap();
    greeting_data.counter += 1;
    greeting_array[..].copy_from_slice(&to_vec(&greeting_data).unwrap());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_greeting_from_slice() {
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let accounts = vec![account];
        let mut greeting_array = accounts[0].data.try_borrow_mut().unwrap();
        let mut greeting_data: GreetingAccount = from_slice::<GreetingAccount>(&greeting_array[..]).unwrap();
        assert_eq!(greeting_data.counter, 0);
        greeting_data.counter += 1;
        greeting_array[..].copy_from_slice(&to_vec(&greeting_data).unwrap());
        assert_eq!(greeting_data.counter, 1);
    }

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();
        let accounts = vec![account];
        let greeting_array = accounts[0].data.try_borrow().unwrap();
        let greeting_data: GreetingAccount = from_slice::<GreetingAccount>(&greeting_array[..]).unwrap();
        assert_eq!(greeting_data.counter, 0);

        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        let greeting_array1 = accounts[0].data.try_borrow().unwrap();
        let greeting_data1: GreetingAccount = from_slice::<GreetingAccount>(&greeting_array1[..]).unwrap();
        assert_eq!(greeting_data1.counter, 1);
    }

}
