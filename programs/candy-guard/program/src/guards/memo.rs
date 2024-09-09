use super::*;

use solana_program::program::invoke;
use spl_memo::build_memo;

use crate::{state::GuardType, utils::assert_keys_equal};

/// Guard that logs the items minted from our candy machine upon each mint and the minter address.
///
/// List of accounts required:
///
///   0. `[]` Account minting the NFT.
///
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Memo {
    pub minter: Pubkey,
}

impl Guard for Memo {
    fn size() -> usize {
        32 // minter
    }

    fn mask() -> u64 {
        GuardType::as_mask(GuardType::Memo)
    }
}

impl Condition for Memo {
    fn validate<'info>(
        &self,
        ctx: &mut EvaluationContext,
        _guard_set: &GuardSet,
        _mint_args: &[u8],
    ) -> Result<()> {
        // current rem accounts cursor
        let index = ctx.account_cursor;

        // validate minter acc is correct is similar to one in accs context
        // by getting it from the arguments supplied
        let minter = try_get_account_info(ctx.accounts.remaining, index)?;

        // assert that the keys are equal
        assert_keys_equal(minter.key, &self.minter)?;

        // consume the account index
        ctx.account_cursor += 1;

        // map consumed account to use in next ix
        ctx.indices.insert("minter_acc_index", index);

        Ok(())
    }

    fn post_actions<'info>(
        &self,
        ctx: &mut EvaluationContext,
        _guard_set: &GuardSet,
        _mint_args: &[u8],
    ) -> Result<()> {
        // get the minter acc using acc index we saved in validate
        let minter = try_get_account_info(ctx.accounts.remaining, ctx.indices["minter_acc_index"])?;

        // items redeemed from the candy machine
        let items_redeemed = ctx.accounts.candy_machine.items_redeemed;

        // build our memo instruction
        let redeemed_items_msg = format!("Items minted from CM -> {items_redeemed}");
        let minter_msg = format!("Latest items minted by -> {minter:?}");

        let redeemed_items_memo_ix = build_memo(redeemed_items_msg.as_bytes(), &[minter.key]);
        let minter_memo_ix = build_memo(minter_msg.as_bytes(), &[minter.key]);

        invoke(
            &redeemed_items_memo_ix,
            &[ctx.accounts.payer.to_account_info()],
        )?;
        invoke(&minter_memo_ix, &[ctx.accounts.payer.to_account_info()])?;

        Ok(())
    }
}
