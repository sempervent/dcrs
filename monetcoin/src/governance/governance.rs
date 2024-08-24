// governance/src/governance.rs

use crate::blockchain::Blockchain;

pub struct Governance {
    pub proposals: Vec<Proposal>,
}

pub struct Proposal {
    pub id: u64,
    pub description: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub coins_burned: u64,
}

impl Governance {
    pub fn new() -> Self {
        Governance {
            proposals: Vec::new(),
        }
    }

    pub fn propose(&mut self, description: String) {
        let id = self.proposals.len() as u64;
        let proposal = Proposal {
            id,
            description,
            votes_for: 0,
            votes_against: 0,
            coins_burned: 0,
        };
        self.proposals.push(proposal);
    }

    pub fn vote(&mut self, proposal_id: u64, vote_for: bool, coins: u64) {
        if let Some(proposal) = self.proposals.iter_mut().find(|p| p.id == proposal_id) {
            if vote_for {
                proposal.votes_for += 1;
            } else {
                proposal.votes_against += 1;
            }
            proposal.coins_burned += coins;
        }
    }

    pub fn finalize_proposal(&mut self, proposal_id: u64, blockchain: &mut Blockchain) {
        if let Some(proposal) = self.proposals.iter().find(|p| p.id == proposal_id) {
            if proposal.votes_for > proposal.votes_against {
                // Implement proposal effect on blockchain
            }
            // Burn the coins associated with the proposal
            blockchain.burn_coins(proposal.coins_burned);
        }
    }
}
