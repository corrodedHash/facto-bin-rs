use std::io::stdout;

use facto::FactoringEventSubscriptor;
use termion::raw::IntoRawMode;

enum FactorType {
    Prime,
    Composite,
    Unknown,
}

pub struct VerboseFactoring {
    primes: Vec<String>,
    composite: Vec<(facto::Integer, Vec<(facto::Integer, FactorType)>)>,
    unknown: Vec<facto::Integer>,
    historic: bool,
}

impl VerboseFactoring {
    pub fn new(n: facto::Integer, historic: bool) -> Self {
        Self {
            primes: Default::default(),
            composite: vec![(n, vec![])],
            unknown: Default::default(),
            historic,
        }
    }

    pub fn print_state(&self) {
        if self.historic {
            self.print_state_history()
        } else {
            self.print_state_succinct()
        }
    }

    #[allow(dead_code)]
    fn print_state_succinct(&self) {
        use std::io::Write;
        let mut write_buffer = vec![];

        write!(
            write_buffer,
            "Prime factors: [{}{}{}]\n\r",
            termion::color::Green.fg_str(),
            self.primes.join(" "),
            termion::color::Reset.fg_str(),
        )
        .unwrap();
        if self.composite.iter().any(|(_, l)| l.is_empty()) {
            write!(
                write_buffer,
                "\n\rComposite factors: [{}{}{}]\n\r",
                termion::color::Red.fg_str(),
                self.composite
                    .iter()
                    .filter(|(_, l)| l.is_empty())
                    .map(|x| x.0.to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
                termion::color::Reset.fg_str(),
            )
            .unwrap();
        }
        if !self.unknown.is_empty() {
            write!(
                write_buffer,
                "\n\rFactors being checked for primality: [{}]\n\r",
                self.unknown
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
            .unwrap();
        }
        let mut s = stdout().into_raw_mode().unwrap();
        write!(
            s,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::AfterCursor
        )
        .unwrap();
        s.write_all(&write_buffer).unwrap();
    }

    #[allow(dead_code)]
    fn print_state_history(&self) {
        use std::io::Write;
        let mut write_buffer = vec![];
        // let screen = termion::screen::AlternateScreen::from(_stdout);
        write!(
            write_buffer,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::AfterCursor
        )
        .unwrap();
        for (c, f) in &self.composite {
            if f.is_empty() {
                write!(
                    write_buffer,
                    "{}{}{}{}{}\n\r",
                    termion::color::Red.fg_str(),
                    termion::style::Bold,
                    c.to_string(),
                    termion::style::NoBold,
                    termion::color::Reset.fg_str(),
                )
                .unwrap();
            } else {
                write!(
                    write_buffer,
                    "{}{}{}\n\r      ",
                    termion::color::Rgb(120, 120, 120).fg_string(),
                    c.to_string(),
                    termion::color::Reset.fg_str(),
                )
                .unwrap();

                for (factor, factor_type) in f {
                    let fg_str = match factor_type {
                        FactorType::Prime => termion::color::Green.fg_str(),
                        FactorType::Composite => termion::color::Red.fg_str(),
                        FactorType::Unknown => "",
                    };
                    write!(
                        write_buffer,
                        "{}{}{} ",
                        fg_str,
                        factor.to_string(),
                        termion::color::Reset.fg_str()
                    )
                    .unwrap();
                }
                write!(write_buffer, "\n\r").unwrap();
            }
        }
        if !self.primes.is_empty() {
            write!(write_buffer, "\n\r").unwrap();
            write!(
                write_buffer,
                "Found prime divisors: [{}{}{}]\n\r",
                termion::color::Green.fg_str(),
                self.primes.join(" "),
                termion::color::Reset.fg_str()
            )
            .unwrap();
        }
        let mut s = stdout().into_raw_mode().unwrap();
        s.write_all(&write_buffer).unwrap();
    }
}

impl FactoringEventSubscriptor<facto::Integer> for VerboseFactoring {
    fn factorized(
        &mut self,
        n: &facto::Integer,
        primes: &[facto::Integer],
        composites: &[facto::Integer],
        unknown: &[facto::Integer],
    ) {
        self.primes.extend(primes.iter().map(|x| x.to_string()));
        self.primes.sort_unstable();

        for c in composites {
            self.composite.push((c.clone(), vec![]))
        }
        let i = self
            .composite
            .iter_mut()
            .position(|(x, items)| x == n && items.is_empty())
            .unwrap_or_else(|| {
                self.composite.push((n.clone(), vec![]));
                self.composite.len() - 1
            });
        let items = &mut self.composite[i].1;
        for p in primes {
            items.push((p.clone(), FactorType::Prime));
        }
        for p in composites {
            items.push((p.clone(), FactorType::Composite));
        }
        for p in unknown {
            items.push((p.clone(), FactorType::Unknown));
        }
        self.unknown.extend_from_slice(unknown);
        self.print_state();
    }

    fn is_prime(&mut self, n: &facto::Integer) {
        self.primes.push(n.to_string());
        self.primes.sort_unstable();
        for (_, items) in self.composite.iter_mut() {
            for (_, t) in items.iter_mut().filter(|(v, _)| v == n) {
                *t = FactorType::Prime;
            }
        }
        self.unknown
            .remove(self.unknown.iter().position(|x| x == n).unwrap());
        self.print_state();
    }

    fn is_composite(&mut self, n: &facto::Integer) {
        self.composite.push((n.clone(), vec![]));
        for (_, items) in self.composite.iter_mut() {
            for (_, t) in items.iter_mut().filter(|(v, _)| v == n) {
                *t = FactorType::Composite;
            }
        }
        self.unknown
            .remove(self.unknown.iter().position(|x| x == n).unwrap());
        self.print_state();
    }
}
