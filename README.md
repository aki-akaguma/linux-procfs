# linux-procfs

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

The linux-procfs crate is the data snap library for the `/proc/` filesystem on the linux os.

This crate reads from `/proc` filesystem, scanne it, stores the value into the struct and returns it. This scans and holds only the required values by specifying `feature`.

## Feature

- minimum support rustc 1.58.1 (db9d1b20b 2022-01-20)

## Example

### Example 1: load average

```rust
use linux_procfs::System;
let mut sys = System::new("/");
let loadavg = sys.get_loadavg();
println!("{}, {}, {}, {}", loadavg.a1, loadavg.a5, loadavg.a15, loadavg.last_pid);
```

### Example 2: disk stats

```rust
use linux_procfs::System;
let mut sys = System::new("/");
let diskstats = sys.get_diskstats();
for disk in diskstats.disks {
    println!("{}, {}, {}", disk.name, disk.rblk, disk.wblk);
}
```

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/linux-procfs/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/linux-procfs.svg
[crate-link]: https://crates.io/crates/linux-procfs
[docs-image]: https://docs.rs/linux-procfs/badge.svg
[docs-link]: https://docs.rs/linux-procfs/
[rustc-image]: https://img.shields.io/badge/rustc-1.58+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/linux-procfs/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/linux-procfs/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/linux-procfs/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/linux-procfs/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/linux-procfs/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/linux-procfs/actions/workflows/test-windows.yml
