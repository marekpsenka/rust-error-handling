---
marp: true
footer: github.com/marekpsenka/rust-error-handling
---

# Error Handling in Rust

---

<!-- _footer: in/marek-psenka -->

![bg right:33%](me.jpg)

Marek Pšenka

- Technical Lead at Edhouse
- 7 years of experience
- Worked with C++, C# for the most of my career
- Been using Rust for the last 2 years

---

```rust
pub struct CoffeeMachine {
    water_tank_volume: f64,
    available_coffee_beans: f64,
}

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
```

---

```rust
    #[test]
    fn error_returned_when_making_espresso_without_beans() {
        let machine = CoffeeMachine { water_tank_volume: 300.0, available_coffee_beans: 2.0 };

        let result = machine.make_espresso();
        assert!(result.is_err());
        assert_eq!(result, Err("Not enough coffee beans"));
    }

    #[test]
    fn espresso_is_made_with_water_and_beans() {
        let machine = CoffeeMachine { water_tank_volume: 300.0, available_coffee_beans: 7.0 };

        let result = machine.make_espresso();
        assert!(result.is_ok());
    }
```

---

![bg](./img/basic_diagram.jpg)

---

## Philosophy

The idea of making space for error information in function return value is not new

```C
int main(void)
{
    FILE *f = fopen("non_existent", "r");
    if (f == NULL) {
        perror("fopen() failed");
    } else {
        fclose(f);
    }
}
```

```text
fopen() failed: No such file or directory
```

---

## Rust makes it really easy

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```rust
fn open_nonexistent_file() {
    match std::fs::File::open("non_existent") {
        Ok(file) => drop(file),
        Err(err) => println!("open() failed: {}", err),
    }
}
```

```text
open() failed: The system cannot find the file specified. (os error 2)
```

---

## Side-by-side

```C
int main(void)
{
    FILE *f = fopen("non_existent", "r");
    if (f == NULL) {
        perror("fopen() failed");
    } else {
        fclose(f);
    }
}
```

```rust
fn open_nonexistent_file() {
    match std::fs::File::open("non_existent") {
        Ok(file) => drop(file),
        Err(err) => println!("open() failed: {}", err),
    }
}
```