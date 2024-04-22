use core::{marker::PhantomData, str::FromStr};

use crate::{Boss, Fractal, Rating};

pub struct Opts {
    pub num: u16,
    pub boss: Option<Boss>,
    pub fractal: Option<Fractal>,
    pub threshold: Rating,
}

struct Args<'a> {
    count: usize,
    values: *const *const u8,
    phantom: PhantomData<&'a [&'a [u8]]>,
}

struct ArgsIter<'a> {
    args: &'a Args<'a>,
    idx: usize,
}

impl<'a> Iterator for ArgsIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.args.count {
            let arg = unsafe {
                let mut len = 0;
                let mut arg = *self.args.values.add(self.idx);
                while *arg != b'\0' {
                    len += 1;
                    arg = arg.add(1);
                }

                core::slice::from_raw_parts(*self.args.values.add(self.idx), len)
            };

            self.idx += 1;
            Some(arg)
        } else {
            None
        }
    }
}

impl<'a> Args<'a> {
    fn new(argc: isize, argv: *const *const u8) -> Self {
        Self {
            count: argc as usize,
            values: argv,
            phantom: PhantomData,
        }
    }

    fn iter(&'a self) -> ArgsIter<'a> {
        ArgsIter { args: self, idx: 0 }
    }
}

trait ByteSliceExt {
    fn as_str(&self) -> &str;
}

impl ByteSliceExt for &[u8] {
    fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self) }
    }
}

impl Opts {
    pub fn parse(argc: isize, argv: *const *const u8) -> Self {
        let mut opts = Self {
            num: 1,
            boss: None,
            fractal: None,
            threshold: Rating::Unplayable,
        };

        let args = Args::new(argc, argv);
        let mut args_iter = args.iter();
        args_iter.next();

        while let Some(arg) = args_iter.next() {
            match arg {
                b"-n" => {
                    if let Some(num) = args_iter
                        .next()
                        .and_then(|s| s.as_str().parse::<u16>().ok())
                    {
                        opts.num = num;
                    } else {
                        panic!("no matching arg for -n");
                    }
                }
                b"-b" => {
                    if let Some(boss) = args_iter
                        .next()
                        .and_then(|s| Boss::from_str(s.as_str()).ok())
                    {
                        opts.boss = Some(boss);
                    } else {
                        panic!("no matching arg for -b");
                    }
                }
                b"-f" => {
                    if let Some(frac) = args_iter
                        .next()
                        .and_then(|s| Fractal::from_str(s.as_str()).ok())
                    {
                        opts.fractal = Some(frac);
                        opts.boss = None;
                    } else {
                        panic!("no matching arg for -f");
                    }
                }
                b"-t" => {
                    if let Some(rating) = args_iter
                        .next()
                        .and_then(|s| Rating::from_str(s.as_str()).ok())
                    {
                        opts.threshold = rating;
                    } else {
                        panic!("no matching arg for -t");
                    }
                }
                _ => {
                    panic!("invalid arg {}", arg.as_str());
                }
            }
        }

        opts
    }
}
