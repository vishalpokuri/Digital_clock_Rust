use chrono::prelude::*;
use colored::*;
use std::{thread::sleep, time::Duration};
fn main() {
    #[allow(unused_variables)]
    let mystrings = [
        String::from("ow&9!j%-c0gL6P=3aA86r5v+SICpm47qoZUNSgfr*1$m+8ib5K/k83E7tB~O#d~NQPDXHR4)^12y9_Lr(FJ7@M2x8Y$D1v~4&7jZb2Sgfr*1$m=3aA86r5v+"),String::from(
        "kxwUE8@$!G2r+I%vwrsst4*4i&sB#k3HRa7d!$nFzj6P)YX6AzfDQ#UQ9h$*7cWL8T!o^^0KFhVEgbOmd2YilW4&2KzN@7b5#9q$6L#k3HRa7d!$nUE8@$!G"),
        String::from(
        "!%L!38^r62Hj_dK*D2oJ^$ak56eR+9mX9FL01w57Pv&i!zgT#5UHYu#*u3@pdBqi44+s6X7SS$FitQ/7AtF2vQ1^8kM&0J5w7L9Xv;dK*D2oJ^$ak56L!38^"),
        String::from(
        "~0Pl+@Q!9Wr@7Tu$Yj%8Xo_iJ2L#F^3JdS6&5Am6v7N!o4U7i$d+S9*3p2@8X1tZ&vY0K5F3hT4b9L^6H8e2JX1@v2Y^0L8k7#9T6$9Wr@7TuN!o4Pl+@Qv7"),
        String::from(
        "$g@K!5jP3i^N6rQ&L7d4mT9oB7k+2sU%0vX5F8!6H*8L4i2e5J@3Y^5x4wQ!8L0z3j9NP3i^N6r$g@K!5jP3i7#q9$Fv_U4pXk^t7T2d4J0h+i8S1rB2H7@*"),
        String::from(
        "Qz&y!W5U2J1k%pF3v9N^L7m4oX8B@6jT$d5i6S+7tK8w4@9Fh~0L2Y3S7i^5J@dX5*9H4P6!2o7kvT8r4pB2U8&@^5n3L7z1j9Q6wY8e7i^5J@dU2J1k%pF3"),
        String::from(
        "rX6*5L^8d3m4F7@H+v0!T2Q$9iJ6S&7tY9p1k8J2~o5i4S6@3t+d5F7$8U!9k6^B3L4X0H2!p7wT2rX8q0ruFM2^4&0j7v9N6K8$1L~o5i4S+v0!T2Q$*5L^"),
        String::from(
        "F6o$4U3!9tX5S@7i2k8+vL&d7mJ0^Q2p8!6Jr5~T7L4i9B6k^w0q3d7@X9U5T2r5oX+2pH4!9i6r6*7L3J2@vX4&^8n0J7L5k1T9Y20q3d7@d7mJ0^Q5S@hG"),
        String::from(
        "9w!6J7t7o4i^Q5d2@F7S+v06L8r8m3k&29i5pX@9L4JS8!3t6d5*7F4H0X^B7q0rL8T2w3o6J+9i!5p4d6r7UX1^&0v8jL4K3n9T5Y2@F7S+v0T6LHYu#*^$"),
        String::from(
        "L6*3X9F!7T0o4d2^@7i7S5J8U2+T9pL4J!3m8r0k7&5X6Q@2r9t4B8!6w5^3i2d7p0o6S7F+7L2H42v5i3U8X9^&1n0L7Kj2T5Y8m2r+I%v4S6@3t7i9B6k^")
    ];

    fn clear_screen() {
        println!("\x1B[2J\x1B[H");
        //Ansi code to clear the terminal and move cursor to the top left
    }
    fn what_to_print(
        front_space: &mut Vec<String>,
        hour_tens: &mut Vec<String>,
        hour_ones: &mut Vec<String>,
        space1: &mut Vec<String>,
        bigspace1: &mut Vec<String>,
        dot1: &mut Vec<String>,
        bigspace2: &mut Vec<String>,
        minute_tens: &mut Vec<String>,
        minute_ones: &mut Vec<String>,
        space2: &mut Vec<String>,
        bigspace3: &mut Vec<String>,
        dot2: &mut Vec<String>,
        bigspace4: &mut Vec<String>,
        second_tens: &mut Vec<String>,
        second_ones: &mut Vec<String>,
        space3: &mut Vec<String>,
        backspace: &mut Vec<String>,
    ) {
        //colorise the stuff, this will go to the infinite loop, hence this will colorise the variables down
        //1. take the values
        #[allow(unused_variables)]
        #[allow(unused_imports)]
        let now = Local::now().time();
        let hour = format!("{:02}", now.hour());
        let minute = format!("{:02}", now.minute());
        let second = format!("{:02}", now.second());

        let what_hour_tens = hour.chars().nth(0).unwrap();
        let what_hour_ones = hour.chars().nth(1).unwrap();
        let what_minute_tens = minute.chars().nth(0).unwrap();
        let what_minute_ones = minute.chars().nth(1).unwrap();
        let what_second_tens = second.chars().nth(0).unwrap();
        let what_second_ones = second.chars().nth(1).unwrap();

        //1.1

        //Colorify functions

        let colorify_1 = |array: &mut Vec<String>| -> Vec<String> {
            let mut my_vector = Vec::new();

            for (i, item) in array.iter().enumerate() {
                let stringy = match i + 1 {
                    2 => format!(
                        "{}{}{}",
                        &item[0..4].truecolor(20, 20, 20),
                        &item[4..8].bright_yellow(),
                        &item[8..12].truecolor(20, 20, 20),
                    ),
                    3 => format!(
                        "{}{}{}",
                        &item[0..2].truecolor(20, 20, 20),
                        &item[2..8].bright_yellow(),
                        &item[8..12].truecolor(20, 20, 20),
                    ),
                    4..=8 => format!(
                        "{}{}{}",
                        &item[0..5].truecolor(20, 20, 20),
                        &item[5..8].bright_yellow(),
                        &item[8..12].truecolor(20, 20, 20),
                    ),
                    9 => format!("{}", &item.bright_yellow()),
                    _ => format!("{}", &item.truecolor(20, 20, 20)),
                };
                my_vector.push(stringy);
            }

            return my_vector;
        };

        let colorify_2 = |array: &mut Vec<String>| -> Vec<String> {
            let mut my_vector = Vec::new();

            for (i, item) in array.iter().enumerate() {
                let stringy = match i + 1 {
                    2 => format!(
                        "{}{}{}",
                        &item[0..3].truecolor(20, 20, 20),
                        &item[3..9].bright_yellow(),
                        &item[9..12].truecolor(20, 20, 20)
                    ),
                    3 => format!(
                        "{}{}{}{}{}",
                        &item[0..1].truecolor(20, 20, 20),
                        &item[1..4].bright_yellow(),
                        &item[4..8].truecolor(20, 20, 20),
                        &item[8..11].bright_yellow(),
                        &item[11..12].truecolor(20, 20, 20)
                    ),
                    4 => format!(
                        "{}{}{}",
                        &item[0..3].bright_yellow(),
                        &item[3..9].truecolor(20, 20, 20),
                        &item[9..12].bright_yellow()
                    ),
                    5 => format!(
                        "{}{}",
                        &item[0..9].truecolor(20, 20, 20),
                        &item[9..12].bright_yellow(),
                    ),
                    6 => format!(
                        "{}{}{}",
                        &item[0..7].truecolor(20, 20, 20),
                        &item[7..11].bright_yellow(),
                        &item[11..12].truecolor(20, 20, 20),
                    ),
                    7 => format!(
                        "{}{}{}",
                        &item[0..5].truecolor(20, 20, 20),
                        &item[5..9].bright_yellow(),
                        &item[9..12].truecolor(20, 20, 20),
                    ),
                    8 => format!(
                        "{}{}{}",
                        &item[0..2].truecolor(20, 20, 20),
                        &item[2..6].bright_yellow(),
                        &item[6..12].truecolor(20, 20, 20),
                    ),
                    9 => format!("{}", &item.bright_yellow()),
                    _ => format!("{}", &item.truecolor(20, 20, 20)),
                };
                my_vector.push(stringy);
            }
            return my_vector;
        };

        let colorify_0 = |array: &mut Vec<String>| -> Vec<String> {
            let mut my_vector = Vec::new();

            for (i, item) in array.iter().enumerate() {
                let colored_string = match i + 1 {
                    2 | 9 => format!(
                        "{}{}{}",
                        &item[0..3].truecolor(20, 20, 20),
                        &item[3..9].bright_yellow(),
                        &item[9..12].truecolor(20, 20, 20)
                    ),
                    3 | 8 => format!(
                        "{}{}{}{}{}",
                        &item[0..1].truecolor(20, 20, 20),
                        &item[1..4].bright_yellow(),
                        &item[4..8].truecolor(20, 20, 20),
                        &item[8..11].bright_yellow(),
                        &item[11..12].truecolor(20, 20, 20)
                    ),
                    4 => format!(
                        "{}{}{}{}{}",
                        &item[0..3].bright_yellow(),
                        &item[3..4].truecolor(20, 20, 20),
                        &item[4..5].bright_yellow(),
                        &item[5..9].truecolor(20, 20, 20),
                        &item[9..12].bright_yellow()
                    ),
                    5 => format!(
                        "{}{}{}{}{}",
                        &item[0..3].bright_yellow(),
                        &item[3..5].truecolor(20, 20, 20),
                        &item[5..6].bright_yellow(),
                        &item[6..9].truecolor(20, 20, 20),
                        &item[9..12].bright_yellow()
                    ),
                    6 => format!(
                        "{}{}{}{}{}",
                        &item[0..3].bright_yellow(),
                        &item[3..6].truecolor(20, 20, 20),
                        &item[6..7].bright_yellow(),
                        &item[7..9].truecolor(20, 20, 20),
                        &item[9..12].bright_yellow()
                    ),
                    7 => format!(
                        "{}{}{}{}{}",
                        &item[0..3].bright_yellow(),
                        &item[3..7].truecolor(20, 20, 20),
                        &item[7..8].bright_yellow(),
                        &item[8..9].truecolor(20, 20, 20),
                        &item[9..12].bright_yellow()
                    ),
                    _ => format!("{}", item.truecolor(20, 20, 20)),
                };
                my_vector.push(colored_string);
            }

            my_vector
        };

        let colorify_3 = |array: &mut Vec<String>| -> Vec<String> {
            let mut my_vector = Vec::new();

            for (i, item) in array.iter().enumerate() {
                let colored_string = match i + 1 {
                    2 => format!("{}", &item.bright_yellow()),
                    3 => format!(
                        "{}{}{}",
                        &item[0..3].bright_yellow(),
                        &item[3..9].truecolor(20, 20, 20),
                        &item[9..12].bright_yellow()
                    ),
                    4 => format!(
                        "{}{}",
                        &item[0..7].truecolor(20, 20, 20),
                        &item[7..12].bright_yellow(),
                    ),
                    5 | 9 => format!(
                        "{}{}{}",
                        &item[0..3].truecolor(20, 20, 20),
                        &item[3..9].bright_yellow(),
                        &item[9..12].truecolor(20, 20, 20)
                    ),
                    6 => format!(
                        "{}{}{}",
                        &item[0..7].truecolor(20, 20, 20),
                        &item[7..11].bright_yellow(),
                        &item[11..12].truecolor(20, 20, 20),
                    ),
                    7 => format!(
                        "{}{}{}",
                        &item[0..2].bright_yellow(),
                        &item[2..8].truecolor(20, 20, 20),
                        &item[8..12].bright_yellow()
                    ),
                    8 => format!(
                        "{}{}{}{}",
                        &item[0..4].bright_yellow(),
                        &item[4..8].truecolor(20, 20, 20),
                        &item[8..11].bright_yellow(),
                        &item[11..12].truecolor(20, 20, 20)
                    ),
                    _ => format!("{}", item.truecolor(20, 20, 20)),
                };
                my_vector.push(colored_string);
            }

            my_vector
        };

        let colorify_7 = |array: &mut Vec<String>| -> Vec<String> {
            let mut my_vector = Vec::new();

            for (i, item) in array.iter().enumerate() {
                let colored_string = match i + 1 {
                    2 => format!("{}", &item.bright_yellow()),
                    3 => format!(
                        "{}{}{}",
                        &item[0..3].bright_yellow(),
                        &item[3..9].truecolor(20, 20, 20),
                        &item[9..12].bright_yellow()
                    ),
                    4 => format!(
                        "{}{}",
                        &item[0..7].truecolor(20, 20, 20),
                        &item[7..12].bright_yellow(),
                    ),
                    5 => format!(
                        "{}{}{}",
                        &item[0..6].truecolor(20, 20, 20),
                        &item[6..9].bright_yellow(),
                        &item[9..12].truecolor(20, 20, 20)
                    ),
                    6..=8 => format!(
                        "{}{}{}",
                        &item[0..4].truecolor(20, 20, 20),
                        &item[4..7].bright_yellow(),
                        &item[7..12].truecolor(20, 20, 20),
                    ),
                    9 => format!(
                        "{}{}{}",
                        &item[0..3].truecolor(20, 20, 20),
                        &item[3..8].bright_yellow(),
                        &item[8..12].truecolor(20, 20, 20),
                    ),
                    _ => format!("{}", item.truecolor(20, 20, 20)),
                };
                my_vector.push(colored_string);
            }

            my_vector
        };

        let colorify_4 = |array: &mut Vec<String>| -> Vec<String> {
            let mut my_vector = Vec::new();

            for (i, item) in array.iter().enumerate() {
                let colored_string = match i + 1 {
                    2 => format!(
                        "{}{}{}",
                        &item[0..4].truecolor(20, 20, 20),
                        &item[4..10].bright_yellow(),
                        &item[10..12].truecolor(20, 20, 20)
                    ),
                    3 => format!(
                        "{}{}{}{}{}",
                        &item[0..3].truecolor(20, 20, 20),
                        &item[3..6].bright_yellow(),
                        &item[6..7].truecolor(20, 20, 20),
                        &item[7..10].bright_yellow(),
                        &item[10..12].truecolor(20, 20, 20),
                    ),
                    4 => format!(
                        "{}{}{}{}{}",
                        &item[0..2].truecolor(20, 20, 20),
                        &item[2..5].bright_yellow(),
                        &item[5..7].truecolor(20, 20, 20),
                        &item[7..10].bright_yellow(),
                        &item[10..12].truecolor(20, 20, 20),
                    ),
                    5 => format!(
                        "{}{}{}{}{}",
                        &item[0..1].truecolor(20, 20, 20),
                        &item[1..4].bright_yellow(),
                        &item[4..7].truecolor(20, 20, 20),
                        &item[7..10].bright_yellow(),
                        &item[10..12].truecolor(20, 20, 20),
                    ),
                    6 => format!("{}", &item.bright_yellow()),
                    7 | 8 => format!(
                        "{}{}{}",
                        &item[0..7].truecolor(20, 20, 20),
                        &item[7..10].bright_yellow(),
                        &item[10..12].truecolor(20, 20, 20),
                    ),
                    9 => format!(
                        "{}{}",
                        &item[0..4].truecolor(20, 20, 20),
                        &item[4..12].bright_yellow(),
                    ),
                    _ => format!("{}", item.truecolor(20, 20, 20)),
                };
                my_vector.push(colored_string);
            }

            my_vector
        };
        let colorify_5 = |array: &mut Vec<String>| -> Vec<String> {
            let mut my_vector = Vec::new();

            for (i, item) in array.iter().enumerate() {
                let colored_string = match i + 1 {
                    2 => format!("{}", item.bright_yellow()),
                    3 => format!(
                        "{}{}{}",
                        &item[0..3].bright_yellow(),
                        &item[3..9].truecolor(20, 20, 20),
                        &item[9..12].bright_yellow()
                    ),
                    4 => format!(
                        "{}{}",
                        &item[0..3].bright_yellow(),
                        &item[3..12].truecolor(20, 20, 20)
                    ),
                    7 => format!(
                        "{}{}",
                        &item[0..8].truecolor(20, 20, 20),
                        &item[8..12].bright_yellow()
                    ),
                    6 => format!(
                        "{}{}{}",
                        &item[0..7].truecolor(20, 20, 20),
                        &item[7..11].bright_yellow(),
                        &item[11..12].truecolor(20, 20, 20)
                    ),
                    5 => format!(
                        "{}{}",
                        &item[0..8].bright_yellow(),
                        &item[8..12].truecolor(20, 20, 20)
                    ),
                    8 => format!(
                        "{}{}{}{}",
                        &item[0..4].bright_yellow(),
                        &item[4..7].truecolor(20, 20, 20),
                        &item[7..11].bright_yellow(),
                        &item[11..12].truecolor(20, 20, 20)
                    ),
                    9 => format!(
                        "{}{}{}",
                        &item[0..3].truecolor(20, 20, 20),
                        &item[3..9].bright_yellow(),
                        &item[9..12].truecolor(20, 20, 20)
                    ),
                    _ => format!("{}", item.truecolor(20, 20, 20)),
                };
                my_vector.push(colored_string);
            }

            my_vector
        };
        let colorify_6 = |array: &mut Vec<String>| -> Vec<String> {
            let mut my_vector = Vec::new();
            for (i, item) in array.iter().enumerate() {
                let stringy = match i + 1 {
                    2 => format!(
                        "{}{}{}",
                        &item[0..2].truecolor(20, 20, 20),
                        &item[2..11].bright_yellow(),
                        &item[11..12].truecolor(20, 20, 20)
                    ),
                    3 => format!(
                        "{}{}{}{}",
                        &item[0..1].truecolor(20, 20, 20),
                        &item[1..4].bright_yellow(),
                        &item[4..9].truecolor(20, 20, 20),
                        &item[9..12].bright_yellow(),
                    ),
                    4 => format!(
                        "{}{}",
                        &item[0..3].bright_yellow(),
                        &item[3..12].truecolor(20, 20, 20),
                    ),
                    5 => format!(
                        "{}{}",
                        &item[0..10].bright_yellow(),
                        &item[10..12].truecolor(20, 20, 20),
                    ),
                    6..=8 => format!(
                        "{}{}{}",
                        &item[0..3].bright_yellow(),
                        &item[3..9].truecolor(20, 20, 20),
                        &item[9..12].bright_yellow()
                    ),
                    9 => format!(
                        "{}{}{}",
                        &item[0..3].truecolor(20, 20, 20),
                        &item[3..9].bright_yellow(),
                        &item[9..12].truecolor(20, 20, 20)
                    ),
                    _ => format!("{}", &item.truecolor(20, 20, 20)),
                };
                my_vector.push(stringy);
            }
            my_vector
        };
        let colorify_8 = |array: &mut Vec<String>| -> Vec<String> {
            let mut my_vector = Vec::new();
            for (i, item) in array.iter().enumerate() {
                let colored_string = match i + 1 {
                    2 | 5 | 6 | 9 => format!(
                        "{}{}{}",
                        &item[0..2].truecolor(20, 20, 20),
                        &item[2..10].bright_yellow(),
                        &item[10..12].truecolor(20, 20, 20)
                    ),
                    3 | 4 | 7 | 8 => format!(
                        "{}{}{}",
                        &item[0..3].bright_yellow(),
                        &item[3..9].truecolor(20, 20, 20),
                        &item[9..12].bright_yellow()
                    ),
                    _ => format!("{}", &item.truecolor(20, 20, 20)),
                };
                my_vector.push(colored_string);
            }

            my_vector
        };
        let colorify_9 = |array: &mut Vec<String>| -> Vec<String> {
            let mut my_vector = Vec::new();

            for (i, item) in array.iter().enumerate() {
                let stringy = match i + 1 {
                    2 => format!(
                        "{}{}{}",
                        &item[0..3].truecolor(20, 20, 20),
                        &item[3..9].bright_yellow(),
                        &item[9..12].truecolor(20, 20, 20)
                    ),
                    3..=5 => format!(
                        "{}{}{}",
                        &item[0..3].bright_yellow(),
                        &item[3..9].truecolor(20, 20, 20),
                        &item[9..12].bright_yellow()
                    ),

                    6 => format!(
                        "{}{}",
                        &item[0..2].truecolor(20, 20, 20),
                        &item[2..12].bright_yellow(),
                    ),
                    7 => format!(
                        "{}{}",
                        &item[0..9].truecolor(20, 20, 20),
                        &item[9..12].bright_yellow(),
                    ),
                    8 => format!(
                        "{}{}{}{}",
                        &item[0..3].bright_yellow(),
                        &item[3..8].truecolor(20, 20, 20),
                        &item[8..11].bright_yellow(),
                        &item[11..12].truecolor(20, 20, 20),
                    ),
                    9 => format!(
                        "{}{}{}",
                        &item[0..1].truecolor(20, 20, 20),
                        &item[1..10].bright_yellow(),
                        &item[10..12].truecolor(20, 20, 20)
                    ),
                    _ => format!("{}", &item.truecolor(20, 20, 20)),
                };
                my_vector.push(stringy);
            }
            my_vector
        };
        let colorify_dot = |array: &mut Vec<String>| -> Vec<String> {
            let mut my_vector = Vec::new();
            for (i, item) in array.iter().enumerate() {
                let stringy = match i + 1 {
                    2 | 5 | 6 | 9 => format!("{}", &item[..].truecolor(20, 20, 20)),
                    3 | 4 | 7 | 8 => format!("{}", &item[..].bright_yellow()),
                    _ => format!("{}", &item.truecolor(20, 20, 20)),
                };
                my_vector.push(stringy);
            }
            my_vector
        };
        let colorify_nodot = |array: &mut Vec<String>| -> Vec<String> {
            let mut my_vector = Vec::new();

            for (i, item) in array.iter().enumerate() {
                let stringy = match i + 1 {
                    2..=9 => format!("{}", &item[..].truecolor(20, 20, 20)),
                    _ => format!("{}", &item.truecolor(20, 20, 20)),
                };
                my_vector.push(stringy);
            }
            my_vector
        };
        //2. match statemnts as per the number to use the colorify function
        match what_hour_tens {
            '0' => *hour_tens = colorify_0(hour_tens),
            '1' => *hour_tens = colorify_1(hour_tens),
            '2' => *hour_tens = colorify_2(hour_tens),
            _ => {}
        };
        match what_hour_ones {
            '0' => *hour_ones = colorify_0(hour_ones),
            '1' => *hour_ones = colorify_1(hour_ones),
            '2' => *hour_ones = colorify_2(hour_ones),
            '3' => *hour_ones = colorify_3(hour_ones),
            '4' => *hour_ones = colorify_4(hour_ones),
            '5' => *hour_ones = colorify_5(hour_ones),
            '6' => *hour_ones = colorify_6(hour_ones),
            '7' => *hour_ones = colorify_7(hour_ones),
            '8' => *hour_ones = colorify_8(hour_ones),
            '9' => *hour_ones = colorify_9(hour_ones),
            _ => {}
        };
        match what_minute_tens {
            '0' => *minute_tens = colorify_0(minute_tens),
            '1' => *minute_tens = colorify_1(minute_tens),
            '2' => *minute_tens = colorify_2(minute_tens),
            '3' => *minute_tens = colorify_3(minute_tens),
            '4' => *minute_tens = colorify_4(minute_tens),
            '5' => *minute_tens = colorify_5(minute_tens),
            _ => {}
        };
        match what_minute_ones {
            '0' => *minute_ones = colorify_0(minute_ones),
            '1' => *minute_ones = colorify_1(minute_ones),
            '2' => *minute_ones = colorify_2(minute_ones),
            '3' => *minute_ones = colorify_3(minute_ones),
            '4' => *minute_ones = colorify_4(minute_ones),
            '5' => *minute_ones = colorify_5(minute_ones),
            '6' => *minute_ones = colorify_6(minute_ones),
            '7' => *minute_ones = colorify_7(minute_ones),
            '8' => *minute_ones = colorify_8(minute_ones),
            '9' => *minute_ones = colorify_9(minute_ones),
            _ => {}
        };
        match what_second_tens {
            '0' => *second_tens = colorify_0(second_tens),
            '1' => *second_tens = colorify_1(second_tens),
            '2' => *second_tens = colorify_2(second_tens),
            '3' => *second_tens = colorify_3(second_tens),
            '4' => *second_tens = colorify_4(second_tens),
            '5' => *second_tens = colorify_5(second_tens),
            _ => {}
        };
        match what_second_ones {
            '0' => *second_ones = colorify_0(second_ones),
            '1' => *second_ones = colorify_1(second_ones),
            '2' => *second_ones = colorify_2(second_ones),
            '3' => *second_ones = colorify_3(second_ones),
            '4' => *second_ones = colorify_4(second_ones),
            '5' => *second_ones = colorify_5(second_ones),
            '6' => *second_ones = colorify_6(second_ones),
            '7' => *second_ones = colorify_7(second_ones),
            '8' => *second_ones = colorify_8(second_ones),
            '9' => *second_ones = colorify_9(second_ones),
            _ => {}
        };
        if what_second_ones.to_digit(10).unwrap() % 2 == 0 {
            *dot1 = colorify_dot(dot1);
            *dot2 = colorify_dot(dot2);
        } else {
            *dot1 = colorify_nodot(dot1);
            *dot2 = colorify_nodot(dot2);
        }
        //3. print them with for loop
        for i in 0..front_space.len() {
            println!(
                "                                      {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                front_space[i].truecolor(20, 20, 20),
                hour_tens[i],
                space1[i].truecolor(20, 20, 20),
                hour_ones[i],
                bigspace1[i].truecolor(20, 20, 20),
                dot1[i],
                bigspace2[i].truecolor(20, 20, 20),
                minute_tens[i],
                space2[i].truecolor(20, 20, 20),
                minute_ones[i],
                bigspace3[i].truecolor(20, 20, 20),
                dot2[i],
                bigspace4[i].truecolor(20, 20, 20),
                second_tens[i],
                space3[i].truecolor(20, 20, 20),
                second_ones[i],
                backspace[i].truecolor(20, 20, 20),
            );
        }
    }

    loop {
        clear_screen();
        let mut front_space: Vec<String> = mystrings.iter().map(|s| s[0..6].to_owned()).collect();
        let mut hour_tens: Vec<String> = mystrings.iter().map(|s| s[6..18].to_owned()).collect();
        let mut space1: Vec<String> = mystrings.iter().map(|s| s[18..22].to_owned()).collect();
        let mut hour_ones: Vec<String> = mystrings.iter().map(|s| s[22..34].to_owned()).collect();
        let mut bigspace1: Vec<String> = mystrings.iter().map(|s| s[34..39].to_owned()).collect();
        let mut dot1: Vec<String> = mystrings.iter().map(|s| s[39..41].to_owned()).collect();
        let mut bigspace2: Vec<String> = mystrings.iter().map(|s| s[41..46].to_owned()).collect();
        let mut minute_tens: Vec<String> = mystrings.iter().map(|s| s[46..58].to_owned()).collect();
        let mut space2: Vec<String> = mystrings.iter().map(|s| s[58..62].to_owned()).collect();
        let mut minute_ones: Vec<String> = mystrings.iter().map(|s| s[62..74].to_owned()).collect();
        let mut bigspace3: Vec<String> = mystrings.iter().map(|s| s[74..79].to_owned()).collect();
        let mut dot2: Vec<String> = mystrings.iter().map(|s| s[79..81].to_owned()).collect();
        let mut bigspace4: Vec<String> = mystrings.iter().map(|s| s[81..86].to_owned()).collect();
        let mut second_tens: Vec<String> = mystrings.iter().map(|s| s[86..98].to_owned()).collect();
        let mut space3: Vec<String> = mystrings.iter().map(|s| s[98..102].to_owned()).collect();
        let mut second_ones: Vec<String> =
            mystrings.iter().map(|s| s[102..114].to_owned()).collect();
        let mut backspace: Vec<String> = mystrings.iter().map(|s| s[114..].to_owned()).collect();
        what_to_print(
            &mut front_space,
            &mut hour_tens,
            &mut hour_ones,
            &mut space1,
            &mut bigspace1,
            &mut dot1,
            &mut bigspace2,
            &mut minute_tens,
            &mut minute_ones,
            &mut space2,
            &mut bigspace3,
            &mut dot2,
            &mut bigspace4,
            &mut second_tens,
            &mut second_ones,
            &mut space3,
            &mut backspace,
        );
        sleep(Duration::from_secs(1));
    }
}
