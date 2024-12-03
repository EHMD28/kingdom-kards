pub trait ToAction {
    fn to_action(&self) -> Action;
}

pub enum Actions {
    None,
    PlayBlackAce,
    PlayRedAce,
    PlayNumber,
    PlayJack,
    PlayQueen,
    PlayKing,
}

pub struct Action(pub Actions);

impl Action {
    pub fn to_str(&self) {}
}
