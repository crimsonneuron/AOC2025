fn main() {
    let string = "1,2
3,4";
    for line in string.split('n') {
        println!("{line}");
    }
}


