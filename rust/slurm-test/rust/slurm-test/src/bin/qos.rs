use subprocess::{Popen, PopenConfig, Redirection, Result};
/// Verify that quality of service is supported by checking
/// that the output contains more than the two header rows.
fn main() -> Result<()> {
    let mut p = Popen::create(
        &["sacctmgr", "show", "qos"],
        PopenConfig {
            stdout: Redirection::Pipe,
            stderr: Redirection::Pipe,
            ..Default::default()
        },
    )?;
    let (out, err) = p.communicate(None)?;
    if let Some(e) = err {
        if !e.is_empty() {
            eprintln!("ERROR: {}", e);
            std::process::exit(1);
        }
    }
    if let Some(c) = out {
        let numrows = c
            .split('\n')
            .skip(2)
            .filter(|line| !line.is_empty())
            .count();
        if numrows > 0 {
            println!("PASS");
        } else {
            println!("FAIL");
        }
    } else {
        eprintln!("ERROR");
    }
    Ok(())
}
