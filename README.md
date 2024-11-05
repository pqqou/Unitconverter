
# Metric Unit Converter

This is a command-line-based Metric Unit Converter in Rust that enables conversions between various metric units (millimeters, centimeters, decimeters, meters, kilometers) and certain imperial units (inches and feet). Users can convert distances in both directions between different units by providing a specific command.


## Features

- **Metric to Metric Conversions**: Millimeters, Centimeters, Decimeters, Meters, and Kilometers.
- **Metric to Imperial Conversions**: Convert between metric units and inches or feet.
- **Imperial to Metric Conversions**: Supports conversions from inches and feet to metric units.



## Usage

1. **Compile** the code by running:
   ```bash
   rustc main.rs
   ```

2. **Run** the compiled program:
   ```bash
   ./main
   ```

3. **Enter a command** based on the conversion you want. Commands should be in the format:
   ```
   "<unit1> into <unit2>"
   ```

   Examples:
   - Convert meters to centimeters:
     ```plaintext
     m into cm
     ```
   - Convert kilometers to feet:
     ```plaintext
     km into feet
     ```
   - Convert inches to meters:
     ```plaintext
     inch into m
     ```



## Available Commands

| Command          | Description                               |
|------------------|-------------------------------------------|
| `m into cm`      | Convert meters to centimeters             |
| `cm into m`      | Convert centimeters to meters             |
| `m into dm`      | Convert meters to decimeters              |
| `dm into m`      | Convert decimeters to meters              |
| `m into km`      | Convert meters to kilometers              |
| `km into m`      | Convert kilometers to meters              |
| `m into mm`      | Convert meters to millimeters             |
| `mm into m`      | Convert millimeters to meters             |
| `cm into mm`     | Convert centimeters to millimeters        |
| `mm into cm`     | Convert millimeters to centimeters        |
| `dm into mm`     | Convert decimeters to millimeters         |
| `mm into dm`     | Convert millimeters to decimeters         |
| `km into mm`     | Convert kilometers to millimeters         |
| `mm into km`     | Convert millimeters to kilometers         |
| `cm into dm`     | Convert centimeters to decimeters         |
| `dm into cm`     | Convert decimeters to centimeters         |
| `cm into km`     | Convert centimeters to kilometers         |
| `km into cm`     | Convert kilometers to centimeters         |
| `dm into km`     | Convert decimeters to kilometers          |
| `km into dm`     | Convert kilometers to decimeters          |
| `mm into inch`   | Convert millimeters to inches             |
| `inch into mm`   | Convert inches to millimeters             |
| `cm into inch`   | Convert centimeters to inches             |
| `inch into cm`   | Convert inches to centimeters             |
| `dm into inch`   | Convert decimeters to inches              |
| `inch into dm`   | Convert inches to decimeters              |
| `m into inch`    | Convert meters to inches                  |
| `inch into m`    | Convert inches to meters                  |
| `km into inch`   | Convert kilometers to inches              |
| `inch into km`   | Convert inches to kilometers              |
| `mm into feet`   | Convert millimeters to feet               |
| `feet into mm`   | Convert feet to millimeters               |
| `cm into feet`   | Convert centimeters to feet               |
| `feet into cm`   | Convert feet to centimeters               |
| `dm into feet`   | Convert decimeters to feet                |
| `feet into dm`   | Convert feet to decimeters                |
| `m into feet`    | Convert meters to feet                    |
| `feet into m`    | Convert feet to meters                    |
| `km into feet`   | Convert kilometers to feet                |
| `feet into km`   | Convert feet to kilometers                |
| `inch into feet` | Convert inches to feet                    |
| `feet into inch` | Convert feet to inches                    |




## Code Overview

The program is structured into separate functions for each type of conversion. Each function accepts a command (e.g., `m into cm`) and prompts the user for input in the originating unit. It then performs the conversion and displays the result.



### Example Functions

- **`m_cm`**: Converts meters to centimeters or centimeters to meters.
- **`m_dm`**: Converts meters to decimeters or decimeters to meters.
- **`mm_inch`**: Converts millimeters to inches or inches to millimeters.
- **`km_foot`**: Converts kilometers to feet or feet to kilometers.



## Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue or create a pull request.



## License

This project is licensed under the MIT License.

---

## Author

**pqqou**  
Version: 1.0  
Date: 01.11.2024

---
