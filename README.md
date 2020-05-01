# Rust interface to the SPI `LS7366` Quadrature encoder buffer.

The full features of the chip have been implemented as per the docsheet, and are exposed by this driver.

This driver should work for any concrete `embedded_hal::blocking::spi` implementation.

Testing was done against a [Dual LS7366R buffer chip](https://www.superdroidrobots.com/shop/item.aspx/dual-ls7366r-quadrature-encoder-buffer/1523/)
On a RPi Model 4B.

See documentation for full driver details.

## Building the [quickstart](./examples/quickstart.rs):

The quickstart is desinged against `rppal`, and is intended to be run on a RPi.
That said, it should be trivial enough to swap out the `rppal` elements for any other implementation.4B

```bash
cargo build --target=armv7-unknown-linux-gnueabihf --example quickstart
```