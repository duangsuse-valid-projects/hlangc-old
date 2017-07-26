extern crate test;

use std::process::exit;
use std::time::SystemTime;

#[derive(Debug)]
pub struct MonkeyAST {
    pub cmd: Vec<HCommands>,
    pub dat: Vec<HDataTypes>,
    pub tags: TagManager,
}
impl MonkeyAST {
    pub fn new() -> MonkeyAST {
        MonkeyAST {
            cmd: Vec::<HCommands>::new(),
            dat: Vec::<HDataTypes>::new(),
            tags: TagManager::new(),
        }
    }
}

#[derive(Debug)]
pub enum HCommands {
    RED, //read memory to x
    RAD, //read memory,add one, write back
    RSB, //read memory,sub one, write back
    SUB, //sub 1(ptr, iptr) x
    ADD, //plus 1(ptr, iptr) x
    WRT, //write x to memory(ptr, iptr)
    JMP, //jump to tag
    QNU, //if x is blank,jump to tag
    QPJ, //if x is positive,jump to tag
    QZJ, //if x is zero,jump to tag
    QNJ, //if x is negative,tump to tag
    O, //put x to numeric output
    AO, //put x to ascii output(as ascii char)
    I, //input next argument to x
}

#[derive(Debug)]
pub enum HDataTypes {
    NumLiteral(i32),
    Pointer(usize),
    IndirectPointer(usize),
    Nil,
}
#[derive(Debug)]
pub struct Tag {
    id: i32,
    lo: u32,
}
impl Tag {
    pub fn new(id: i32, lo: u32) -> Tag {
        Tag { id: id, lo: lo }
    }
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_lo(&self) -> u32 {
        self.lo
    }
}

#[derive(Debug)]
pub struct TagManager {
    pub tags: Vec<Tag>,
}
impl TagManager {
    pub fn new() -> TagManager {
        TagManager { tags: Vec::<Tag>::new() }
    }
    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.push(tag);
    }
}

impl HCommands {
    pub fn to_str(&self) -> &str {
        match self {
            &HCommands::ADD => ":monkey_face:",
            &HCommands::AO => ":loudspeaker:",
            &HCommands::I => ":poultry_leg:",
            &HCommands::JMP => ":monkey:",
            &HCommands::O => ":hankey:",
            &HCommands::QNJ => ":question::scream::monkey:",
            &HCommands::QNU => ":question::mailbox_with_no_mail::monkey:",
            &HCommands::QPJ => ":question::banana::monkey:",
            &HCommands::QZJ => ":question::ghost::monkey:",
            &HCommands::RAD => ":thumbsup:",
            &HCommands::RED => ":eyes:",
            &HCommands::RSB => ":thumbsdown:",
            &HCommands::SUB => ":see_no_evil:",
            &HCommands::WRT => ":memo:",
        }
    }
}
#[cfg(test)]
mod parser_tests {
    use parser::{HCommands, HDataTypes, parse_program};
    use parser::test::Bencher;
    #[test]
    fn it_works() {
        let program = "
        :point_right: 2
        :see_no_evil::point_right:3:point_right:
        :see_no_evil:3
        :hankey:
        ";
        let r = parse_program(program, false, false);
        match r.cmd[0] {
            HCommands::SUB => {}
            _ => panic!("parser err, expected SUB but {} given", r.cmd[0].to_str()),
        }
        match r.cmd[1] {
            HCommands::SUB => {}
            _ => panic!("parser err, expected SUB but {} given", r.cmd[1].to_str()),
        }
        match r.cmd[2] {
            HCommands::O => {}
            _ => panic!("parser err, expected O but {} given", r.cmd[2].to_str()),
        }
        match r.dat[0] {
            HDataTypes::IndirectPointer(3) => {}
            _ => panic!("parser err, expected IPtr(3) but {:?} given", r.dat[0]),
        }
        match r.dat[1] {
            HDataTypes::NumLiteral(3) => {}
            _ => panic!("parser err, expected 3 but {:?} given", r.dat[1]),
        }
        assert_eq!(r.tags.tags[0].get_id(), 2);
    }
    #[bench]
    fn parse_speed(b: &mut Bencher) {
        b.iter(|| parse_bench())
    }
    fn parse_bench() {
        let program = "
//[prime_factorizer]
:point_right:1 0
:poultry_leg:
:memo::point_right: 1
:question::mailbox_with_no_mail::monkey: 4
:memo::point_right: 3
:eyes: 2
:memo::point_right: 2
:eyes: 0
:memo::point_right: 4
:point_right:2
:thumbsup::point_right: 4
:eyes::point_right: 2
:memo::point_right: 1023
:eyes::point_right: 1
:see_no_evil::point_right: 1023
:memo::point_right: 1
:question::banana::monkey: 2
:question::ghost::monkey: 3
:eyes: 0
:memo::point_right: 4
:eyes::point_right: 3
:memo::point_right: 1
:thumbsup::point_right: 2
:monkey: 2
:point_right:3
:eyes::point_right: 2
:hankey:
:thumbsdown::point_right: 4
:question::ghost::monkey: 1
:monkey_face: 1
:memo::point_right: 1
:memo::point_right: 3
";
        parse_program(program, false, false);
    }
}

pub fn parse_program(prog: &str, verbose: bool, debug: bool) -> MonkeyAST {
    let time_start = SystemTime::now();
    let mut ret = MonkeyAST::new();
    for (n, l) in prog.lines().enumerate() {
        let line_trimed = l.split("//").next().unwrap_or(l);
        if line_trimed.trim() != "" {
            parse_line(&n, l.replace(" ", "").as_str(), &mut ret, debug);
        }
    }
    let time_duration = SystemTime::now().duration_since(time_start);
    if debug {
        println!("[Parser] parse finished in {:?}", time_duration);
    }
    if verbose {
        println!(
            "[Parser] parse finished in {} secs. result: {:?}",
            time_duration
                .unwrap_or_else(|e| panic!("cannot get time duration:{:?}", e))
                .as_secs(),
            ret
        );
    }
    ret
}
fn parse_line(ln_num: &usize, line: &str, target: &mut MonkeyAST, debug: bool) {
    if debug {
        println!("[Parser] parsing line {}:{}", ln_num, line);
    }
    if line.starts_with(":monkey_") {
        target.cmd.push(HCommands::ADD);
        target.dat.push(datparse(HCommands::ADD, line, ln_num));
    } else if line.starts_with(":l") {
        target.cmd.push(HCommands::AO);
        target.dat.push(datparse(HCommands::AO, line, ln_num));
    } else if line.starts_with(":pou") {
        target.cmd.push(HCommands::I);
        target.dat.push(datparse(HCommands::I, line, ln_num));
    } else if line.starts_with(":poi") {
        let mut trimed = line.replace(":point_right:", "");
        if debug {
            println!("[Parser (parsetag)]: trimed line:{}", trimed);
        }
        trimed = trimed.split("//").next().unwrap().to_string();
        if debug {
            println!("parser[parsetag]: splited line:{}", trimed);
        }
        target.tags.add_tag(Tag::new(
            trimed.parse::<i32>().unwrap(),
            target.cmd.len() as u32,
        ));
    } else if line.starts_with(":monkey:") {
        target.cmd.push(HCommands::JMP);
        target.dat.push(datparse(HCommands::JMP, line, ln_num));
    } else if line.starts_with(":h") {
        target.cmd.push(HCommands::O);
        target.dat.push(datparse(HCommands::O, line, ln_num));
    } else if line.starts_with(":question::s") {
        target.cmd.push(HCommands::QNJ);
        target.dat.push(datparse(HCommands::QNJ, line, ln_num));
    } else if line.starts_with(":question::m") {
        target.cmd.push(HCommands::QNU);
        target.dat.push(datparse(HCommands::QNU, line, ln_num));
    } else if line.starts_with(":question::b") {
        target.cmd.push(HCommands::QPJ);
        target.dat.push(datparse(HCommands::QPJ, line, ln_num));
    } else if line.starts_with(":question::g") {
        target.cmd.push(HCommands::QZJ);
        target.dat.push(datparse(HCommands::QZJ, line, ln_num));
    } else if line.starts_with(":thumbsu") {
        target.cmd.push(HCommands::RAD);
        target.dat.push(datparse(HCommands::RAD, line, ln_num));
    } else if line.starts_with(":e") {
        target.cmd.push(HCommands::RED);
        target.dat.push(datparse(HCommands::RED, line, ln_num));
    } else if line.starts_with(":thumbsd") {
        target.cmd.push(HCommands::RSB);
        target.dat.push(datparse(HCommands::RSB, line, ln_num));
    } else if line.starts_with(":s") {
        target.cmd.push(HCommands::SUB);
        target.dat.push(datparse(HCommands::SUB, line, ln_num));
    } else if line.starts_with(":m") {
        target.cmd.push(HCommands::WRT);
        target.dat.push(datparse(HCommands::WRT, line, ln_num));
    } else {
        println!("fatal: cannot parse command at line {}", ln_num + 1);
        exit(2);
    }
}
fn datparse(cmdtpe: HCommands, line: &str, ln: &usize) -> HDataTypes {
    let mut tmp: String = line.replace(cmdtpe.to_str(), "");
    tmp = tmp.split("//").next().unwrap().to_string();
    if let Ok(i) = tmp.parse::<i32>() {
        HDataTypes::NumLiteral(i)
    } else {
        if tmp.starts_with(":point_right:") && tmp.ends_with(":point_right:") {
            let replaced = tmp.replace(":point_right:", "");
            HDataTypes::IndirectPointer(replaced.parse::<usize>().unwrap())
        } else {
            if tmp == "" {
                HDataTypes::Nil
            } else {
                let replaced = tmp.replace(":point_right:", "");
                if let Ok(i) = replaced.parse::<usize>() {
                    HDataTypes::Pointer(i)
                } else {
                    println!("fatal: cannot parse data at line {}", ln + 1);
                    exit(2);
                }
            }
        }
    }
}
