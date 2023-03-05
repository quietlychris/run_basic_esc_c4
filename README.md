## BasicESC

This has been tested to work on Odroid-C4 using the stock Ubuntu Odroid image
May need to add 
```
overlays="spi0 i2c0 i2c1 uart0 pwm_ab pwm_cd pwm_ef"
```

to the /media/boot/config.ini file and restart before the PWM pins are available

| Pin #   | pwmchip  | channel   |
|---------|----------|-----------|
| Pin #35 | pwmchip0 | channel 1 |
| Pin #33 | pwmchip0 | channel 0 |
| Pin #11 | pwmchip4 | channel 1 |
| Pin #7  | pwmchip4 | channel 0 |
| Pin #15 | pwmchip8 | channel 1 |
| Pin #12 | pwmchip8 | channel 0 |

```sh
$ cross build --release --target aarch64-unknown-linux-gnu
$ scp target/aarch64-unknown-linux-gnu/release/run_esc user@sbc-ip:~
```