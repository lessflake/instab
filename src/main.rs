#![no_std]
#![no_main]
#![windows_subsystem = "console"]

use libc_print::std_name::{print, println};

use stab::{
    color::ColorByRater, Boss, Fractal, Opts, Parsable, Rateable, Rater, Rating, Searchable, Set,
};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    #[cfg(any(windows, debug_assertions))]
    libc_print::libc_eprintln!("{}", _info);
    unsafe { libc::exit(libc::EXIT_FAILURE) }
}

#[cfg(windows)]
mod windows {
    #[cfg(target_feature = "crt-static")]
    #[link(name = "libcmt")]
    extern "C" {}

    #[cfg(not(target_feature = "crt-static"))]
    #[link(name = "msvcrt")]
    extern "C" {}
}

#[no_mangle]
pub extern "C" fn main(argc: isize, argv: *const *const u8) -> isize {
    let opts = Opts::parse(argc, argv);

    if let Some(boss) = opts.boss {
        run_for_boss(boss, opts.num, opts.threshold);
        return 0;
    } else if let Some(frac) = opts.fractal {
        run_for_fractal(frac, opts.num, opts.threshold);
        return 0;
    }

    use stab::color::SetColored;

    for (i, date, set) in stab::date::future_days()
        .take(opts.num as usize)
        .enumerate()
        .map(|(i, d)| (i, d, Set::parse(d)))
    {
        println!("{}", date);
        print!("{}", set.colored());
        if i < opts.num as usize - 1 {
            println!();
        }
    }

    0
}

fn run<U>(fractal: Fractal, n: u16, threshold: Rating, comp: &U)
where
    U: Rater + Copy,
{
    for (date, set, daily) in stab::date::future_days()
        .map(|d| (d, Set::parse(d)))
        .filter_map(|(date, s)| s.find_fractal(&fractal).map(|d| (date, s, d)))
        .filter(|(_, s, r)| s.get(r).unwrap().rate(comp) >= threshold)
        .take(n as usize)
    {
        let daily = set.get(&daily).unwrap();
        // let rating = daily.rate(comp);
        // println!("{} {}", date.color_by_rating(rating), daily.instabs());
        println!("{} {}", date, daily.color_by_rater(*comp));
        // println!(
        //     "{} {}: {}",
        //     date,
        //     daily.fractal(), /*.color_by_rating(rating)*/
        //     daily.instabs()
        // );
    }
}

fn run_for_fractal(fractal: Fractal, n: u16, threshold: Rating) {
    run::<Fractal>(fractal, n, threshold, &fractal)
}

fn run_for_boss(boss: Boss, n: u16, threshold: Rating) {
    run::<Boss>(boss.home(), n, threshold, &boss)
}
