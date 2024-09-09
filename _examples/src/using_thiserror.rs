use thiserror::Error;

pub struct CoffeeMachine {
    water_tank_volume: f64,
    available_coffee_beans: f64,
}

#[derive(PartialEq, Debug)]
pub struct Espresso {}

#[derive(PartialEq, Debug, Error)]
pub enum MakeEspressoError {
    #[error("Not enough water in tank")]
    NotEnoughWater,
    #[error("Not enough coffee beans")]
    NotEnoughBeans,
}

impl CoffeeMachine {
    pub fn make_espresso(&self) -> Result<Espresso, MakeEspressoError> {
        if self.water_tank_volume < 25.0 {
            Err(MakeEspressoError::NotEnoughWater)
        } else if self.available_coffee_beans < 7.0 {
            Err(MakeEspressoError::NotEnoughBeans)
        } else {
            Ok(Espresso {})
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Toast {}

#[derive(PartialEq, Debug)]
pub struct Breakfast {
    pub espresso: Espresso,
    pub toast: Toast,
}

#[derive(PartialEq, Debug, Error)]
pub enum MakeBreakfastError {
    #[error("Unable to make espresso, {0}")]
    UnableToMakeEspresso(#[from] MakeEspressoError),
    #[error("Unable to make toast")]
    UnableToMakeToast,
}

pub fn make_breakfast(coffee_machine: CoffeeMachine) -> Result<Breakfast, MakeBreakfastError> {
    Ok(Breakfast {
        espresso: coffee_machine.make_espresso()?,
        toast: Toast {},
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_returned_when_making_espresso_without_beans() {
        let machine = CoffeeMachine {
            water_tank_volume: 300.0,
            available_coffee_beans: 2.0,
        };

        let result = machine.make_espresso();
        assert!(result.is_err());
        assert_eq!(result, Err(MakeEspressoError::NotEnoughBeans));
    }

    #[test]
    fn error_returned_when_making_espresso_without_water() {
        let machine = CoffeeMachine {
            water_tank_volume: 0.0,
            available_coffee_beans: 7.0,
        };

        let result = machine.make_espresso();
        assert!(result.is_err());
        assert_eq!(result, Err(MakeEspressoError::NotEnoughWater));
    }

    #[test]
    fn espresso_is_made_with_water_and_beans() {
        let machine = CoffeeMachine {
            water_tank_volume: 300.0,
            available_coffee_beans: 7.0,
        };

        let result = machine.make_espresso();
        assert!(result.is_ok());
    }

    #[test]
    fn error_returned_when_making_breakfast_without_beans() {
        let coffee_machine = CoffeeMachine {
            water_tank_volume: 300.0,
            available_coffee_beans: 2.0,
        };

        let result = make_breakfast(coffee_machine);
        assert!(result.is_err());
        assert_eq!(
            result,
            Err(MakeBreakfastError::UnableToMakeEspresso(
                MakeEspressoError::NotEnoughBeans
            ))
        );
    }
}
