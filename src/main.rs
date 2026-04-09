// devvo refatorar o codigo para q os newtypes saibam se converter para os outros tipos com impl inerente
//
// devo pesquisar como fazer tratamento de erros mais personalizados
use std::io::{self, Write};

struct Celsius(f32);
struct Fahrenheit(f32);
struct Kelvin(f32);

struct Km(f32);
struct M(f32);
struct Cm(f32);
struct Mm(f32);

fn input(p: &str) -> io::Result<String> {
    println!("{p}");
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}


enum Tipos {
    Temperatura(AcaoTemperatura),
    Distancia(AcaoDistancia),
}

enum AcaoDistancia {
    Km(Km),
    M(M),
    Cm(Cm),
    Mm(Mm),
}

enum AcaoTemperatura {
    C(Celsius),
    F(Fahrenheit),
    K(Kelvin),
}

fn inicio_principal() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("-----Escolha oque fazer-----");
        println!("[1] Temperatura.");
        println!("[2] Distancia.");
        println!("[3] Sair.");
        let escolha = input("R: ")?;
        match escolha.as_str() {
            "1" => inicio_temperatura()?,
            "2" => inicio_distancia()?,
            "3" => break,
            _ => {
                println!("Opção invalida");
            }
        }
    }
    Ok(())
}

fn inicio_temperatura() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("-----Escolha o tipo para converter para outros-----");
        println!("[1] Celsius.");
        println!("[2] Fahrenheit.");
        println!("[3] Kelvin.");
        println!("[4] Sair.");
        let escolha = input("R: ")?;
        match escolha.as_str() {
            "1" => {
                let n = input("Digite a temperatura em C: ")?;
                match n.parse::<f32>() {
                    Ok(num) => {
                        let celsius = Celsius(num);
                        let tipo = Tipos::Temperatura(AcaoTemperatura::C(celsius));
                        conversao(tipo)?;
                    }
                    Err(_e) => {
                        println!("Erro ao converter!");
                    }
                }
            }
            "2" => {
                let n = input("Digite a temperatura em F: ")?;
                match n.parse::<f32>() {
                    Ok(num) => {
                        let fahrenheit = Fahrenheit(num);
                        let tipo = Tipos::Temperatura(AcaoTemperatura::F(fahrenheit));
                        conversao(tipo)?;
                    }
                    Err(_e) => {
                        println!("Erro ao converter!")
                    }
                }
            }
            "3" => {
                let n = input("Digite a temperatura em K: ")?;
                match n.parse::<f32>() {
                    Ok(num) => {
                        let kelvin = Kelvin(num);
                        let tipo = Tipos::Temperatura(AcaoTemperatura::K(kelvin));
                        conversao(tipo)?;
                    }
                    Err(_e) => {
                        println!("Erro ao converter!")
                    }
                }
            }
            "4" => break,
            _ => {
                println!("Opção invalida");
            }
        }
    }
    Ok(())
}

fn inicio_distancia() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("-----Escolha o tipo para converter para outros-----");
        println!("[1] Km.");
        println!("[2] M.");
        println!("[3] Cm.");
        println!("[4] Mm.");
        println!("[5] Sair.");
        let escolha = input("R: ")?;
        match escolha.as_str() {
            "1" => {
                let n = input("Digite a unidade em Km: ")?;
                match n.parse::<f32>() {
                    Ok(num) => {
                        let km = Km(num);
                        let tipo = Tipos::Distancia(AcaoDistancia::Km(km));
                        conversao(tipo)?;
                    }
                    Err(_e) => {
                        println!("Erro ao converter!");
                    }
                }
            }
            "2" => {
                let n = input("Digite a unidade em M: ")?;
                match n.parse::<f32>() {
                    Ok(num) => {
                        let m = M(num);
                        let tipo = Tipos::Distancia(AcaoDistancia::M(m));
                        conversao(tipo)?;
                    }
                    Err(_e) => {
                        println!("Erro ao converter!");
                    }
                }
            }
            "3" => {
                let n = input("Digite a unidade em Cm: ")?;
                match n.parse::<f32>() {
                    Ok(num) => {
                        let cm = Cm(num);
                        let tipo = Tipos::Distancia(AcaoDistancia::Cm(cm));
                        conversao(tipo)?;
                    }
                    Err(_e) => {
                        println!("Erro ao converter!");
                    }
                }
            }
            "4" => {
                let n = input("Digite a unidade em Mm: ")?;
                match n.parse::<f32>() {
                    Ok(num) => {
                        let mm = Mm(num);
                        let tipo = Tipos::Distancia(AcaoDistancia::Mm(mm));
                        conversao(tipo)?;
                    }
                    Err(_e) => {
                        println!("Erro ao converter!");
                    }
                }
            }
            "5" => {
                break;
            }
            _ => {
                println!("Resposta invalida");
            }
        }
    }
    Ok(())
}

fn conversao(n: Tipos) -> Result<(), Box<dyn std::error::Error>> {
    match n {
        Tipos::Temperatura(temp) => match temp {
            AcaoTemperatura::C(Celsius(v)) => {
                let nc: f32 = v;
                let f = (nc * 1.8) + 32.0;
                let k = nc + 273.15;
                println!("F: {}, K: {}", f, k);
            }
            AcaoTemperatura::F(Fahrenheit(v)) => {
                let nf: f32 = v;
                let c = (nf - 32.0) / 1.8;
                let k = (nf + 459.67) * 5.0 / 9.0;
                println!("C: {}, K: {}", c, k);
            }
            AcaoTemperatura::K(Kelvin(v)) => {
                let nk: f32 = v;
                let c = nk - 273.15;
                let f = (nk - 273.15) * 1.8 + 32.0;
                println!("C: {}, F: {}", c, f);
            }
        },
        Tipos::Distancia(dist) => match dist {
            AcaoDistancia::Km(Km(v)) => {
                let nkm: f32 = v;
                let m = nkm * 1000.0;
                let cm = m * 100.0;
                let mm = cm * 10.0;
                println!("M: {}, Cm: {}, Mm: {}", m, cm, mm);
            }
            AcaoDistancia::M(M(v)) => {
                let nm: f32 = v;
                let km = nm / 1000.0;
                let cm = nm * 10.0 * 10.0;
                let mm = cm * 10.0;
                println!("Km: {}, Cm: {}, Mm: {}", km, cm, mm);
            }
            AcaoDistancia::Cm(Cm(v)) => {
                let ncm: f32 = v;
                let m = ncm / 100.00;
                let km = m / 1000.0;
                let mm = ncm * 10.0;
                println!("km: {}, M: {}, Mm {}", km, m, mm);
            }
            AcaoDistancia::Mm(Mm(v)) => {
                let nmm: f32 = v;
                let km = nmm / 1_000_000.0;
                let m = km * 1000.0;
                let cm = nmm / 10.0;
                println!("Km: {}, M: {}, Cm: {}", km, m, cm);
            }
        },
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    inicio_principal()?;
    Ok(())
}
