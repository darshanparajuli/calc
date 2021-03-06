# calc

A simple command line calculator written in Rust.

[![Build Status](https://travis-ci.org/darshanparajuli/calc.svg?branch=master)](https://travis-ci.org/darshanparajuli/calc)

## Install

#### From source
[Click here](https://www.rustup.rs) to install rust if you don't have it already.
```
1. git clone https://github.com/darshanparajuli/calc
2. cd calc
3. cargo install
```

#### Arch Linux
Install `calc-git` package from AUR.

## Example
```
$ calc
λ 1+2
=> 3

λ a = 23
=> a = 23

λ a*2
=> 46

λ b = sin(a) * 20^2
=> b = -338.4881616700683

λ b
=> -338.4881616700683

λ ans
=> -338.4881616700683

λ ans / 100
=> -3.384881616700683

λ log10(10)
=> 1

λ log
=> log(n, base)

λ log2(8)
=> 3

λ log10(100)
=> 2

λ ln(e)
=> 1

λ sin(pi/2)
=> 1

λ exit
```
