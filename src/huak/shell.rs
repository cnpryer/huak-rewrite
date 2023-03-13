use std::io::Write;

use termcolor::{self, Color, ColorSpec, StandardStream, WriteColor};
use termcolor::{
    Color::{Cyan, Green, Red, Yellow},
    ColorChoice,
};

use crate::error::HuakResult;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Verbosity {
    #[default]
    Verbose,
    Normal,
    Quiet,
}

/// An abstraction around console output that remembers preferences for output
/// verbosity and color (inspired by cargo's implementation).
pub struct Shell {
    /// A write object for shell output.
    output: ShellOut,
    /// How verbose messages should be.
    verbosity: Verbosity,
}

impl Shell {
    /// Create a new shell with maximum verbosity.
    pub fn new() -> Shell {
        Shell {
            verbosity: Verbosity::Verbose,
            output: ShellOut::Stream {
                stdout: StandardStream::stdout(ColorChoice::Auto),
                stderr: StandardStream::stderr(ColorChoice::Auto),
                color_choice: ColorChoice::Auto,
            },
        }
    }

    /// Shortcut to right-align and color green a status message.
    pub fn status<T, U>(&mut self, status: T, message: U) -> HuakResult<()>
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        self.print(&status, Some(&message), Green, true)
    }

    pub fn status_header<T>(&mut self, status: T) -> HuakResult<()>
    where
        T: std::fmt::Display,
    {
        self.print(&status, None, Cyan, true)
    }

    /// Shortcut to right-align a status message.
    pub fn status_with_color<T, U>(&mut self, status: T, message: U, color: Color) -> HuakResult<()>
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        self.print(&status, Some(&message), color, true)
    }

    /// Print an error message.
    pub fn print_error<T: std::fmt::Display>(&mut self, message: T) -> HuakResult<()> {
        self.output
            .message_stderr(&"error", Some(&message), Red, false)
    }

    /// Prints a warning message.
    pub fn print_warning<T: std::fmt::Display>(&mut self, message: T) -> HuakResult<()> {
        match self.verbosity {
            Verbosity::Quiet => Ok(()),
            _ => self.print(&"warning", Some(&message), Yellow, false),
        }
    }

    /// Prints a note message.
    pub fn print_note<T: std::fmt::Display>(&mut self, message: T) -> HuakResult<()> {
        self.print(&"note", Some(&message), Cyan, false)
    }

    /// Prints a message, where the status will have `color` color, and can be justified.
    /// The messages follows without color.
    /// NOTE: Messages are printed to stderr. This is behavior cargo implements as well to
    /// avoid poluting stdout for end users. See https://github.com/rust-lang/cargo/issues/1473
    fn print(
        &mut self,
        status: &dyn std::fmt::Display,
        message: Option<&dyn std::fmt::Display>,
        color: Color,
        justified: bool,
    ) -> HuakResult<()> {
        match self.verbosity {
            Verbosity::Quiet => Ok(()),
            _ => self
                .output
                .message_stderr(status, message, color, justified),
        }
    }

    /// Gets a reference to the underlying stdout writer.
    pub fn stdout(&mut self) -> &mut dyn Write {
        self.output.stdout()
    }

    /// Gets a reference to the underlying stderr writer.
    pub fn stderr(&mut self) -> &mut dyn Write {
        self.output.stderr()
    }

    /// Set the verbosity level.
    pub fn set_verbosity(&mut self, verbosity: Verbosity) {
        self.verbosity = verbosity;
    }

    /// Get a reference to the verbosity level.
    pub fn verbosity(&self) -> &Verbosity {
        &self.verbosity
    }

    /// Gets the current color choice.
    ///
    /// If we are not using a color stream, this will always return `Never`, even if the color
    /// choice has been set to something else.
    pub fn color_choice(&self) -> ColorChoice {
        match self.output {
            ShellOut::Stream { color_choice, .. } => color_choice,
            ShellOut::Write(_) => ColorChoice::Never,
        }
    }
}

impl Default for Shell {
    fn default() -> Self {
        Self::new()
    }
}

/// Objects for writing shell output to.
enum ShellOut {
    /// A basic write object without support for color
    Write(Box<dyn Write>),
    /// Color-enabled stdio with information on whether color should be used
    Stream {
        stdout: StandardStream,
        stderr: StandardStream,
        color_choice: ColorChoice,
    },
}

impl ShellOut {
    /// Prints out a message with a status. The status comes first, and is bold plus
    /// the given color. The status can be justified, in which case the max width that
    /// will right align is DEFAULT_MESSAGE_JUSTIFIED_CHARS chars.
    fn message_stderr(
        &mut self,
        status: &dyn std::fmt::Display,
        message: Option<&dyn std::fmt::Display>,
        color: Color,
        justified: bool,
    ) -> HuakResult<()> {
        match *self {
            ShellOut::Write(ref mut w) => {
                if justified {
                    write!(w, "{:>12}", status)?;
                } else {
                    write!(w, "{}:", status)?;
                }
                match message {
                    Some(message) => writeln!(w, " {}", message)?,
                    None => write!(w, " ")?,
                }
            }
            ShellOut::Stream { ref mut stderr, .. } => {
                stderr.reset()?;
                stderr.set_color(ColorSpec::new().set_bold(true).set_fg(Some(color)))?;
                if justified {
                    write!(stderr, "{:>12}", status)?;
                } else {
                    write!(stderr, "{}", status)?;
                    stderr.set_color(ColorSpec::new().set_bold(true))?;
                    write!(stderr, ":")?;
                }
                stderr.reset()?;
                match message {
                    Some(message) => writeln!(stderr, " {}", message)?,
                    None => write!(stderr, " ")?,
                }
            }
        }
        Ok(())
    }

    /// Get a mutable reference to the stdout writer.
    pub fn stdout(&mut self) -> &mut dyn Write {
        match *self {
            ShellOut::Write(ref mut w) => w,
            ShellOut::Stream { ref mut stdout, .. } => stdout,
        }
    }

    /// Get a mutable reference to the stderr writer.
    pub fn stderr(&mut self) -> &mut dyn Write {
        match *self {
            ShellOut::Write(ref mut w) => w,
            ShellOut::Stream { ref mut stderr, .. } => stderr,
        }
    }
}
