fn main() {
    use ding_machine::{ding, on_off, print_array, print_difference, print_distance};

    let coords: (f32, f32) = (6.3, 15.0);
    print_difference(coords.0, coords.1);

    let coords_arr = [coords.0, coords.1];
    print_array(coords_arr);        // and pass it in here (this line doesn't need to change)

    let series = [1, 1, 2, 3, 5, 8, 13];
    ding(series[6]);


    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    // 4. Pass the `on_off` function the value `true` from the variable `mess`.  Done correctly,
    // `cargo run` will produce the additional output "Lights are on!" I'll get you started:
    let (on, _) = mess.2[1];
    on_off(on);

    // Challenge: Uncomment the line below, run the code, and examine the
    // output. Then go refactor the print_distance() function according to the
    // instructions in the comments inside that function.

    print_distance(coords);
}
