# Colorverse

Color vision diversity (色覚多様性) convertor - not only simulate but also daltonize (色の補正)

[![crates.io](https://img.shields.io/crates/v/colorverse?label=latest)](https://crates.io/crates/colorverse)
[![Documentation](https://docs.rs/colorverse/badge.svg?version=latest)](https://docs.rs/colorverse)
[![Dependency Status](https://deps.rs/crate/colorverse/latest/status.svg)](https://deps.rs/crate/colorverse)
[![Executable](https://github.com/accessible-colors/colorverse/actions/workflows/release-executable.yaml/badge.svg)](https://github.com/accessible-colors/colorverse/actions/workflows/release-executable.yaml)
[![License](https://img.shields.io/github/license/accessible-colors/colorverse)](https://github.com/accessible-colors/colorverse/blob/main/LICENSE)

Based on [Machado, Oliveira & Fernandes (2009)](https://www.inf.ufrgs.br/~oliveira/pubs_files/CVD_Simulation/CVD_Simulation.html) model.
Customized with dynamic simulation level parameter introduced and nonlinear easing of the level.

## Usage

### Executable

Executables on multiple platforms are found in [Releases](https://github.com/accessible-colors/colorverse/releases/latest). Just run it without any installation.

```console
$ # usage
$ ./colorverse simulate -c ${color vision} -l ${simulation level} -o ${output file path} ${input file path}
$ ./colorverse daltonize -c ${color vision} -l ${simulation level} -s {daltonization strength} -o ${output file path} ${input file path}

$ # for example
$ ./colorverse simulate -c protanomaly -l 0.75 -o simulate-out.png in.png
$ ./colorverse daltonize -c deuteranomaly -l 1.0 -s 1.0 -o daltonize-out.png in.png

$ # help
$ ./colorverse --help
$ ./colorverse simulate --help
$ ./colorverse daltonize --help
```

### Rust and `cargo`

See [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md).

---

## Samples

### Trichromacy

![trichromacy](docs/assets/trichromacy.png)

### Protanomaly

#### Simulation 50%

![protanomaly](docs/assets/protanomaly-50.png)

#### Simulation 100% (Protanopia)

![protanopia](docs/assets/protanomaly-100.png)

#### Simulation 100% + Daltonization 50%

![protanomaly 100% - daltonization 50%](docs/assets/protanomaly-100-daltonize-50.png)

#### Simulation 100% + Daltonization 100%

![protanomaly 100% - daltonization 100%](docs/assets/protanomaly-100-daltonize-100.png)

### Deuteranomaly

#### Simulation 50%

![deuteranomaly](docs/assets/deuteranomaly-50.png)

### Simulation 100% (Deuteranopia)

![deuteranopia](docs/assets/deuteranomaly-100.png)

#### Simulation 100% + Daltonization 50%

![deuteranomaly 100% - daltonization 50%](docs/assets/deuteranomaly-100-daltonize-50.png)

#### Simulation 100% + Daltonization 100%

![deuteranomaly 100% - daltonization 100%](docs/assets/deuteranomaly-100-daltonize-100.png)

### Tritanomaly

#### Simulation 50%

![tritanomaly](docs/assets/tritanomaly-50.png)

### Simulation 100% (Tritanopia)

![tritanopia](docs/assets/tritanomaly-100.png)

#### Simulation 100% + Daltonization 50%

![tritanomaly 100% - daltonization 50%](docs/assets/tritanomaly-100-daltonize-50.png)

#### Simulation 100% + Daltonization 100%

![tritanomaly 100% - daltonization 100%](docs/assets/tritanomaly-100-daltonize-100.png)

### Achromatomaly

#### Simulation 50%

![achromatomaly](docs/assets/achromatomaly-50.png)

### Simulation 100% (Achromatopsia)

![achromatopsia](docs/assets/achromatomaly-100.png)


---

## Open-source, with care

This project is lovingly built and maintained by volunteers.  
We hope it helps streamline your work.  
Please understand that the project has its own direction — while we welcome feedback, it might not fit every edge case 🌱

## Acknowledgements

Depends on [image](https://github.com/image-rs/image) / [nalgebra](https://github.com/dimforge/nalgebra).
Also, on [argh](https://github.com/google/argh) on CLI I/F.
