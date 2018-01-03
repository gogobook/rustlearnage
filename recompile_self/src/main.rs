#[macro_use] //doc: file:///home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/share/doc/rust/html/book/first-edition/macros.html#scoping-and-macro-importexport
extern crate log;// ^ #[macro_use] is for this!

extern crate env_logger;
extern crate filetime;


//use std::env;

// eg. /home/xftroxgpx/build/2nonpkgs/rust.stuff/rustlearnage/compiletime_env
const PWD_AT_COMPILETIME: &'static str = env!("CARGO_MANIFEST_DIR");
//const OUTPUT_EXE_AT_COMPILETIME: &'static str = env!("CARGO_PKG_NAME2"); //not seen if set by build.rs , kinda obvious, but still!
const OPTION_OUTPUT_EXE_AT_COMPILETIME: Option<&'static str> = option_env!("CARGO_TARGET_BINFILE_FULLPATH");//CARGO_TARGET_BINFILE_FULLPATH");//CARGO_TARGET_DIR"); 
//CARGO_PKG_NAME  seems to be fname(without path), unless overriden inside Cargo.toml!

/*
#[cfg(debug_assertions)] //thanks to Arnavion on irc
const CARGO_MODE: &'static str = //this repetition is necessary
"";
#[cfg(not(debug_assertions))]
const CARGO_MODE: &'static str = //this repetition is necessary
"--release";
*/

//lol my first macro, without reading the docs for how to macro, but used: file:///home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/share/doc/rust/html/src/std/macros.rs.html#315
//macro_rules! fflush { ($name:expr) => ({ $name.flush().ok().expect(stringify!(Could not flush $name)); }) }
//"Could not flush $name");}) }

macro_rules! fflush {
    () => ({
        fflush!(stdout);
        fflush!(stderr);
    });
    ($stdwhat:ident) => ({
        use std::io::Write; //XXX: needed for flush() to be seen in scope!
        std::io::$stdwhat().flush().ok().expect(stringify!(Could not flush $stdwhat));
//XXX: how to place $name into str; find a better way?
    });
}


fn main() {
    // XXX: Executables should choose a logging framework and initialize it early in the runtime of
    // the program. Logging frameworks will typically include a function to do this. Any log
    // messages generated before the framework is initialized will be ignored.
    // src:https://docs.rs/log/0.3.8/log/#in-executables
    //
    // Select env_logger, one possible logger implementation
    // (see https://doc.rust-lang.org/log/env_logger/index.html)
    env_logger::init().unwrap();//required to show log msgs! in executables! (not for libs tho!)   XXX: Note: execute like: RUST_LOG=debug ./$0  now you can see the debug!() messages too! (warn ones are shown also, but not by default! so you need to spec. a RUST_LOG=warn at least)
    info!("Starting up...");

    let for_info_only_output_exe_at_compiletime: &'static str = OPTION_OUTPUT_EXE_AT_COMPILETIME.unwrap_or_else(|| { 
        warn!("!! You are not using my modified cargo, ergo when using hardlinked binaries I won't be able to tell you which is the real exe fullpath filename after it got recompiled/updated! See: https://github.com/rust-lang/cargo/issues/2841#issuecomment-354932455 for the cargo patch or fname need_this_env_var.patch"); //to see this warning, run with: RUST_LOG=warn ./$0
        return "";
    }//closure
    ); //assignment

    // FIXME: find better way to detect main.rs and others
    let sources = [std::path::Path::new(&PWD_AT_COMPILETIME).join("src/main.rs")];
    //TODO: use * glob to find all *.rs in src/ ! or something

    // detect if source changed!
    let exe_full_name=std::env::current_exe().unwrap();
    debug!("exe_full_name={:?}", exe_full_name);//TODO: try symlink to it, yep it doesn't see the symlink filename, it sees the target fname always! Although doc says "The path returned is not necessarily a "real path" of the executable as there may be intermediate symlinks.", ok checked: they clearly mean hardlinks! Yeah realpath doesn't make sense for hardlinks; Let's get this fixed https://github.com/rust-lang/rust/pull/46987
    debug!("exe args[0]={}", std::env::args().nth(0).expect("failed to get argv[0]"));
    debug!("exe file name at compile time = '{}' (if empty, you're missing patched cargo!)", for_info_only_output_exe_at_compiletime);
    let metadata0 = std::fs::metadata(&exe_full_name).unwrap();
    let mtime0 = filetime::FileTime::from_last_modification_time(&metadata0);
    debug!("old exe mtime={}", mtime0);

    let mut changed=false;
    for each in &sources {
        let metadatax = std::fs::metadata(
            std::path::Path::new(&PWD_AT_COMPILETIME)
            .join(each)
            ).unwrap();
        let mtimex=filetime::FileTime::from_last_modification_time(&metadatax);
        if mtimex > mtime0 {
            /*//#[cfg(debug_assertions)] {
            if cfg!(debug_assertions) {
                eprintln!("{:?} is newer than {:?}", each, exe_full_name);
            }*/
            info!("{:?} is newer than {:?}", each, exe_full_name);
            if !changed {changed=true}
        }
    }

    if changed {
        eprint!("!! Recompiling executable due to source changed...");
        //std::io::stdout().flush().ok().expect("Could not flush stdout");
        //fflush!(std::io::stdout());
        fflush!();
        let args=vec!["build","-v",
        #[cfg(not(debug_assertions))]
        "--release"
        ]; //XXX: nvm: replace with 'run' so we don't have to manually also run it below! Actually NO, because then we have to show stdout/stderr from compilation too!
        /*if !CARGO_MODE.is_empty() {
            args.push(CARGO_MODE);
        }*/
        let output=std::process::Command::new("cargo")
            //FIXME: cargo command is assumed to be in PATH, instead of using CARGO env var.; perhaps
            //it's for the best? but should have a fallback!
            .current_dir(PWD_AT_COMPILETIME)
            .args(&args)
            .output()
            .expect("failed to execute process");
        if output.status.success() {
            eprintln!("all good! {}", output.status);
        }else{
            eprintln!("failed! {}", output.status);
            eprintln!("!! stdout: {}", String::from_utf8_lossy(&output.stdout));
            eprintln!("!! stderr: {}", String::from_utf8_lossy(&output.stderr));
            warn!("Exiting... with {}",output.status);
            std::process::exit(output.status.code().unwrap());
        }
        //prevent possible endless loop by making sure the timestamps for the old exe and new exe
        //are now different due to compilation placing the new exe in the same place!
        info!("Re-executing self after recompile succeeded");
        let metadata1 = std::fs::metadata(&exe_full_name).unwrap();
        let mtime1 = filetime::FileTime::from_last_modification_time(&metadata1);
        if mtime0 == mtime1 {
            //so, compile succeeded AND
            //mtime isn't updated?!
            //this can only mean one thing, so far, the exe is a hardlink!
            println!("Hardlink detected selfexe={:?} You will have to update this manually! It is currently the outdated binary! The compiled/updated binary is: '{}'",exe_full_name, OPTION_OUTPUT_EXE_AT_COMPILETIME.unwrap_or("not available because you weren't using a patched cargo"));
            //assert!(false);//FIXME: temp, it will crash at the next assert! below anyway
            let exit_code=3;
            warn!("Exiting... with {}", exit_code);
            std::process::exit(exit_code); //TODO FIXME: only exist if cargo isn't patched! TODO: Re-assert mtime diff!
        }
        assert!(mtime0 < mtime1, "old exe mtime {} isn't less than newly compiled exe mtime {}! This means it's probably the same mtime if it's a hardlink! Otherwise some unexpected&unknown bug is at hand!", mtime0, mtime1);//FIXME: hitting this when running a hardlink to the exe!
        //now have to re-execute self
        let child=std::process::Command::new(exe_full_name)
            .args(std::env::args())
            .status()
            .expect("failed to re-execute self after recompilation");
        //exit with the above exit code, to prevent executing the old program
        let exit_code=child.code().unwrap();
        warn!("Exiting... with {}", exit_code);
        std::process::exit(exit_code);
        #[allow(unreachable_code)] { //thanks to mbrubeck for the block idea!
            unreachable!();
        }
    }

    println!("Exe fname is {:?}", exe_full_name);
    //all envs at runtime
    /*    for (key, value) in std::env::vars() {
          println!("{}: {}", key, value);
          }*/

    //one compile time env:
    println!("Hello, world! CARGO_MANIFEST_DIR={}", PWD_AT_COMPILETIME);
    //println!("{}", env!("CARGO_TARGET_DIR"));
    //println!("{}", env!("OUT_DIR"));
}
