fn main() {
    let ranges: [(u32, u32); 13] = [
        (0x0021, 0x0021),
        (0x0023, 0x0026),
        (0x0028, 0x007E),
        (0x00A1, 0x00AC),
        (0x00AE, 0x00FF),
        (0x0100, 0x017F),
        (0x0180, 0x024F),
        (0x2C60, 0x2C7F),
        (0x16A0, 0x16F0),
        (0x0370, 0x0377),
        (0x037A, 0x037E),
        (0x0384, 0x038A),
        (0x038C, 0x038C),
    ];

    for (x, y) in ranges.iter() {
        //println!("vec contained {} {}", x, y);
        let deb: u32 = *x;
        let fin: u32 = *y + 1;
        for c in deb..fin {
            //print!("{}", c);
            print!("{}", char::from_u32(c).unwrap())
        }
    }
}
