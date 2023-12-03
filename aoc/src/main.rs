use trebuchet;
use cube_conundrum;
use cube_conundrum::CubeResult;

fn main() {
    let trebuchet_result: u32 = trebuchet::go();
    println!("Day 1, Part 1; Trebuchet Result: {:?}", trebuchet_result);

    let cube_result: CubeResult = cube_conundrum::go();
    println!("Day 1, Part 2; Cube Conundrum Result: {:?}", cube_result);
}
