use std::io::Cursor;
use oqa_jobfilter::process_input;

#[test]
fn test_dedup() {
    let input = Cursor::new("foo -> https://openqa.suse.de/tests/123\nbar -> https://openqa.opensuse.org/tests/456\nbaz -> https://openqa.suse.de/tests/123\n");
    let mut output = Vec::new();
    process_input(input, &mut output).unwrap();
    assert_eq!(
        String::from_utf8(output).unwrap(),
        "openqa-mon https://openqa.suse.de/tests/123 https://openqa.opensuse.org/tests/456\n"
    );
}

#[test]
fn test_noisy_input() {
    let input = Cursor::new("Cloning parents of sle-12-SP5-Server-DVD-Updates-x86_64-Build20250108-1-selinux@64bit\n
        1 job has been created:\n
         - sle-12-SP5-Server-DVD-Updates-x86_64-Build20250108-1-selinux@64bit -> https://openqa.suse.de/tests/16418915\n
        Cloning parents of sle-15-SP2-Server-DVD-Updates-x86_64-Build20250108-1-selinux@64bit\n
        1 job has been created:\n
         - sle-15-SP2-Server-DVD-Updates-x86_64-Build20250108-1-selinux@64bit -> https://openqa.suse.de/tests/16418917\n
        Cloning parents of sle-15-SP3-Server-DVD-Updates-x86_64-Build20250108-1-selinux@64bit\n
        Cloning parents of sle-15-SP7-Online-x86_64-Build51.1-selinux@64bit\n
        1 job has been created:\n
         - sle-15-SP7-Online-x86_64-Build51.1-selinux@64bit -> https://openqa.opensuse.org/tests/16418919");
         let mut output = Vec::new();
         process_input(input, &mut output).unwrap();
         let expected = "openqa-mon https://openqa.suse.de/tests/16418915 https://openqa.suse.de/tests/16418917 https://openqa.opensuse.org/tests/16418919\n";
         assert_eq!(String::from_utf8(output).unwrap(), expected);
        
}

#[test]
fn test_consecutive_ids() {
    let input = Cursor::new(
        "foo -> https://openqa.suse.de/tests/123\n\
         bar -> https://openqa.suse.de/tests/124\n\
         baz -> https://openqa.suse.de/tests/125\n"
    );
    let mut output = Vec::new();
    process_input(input, &mut output).unwrap();
    assert_eq!(
        String::from_utf8(output).unwrap(),
        "openqa-mon https://openqa.suse.de/tests/123+2\n"
    );
}

#[test]
fn test_compact_output() {
    let input = Cursor::new(
        "test1 -> https://openqa.suse.de/tests/123\n\
         test2 -> https://openqa.suse.de/tests/125\n\
         test3 -> https://openqa.suse.de/tests/127\n"
    );
    let mut output = Vec::new();
    process_input(input, &mut output).unwrap();
    assert_eq!(
        String::from_utf8(output).unwrap(),
        "openqa-mon https://openqa.suse.de 123,125,127\n"
    );
}
