use crate::brewable::Brewable;

pub trait Crushable {
    fn crushing_output() -> Box<dyn Brewable>;
}
