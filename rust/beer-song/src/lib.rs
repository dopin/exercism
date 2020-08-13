pub fn verse(count: u32) -> String {
    match count {
        0 => {
            format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the \
                     store and buy some more, 99 bottles of beer on the wall.\n")
        }
        1 => {
            format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it \
                     around, no more bottles of beer on the wall.\n")
        }
        2 => {
            format!("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it \
                     around, 1 bottle of beer on the wall.\n")
        }
        _ => {
            format!("{current} bottles of beer on the wall, {current} bottles of beer.\nTake one \
                     down and pass it around, {next} bottles of beer on the wall.\n",
                    current = count,
                    next = count - 1)
        }
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut lyrics = String::new();
    for i in (end..start + 1).rev() {
        lyrics.push_str(&verse(i));
        if end < i {
            lyrics.push_str("\n");
        }
    }
    lyrics
}