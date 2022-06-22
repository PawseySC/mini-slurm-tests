use subprocess::{Popen, PopenConfig, Redirection, Result};
/// Just verify that it is possible to request an allocation.
fn main() -> Result<()> {
    let mut p = Popen::create(
        &["salloc", "--mem", "1GB", "echo", "ok"],
        PopenConfig {
            stdout: Redirection::Pipe,
            stderr: Redirection::Pipe,
            ..Default::default()
        },
    )?;
    let (out, err) = p.communicate(None)?;
    if let Some(e) = err {
        if !e.trim().to_lowercase().contains("granted job allocation") {
            println!("FAIL");
            std::process::exit(1);
        }
    }
    if let Some(r) = out {
        if r == "ok\n" {
            println!("PASS");
        } else {
            println!("FAIL");
        }
    } else {
        eprintln!("FAIL");
    }
    Ok(())
}
