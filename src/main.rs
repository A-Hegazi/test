fn div(u: i32, l: i32) -> Result<i32, String> {
    if l == 0 {
        return Err("Cant div by zero".to_string());
    }
    Ok(u / l)
}

fn main() {
    let y = div(5, 0);
    let z = match y {
        Ok(x) => x,
        Err(e) => 0,
    };
    print!("{z}");
}
