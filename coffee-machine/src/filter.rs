use crate::brewable::Brewable;

pub struct Filter {
    item: Option<Box<dyn Brewable>>,
}

impl Filter {
    pub fn new(item: Option<Box<dyn Brewable>>) -> Filter {
        Filter { item }
    }

    pub fn is_empty(&self) -> bool {
        match self.item {
            Some(_) => false,
            None => true,
        }
    }

    pub fn add(&mut self, item: Box<dyn Brewable>) -> Result<(), &'static str> {
        match self.item {
            Some(_) => Err("Filter is already full."),
            None => {
                self.item = Some(item);
                Ok(())
            }
        }
    }

    pub fn use_filter(&mut self) -> Result<&'static str, &'static str> {
        match self.item {
            Some(n) => {
                self.item = None;
                Ok(n.liquid_type())
            }
            None => Err("Cannot use filter, as it is empty."),
        }
    }
}
