# Debug with GDB

## Prerequisites

While `esp-idf` is automatically installed by cargo inside `.embuild` directory,
you need `openocd` and `riscv32-elf-gdb`.

On Arch Linux:

```shell
yay -S openocd-esp32 riscv32-elf-gdb
```

Install a full esp-idf (for some reasons, the one in `.embuild` directory does
not provide openocd).

Make sure local gdbinit is enabled in `~/.gdbinit`:

```text
set auto-load local-gdbinit on
add-auto-load-safe-path /
```

## Launch openocd

Launch `openocd` with sudo (TODO how to fix permissions?)

```shell
sudo openocd-esp32openocd -f board/esp32c3-builtin.cfg
```

## CLion

Configure CLion to use remote gdb

 - Edit configurations
 - Add "Remote debug"
 - Select the debugger: Custom GDB Executable -> /usr/bin/riscv32-elf-gdb
 - Target remote args: `:3333`
 - Select symbol file `/path/to/project/target/riscv32imc-esp-espidf/debug/elf-file` (change path to project and elf file name)

To check if the debugger is working, just put a breakpoint at the beginning of 
main and start the Debug configuration.