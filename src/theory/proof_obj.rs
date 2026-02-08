#[derive(Debug, Clone)]
pub struct ProofStep {
    pub rule: String,
    pub from: Vec<usize>,
    pub formula: String,
}

#[derive(Debug, Clone)]
pub struct Proof {
    pub steps: Vec<ProofStep>,
    pub conclusion: String,
    pub success: bool,
}
