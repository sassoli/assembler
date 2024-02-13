use std::env;
use std::process::exit;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Passare il nome del file da leggere come argomento");
        exit(0);
    }
        
    let mut code:Vec<String> = Vec::new();
    let mut symbol_table = HashMap::from([
        ("R0".to_string(), 0),
        ("R1".to_string(), 1),
        ("R2".to_string(), 2),
        ("R3".to_string(), 3),
        ("R4".to_string(), 4),
        ("R5".to_string(), 5),
        ("R6".to_string(), 6),
        ("R7".to_string(), 7),
        ("R8".to_string(), 8),
        ("R9".to_string(), 9),
        ("R10".to_string(), 10),
        ("R11".to_string(), 11),
        ("R12".to_string(), 12),
        ("R13".to_string(), 13),
        ("R14".to_string(), 14),
        ("R15".to_string(), 15),
        ("SCREEN".to_string(), 16384),
        ("KBD".to_string(), 24576),
        ("SP".to_string(), 0),
        ("LCL".to_string(), 1),
        ("ARG".to_string(), 2),
        ("THIS".to_string(), 3),
        ("THAT".to_string(), 4),
    ]);

    let filename = &args[1];
    let mut result = read_lines(&args[1]);
    result = clean_lines(result);
    //PRIMO PASSAGGIO ESTRAGGO IL NUMERO DI LINEA DELLE ANNOTAZIONI (LOOP)
    symbol_table = first_pass(&result, symbol_table);
    //SECONDO PASSAGGIO INSERISCO LE VARIABILI IN MEMORIA PARTENDO DA 16
    symbol_table = second_pass(&result, symbol_table);
    //dbg!(&symbol_table);

    code = parse_result(result, &symbol_table);
    save_file(filename, code);
}

fn first_pass(result:&Vec<String>, mut symbol_table:HashMap<String, u16>)->HashMap<String, u16>{
    let mut counter = 0;
    for (index, line) in result.iter().enumerate() {
        if line.starts_with("(") {
            let clean_str = line.replace("(", "").replace(")", "").to_string();
            symbol_table.insert(clean_str, counter.try_into().unwrap());
        }else{
            counter+=1;
        }
    }
    symbol_table
}

fn second_pass(result:&Vec<String>, mut symbol_table:HashMap<String, u16>)->HashMap<String, u16>{
    let mut counter = 16;
    for line in result {
        if line.starts_with("@") {
            let clean_str = line.replace("@", "").to_string();
            if !clean_str.parse::<u16>().is_ok() && !symbol_table.contains_key(&clean_str){
                symbol_table.insert(clean_str, counter);
                counter+=1;
            }
        }
    }
    symbol_table
}

fn save_file(filename: &str, code: Vec<String>){
    let filebase = filename.split(".asm");
    let savefilename = format!("{}.jack", filebase.collect::<Vec<&str>>()[0]);
    let mut buffer = File::create(savefilename).expect("errore in creazione");
    buffer.write(code.join("\n").as_bytes()).expect("errore in scrittura");
}

/** Legge una riga alla volta il file passato in input */
fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}


/** funzione che itera il vettore di stringhe skippando le righe di commenti e vuote */
fn clean_lines(lines: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    for line in lines {
        if line.starts_with("//") || line.is_empty() {
            continue;
        }
        result.push(line.replace(" ", ""));
    }
    result
}

/** funzione che itera il vettore di stringhe e restituisce un array di stringhe con le istruzioni in binario */
fn parse_result(lines: Vec<String>, symbol_table:&HashMap<String, u16>) -> Vec<String> {
    let mut result = Vec::new();
    for line in lines {
        let instruction;
        if line.starts_with("("){
            continue;
        } else if line.starts_with("@") {
            instruction = parse_a_instruction(line, &symbol_table);
        } else {
            instruction = parse_c_instruction(line);
        }
        result.push(instruction);
    }
    result
}

/** funzione che prende in input una stringa e restituisce una stringa con l'istruzione in binario */
fn parse_a_instruction(line: String, symbol_table:&HashMap<String, u16>) -> String {
    let mut result = String::new();
    let value = line.replace("@", "");
    if value.parse::<u16>().is_ok(){
        let mut binary_value = format!("{:b}", value.parse::<u16>().unwrap());
        binary_value = format!("{:0>16}", binary_value);
        result.push_str(&binary_value);
    }else{
        let mut binary_value = format!("{:b}", symbol_table.get(&value).unwrap());
        binary_value = format!("{:0>16}", binary_value);
        result.push_str(&binary_value);
    }
    result
}


fn parse_c_instruction(line: String) -> String {
    let mut result = String::new();
    let mut dest = String::new();
    let comp;
    let compjmp;
    let mut jump = String::new();
    let mut instruction = line.split("=");
    if instruction.clone().count() == 2 {
        dest = instruction.next().unwrap().to_string();
        compjmp = instruction.next().unwrap().to_string();
    } else {
        compjmp = instruction.next().unwrap().to_string();
    }
    let mut instruction = compjmp.split(";");
    if instruction.clone().count() == 2 {
        comp = instruction.next().unwrap().to_string();
        jump = instruction.next().unwrap().to_string();
    } else {
        comp = instruction.next().unwrap().to_string();
    }
    result.push_str("111");
    result.push_str(&parse_comp(comp));
    result.push_str(&parse_dest(dest));
    result.push_str(&parse_jump(jump));
    result
}

fn parse_comp(comp: String) -> String {
    let mut comp = comp.replace(" ", "");
    let mut partial;
    match comp.as_str(){ 
        "0" =>partial = "0101010", 
        "1" =>partial = "0111111", 
        "-1"=>partial = "0111111", 
        "D" =>partial = "0001100", 
        "!D" =>partial = "0001101", 
        "-D" =>partial = "0001111", 
        "-D" =>partial = "0001111", 
        "D+1" =>partial = "0011111", 
        "D-1" =>partial = "0001110", 
        _=>partial = "X", 
    } 
    
    if partial == "X"{
        let mut a = "0".to_string();
        if !comp.contains("A"){
            a = "1".to_string();
        }
        comp = comp.replace("A", "Z");
        comp = comp.replace("M", "Z");

        match comp.as_str(){ 
            "Z" =>partial = "110000", 
            "!Z" =>partial = "110001", 
            "-Z" =>partial = "110011", 
            "-Z" =>partial = "110011", 
            "Z+1" =>partial = "110111", 
            "Z-1" =>partial = "110010", 
            "D+Z" =>partial = "000010", 
            "D-Z" =>partial = "010011", 
            "Z-D" =>partial = "000111", 
            "D&Z" =>partial = "000000", 
            "D|Z" =>partial = "010101", 
            
            _=>partial = "C", 
        } 
        a.push_str(&partial); 
        return a;
    }
    partial.to_string()
}


fn parse_dest(dest: String) -> String {
    let mut a = "0";
    let mut d = "0";
    let mut m = "0";
    if dest.contains("A"){
        a = "1";
    }
    if dest.contains("D"){
        d = "1";
    }
    if dest.contains("M"){
        m = "1";
    }
    return a.to_owned()+d+m;
}

fn parse_jump(jump: String) -> String {
    return match jump.as_str(){ 
        "JGT"=> "001".to_string(),
        "JEQ"=> "010".to_string(),
        "JGE"=> "011".to_string(),
        "JLT"=> "100".to_string(),
        "JNE"=> "101".to_string(),
        "JLE"=> "110".to_string(),
        "JMP"=> "111".to_string(),
        _=>"000".to_string(),
    };
}
