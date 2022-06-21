use subprocess::{Popen, PopenConfig, Redirection, Result};
fn main() -> Result<()> {
    let mut p = Popen::create(
        &["ps", "x"],
        PopenConfig {
            stdout: Redirection::Pipe,
            ..Default::default()
        },
    )?;
    let (out, err) = p.communicate(None)?;
    if let Some(x) = out {
        let a: Vec<_> = x
            .split('\n')
            .skip(1)
            .filter_map(|line| {
                if line.is_empty() {
                    None
                } else {
                    line.trim().split(' ').next()
                }
            })
            .collect();
        println!("{:?}", a);
    } else if let Some(x) = err {
        eprintln!("{}", x);
    }
    Ok(())
}
