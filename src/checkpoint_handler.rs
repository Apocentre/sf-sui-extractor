use eyre::{Result, Report};
use backoff::{ExponentialBackoff, future::retry};
use jsonrpsee::http_client::{HttpClient};
use futures::future::join_all;
use futures::FutureExt;
use sui_indexer::{models::objects::ObjectStatus, types::CheckpointTransactionBlockResponse, store::CheckpointData};
use sui_json_rpc::api::ReadApiClient;
use sui_types::base_types::{TransactionDigest, ObjectID, SequenceNumber};
use sui_json_rpc_types::{
  Checkpoint, OwnedObjectRef, SuiTransactionBlockEffects, SuiObjectData, SuiTransactionBlockEffectsAPI,
  SuiGetPastObjectRequest, SuiObjectDataOptions, SuiTransactionBlockResponseOptions
};

const MULTI_GET_CHUNK_SIZE: usize = 50;

type CheckpointSequenceNumber = u64;

pub struct CheckpointHandler {
  http_client: HttpClient,
}

impl CheckpointHandler {
  pub fn new(
    http_client: HttpClient,
  ) -> Self {
    Self {
      http_client,
    }
  }

  /// Download all the data we need for one checkpoint.
  pub async fn download_checkpoint_data(&self, seq: CheckpointSequenceNumber) -> Result<CheckpointData> {
    let checkpoint = self.get_checkpoint(seq).await?;
    let transactions = join_all(checkpoint.transactions.chunks(MULTI_GET_CHUNK_SIZE)
    .map(|digests| Self::multi_get_full_transactions(self.http_client.clone(), digests.to_vec())))
    .await
    .into_iter()
    .try_fold(vec![], |mut acc, chunk| {
      acc.extend(chunk?);
      Ok::<_, Report>(acc)
    })?;

    let object_changes = transactions
    .iter()
    .flat_map(|tx| Self::get_object_changes(&tx.effects))
    .collect::<Vec<_>>();
    let changed_objects = Self::fetch_changed_objects(self.http_client.clone(), object_changes).await?;

    Ok(CheckpointData {
      checkpoint,
      transactions,
      changed_objects,
    })
  }

  async fn get_checkpoint(&self, seq: CheckpointSequenceNumber) -> Result<Checkpoint> {
    let checkpoint = retry(ExponentialBackoff::default(), || async {
      let checkpoint = self.http_client
      .get_checkpoint(seq.into())
      .await
      .map_err(|e| {
        Report::msg(format!("Failed to get checkpoint with sequence number {} and error {:?}", seq, e))
      })?;

      Ok(checkpoint)
    }).await?;

    Ok(checkpoint)
  }

    // TODO(gegaowp): re-orgnize object util functions below
  pub fn get_object_changes(effects: &SuiTransactionBlockEffects,) -> Vec<(ObjectID, SequenceNumber, ObjectStatus)> {
    let created = effects.created().iter().map(|o: &OwnedObjectRef| {
      (o.reference.object_id, o.reference.version, ObjectStatus::Created)
    });
    let mutated = effects.mutated().iter().map(|o: &OwnedObjectRef| {
      (o.reference.object_id, o.reference.version, ObjectStatus::Mutated,)
    });
    let unwrapped = effects.unwrapped().iter().map(|o: &OwnedObjectRef| {
      (o.reference.object_id, o.reference.version, ObjectStatus::Unwrapped,)
    });

    created.chain(mutated).chain(unwrapped).collect()
  }

  pub async fn fetch_changed_objects(
    http_client: HttpClient,
    object_changes: Vec<(ObjectID, SequenceNumber, ObjectStatus)>,
  ) -> Result<Vec<(ObjectStatus, SuiObjectData)>> {
    join_all(object_changes.chunks(MULTI_GET_CHUNK_SIZE).map(|objects| {
        let wanted_past_object_statuses: Vec<ObjectStatus> =objects.iter().map(|(_, _, status)| *status).collect();
        let wanted_past_object_request = objects
        .iter()
        .map(|(id, seq_num, _)| SuiGetPastObjectRequest {
          object_id: *id,
          version: *seq_num,
        })
        .collect();

        http_client
        .try_multi_get_past_objects(
          wanted_past_object_request,
          Some(SuiObjectDataOptions::bcs_lossless()),
        )
        .map(move |resp| (resp, wanted_past_object_statuses))
    }))
    .await
    .into_iter()
    .try_fold(vec![], |mut acc, chunk| {
      let object_data = chunk.0?.into_iter().try_fold(vec![], |mut acc, resp| {
        let object_data = resp.into_object()?;
        acc.push(object_data);
        Ok::<Vec<SuiObjectData>, Report>(acc)
      })?;
      let mutated_object_chunk = chunk.1.into_iter().zip(object_data);
      acc.extend(mutated_object_chunk);
      Ok(acc)
    })
    .map_err(|e: Report| {
      Report::msg(format!("Failed to generate changed objects of checkpoint with err {:?}", e))
    })
  }

  pub async fn multi_get_full_transactions(
    http_client: HttpClient,
    digests: Vec<TransactionDigest>,
  ) -> Result<Vec<CheckpointTransactionBlockResponse>> {
    return Err(Report::msg(">>>>>>>>>"));
    let sui_transactions = http_client.multi_get_transaction_blocks(
      digests.clone(),
      // MUSTFIX(gegaowp): avoid double fetching both input and raw_input
      Some(
        SuiTransactionBlockResponseOptions::new()
        .with_input()
        .with_effects()
        .with_events()
        .with_raw_input(),
      ),
    )
    .await
    .map_err(|e| {
      Report::msg(format!("Failed to get transactions {:?} with error: {:?}", digests.clone(), e))
    })?;
    let sui_full_transactions: Vec<CheckpointTransactionBlockResponse> = sui_transactions
    .into_iter()
    .map(CheckpointTransactionBlockResponse::try_from)
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| {
      Report::msg(format!("Unexpected None value in SuiTransactionBlockFullResponse with error {:?}", e))
    })?;

    Ok(sui_full_transactions)
  }
}
