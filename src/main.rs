#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    convert::TryInto,
    fs::{self, OpenOptions},
    io::{BufWriter, Write},
    time::Instant,
};

type IP = [u32; 4];
static DEBUG: bool = true;

fn main() {
    // get ip
    let mut ip_in = String::new();
    print!("Enter Ip: ");

    let _ = std::io::stdout().flush();
    std::io::stdin()
        .read_line(&mut ip_in)
        .expect("Something bad happened");

    //parse ip and subnet
    let ip_one = ip_in.split("/").collect::<Vec<&str>>()[0];
    let ip: IP = ip_one
        .split(".")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap();
    let subnet: usize = ip_in.split("/").collect::<Vec<&str>>()[1]
        .trim_end()
        .parse()
        .unwrap();
    let subnet_bin = convSubBin(subnet);

    // get net size
    let mut net_size_in = String::new();
    print!("How many IPs per net?: ");
    let _ = std::io::stdout().flush();
    std::io::stdin()
        .read_line(&mut net_size_in)
        .expect("Something bad happened");

    // calc some useful variables
    let min_net_size: u32 = net_size_in.trim().parse().unwrap();
    let target_sub: u32 = 32 - format!("{:b}", min_net_size).len() as u32;
    let net_size: u32 = u32::pow(2, 32 - target_sub);
    let net_upper_bound: u32 = u32::pow(2, target_sub - subnet as u32) * net_size;

    let mut now = Instant::now();
    let subs = calcSubnet(0, net_upper_bound, net_size, ip);
    let _ = subs[subs.len() - 1].strip_suffix(",");
    if DEBUG {
        println!(
            "{}s for calculation",
            now.elapsed().as_millis() as f32 / 1000.0
        );
        println!("{} elements", subs.len());
        // println!("Last Element: \n{:?}", subs[subs.len() - 1]);
    }

    now = Instant::now();
    let file_name = format!("{}_{}_{}.json", convSegIpStr(ip), target_sub, subnet);
    let _ = fs::remove_file(&file_name);
    // let _ = fs::File::create(&file_name);
    // let mut file = OpenOptions::new().append(true).open(file_name).unwrap();
    let mut file = BufWriter::new(fs::File::create(&file_name).unwrap());
    let _ = write!(file, "[");
    for i in subs.iter() {
        let _ = write!(file, "{}", i);
    }
    let _ = write!(file, "\n]");
    if DEBUG {
        println!("{}s for write", now.elapsed().as_millis() as f32 / 1000.0);
    }
}

fn calcSubnet(start: u32, end: u32, net_size: u32, _ip: IP) -> Vec<String> {
    let mut ip = _ip;
    let mut out: Vec<String> = Vec::new();
    ip = incIpByN(&mut ip, start, 1);
    out.push(calcNetInfo(&mut ip, net_size));
    let loop_up_range = (end - start) / net_size - 1;
    for _ in 0..loop_up_range {
        ip = incIpByN(&mut ip, 1, 1);
        out.push(calcNetInfo(&mut ip, net_size));
    }
    return out;
}
fn calcNetInfo(ip: &mut IP, net_size: u32) -> String {
    return format!(
        "\n\t{{\n\
        \t\t\"Network\": \"{}\",\n\
        \t\t\"First\": \"{}\",\n\
        \t\t\"Last\": \"{}\",\n\
        \t\t\"Broadcast\": \"{}\",\n\
        \t\t\"Size\": \"{}\"\n\t}},",
        convSegIpStr(*ip),
        convSegIpStr(incIpByN(ip, 1, 1)),
        convSegIpStr(incIpByN(ip, net_size - 2 - 1, 1)),
        convSegIpStr(incIpByN(ip, 1, 1)),
        net_size - 2
    );
}

fn convSubBin(sub: usize) -> String {
    format!("{}{}", "1".repeat(sub), "0".repeat(32 - sub))
}

fn incIpByN(ip: &mut IP, mut n: u32, start: usize) -> IP {
    // let mut ip: IP = _ip;
    // let mut n = _n;
    let pos: usize = 4 - start;
    if n > 65535 {
        *ip = incIpByN(ip, n / 256, start + 1);
        n = n % 256;
    }
    ip[pos] += n;
    while ip[pos] > 255 {
        ip[pos] -= 256;
        *ip = incIpByN(ip, 1, start + 1);
    }
    return *ip;
}

fn convSegIpStr(ip: IP) -> String {
    format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3])
}
