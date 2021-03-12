#[macro_export]
macro_rules! info {
    // shorthand for success message
    ($arg:expr) => {{
        use colored::Colorize;
        println!("{}", $arg.to_string().color("green"));
    }};
    // shorthand for unformatted string
    ($color:expr; $arg:expr) => {{
        use colored::Colorize;
        println!("{}", $arg.to_string().color($color));
    }};
    // formatted string
    ($color:expr; f$($args:tt)*) => {{
        use colored::Colorize;
        println!("{}", format!($($args)*).color($color));
    }};
}

#[macro_export]
macro_rules! seppuku {
    // shorthand for early exit
    ($arg:expr) => {{
        use colored::Colorize;
        println!("{}", $arg.to_string().color("red"));
        std::process::exit(1);
    }};
    // unformatted string with early exit
    ($code:expr => $color:expr; $arg:expr) => {{
        use colored::Colorize;
        println!("{}", $arg.to_string().color($color));
        std::process::exit($code);
    }};
    // formatted string and early exit
    ($code:expr => $color:expr; f$($args:tt)*) => {{
        use colored::Colorize;
        println!("{}", format!($($args)*).color($color));
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
    // spawn raw cmd with variadic args
    (@ $cmd:expr; $($args:expr),* $(,)?) => {{
        std::process::Command::new($cmd)
            .args(&[$($args,)*])
            .spawn()
    }};
    // spawn raw cmd and cwd with variadic args
    (@ $cmd:expr => $($args:expr),* $(,)?) => {{
        use std::process::{Command, Stdio};
        Command::new($cmd)
            $(.args($args,))*
            .stdout(Stdio::null())
            .spawn()
    }};
}
