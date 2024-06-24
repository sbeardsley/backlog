// pub trait Atomic {
//     type TransactionResult;
//     async fn start_tx(self) -> Result<Self::TransactionResult, ConnectionError>;
//     async fn abort_tx(tx: Self::TransactionResult) -> Result<(), ConnectionError>;
//     async fn commit_tx(tx: Self::TransactionResult) -> Result<(), ConnectionError>;
// }
