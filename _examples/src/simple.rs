pub struct CoffeeMachine {
    water_tank_volume: f64,
    available_coffee_beans: f64,
}

#[derive(PartialEq, Debug)]
pub struct CupOfCoffee {}

impl CoffeeMachine {
    pub fn make_espresso(&self) -> Result<CupOfCoffee, &'static str> {
        if self.water_tank_volume < 25.0 {
            Err("Not enough water in tank")
        } else if self.available_coffee_beans < 7.0 {
            Err("Not enough coffee beans")
        } else {
            Ok(CupOfCoffee {})
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_returned_when_making_espresso_without_beans() {
        let machine = CoffeeMachine { water_tank_volume: 300.0, available_coffee_beans: 2.0 };

        let result = machine.make_espresso();
        assert!(result.is_err());
        assert_eq!(result, Err("Not enough coffee beans"));
    }

    #[test]
    fn error_returned_when_making_espresso_without_water() {
        let machine = CoffeeMachine { water_tank_volume: 0.0, available_coffee_beans: 7.0 };

        let result = machine.make_espresso();
        assert!(result.is_err());
        assert_eq!(result, Err("Not enough water in tank"));
    }

    #[test]
    fn espresso_is_made_with_water_and_beans() {
        let machine = CoffeeMachine { water_tank_volume: 300.0, available_coffee_beans: 7.0 };

        let result = machine.make_espresso();
        assert!(result.is_ok());
    }
}