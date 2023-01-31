# Uncomment next line if you're not using CLion Remote Debug
# target extended-remote :3333

set remote hardware-watchpoint-limit 2
mon reset halt
maintenance flush register-cache