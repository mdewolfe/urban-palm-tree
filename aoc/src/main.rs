use cube_conundrum;
use cube_conundrum::CubeResult;
use trebuchet;
use trebuchet::Trebuchet;

fn main() {
    let trebuchet_result: Trebuchet = trebuchet::go();
    println!("Day 1, Part 1; Trebuchet Result: {:?}", trebuchet_result);

    let cube_result: CubeResult = cube_conundrum::go();
    println!("Day 2, Cube Conundrum Result: {:?}", cube_result);
}
