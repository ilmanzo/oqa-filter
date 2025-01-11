use std::io::{self, BufRead, BufReader, Read, Write};

fn process_urls<R: Read, W: Write>(input: R, mut output: W) -> io::Result<()> {
    let reader = BufReader::new(input);
    let mut list: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Some(url) = line.split("->").nth(1) {
            if url.contains("https://openqa.suse.de/tests/") || url.contains("https://openqa.opensuse.org/tests/") {
                list.push(url.trim().to_string());
            }
        }
    }

    list.sort();
    list.dedup();

    for url in list {
        writeln!(output, "openqa-mon {}", url)?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    process_urls(io::stdin(), io::stdout())
}


// write test for process_urls
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test1() {
        let input = Cursor::new("foo -> https://openqa.suse.de/tests/123\nbar -> https://openqa.suse.de/tests/456\nbaz -> https://openqa.suse.de/tests/123\n");
        let mut output = Vec::new();

        process_urls(input, &mut output).unwrap();

        let expected = "openqa-mon https://openqa.suse.de/tests/123\nopenqa-mon https://openqa.suse.de/tests/456\n";
        assert_eq!(String::from_utf8(output).unwrap(), expected);
    }

    #[test]
    fn test2() {
        let input = Cursor::new("Cloning parents of sle-12-SP5-Server-DVD-Updates-x86_64-Build20250108-1-selinux@64bit\n
            1 job has been created:\n
             - sle-12-SP5-Server-DVD-Updates-x86_64-Build20250108-1-selinux@64bit -> https://openqa.suse.de/tests/16418915\n
            Cloning parents of sle-15-SP2-Server-DVD-Updates-x86_64-Build20250108-1-selinux@64bit\n
            1 job has been created:\n
             - sle-15-SP2-Server-DVD-Updates-x86_64-Build20250108-1-selinux@64bit -> https://openqa.suse.de/tests/16418916\n
            Cloning parents of sle-15-SP3-Server-DVD-Updates-x86_64-Build20250108-1-selinux@64bit\n
            Cloning parents of sle-15-SP7-Online-x86_64-Build51.1-selinux@64bit\n
            1 job has been created:\n
             - sle-15-SP7-Online-x86_64-Build51.1-selinux@64bit -> https://openqa.suse.de/tests/16418917");
             let mut output = Vec::new();
             process_urls(input, &mut output).unwrap();
             let expected = "openqa-mon https://openqa.suse.de/tests/16418915\nopenqa-mon https://openqa.suse.de/tests/16418916\nopenqa-mon https://openqa.suse.de/tests/16418917\n";
             assert_eq!(String::from_utf8(output).unwrap(), expected);
            
    }

}