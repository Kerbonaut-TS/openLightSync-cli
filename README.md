# openLightsSync CLI

A native Linux command-line controller for SyncLight / iCUE-compatible USB LED
bars. It uses the device's HID protocol and provides built-in lighting effects.

## Supported devices

| Vendor ID | Product ID | Description |
|-----------|------------|-------------|
| `0x1a86`  | `0xfe07`   | SyncLight Bar (HID) |
| `0x1a86`  | `0xfe0c`   | SyncLight Bar (CDC variant) |

## Build and run

See [BUILD.md](BUILD.md) for build dependencies and instructions, and
[INSTALL.md](INSTALL.md) for USB permissions.

```bash
ols-cli --brightness 10 --speed 1 --effect fire -power_on
```

Use `ols-cli --help` to display the command usage. Available effects are
`rainbow`, `pulse`, `chase`, `chase_bounce`, `breathe`, `fire`, `wave`,
`sparkle`, `heartbeat`, and `static`.

## Options

- `--effect <name>` selects an effect (required).
- `--brightness <0-255>` sets brightness.
- `--speed <1-10>` sets effect speed (default: `5`).
- `-power_on` connects to the device and turns the LEDs on before starting the
  effect.
- `-h` or `--help` prints usage.

The CLI must connect to the device before an effect can play. Color and
brightness preferences are persisted in `~/.config/synclights/state.json`.

## HID protocol

- **RB frames** — 64-byte packets with checksum for color, brightness, and power.
- **Per-LED segments** — Action `0x86` with 1-based LED indices.
- **Keepalive** — Action `0x97` prevents firmware idle timeout.

## License

[CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/) — See [LICENSE](LICENSE).
