use subprocess::{Popen, PopenConfig, Redirection, Result};
/// Verify that it is not possible to use more resources than originally
/// requested
fn main() -> Result<()> {
    let mut p = Popen::create(
        &["salloc", "--mem=1GB", "--ntasks=2"],
        PopenConfig {
            stdout: Redirection::Pipe,
            stdin: Redirection::Pipe,
            stderr: Redirection::Pipe,
            ..Default::default()
        },
    )?;
    let (_out, err) = p.communicate(Some("srun --ntasks=3 hostname"))?;
    if let Some(e) = err {
        if e.to_lowercase()
            .contains(&"more processors requested than permitted")
        {
            println!("PASS");
        } else {
            println!("FAIL");
        }
    } else {
        println!("FAIL");
    }
    Ok(())
}
