#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");
// declare_id!("AyD5wnZEk5po81hTJBG8LxVRHb7GdewWUVhVHv24dm3");

#[program]
pub mod votingapp {
    use super::*;

    // Initialize the poll
    pub fn initialize_poll(
        ctx: Context<InitializePoll>, 
        poll_id: u64,
        description: String,
        poll_start: u64,
        poll_end: u64
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.poll_id = poll_id;
        poll.description = description;
        poll.poll_start = poll_start;
        poll.poll_end = poll_end;
        poll.candidate_amount = 0;
        Ok(())
    }
}
// 1
#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init, // TO INITIALIZE ACCOUNT AUTOMATICALLY
        payer = signer,
        space = 8 + Poll::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}
// 2
#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id: u64,
    #[max_len(200)]
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64,
}






















// #![allow(clippy::result_large_err)]

// use anchor_lang::prelude::*;

// declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

// #[program]
// pub mod votingapp {
//     use super::*;

//     // Everything to Initialize the POLL.
//     pub fn initialize_poll(ctx: Context<InitializePoll>, 
//                             poll_id: u64,
//                             description: String,
//                             poll_start: u64,
//                             poll_end: u64) -> Result<()> {
//         let poll = &mut ctx.accounts.poll;
//         poll.poll_id = poll_id;
//         poll.description = description;
//         poll.poll_start = poll_start;
//         poll.poll_end = poll_end;
//         poll.candidate_amount = 0;
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// #[instruction(poll_id: u64)]
// pub struct InitializePoll<'info> {
//   #[account(mut)]
//   pub signer: Signer<'info>,
//   #[account(
//       init, // when you have init, it's create an account.
//       payer = signer,
//       space = 8 + Poll::INIT_SPACE,
//       seeds = [poll_id.to_le_bytes().as_ref()],
//       bump,
//   )]
//   pub poll: Account<'info, Poll>,

//   pub system_program: Program<'info, System>,
// }

// #[account]
// #[derive(InitSpace)]
// pub struct Poll {
//   pub poll_id: u64,
//   #[max_len(200)]
//   pub description: String,
//   pub poll_start: u64,
//   pub poll_end: u64,
//   pub candidate_amount: u64,
// }




// //   pub fn decrement(ctx: Context<Update>) -> Result<()> {
// //     ctx.accounts.votingapp.count = ctx.accounts.votingapp.count.checked_sub(1).unwrap();
// //     Ok(())
// //   }

// //   pub fn increment(ctx: Context<Update>) -> Result<()> {
// //     ctx.accounts.votingapp.count = ctx.accounts.votingapp.count.checked_add(1).unwrap();
// //     Ok(())
// //   }

// //   pub fn initialize(_ctx: Context<InitializeVotingapp>) -> Result<()> {
// //     Ok(())
// //   }

// //   pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
// //     ctx.accounts.votingapp.count = value.clone();
// //     Ok(())
// //   }


// // #[derive(Accounts)]
// // pub struct InitializeVotingapp<'info> {
// //   #[account(mut)]
// //   pub payer: Signer<'info>,

// //   #[account(
// //   init,
// //   space = 8 + Votingapp::INIT_SPACE,
// //   payer = payer
// //   )]
// //   pub votingapp: Account<'info, Votingapp>,
// //   pub system_program: Program<'info, System>,
// // }
// // #[derive(Accounts)]
// // pub struct CloseVotingapp<'info> {
// //   #[account(mut)]
// //   pub payer: Signer<'info>,

// //   #[account(
// //   mut,
// //   close = payer, // close account and return lamports to payer
// //   )]
// //   pub votingapp: Account<'info, Votingapp>,
// // }

// // #[derive(Accounts)]
// // pub struct Update<'info> {
// //   #[account(mut)]
// //   pub votingapp: Account<'info, Votingapp>,
// // }

// // #[account]
// // #[derive(InitSpace)]
// // pub struct Votingapp {
// //   count: u8,
// // }
