use subprocess::{Popen, PopenConfig, Redirection, Result};
fn main() -> Result<()> {
    let test_path = std::env::args()
        .nth(1)
        .expect("Error: expected test folder path");
    let partitions = [str("askaprt"), str("highmem"), str("debug"), str("long"), str("work"), str("copy")];
    let execs = [
        [test_path.to_owned() + "/" + "numprocs"],
        [test_path.to_owned() + "/" + "qos"],
        [test_path.to_owned() + "/" + "salloc"],
        [[test_path + "/" + "sinfo"],partitions].concat()
    ];
    for e in &execs {
        let mut p = Popen::create(
            e,
            PopenConfig {
                stdout: Redirection::Pipe,
                stderr: Redirection::Pipe,
                ..Default::default()
            },
        )?;
        let (out, _err) = p.communicate(None)?;
        if out == None {
            println!("{}: FAIL", e[0]);
        } else if let Some(r) = out {
            let width = 23;
            println!("{:width$}: {}", e[0], r);
        }
    }
    Ok(())
}
fn str(s: &str) -> String { String::from_str(s) }
