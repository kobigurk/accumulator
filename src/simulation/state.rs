use crate::accumulator::{Accumulator, MembershipProof};
use crate::group::UnknownOrderGroup;
use uuid::Uuid;

#[derive(Clone, Hash)]
pub struct Utxo {
  id: Uuid,
}

#[derive(Clone)]
// TODO: Maybe don't use pub(super) everywhere.
pub struct Transaction<G: UnknownOrderGroup> {
  pub(super) utxos_added: Vec<Utxo>,
  pub(super) utxos_deleted: Vec<(Utxo, Accumulator<G>)>,
}

pub struct Block<G: UnknownOrderGroup> {
  pub(super) height: u64,
  pub(super) _transactions: Vec<Transaction<G>>,
  pub(super) new_acc: Accumulator<G>,
  pub(super) proof_added: MembershipProof<G>,
  pub(super) proof_deleted: MembershipProof<G>,
}
