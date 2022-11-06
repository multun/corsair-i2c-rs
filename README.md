# Description

This is an unofficial **Linux** userland driver for Corsair single color LED **DDR4** modules. It was tested with:
 - Corsair Vengeance LED Ram

It comes with a bunch of caveats:
 - the `i2c-dev` and `ee1004` kernel modules have to be loaded
 - it only works with a subset of corsair's DDR4 modules, as far as I know
 - it does not support the dynamic lighting modes
 - it sets the same configuration for all modules

# Usage

1) Make sure your memory modules are DDR4 and from Corsair
2) Ensure `i2c-dev` and `ee1004` are loaded (or load them with `modprobe`)
3) Find which i2c bus hosts your memory modules. You can list buses with `i2cdetect -l`. Replace FIXME by the bus number.
4) Run the driver as root, with your command. It will run the same command accross all compatible modules.

```sh
sudo corsair-i2c-rs --bus FIXME set-brightness 42
sudo corsair-i2c-rs --bus FIXME disable
```
