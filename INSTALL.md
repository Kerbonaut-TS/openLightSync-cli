# Installing and running

Build the CLI as described in [BUILD.md](BUILD.md), then run the binary:

```bash
src-tauri/target/release/ols-cli --brightness 10 --speed 1 --effect fire -power_on
```

`--brightness` accepts a value from 0 to 255, `--speed` accepts 1 to 10, and
`--effect` accepts `rainbow`, `pulse`, `chase`, `chase_bounce`, `breathe`,
`fire`, `wave`, `sparkle`, `heartbeat`, or `static`. `-power_on` connects to
the device and turns it on before starting the effect.

## USB permissions (udev)

USB HID devices require a udev rule to allow non-root access:

```bash
sudo tee /etc/udev/rules.d/99-synclights.rules << 'EOF'
# SyncLight Bar (HID)
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1a86", ATTRS{idProduct}=="fe07", MODE="0666"
# SyncLight Bar (CDC)
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1a86", ATTRS{idProduct}=="fe0c", MODE="0666"
EOF

sudo udevadm control --reload-rules
sudo udevadm trigger
```

Unplug and replug the device after applying the rule.
