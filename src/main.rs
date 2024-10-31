//Deve se usar Const quando deseja usar valores que não havera mudança e já sabe seu valor
// antes de buildar, exemplo: PI (OBS: Declarado de forma global)

const PI: f32 = 3.14;

static mut GLOBAL: u8 = 1;


fn main() {
    /*escopo();
    soma(10, 20);
    funcao_condicionais();
    exibir_tabuada();
    funcao_match_statement();
    // ownership();*/
    // pattern_matching();
    // erro();
    // erro_with_backtrace();
    erro_recuperal();
}

fn erro_recuperal() {
    match resultado() {
        Ok(s) => println!("Mensagem de sucesso: {}", s),
        Err(e) => println!("Mensagem de erro: {}", e)
    };
}

fn erro_with_backtrace() {
    // Ao rodar este código, ele dara um panic!, logo, irecuperavel
    // let v = vec![1, 2, 3];
    // v[99];
}

fn resultado() -> Result<String, u8> {
    // Ok("Mensagem concluído com sucesso")
    Err(42)

}

fn erro() {
    panic!("Erro proposital");
}

/*fn resultado() -> Result<String,u8>{

}*/

fn pattern_matching() {
    for i in 1..=15 {
        println!("{}: {}", i, match i {
            // O Match sempre vai considerar o primeiro Match e descartar o restante
            3 | 6 => "Pode ser 3 ou 6",
            4..=10 => "Entre 4 a 10",
            _ if i % 2 == 0 => "Par",
            _ if i % 2 != 0 => "Ímpar",
            _ => "Desconhecido",
        });
    }
}

fn ownership() {
    let mut palavra = String::from("Eduardo");
    rouba(&mut palavra);
    println!("{}", palavra);
    //Como variaveis do tipo string são armazenados dentro da memoria Heap, ela não contem o `copy`
    //dentro de sua estrutura, tendo a sua estrutura como ownership

}

fn rouba(palavra_roubada: &mut String) {
    palavra_roubada.push_str(" Hubner");
    println!("{}", palavra_roubada)
}

fn funcao_match_statement() {
    let linguagem = "PHP";
    let proposito = match linguagem {
        "PHP" => "WEB",
        "Python" => "Analise de dados",
        _ => "Desconhecido",
    };

    println!("A linguagem {} eh {}", linguagem, proposito);
}

fn exibir_tabuada() {
    let mut multiplicador: u8 = 8;
    let mut contador: u8 = 0;

    //using while
    while contador <= 10 {
        println!(" {} * {} = {}", multiplicador, contador, multiplicador * contador);
        contador += 1;
    }

    //using for
    multiplicador = 9;
    for i in 1..=10 {
        println!(" {} * {} = {}", multiplicador, i, multiplicador * i);
    }
}

fn funcao_condicionais() {
    let idade: u8 = 20;
    let pessoa_responsavel = true;

    let eh_maior = idade >= 18;

    if eh_maior || pessoa_responsavel {
        println!("pode entrar na balada");
    } else {
        println!("Não pode entrar na balada");
    }
    let condicao: &str;

    condicao = if eh_maior { "Maior" } else { "Menor" };
    println!("A pessoa é {} de idade", condicao);
}

fn escopo() {
    unsafe { println!("{}", GLOBAL); }

    let variavel: u8 = 128;
    println!("{}, esta variável tem {} bytes", variavel, std::mem::size_of_val(&variavel));

    let ponto_flutuante = 3.5;
    println!("{}", ponto_flutuante);

    let booleana: bool = false;
    println!("{}", booleana);

    println!("Valor de PI: {}", PI);

    let caracter: char = 'c';
    println!("{}", caracter);

    let palavra: &str = "Mundo";
    println!("{}", palavra);
}

fn soma(a: u8, b: u8) -> u8 {
    println!("soma {} + {} = {}", a, b, a + b);
    a + b
}