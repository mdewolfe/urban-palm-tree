use cube_conundrum;
use cube_conundrum::CubeResult;
use trebuchet;
use trebuchet::Trebuchet;
use gear_ratios;
use gear_ratios::GearRatio;
use scratchcards;
use scratchcards::ScratchResult;

fn main() {
    day_one();
    day_two();
    day_three();
    day_four();
}

fn day_one() {
    let trebuchet_result: Trebuchet = trebuchet::go();
    println!("Day 1, Trebuchet Result: {:?}", trebuchet_result);
}

fn day_two() {
    let cube_result: CubeResult = cube_conundrum::go();
    println!("Day 2, Cube Conundrum Result: {:?}", cube_result);
}

fn day_three() {
    let gear_ratio: GearRatio = gear_ratios::go();
    println!("Day 3, Gear Ratios: {:?}", gear_ratio);
}

fn day_four() {
    let scratch_result: ScratchResult = scratchcards::go();
    println!("Day 4, Scratch Cards: {:?}", scratch_result);
}
