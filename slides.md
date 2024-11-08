---
marp: true
footer: github.com/marekpsenka/rust-error-handling
---

<style>
img[alt~="logo"] {
  position: absolute;
  top: 30px;
  right: 30px;
  width: 180px
}
</style>

![logo](img/edhouse_logo.jpg)

# Práce s chybami v Rustu

---

<!-- _footer: in/marek-psenka -->

![bg right:33%](./img/me.jpg)

## Marek Pšenka

- Technický vedoucí v Edhouse
- 7 let zkušeností
- Většinu kariéry jsem pracoval s C++ a C#
- Rust používám již dva roky

---

<!-- paginate: true -->

![logo](img/edhouse_logo.jpg)

```rust
pub struct CoffeeMachine {
    water_tank_volume: f64,
    available_coffee_beans: f64,
}

impl CoffeeMachine {
    pub fn make_espresso(&self) -> Result<Espresso, String> {
        if self.water_tank_volume < 25.0 {
            Err("Not enough water in tank".to_string())
        } else if self.available_coffee_beans < 7.0 {
            Err("Not enough coffee beans".to_string())
        } else {
            Ok(Espresso {})
        }
    }
}
```

---

![logo](img/edhouse_logo.jpg)

```rust
    #[test]
    fn error_returned_when_making_espresso_without_beans() {
        let machine = CoffeeMachine {
            water_tank_volume: 300.0,
            available_coffee_beans: 2.0,
        };

        let result = machine.make_espresso();
        assert!(result.is_err());
        assert_eq!(result, Err("Not enough coffee beans".to_string()));
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
```

---

![bg](./img/basic_diagram.jpg)

---

![logo](img/edhouse_logo.jpg)

## Filozofie

Myšlenka vyhradit prostor pro chybové informace v návratové hodnotě není nová

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

![logo](img/edhouse_logo.jpg)

## Rust nám to usnadňuje

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

![logo](img/edhouse_logo.jpg)

## Porovnej

```C
int main(void) {
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

---

## Jiná strategie - výjimky v C\#

![logo](img/edhouse_logo.jpg)

```C#
public static System.IO.FileStream Open (string path, System.IO.FileMode mode);
```

Kde se dozvím jak vypadá chyba? __V dokumentaci__:
> ArgumentNullException
> PathTooLongException
> (...)

Rust je explicitní. Dozvím se to __v kódu__:

```rust
pub fn open<P: AsRef<Path>>(path: P) -> std::Result<T, std::io::Error>;
```

---

![logo](img/edhouse_logo.jpg)

## Shrnutí

- Myšlenka vyhradit prostor pro chybové informace v návratové hodnotě není nová
- Rust nám to usnadňuje standardním typem `Result<T, E>`

---

<!-- _footer: "" -->

![bg 50%](./img/ferris.svg)
