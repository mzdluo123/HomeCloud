
fn main(){
    println!("cargo:rustc-link-search=native=/libpcap");
    println!("cargo:rustc-link-lib=static=pcap");
}