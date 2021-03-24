#[macro_export]
macro_rules! seppuku {
    // shorthand for early exit
    ($arg:expr) => {{
        eprintln!("{}", $arg);
        std::process::exit(1);
    }};
    // unformatted string with early exit
    ($code:expr => $arg:expr) => {{
        eprintln!("{}", $arg);
        std::process::exit($code);
    }};
    // formatted string and early exit
    ($code:expr => f$($args:tt)*) => {{
        println!("{}", format!($($args)*));
        std::process::exit($code);
    }};
}

#[macro_export]
macro_rules! run_cmd {
    // capture raw cmd with variadic args
    ($cmd:expr; $($args:expr),* $(,)?) => {{
        use std::process::{Command, Stdio};
        Command::new($cmd)
            .args(&[$($args,)*])
            .stdout(Stdio::null())
            .output()
    }};
    // capture raw cmd and cwd with variadic args
    ($cmd:expr, $wd:expr; $($args:expr),* $(,)?) => {{
        use std::process::{Command, Stdio};
        Command::new($cmd)
            .current_dir($wd)
            .args(&[$($args,)*])
            .stdout(Stdio::null())
            .output()
    }};
    (OS => $($args:tt)*) => {{
        use std::process::{Command, Stdio};
        Command::new("xdg-open")
            .args($($args)*)
            .stdout(Stdio::null())
            .spawn()
    }};
    (@ cmd:expr =>) => {{ std::process::Command::new($cmd).spawn() }};
    // spawn raw cmd with variadic args
    (@ $cmd:expr; $($args:expr),* $(,)?) => {{
        std::process::Command::new($cmd)
            .args(&[$($args,)*])
            .spawn()
            .map(|_| ())
            .map_err(|e| anyhow::anyhow!(e))
    }};
    // spawn raw cmd and cwd with variadic args
    (@ $cmd:expr => $($args:expr),* $(,)?) => {{
        use std::process::{Command, Stdio};
        Command::new($cmd)
            $(.args($args,))*
            .stdout(Stdio::null())
            .spawn()
            .map(|_| ())
            .map_err(|e| anyhow::anyhow!(e))
    }};
}
