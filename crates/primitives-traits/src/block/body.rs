//! Block body abstraction.

use alloc::{fmt, vec::Vec};

use alloy_consensus::{BlockHeader, Transaction, TxType};
use alloy_eips::{eip4895::Withdrawal, eip7685::Requests};
use alloy_primitives::{Address, B256};

use crate::InMemorySize;

/// Abstraction for block's body.
pub trait BlockBody:
    Send
    + Sync
    + Unpin
    + Clone
    + Default
    + fmt::Debug
    + PartialEq
    + Eq
    + serde::Serialize
    + for<'de> serde::Deserialize<'de>
    + alloy_rlp::Encodable
    + alloy_rlp::Decodable
    + InMemorySize
{
    /// Ordered list of signed transactions as committed in block.
    // todo: requires trait for signed transaction
    type SignedTransaction: Transaction;

    /// Header type (uncle blocks).
    type Header: BlockHeader;

    /// Withdrawals in block.
    type Withdrawals: Iterator<Item = Withdrawal>;

    /// Returns reference to transactions in block.
    fn transactions(&self) -> &[Self::SignedTransaction];

    /// Returns `Withdrawals` in the block, if any.
    // todo: branch out into extension trait
    fn withdrawals(&self) -> Option<&Self::Withdrawals>;

    /// Returns reference to uncle block headers.
    fn ommers(&self) -> &[Self::Header];

    /// Returns [`Requests`] in block, if any.
    fn requests(&self) -> Option<&Requests>;

    /// Calculate the transaction root for the block body.
    fn calculate_tx_root(&self) -> B256;

    /// Calculate the ommers root for the block body.
    fn calculate_ommers_root(&self) -> B256;

    /// Calculate the withdrawals root for the block body, if withdrawals exist. If there are no
    /// withdrawals, this will return `None`.
    // todo: can be default impl if `calculate_withdrawals_root` made into a method on
    // `Withdrawals` and `Withdrawals` moved to alloy
    fn calculate_withdrawals_root(&self) -> Option<B256>;

    /// Recover signer addresses for all transactions in the block body.
    fn recover_signers(&self) -> Option<Vec<Address>>;

    /// Returns whether or not the block body contains any blob transactions.
    fn has_blob_transactions(&self) -> bool {
        self.transactions().iter().any(|tx| tx.ty() == TxType::Eip4844 as u8)
    }

    /// Returns whether or not the block body contains any EIP-7702 transactions.
    fn has_eip7702_transactions(&self) -> bool {
        self.transactions().iter().any(|tx| tx.ty() == TxType::Eip7702 as u8)
    }

    /// Returns an iterator over all blob transactions of the block
    fn blob_transactions_iter(&self) -> impl Iterator<Item = &Self::SignedTransaction> + '_ {
        self.transactions().iter().filter(|tx| tx.ty() == TxType::Eip4844 as u8)
    }

    /// Returns only the blob transactions, if any, from the block body.
    fn blob_transactions(&self) -> Vec<&Self::SignedTransaction> {
        self.blob_transactions_iter().collect()
    }

    /// Returns an iterator over all blob versioned hashes from the block body.
    fn blob_versioned_hashes_iter(&self) -> impl Iterator<Item = &B256> + '_;

    /// Returns all blob versioned hashes from the block body.
    fn blob_versioned_hashes(&self) -> Vec<&B256> {
        self.blob_versioned_hashes_iter().collect()
    }
}
