use druid::Data;
use super::Switch;

#[derive(Debug, Clone)]
pub struct ViewState {
    pub switches: Vec<Switch>,
    pub tx: crossbeam::channel::Sender<u32>,
}
impl Data for ViewState {
    fn same(&self, other: &Self) -> bool {
        if self.switches.len() != other.switches.len() {
            return false;
        }
        for (lhs, rhs) in self.switches.iter().zip(other.switches.iter()) {
            if lhs.name != rhs.name {
                return false;
            }
        }
        true
    }
}