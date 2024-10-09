use clap::{Arg, Command};
use rand::{Rng};
use std::fs::{OpenOptions, File};
use std::io::{Write, Read};
use std::path::Path;
use sha2::{Sha256, Digest};
use std::error::Error;

fn gerar_senha(tamanho: usize, usar_maiusculas: bool, usar_numeros: bool, usar_especiais: bool) -> String {
    let mut rng = rand::thread_rng();
    let mut caracteres: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    if usar_maiusculas {
        caracteres.extend("ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars());
    }
    if usar_numeros {
        caracteres.extend("0123456789".chars());
    }
    if usar_especiais {
        caracteres.extend("!@#$%^&*()-_+=<>?".chars());
    }

    let senha: String = (0..tamanho)
        .map(|_| {
            let idx = rng.gen_range(0..caracteres.len());
            caracteres[idx]
        })
        .collect();

    senha
}

fn hash_senha(senha: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(senha);
    format!("{:x}", hasher.finalize())
}

fn armazenar_senha(senha: &str, arquivo: &str) -> Result<(), Box<dyn Error>> {
    let hash = hash_senha(senha);
    let mut file = OpenOptions::new().append(true).create(true).open(arquivo)?;
    writeln!(file, "{}:{}", senha, hash)?;
    Ok(())
}

fn listar_senhas(arquivo: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(arquivo);
    if path.exists() {
        let mut file = File::open(arquivo)?;
        let mut conteudo = String::new();
        file.read_to_string(&mut conteudo)?;
        println!("Senhas armazenadas:");
        for linha in conteudo.lines() {
            let partes: Vec<&str> = linha.split(':').collect();
            if partes.len() == 2 {
                println!("Senha: {}, Hash: {}", partes[0], partes[1]);
            }
        }
    } else {
        println!("Nenhuma senha armazenada.");
    }
    Ok(())
}

fn remover_senha(senha: &str, arquivo: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(arquivo);
    if !path.exists() {
        return Err("Arquivo não encontrado.".into());
    }
    
    let mut senhas: Vec<String> = Vec::new();
    let mut file = File::open(arquivo)?;
    let mut conteudo = String::new();
    file.read_to_string(&mut conteudo)?;

    for linha in conteudo.lines() {
        if !linha.starts_with(senha) {
            senhas.push(linha.to_string());
        }
    }

    let mut file = File::create(arquivo)?;
    for s in senhas {
        writeln!(file, "{}", s)?;
    }

    Ok(())
}

fn exportar_senhas(arquivo: &str, destino: &str) -> Result<(), Box<dyn Error>> {
    std::fs::copy(arquivo, destino)?;
    println!("Senhas exportadas para {}", destino);
    Ok(())
}

fn main() {
    let matches = Command::new("Gerador de Senhas")
        .arg(Arg::new("tamanho")
            .short('t')
            .long("tamanho")
            .default_value("16")
            .help("Tamanho da senha"))
        .arg(Arg::new("maiusculas")
            .short('m')
            .action(clap::ArgAction::SetTrue)
            .help("Usar letras maiúsculas"))
        .arg(Arg::new("numeros")
            .short('n')
            .action(clap::ArgAction::SetTrue)
            .help("Usar números"))
        .arg(Arg::new("especiais")
            .short('e')
            .action(clap::ArgAction::SetTrue)
            .help("Usar caracteres especiais"))
        .arg(Arg::new("listar")
            .short('l')
            .action(clap::ArgAction::SetTrue)
            .help("Listar senhas armazenadas"))
        .arg(Arg::new("remover")
            .short('r')
            .value_name("SENHA")
            .help("Remover senha armazenada"))
        .arg(Arg::new("exportar")
            .short('x')
            .value_name("ARQUIVO")
            .help("Exportar senhas para um arquivo"))
        .get_matches();

    let tamanho: usize = matches.get_one::<String>("tamanho").unwrap().parse().unwrap_or(16);
    let usar_maiusculas = matches.get_flag("maiusculas");
    let usar_numeros = matches.get_flag("numeros");
    let usar_especiais = matches.get_flag("especiais");
    let arquivo = "senhas.txt";

    if matches.get_flag("listar") {
        if let Err(e) = listar_senhas(arquivo) {
            eprintln!("Erro ao listar senhas: {}", e);
        }
    } else if let Some(senha_para_remover) = matches.get_one::<String>("remover") {
        if let Err(e) = remover_senha(senha_para_remover, arquivo) {
            eprintln!("Erro ao remover senha: {}", e);
        } else {
            println!("Senha '{}' removida com sucesso.", senha_para_remover);
        }
    } else if let Some(arquivo_destino) = matches.get_one::<String>("exportar") {
        if let Err(e) = exportar_senhas(arquivo, arquivo_destino) {
            eprintln!("Erro ao exportar senhas: {}", e);
        }
    } else {
        let senha = gerar_senha(tamanho, usar_maiusculas, usar_numeros, usar_especiais);
        if let Err(e) = armazenar_senha(&senha, arquivo) {
            eprintln!("Erro ao armazenar senha: {}", e);
        } else {
            println!("Gerando senha com {} caracteres...", tamanho);
            println!("Senha gerada: {}", senha);
        }
    }
}

