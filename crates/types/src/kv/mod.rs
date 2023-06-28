pub mod helpers;

#[derive(Clone, Debug)]
pub struct Pair {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct Pairs {
    pub pairs: Vec<Pair>,
}
