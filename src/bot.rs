use super::parameters::*;
use std::sync::mpsc::Sender;

pub fn bot(parameters: Parameters, tx: Sender<MyKeyCode>) {
    let val = MyKeyCode::A;
    tx.send(val).unwrap();
}
