pub fn blocos() {
    println!("Inicio do programa");
    const X: i32 = 5;
    let y = 6;
    let mut z = 7;
    z = z + 1;

    println!("No inicio os valores são: X={X}, y={y}, z={z}");

    {
        // bloco interno
        const X: i32 = 555;
        let y = 666;
        let mut z = 777;
        z = z + 1;
        println!("Dentro do bloco interno os valores são: X={X}, y={y}, z={z}");
    }

    println!("Depois do bloco interno os valores são: X={X}, y={y}, z={z}");
}

pub fn sombreamento() {
    println!("Inicio do programa");
    let x = 5;
    println!("O valor de x é: {x}");
    let x = x + 1; // x_novo = x_antigo + 1
    println!("O valor de x é: {x}");

    {
        let x = x * 2; // x_novo = x_antigo(novo) * 2
        println!("O valor de x no bloco interno é: {x}");
    }
    println!("O valor de x depois do bloco interno é: {x}");

    let spaces = "   "; // string literal
    let spaces = spaces.len(); // let cria nova variável com novo tipo, unsize = unsigned ou inteiro sem sinal
    println!("O valor de spaces é: {spaces}");

    let mut spaces2 = "   ";
    println!("O valor de spaces2 é: {spaces2}");
    spaces2 = "qwerty"; // mesma variável com o mesmo tipo
    println!("O valor de spaces2 é: {spaces2}");

    //spaces2 = 5; //spaces2.len();		// valor é mutável, o tipo não é mutável
}
