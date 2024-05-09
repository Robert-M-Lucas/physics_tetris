/// Prints formatted text
///
/// # Arguments
/// * `colour` / `colours` - List of formatting for the text
/// * `format! args` - Remaining args formatted like `format!`
///
/// # Example
/// ```
/// use whython_5::col_println;
///
/// col_println!((red, bold), "Sample Text: [{}, {}]", "Text one", "text two");
/// ```
#[macro_export]
macro_rules! col_println {
    ($color: ident, $($arg:tt)*) => {
        {
            use colored::Colorize;
            println!("{}", format!($($arg)*).$color())
        }
    };
    (($($col_args:tt),*), $($arg:tt)*) => {
        {
            use colored::Colorize;
            println!("{}", format!($($arg)*)$(.$col_args())*)
        }
    };
}

/// Prints formatted text
///
/// # Arguments
/// * `colour` / `colours` - List of formatting for the text
/// * `format! args` - Remaining args formatted like `format!`
///
/// # Example
/// ```
/// use whython_5::col_print;
///
/// col_print!((red, bold), "Sample Text: [{}, {}]", "Text one", "text two");
/// ```
#[macro_export]
macro_rules! col_print {
    ($color: ident, $($arg:tt)*) => {
       {
           use colored::Colorize;
           print!("{}", format!($($arg)*).$color())
       }
    };
    (($($col_args:tt),*), $($arg:tt)*) => {
        {
            use colored::Colorize;
            print!("{}", format!($($arg)*)$(.$col_args())*)
        }
    };
}