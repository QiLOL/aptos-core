// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

use crate::types::partial_state_compute_result::PartialStateComputeResult;
use anyhow::Result;
use aptos_crypto::HashValue;
use aptos_executor_types::execution_output::ExecutionOutput;
use aptos_storage_interface::ExecutedTrees;
use do_ledger_update::DoLedgerUpdate;
use do_state_checkpoint::DoStateCheckpoint;

pub mod do_get_execution_output;
pub mod do_ledger_update;
pub mod do_state_checkpoint;

pub struct ApplyExecutionOutput;

impl ApplyExecutionOutput {
    pub fn run(
        chunk_output: ExecutionOutput,
        base_view: &ExecutedTrees,
        known_state_checkpoint_hashes: Option<Vec<Option<HashValue>>>,
    ) -> Result<PartialStateComputeResult> {
        let (result_state, next_epoch_state, state_checkpoint_output) = DoStateCheckpoint::run(
            chunk_output,
            base_view.state(),
            None, // append_state_checkpoint_to_block
            known_state_checkpoint_hashes,
            /*is_block=*/ false,
        )?;
        let (ledger_update_output, _to_discard, _to_retry) =
            DoLedgerUpdate::run(state_checkpoint_output, base_view.txn_accumulator().clone())?;
        let output = PartialStateComputeResult::new(
            base_view.state().clone(),
            result_state,
            next_epoch_state,
        );
        output.set_ledger_update_output(ledger_update_output);

        Ok(output)
    }
}