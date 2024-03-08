// File descriptor - somewhat a misnomer now but it's nice and short.
// Keeps track of the various ports a stdio could be connected to.
#[derive(Debug)]
pub enum Fd {
    Stdin,
    Stdout,
    Stderr,
    Inherit,
    PipeOut(PipeWriter),
    PipeIn(PipeReader),
    FileName(String),
    FileNameAppend(String),
    RawFile(File),
}

impl PartialEq for Fd {
    fn eq(&self, other: &Self) -> bool {
        self.variant() == other.variant()
    }
}

impl Fd {
    fn variant(&self) -> &str {
        match *self {
            Fd::Stdin => "Stdin",
            Fd::Stdout => "Stdout",
            Fd::Stderr => "Stderr",
            Fd::Inherit => "Inherit",
            Fd::PipeOut(_) => "PipeOut",
            Fd::PipeIn(_) => "PipeIn",
            Fd::FileName(_) => "FileName",
            Fd::FileNameAppend(_) => "FileNameAppend",
            Fd::RawFile(_) => "RawFile", // Not completely accurate, but I think fine for now
        }
    }

    // Gets an stdin - all same here as stdout, except that a file is opened, not created
    pub fn get_stdin(&mut self) -> Option<Stdio> {
        match self {
            Fd::FileName(name) => match File::open(&name) {
                Ok(file) => {
                    *self = Fd::RawFile(file.try_clone().unwrap());
                    Some(Stdio::from(file))
                }
                Err(e) => {
                    eprintln!("rush: {}: {}", name, e);
                    None
                }
            },
            _ => self.get_stdout(),
        }
    }

    // All the ways a Fd could be converted to a Stdio
    // What's the proper way to deal with all of these dup unwraps?
    // What is their fail condition?
    pub fn get_stdout(&mut self) -> Option<Stdio> {
        match self {
            Fd::Stdin => Some(Stdio::from(dup_stdin().unwrap())),
            Fd::Stdout => Some(Stdio::from(dup_stdout().unwrap())),
            Fd::Stderr => Some(Stdio::from(dup_stderr().unwrap())),
            Fd::Inherit => Some(Stdio::inherit()),
            Fd::PipeOut(writer) => Some(Stdio::from(writer.try_clone().unwrap())),
            Fd::PipeIn(reader) => Some(Stdio::from(reader.try_clone().unwrap())),
            Fd::RawFile(file) => Some(Stdio::from(file.try_clone().unwrap())),
            Fd::FileName(name) => match File::create(&name) {
                Ok(file) => {
                    *self = Fd::RawFile(file.try_clone().unwrap());
                    Some(Stdio::from(file))
                }
                Err(e) => {
                    eprintln!("rush: {}: {}", name, e);
                    None
                }
            },
            Fd::FileNameAppend(name) => {
                match OpenOptions::new().append(true).create(true).open(&name) {
                    Ok(file) => {
                        *self = Fd::RawFile(file.try_clone().unwrap());
                        Some(Stdio::from(file))
                    }
                    Err(e) => {
                        eprintln!("rush: {}: {}", name, e);
                        None
                    }
                }
            }
        }
    }

    pub fn get_stderr(&mut self) -> Option<Stdio> {
        self.get_stdout()
    }
}
