# Debug with GDB

WORK IN PROGRESS

Install a full esp-idf (for some reasons, the one in `.embuild` directory does
not provide openocd).

Launch `openocd` with sudo (TODO how to fix permissions?)

```shell
sudo ~/.espressif/tools/openocd-esp32/v0.11.0-esp32-20221026/openocd-esp32/bin/openocd -f board/esp32c3-builtin.cfg
```

Edit `~/.gdbinit` to enable local gdbinit

```text
set auto-load local-gdbinit on
add-auto-load-safe-path /home/alepez/workspace/personal/
```

Configure CLion to use remote gdb

 - Edit configurations
 - Add "Remote debug"
 - Select the debugger `/home/alepez/.espressif/tools/riscv32-esp-elf-gdb/11.2_20220823/riscv32-esp-elf-gdb/bin/riscv32-esp-elf-gdb`
 - Target remote args: `:3333`
 - Select symbol file `/path/to/project/target/riscv32imc-esp-espidf/debug/elf-file` (change path to project and elf file name)
