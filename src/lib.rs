#[allow(dead_code)]

pub struct ShoppingCart {
    id: u32,
    owner: u32,
    cart: Vec<Item>,
}

#[derive(Hash)]
pub struct Item {
    id: u32,
    name: String,
    price: u32,
}

impl ShoppingCart {
    pub fn new(id: u32, owner: u32) -> Self {
        Self {
            id,
            owner,
            cart: Vec::new(),
        }
    }

    pub fn total_items(&self) -> usize {
        self.cart.len()
    }

    pub fn add_item(&mut self, item: Item) -> Result<usize, &'static str> {
        if item.price == 0 {
            return Err("Price cannot be zero");
        }
        self.cart.push(item);
        Ok(self.cart.len())
    }

    pub fn delete_item(&mut self, needle: u32) -> Result<usize, &'static str> {
        if self.cart.is_empty() {
            return Err("Cart is empty");
        }

        for (idx, item) in self.cart.iter().enumerate() {
            if item.id == needle {
                self.cart.remove(idx);
                return Ok(self.cart.len());
            }
        }
        Err("Item not found")
    }

    pub fn calculate_total_price(&self) -> u32 {
        let mut sum = 0;
        for item in &self.cart {
            sum += item.price;
        }
        sum
    }

    pub fn apply_discount(&self, percent: u32) -> Result<u32, &'static str> {
        if percent > 100 {
            return Err("Discount percentage greater than 100");
        }
        let total = self.calculate_total_price();
        let discount_total = total - (total * percent / 100);
        return Ok(discount_total);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_valid_item() {
        // Arrange
        let item: Item = Item {
            id: 2,
            name: "Mouse".to_string(),
            price: 550,
        };
        let mut cart = ShoppingCart::new(1, 12); // Now empty

        // Act
        let result = cart.add_item(item);

        // Assert
        assert!(result.is_ok());
        assert_eq!(1, result.unwrap());
    }

    #[test]
    fn add_invalid_item() {
        let item: Item = Item {
            id: 2,
            name: "Dune".to_string(),
            price: 0,
        };
        let mut cart = ShoppingCart::new(1, 12);

        let result = cart.add_item(item);

        assert!(result.is_err());
        assert_eq!(0, cart.total_items());
    }

    #[test]
    fn delete_valid_item() {
        let item1 = Item {
            id: 1,
            price: 40,
            name: "Item1".to_string(),
        };
        let item2 = Item {
            id: 2,
            price: 30,
            name: "Item2".to_string(),
        };
        let item3 = Item {
            id: 3,
            price: 20,
            name: "Item3".to_string(),
        };

        let mut cart = ShoppingCart::new(1, 12);
        let _ = cart.add_item(item1);
        let _ = cart.add_item(item2);
        let _ = cart.add_item(item3);

        let result = cart.delete_item(1);

        assert!(result.is_ok());
        assert_eq!(2, cart.total_items());
    }

    #[test]
    fn delete_invalid_item() {
        let item1 = Item {
            id: 1,
            price: 40,
            name: "Item1".to_string(),
        };
        let item2 = Item {
            id: 2,
            price: 30,
            name: "Item2".to_string(),
        };
        let item3 = Item {
            id: 3,
            price: 20,
            name: "Item3".to_string(),
        };

        let mut cart = ShoppingCart::new(1, 12);
        let _ = cart.add_item(item1);
        let _ = cart.add_item(item2);
        let _ = cart.add_item(item3);

        let result = cart.delete_item(4);

        assert!(result.is_err());
        assert_eq!(3, cart.total_items());
    }

    #[test]
    fn delete_empty_cart() {
        let mut cart = ShoppingCart::new(1, 12);

        let result = cart.delete_item(1);

        assert!(result.is_err());
        if let Err(err_message) = result {
            assert_eq!("Cart is empty", err_message);
        }
    }

    #[test]
    fn total_price_empty_cart() {
        let cart = ShoppingCart::new(1, 12);
        let total = cart.calculate_total_price();

        assert_eq!(0, total);
    }

    #[test]
    fn total_price() {
        let item1 = Item {
            id: 1,
            price: 40,
            name: "Item1".to_string(),
        };
        let item2 = Item {
            id: 2,
            price: 30,
            name: "Item2".to_string(),
        };
        let item3 = Item {
            id: 3,
            price: 20,
            name: "Item3".to_string(),
        };
        let mut cart = ShoppingCart::new(1, 12);
        let _ = cart.add_item(item1);
        let _ = cart.add_item(item2);
        let _ = cart.add_item(item3);

        let total_price = cart.calculate_total_price();
        assert_eq!(90, total_price);
    }

    #[test]
    fn valid_discount() {
        let item1 = Item {
            id: 1,
            price: 40,
            name: "Item1".to_string(),
        };
        let item2 = Item {
            id: 2,
            price: 30,
            name: "Item2".to_string(),
        };
        let item3 = Item {
            id: 3,
            price: 20,
            name: "Item3".to_string(),
        };
        let mut cart = ShoppingCart::new(1, 12);
        let _ = cart.add_item(item1);
        let _ = cart.add_item(item2);
        let _ = cart.add_item(item3);

        let discount1 = cart.apply_discount(10);
        let discount2 = cart.apply_discount(0);

        assert!(discount1.is_ok());
        assert_eq!(81, discount1.unwrap());
        assert!(discount2.is_ok());
        assert_eq!(90, discount2.unwrap());
    }

    #[test]
    fn invalid_discount() {
        let item1 = Item {
            id: 1,
            price: 40,
            name: "Item1".to_string(),
        };
        let item2 = Item {
            id: 2,
            price: 30,
            name: "Item2".to_string(),
        };
        let item3 = Item {
            id: 3,
            price: 20,
            name: "Item3".to_string(),
        };
        let mut cart = ShoppingCart::new(1, 12);
        let _ = cart.add_item(item1);
        let _ = cart.add_item(item2);
        let _ = cart.add_item(item3);

        let discount = cart.apply_discount(101);

        assert!(discount.is_err());
    }
}
