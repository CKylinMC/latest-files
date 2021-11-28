use chrono::{Local, TimeZone};
use chrono_humanize::{Accuracy, HumanTime, Tense};
use std::fs;
use tabular::{Row, Table};
extern crate clap;
use clap::{App, Arg};

fn err_exit(emsg: &str) {
    println!("error: {}\n\nFor more information try --help",emsg);
    std::process::exit(1);
}

fn format_bytes(bytes:u64)->String{
    let mut bytes = bytes;
    let mut unit = "B";
    if bytes > 1024 {
        bytes = bytes / 1024;
        unit = "KB";
    }
    if bytes > 1024 {
        bytes = bytes / 1024;
        unit = "MB";
    }
    if bytes > 1024 {
        bytes = bytes / 1024;
        unit = "GB";
    }
    if bytes > 1024 {
        bytes = bytes / 1024;
        unit = "TB";
    }
    return format!("{} {}", bytes, unit);
}

fn main() {
    let parser = App::new("latest-files")
        .version("1.0")
        .author("CKylinMC <canszj@yeah.net>")
        .about("List latest files from current directory")
        .arg(
            Arg::with_name("short")
                .short("l")
                .long("short-list")
                .help("Only list file names."),
        )
        .arg(
            Arg::with_name("noindex")
                .short("n")
                .long("no-index")
                .help("Do NOT show index of files."),
        )
        .arg(Arg::with_name("relative")
            .short("r")
            .multiple(true)
            .long("relative-time")
            .help(
                "Show relative time(timeago). Repeat 2 times('-rr') to use more detailed relative time format.",
            )
        )
        .arg(
            Arg::with_name("dir")
                .short("p")
                .long("path")
                .takes_value(true)
                .value_name("PATH")
                .default_value(".")
                .help("Sets a custom path instead of the current directory."),
        )
        .arg(
            Arg::with_name("COUNT")
                .help("Specify how many files to show.")
                //    .required(true)
                .default_value("3")
                .index(1),
        )
        .get_matches();
    let dir = parser.value_of("dir").unwrap_or(".");
    let count = parser
        .value_of("COUNT")
        .unwrap_or("3")
        .parse::<usize>()
        .unwrap_or_else(|_| {
            err_exit("Arg 'COUNT' expect number input but got chars.");
            return 0; // a lie to compiler
        });
    let shortlist = parser.is_present("short");
    let notshowindex = parser.is_present("noindex");
    let mut usetimeago = true;
    let mut timeagodetailed = false;
    match parser.occurrences_of("relative") {
        0 => usetimeago = false,
        1 => timeagodetailed = false,
        2 | _ => timeagodetailed = true,
    };
    let dirres = fs::read_dir(dir).unwrap_or_else(|_| {
        err_exit("Path is invalid or unaccessible.");
        return fs::read_dir(".").unwrap(); // a lie to compiler
    });
    let mut paths: Vec<_> = dirres.map(|r| r.unwrap()).collect();

    paths.sort_by_key(|dir| {
        dir.metadata()
            .unwrap()
            .modified()
            .unwrap()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    });
    let mut table;
    if shortlist {
        table = Table::new("{:<}");
    } else {
        table = Table::new("{:>}{:<}  {:<}  {:>}  {:<}");
    }
    let mut i = 0;
    for path in paths.iter().rev() {
        i += 1;
        if i > count {
            break;
        }
        let dt = Local.timestamp(
            path.metadata()
                .unwrap()
                .modified()
                .unwrap()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            0,
        );
        let p = path.path();
        let s = p.metadata().unwrap().len();
        let fp = p.file_name().unwrap().to_str().unwrap();
        //println!("{}\t{}", &fp[2..], dt.to_rfc2822());
        if shortlist {
            table.add_row(Row::new().with_cell(fp));
        } else {
            let tm;
            if usetimeago {
                let ht = HumanTime::from(dt);
                tm = ht.to_text_en(
                    if timeagodetailed {
                        Accuracy::Precise
                    } else {
                        Accuracy::Rough
                    },
                    Tense::Present,
                );
            } else {
                tm = dt.format("%Y-%m-%d %H:%M:%S").to_string();
            }
            let ti = format!("{}| ",i);
            let index = if notshowindex {""} else {ti.as_str()};
            let fs = format_bytes(s);
            table.add_row(Row::new().with_cell(index).with_cell(fp).with_cell(if p.is_dir(){"DIR"}else{""}).with_cell(if p.is_dir(){""}else{fs.as_str()}).with_cell(tm));
        }
    }
    print!("{}", table);
}
