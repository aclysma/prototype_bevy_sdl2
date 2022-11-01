# prototype_bevy_sdl2

This is a prototype [SDL2](https://www.libsdl.org/) integration for Bevy. It requires two upstream changes:
 * Support for raw-window-handle in Bevy (https://github.com/aclysma/bevy/tree/use-raw-window-handle)
 * A patch for macOS in gfx-hal (https://github.com/gfx-rs/gfx/issues/3329)

## Status

Tested on macOS. Only the events generated by the winit backend are supported.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Exceptions:
* Bevy and the example code derived from Bevy is under the MIT license 
* The font [FiraSans](https://github.com/mozilla/Fira) is under the SIL OFL 1.1 license

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).
