use std::collections::BTreeSet;
use subprocess::{Popen, PopenConfig, Redirection, Result};
/// Verify that the list of partitions matches the list specified on
/// the command line.
fn main() -> Result<()> {
    let mut p = Popen::create(
        &["sinfo"],
        PopenConfig {
            stdout: Redirection::Pipe,
            stderr: Redirection::Pipe,
            ..Default::default()
        },
    )?;
    let (out, _err) = p.communicate(None)?;
    if let Some(x) = out {
        let a: BTreeSet<_> = x
            .split('\n')
            .skip(1)
            .filter_map(|line| {
                if line.is_empty() {
                    None
                } else {
                    line.trim()
                        .split_whitespace()
                        .next()
                        .map(|l| l.trim_matches('*').to_owned())
                }
            })
            .collect();
        if BTreeSet::from_iter(std::env::args().skip(1)) == a {
            println!("PASS");
        } else {
            println!("FAIL");
        }
    } else {
        println!("FAIL");
    }
    Ok(())
}
