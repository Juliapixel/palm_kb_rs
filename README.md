# Palm® Portable Keyboard USB conversion

## Setup

Sources for pinouts and extra info can be found in the [Sources](#sources) segment.

### Building

Requires:
- [probe-rs](https://github.com/probe-rs/probe-rs) (or any other tool that
can program an STM32 MCU)
- cargo (best if installed through rustup)
- an ST-Link USB probe
- an STM32F4 dev board (I used the Black Pill V3, it costs like 8 bucks)
- a Palm® Portable Keyboard (duh)

Connect the ST-Link to your STM32's SWD port **without connecting the 3.3V pin**
and a USB cable to your dev board's USB port and run
`cargo run --release --features=defmt`

### Pin setup

B8 -> VCC pin

GND -> GND pin

B4 -> DCD pin

B3 -> RTS pin

A3 -> RXD pin

The signal from the keyboard's RXD pin requires a 100k pull-up resistor and MUST
then be passed through a NOT gate, since the STM32F411CEUx I used doesn't support
the inverted USART signal that is given out by the keyboard

### Connector

TO-DO :P

## Alternate functions

Some keys have alternate functions that are triggered by pressing them along with
`Fn`

- TO-DO

## Sources

- Info on pinouts, protocols and the matrix layout used by the keyboard
were referenced from [here](https://www.splorp.com/pdf/stowawayhwref.pdf) <sup>1</sup>

- JSON-formatted HID codes acquired from [this gist](https://gist.github.com/mildsunrise/4e231346e2078f440969cdefb6d4caa3/) <sup>2</sup>

<sup>
1. Some of the info on there isn't *quite* correct; although it states that the
signal on the RXD line is at "RS-232 level", in the real world, it turns out that
it's only regular ~3.3V inverted USART
</sup>

<sup>
2. "Keyboard International4" was accidentally labeled as "Keyboard International5" and was fixed in this repo
</sup>
