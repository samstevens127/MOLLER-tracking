# MOLLER Tracking

## What this does.

Takes input data with 9 columns in the format (event num, x, y, x_charge, y_charge, hadc, ladc, run_num, high_voltage).

Aligns the GEM planes to improve residuals, then output the corrected hit positions to corrected_xy.txt

## Installation.
```
  git clone https://github.com/samstevens127/MOLLER-tracking.git
  cd MOLLER-tracking
  cargo build --release
```

## Use

Edit the configuration file in `config.toml`

then run `./release/residuals`

`plotting.cc` is there if you want to check the residuals after correction.

## TODO

1. add other scripts (pedcal, sorting)
2. rewrite gradient descent for running in parallel
3. add more to the config
  1. outputfile names/location
  2. Z values
  3. ability to change max iterations of gradient descent
  4. ability to change epsilon for finite difference
