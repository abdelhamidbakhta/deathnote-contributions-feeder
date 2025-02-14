pub struct ContractUpdateStatus {
    pub contribution_id: String,
    pub transaction_hash: String,
}

impl ContractUpdateStatus {
    pub fn new(contribution_id: String, transaction_hash: String) -> Self {
        ContractUpdateStatus {
            contribution_id,
            transaction_hash,
        }
    }
}
