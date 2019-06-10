use std::sync::mpsc;
use std::collections::BinaryHeap;

const N_SHARDS: usize = 4;

pub struct Transaction;

impl Transaction {
    fn get_shard(&self) -> usize {
        3
    }
}

#[derive(PartialEq)]
pub struct MempoolItem {
    id: [u8; 32],
    tx: Transaction,
    package_fee: u32
};

impl From<Transaction> for MempoolItem {
    fn from(tx: Transaction) -> MempoolItem {
        MempoolItem {
            id: [0; 32],
            tx,
            package_fee: 0
        }
    }
}

fn main() {    
    let (tx_send, tx_recv) = mpsc::unbounded();
}

fn coordinator(tx_recv: mpsc::Receiver<Transaction>, shard_send: [mpsc::Sender<Transaction>; N_SHARDS]) {
    tx_recv.for_each(|tx|
        shard_send[tx.get_shard()].send(tx)
    )
}

pub struct Shard {
    tx_recv: mpsc::Receciver<Transaction>,
    queue: BinaryHeap<MempoolItem>
}

impl Shard {
    pub fn create() -> (Shard, mpsc::Sender<Transaction>) {
        // Create shard channel
        let (tx_send, tx_recv) = mpsc::unbounded();
        
        // Create shard
        let shard = Shard {
            tx_recv,
            queue: BinaryHeap::default()
        };

        (shard, tx_send)
    }

    pub fn run(self) {
        self.tx_recv.for_each(|tx| {
            let item = MempoolItem::from(tx);
            
        })
        queue
    }
}