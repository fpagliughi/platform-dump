// platform-dump/src/main.rs

fn main() {
    // ----- target_arch -----

    println!("target_arch");

    #[cfg(target_arch = "x86")]
    println!("\tx86");

    #[cfg(target_arch = "x86_64")]
    println!("\tx86_64");

    // TODO

    // ----- target_feature -----

    println!("\ntarget_feature");

    #[cfg(target_feature = "avx")]
    println!("\tavx");

    #[cfg(target_feature = "avx2")]
    println!("\tavx2");

    #[cfg(target_feature = "crt-static")]
    println!("\tcrt-static");

    #[cfg(target_feature = "rdrand")]
    println!("\trdrand");

    #[cfg(target_feature = "sse")]
    println!("\tsse");

    #[cfg(target_feature = "sse2")]
    println!("\tsse2");

    #[cfg(target_feature = "sse4.1")]
    println!("\tsse4.1");

    // ----- target_os -----

    println!("\ntarget_os");

    #[cfg(target_os = "windows")]
    println!("\twindows");

    #[cfg(target_os = "macos")]
    println!("\tmacos");

    #[cfg(target_os = "ios")]
    println!("\tios");

    #[cfg(target_os = "linux")]
    println!("\tlinux");

    #[cfg(target_os = "android")]
    println!("\tandroid");

    #[cfg(target_os = "freebsd")]
    println!("\tfreebsd");

    #[cfg(target_os = "dragonfly")]
    println!("\tdragonfly");

    #[cfg(target_os = "openbsd")]
    println!("\topenbsd");

    #[cfg(target_os = "netbsd")]
    println!("\tnetbsd");

    // ----- target_family -----

    println!("\ntarget_family");

    #[cfg(target_family = "unix")]
    println!("\tunix");

    #[cfg(target_family = "windows")]
    println!("\twindows");

    #[cfg(target_family = "wasm")]
    println!("\twasm");

    // ----- target_env -----

    println!("\ntarget_env");

    #[cfg(target_env = "")]
    println!("\t\"\"");

    #[cfg(target_env = "gnu")]
    println!("\tgnu");

    #[cfg(target_env = "msvc")]
    println!("\tmsvc");

    #[cfg(target_env = "musl")]
    println!("\tmusl");

    #[cfg(target_env = "sgx")]
    println!("\tsgx");

    // ----- target_endian -----

    println!("\ntarget_endian");

    #[cfg(target_endian = "little")]
    println!("\tlittle");

    #[cfg(target_endian = "big")]
    println!("\tbig");

    // ----- target_pointer_width -----

    println!("\ntarget_pointer_width");

    #[cfg(target_pointer_width = "16")]
    println!("\t16");

    #[cfg(target_pointer_width = "32")]
    println!("\t32");

    #[cfg(target_pointer_width = "64")]
    println!("\t64");

    // ----- target_vendor -----

    println!("\ntarget_vendor");

    #[cfg(target_vendor = "apple")]
    println!("\tapple");

    #[cfg(target_vendor = "fortanix")]
    println!("\tfortanix");

    #[cfg(target_vendor = "pc")]
    println!("\tpc");

    #[cfg(target_vendor = "unknown")]
    println!("\tunknown");

    // ----- target_has_atomic -----

    println!("\ntarget_has_atomic");

    #[cfg(target_has_atomic = "8")]
    println!("\t8");

    #[cfg(target_has_atomic = "16")]
    println!("\t16");

    #[cfg(target_has_atomic = "32")]
    println!("\t32");

    #[cfg(target_has_atomic = "64")]
    println!("\t64");

    #[cfg(target_has_atomic = "128")]
    println!("\t128");

    #[cfg(target_has_atomic = "ptr")]
    println!("\tptr");
}
