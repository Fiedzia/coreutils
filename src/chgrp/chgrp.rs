#![crate_name = "uu_chmod"]

/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Maciej Dziardziel <fiedzia@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

#![allow(unused_variables)]  // only necessary while the TODOs still exist

extern crate aho_corasick;
extern crate getopts;
extern crate libc;
extern crate memchr;
extern crate regex;
extern crate regex_syntax;
extern crate walker;

#[macro_use]
extern crate uucore;

use getopts::Options;
use regex::Regex;
use std::ffi::CString;
use std::io::{Error, Write};
use std::mem;
use std::path::Path;
use walker::Walker;

const NAME: &'static str = "chgrp";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn uumain(args: Vec<String>) -> i32 {
    let mut opts = Options::new();
    opts.optflag("c", "changes", "like verbose but report only when a change is made");
    opts.optflag("f", "quiet", "suppress most error messages");
    opts.optflag("", "silent", "suppress most error messages");
    opts.optflag("v", "verbose", "output a diagnostic for every file processed");
    opts.optflag("", "dereference", "affect the referent of each symbolic link (this is the default), rather than the symbolic link itself");
    opts.optflag("h", "no-dereference", "affect symbolic links instead of any referenced file (useful only on systems that can change the ownership of a symlink)");
    opts.optflag("", "no-preserve-root", "do not treat '/' specially (the default)");
    opts.optflag("", "preserve-root", "fail to operate recursively on '/'");
    opts.optflagopt("", "reference", "use RFILE's mode instead of MODE values", "RFILE");
    opts.optflag("R", "recursive", "change files and directories recursively");
    opts.optflag("H", "", "if a command line argument is a symbolic link to a directory, traverse it");
    opts.optflag("L", "", "traverse every symbolic link to a directory encountered");
    opts.optflag("P", "", "do not traverse any symbolic links (default)");
    opts.optflag("", "help", "display this help and exit");
    opts.optflag("", "version", "output version information and exit");
    // TODO: sanitize input for - at beginning (e.g. chmod -x testfile).  Solution is to add a to -x, making a-x
    let mut matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => { crash!(1, "{}", f) }
    };
    if matches.opt_present("help") {
        let msg = format!("{name} {version}

Usage: chgrp [OPTION]... GROUP FILE...
  or:  chgrp [OPTION]... --reference=RFILE FILE...
Change the group of each FILE to GROU
With --reference, change the group of each FILE to that of RFILE.

  -c, --changes          like verbose but report only when a change is made
  -f, --silent, --quiet  suppress most error messages
  -v, --verbose          output a diagnostic for every file processed
      --dereference      affect the referent of each symbolic link (this is
                         the default), rather than the symbolic link itself
  -h, --no-dereference   affect symbolic links instead of any referenced file
                         (useful only on systems that can change the
                         ownership of a symlink)
      --no-preserve-root  do not treat '/' specially (the default)
      --preserve-root    fail to operate recursively on '/'
      --reference=RFILE  use RFILE's group rather than specifying a
                         GROUP value
  -R, --recursive        operate on files and directories recursively

The following options modify how a hierarchy is traversed when the -R
option is also specified.  If more than one is specified, only the final
one takes effect.

  -H                     if a command line argument is a symbolic link
                         to a directory, traverse it
  -L                     traverse every symbolic link to a directory
                         encountered
  -P                     do not traverse any symbolic links (default)

      --help     display this help and exit
      --version  output version information and exit

Examples:
  chgrp staff /u      Change the group of /u to \"staff\".
  chgrp -hR staff /u  Change the group of /u and subfiles to \"staff\".",
            name = NAME, version = VERSION, program = NAME);

        print!("{}", opts.usage(&msg));
        return 0;
    } else if matches.opt_present("version") {
        println!("{} {}", NAME, VERSION);
    } else if matches.free.is_empty() && matches.opt_present("reference") || matches.free.len() < 2 {
        show_error!("missing an argument");
        show_error!("for help, try '{} --help'", NAME);
        return 1;
    } else {
        /*
        let changes = matches.opt_present("changes");
        let quiet = matches.opt_present("quiet");
        let verbose = matches.opt_present("verbose");
        let preserve_root = matches.opt_present("preserve-root");
        let recursive = matches.opt_present("recursive");
        let fmode = matches.opt_str("reference").and_then(|fref| {
            let mut stat : libc::stat = unsafe { mem::uninitialized() };
            let statres = unsafe { libc::stat(fref.as_ptr() as *const _, &mut stat as *mut libc::stat) };
            if statres == 0 {
                Some(stat.st_mode)
            } else {
                crash!(1, "{}", Error::last_os_error())
            }
        });
        let cmode =
            if fmode.is_none() {
                let mode = matches.free.remove(0);
                match verify_mode(&mode[..]) {
                    Ok(_) => Some(mode),
                    Err(f) => {
                        show_error!("{}", f);
                        return 1;
                    }
                }
            } else {
                None
            };
        match chmod(matches.free, changes, quiet, verbose, preserve_root,
                    recursive, fmode, cmode.as_ref()) {
            Ok(()) => {}
            Err(e) => return e
        }*/
    }

    0
}

