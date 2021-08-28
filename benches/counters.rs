#![feature(test)]
extern crate test;
extern crate loec;

use test::Bencher;

use loec::*;

#[bench]
fn test_count_c(b: &mut Bencher) {
    b.iter(|| count("tests/data/plasma.c"))
}

#[bench]
fn test_count_lua(b: &mut Bencher) {
    b.iter(|| count("tests/data/lua-big.lua"))
}
