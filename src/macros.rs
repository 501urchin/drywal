
#[macro_export]
macro_rules! either {
    ($test:expr => $true_expr:expr ; $false_expr:expr) => {
        if $test { $true_expr } else { $false_expr }
    };
}

#[macro_export]
macro_rules! iferr {
    ($a:expr) => {
        match $a {
            Ok(v) => v,
            Err(e) => {
                println!("an error occured: {}", e);
                return;
            }
        }
    };
}
