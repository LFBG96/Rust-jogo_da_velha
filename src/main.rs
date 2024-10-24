use std::io;
use std::process::Command;

fn criar_matriz() -> Vec<Vec<String>> {
    let matriz = vec![vec!["   ".to_string(); 3]; 3];
    println!("       C0     C1     C2");
    for c in 0..matriz.len() {
        println!("L{}  {:?}", c, matriz[c])
    }
    matriz
}

fn input() -> usize {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error ao ler input");
        let input = input.trim();

        match input.parse::<usize>() {
            Ok(valor) if valor <= 2 => return valor,
            _ => println!("Entrada inválida"),
        }
    }
}

fn imprimir_tabela(matriz: &Vec<Vec<String>>) -> &Vec<Vec<String>> {
    println!("       C0     C1     C2");
    for c in 0..matriz.len() {
        println!("L{}  {:?}", c, matriz[c])
    }

    matriz
}

fn verificar_jogada(coluna: usize, linha: usize, matriz: &Vec<Vec<String>>) -> bool {
    for _v in matriz {
        if matriz[linha][coluna] == "   " {
            return true;
        } else {
            println!("Local ocupado, joque novamente.");
            //break;
            return false;
        }
    }

    false
}

fn x_jogada(coluna: usize, linha: usize, matriz: &mut Vec<Vec<String>>) -> &Vec<Vec<String>> {
    if verificar_jogada(coluna, linha, matriz) == true {
        matriz[linha][coluna] = " X ".to_string();
    } else {
        loop {
            println!("Digite o valor da coluna [0..2]");
            let coluna = input();
            println!("Digite o valor da linha [0..2]");
            let linha = input();

            if verificar_jogada(coluna, linha, matriz) == true {
                matriz[linha][coluna] = " X ".to_string();
                break;
            }
        }
    }
    limpar_tela();
    imprimir_tabela(matriz);
    matriz
}
fn o_jogada(coluna: usize, linha: usize, matriz: &mut Vec<Vec<String>>) -> &Vec<Vec<String>> {
    if verificar_jogada(coluna, linha, matriz) == true {
        matriz[linha][coluna] = " O ".to_string();
    } else {
        loop {
            println!("Digite o valor da coluna [0..2]");
            let coluna = input();
            println!("Digite o valor da linha [0..2]");
            let linha = input();

            if verificar_jogada(coluna, linha, matriz) == true {
                matriz[linha][coluna] = " O ".to_string();
                break;
            }
        }
    }
    limpar_tela();
    imprimir_tabela(matriz);
    matriz
}

fn verificar_vitoria(matriz: &Vec<Vec<String>>) -> Option<String> {
    let tamanho = matriz.len();

    //verificar as linhas

    for linha in 0..tamanho {
        if matriz[linha][0] == matriz[linha][1]
            && matriz[linha][1] == matriz[linha][2]
            && matriz[linha][0] != "   "
        {
            return Some(matriz[linha][0].clone());
        }
    }

    //verificar colunas

    for linha in 0..tamanho {
        if matriz[0][linha] == matriz[1][linha]
            && matriz[1][linha] == matriz[2][linha]
            && matriz[1][linha] != "   "
        {
            return Some(matriz[0][linha].clone());
        }
    }

    //verificar diagonal principal

    if matriz[0][0] == matriz[1][1] && matriz[1][1] == matriz[2][2] && matriz[0][0] != "   " {
        return Some(matriz[0][0].clone());
    }

    //verficar diagonal secundaria

    if matriz[0][2] == matriz[1][1] && matriz[1][1] == matriz[2][0] && matriz[0][2] != "   " {
        return Some(matriz[0][2].clone());
    }

    None
}

fn limpar_tela() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Erro ao executar comando CLS");
    } else {
        Command::new("clear")
            .status()
            .expect("Erro ao executar comando clear ");
    }
}

fn jogo_da_velha() {
    println!("{:-^40}", "JOGO DA VELHA");

    println!("PRIMEIRA JOGADA JOGADOR [X]\n");
    let mut matriz = criar_matriz();

    loop {
        if let Some(vencedor) = verificar_vitoria(&matriz) {
            if vencedor.contains("X") | vencedor.contains("O") {
                println!("O vencedor foi o {}", vencedor);
                break;
            }
        }

        //vez do jogador X
        println!("VEZ DO JOGADOR [X]");
        println!("Digite o valor da coluna [0..2]");
        let coluna = input();
        println!("Digite o valor da linha [0..2]");
        let linha = input();

        x_jogada(coluna, linha, &mut matriz);

        if let Some(vencedor) = verificar_vitoria(&matriz) {
            if vencedor.contains("X") | vencedor.contains("O") {
                println!("O vencedor foi o {}", vencedor);
                break;
            }
        }

        //vez do jogador O
        println!("VEZ DO JOGADOR [O]");
        println!("Digite o valor da coluna [0..2]");
        let coluna = input();
        println!("Digite o valor da linha [0..2]");
        let linha = input();

        o_jogada(coluna, linha, &mut matriz);
    }
}

fn main() {
    loop {
        limpar_tela();
        jogo_da_velha();
        println!("Digite [ 0 ] para fechar, [ 1 ] para continuar");
        let fechar_jogo = input();
        if fechar_jogo == 0 {
            break;
        }
    }
}
