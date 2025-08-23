# Development

How to use this library with Rust and cargo.

## Configure

```console
$ cargo add colorverse
```

## Use

### Simulate

```rust
use colorverse::convert;
use colorverse::core::color_vision::color_vision_type::ColorVisionType;

let input_file_path = "tests/fixtures/input.png";
let output_file_path = format!("output-{}-{}.png", color_vision, simulation_level * 100.0);

let color_vision_type = ColorVisionType::Protanomaly;
let simulation_level = 0.5;

convert(input_file_path, &color_vision, simulation_level)
    .unwrap()
    .save_as(output_file_path.as_str());
```

```rust
use colorverse::convert;
use colorverse::core::color_vision::color_vision_type::ColorVisionType;

let mut color_vision_type_iterator = ColorVisionTypeIterator::new(&ColorVisionType::Trichromacy);
while let Some(color_vision_type) = color_vision_type_iterator.next() {
    for simulation_level in [0.5, 1.0] {
        match convert("tests/fixtures/input.png", &color_vision_type, simulation_level) {
            Ok(x) => {
                let output_file_path = format!("output-{}-{}.png", &color_vision_type, simulation_level * 100.0);
                x.save_as(output_file_path.as_str());
            }
            Err(err) => eprintln!("{}", err),
        }
    }
}
```

### Daltonize

```rust
use colorverse::daltonize;
use colorverse::core::color_vision::color_vision_type::ColorVisionType;

let input_file_path = "tests/fixtures/input.png";
let output_file_path = format!("output-{}-{}.png", color_vision, simulation_level * 100.0);

let color_vision_type = ColorVisionType::Protanomaly;
let simulation_level = 0.5;
let daltonization_strength = 1.0;
let preserve_luminance = true;

daltonize(input_file_path, &color_vision, simulation_level, daltonization_strength, preserve_luminance)
    .unwrap()
    .save_as(output_file_path.as_str());
```
