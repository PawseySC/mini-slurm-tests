use subprocess::{Popen, PopenConfig, Redirection, Result};
/// Launch tests in directory passed on the command line.
fn main() -> Result<()> {
    let test_path = std::env::args()
        .nth(1)
        .expect("Error: expected test folder path");
    let partitions: Vec<String> = "askaprt highmem debug long work copy"
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect();
    let execs = vec![
        vec![test_path.to_owned() + "/" + "numprocs"],
        vec![test_path.to_owned() + "/" + "qos"],
        vec![test_path.to_owned() + "/" + "salloc"],
        [vec![test_path + "/" + "sinfo"], partitions].concat(),
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
