use crate::brewable::Brewable;

pub struct Filter {
    item: Option<Box<dyn Brewable>>,
}

impl Filter {
    pub fn new<T: Brewable + 'static>(item: Option<T>) -> Filter {
        Filter {
            item: item.map(|i| Box::new(i) as Box<dyn Brewable>),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.item {
            Some(_) => false,
            None => true,
        }
    }

    pub fn add<T: Brewable + 'static>(&mut self, item: T) -> Result<(), &'static str> {
        match self.item.replace(Box::new(item)) {
            Some(_) => Err("Filter is already full."),
            None => Ok(()),
        }
    }

    // Returns the liquid type and amount of liquid that results from making this brew.
    pub fn consume_brewable(&mut self) -> (String, u32) {
        match self.item.take() {
            Some(n) => (n.liquid_type().to_string(), n.liquid_amount()),
            None => ("water".to_string(), 1200),
        }
    }
}
