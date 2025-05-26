use core::f64;
use std::io;

fn main() {
    let mut cuenta_uno = crear_cuenta(String::from("Alex"), 4000.0);
    let mut cuenta_dos = crear_cuenta(String::from("Jose"), 1200.0);
    let mut cuenta_tres = crear_cuenta(String::from("NikBok"), 50_000.0);

    let mut accounts = Box::new([&mut cuenta_uno, &mut cuenta_dos, &mut cuenta_tres]);

    'account_select: loop {
        mostrar_cuentas(&accounts);
        println!("Selecciona una cuenta:");
        let mut user_choice: String = String::new();

        io::stdin()
            .read_line(&mut user_choice)
            .expect("Algo salió mal");

        let mut selected_account: usize = 0;
        match user_choice.trim() {
            "1" => {
                selected_account = 0;
                println!("{}'s Account", &accounts[0].0);
            }
            "2" => {
                selected_account = 1;
                println!("{}'s Account", &accounts[1].0);
            }
            "3" => {
                selected_account = 2;
                println!("{}'s Account", &accounts[2].0);
            }
            _ => {
                println!("Please, input a correct account...");
                continue 'account_select;
            }
        }
        'options: loop {
            println!("************************");
            menu();
            println!("************************");
            println!("Select an option:");
            let mut user_choice: String = String::new();

            io::stdin()
                .read_line(&mut user_choice)
                .expect("Algo salió mal...");

            match user_choice.trim() {
                "1" => {
                    let mut cantidad_retiro_str: String = String::new();
                    io::stdin()
                        .read_line(&mut cantidad_retiro_str)
                        .expect("Algo salió mal");

                    let cantidad: f64 =
                        cantidad_retiro_str.trim().parse().expect("Algo salió mal.");
                    depositar(&mut accounts[selected_account], cantidad);
                }
                "2" => {
                    let operation = retirar(&mut accounts[selected_account]);
                    if operation.0 {
                        println!("Operación realizada.");
                    } else {
                        println!("No hay suficiente saldo.");
                    }
                }
                "3" => {
                    transferir(&mut accounts, selected_account);
                }
                "4" => {
                    mostrar_cuenta(&accounts[selected_account]);
                }
                "5" => {
                    let clasificacion = clasificar_cuenta(&accounts[selected_account]);
                    println!("{clasificacion}");
                }
                "6" => {
                    break 'options;
                }
                "7" => {
                    break 'account_select;
                }
                _ => {
                    continue;
                }
            }
        }
    }
}

fn crear_cuenta(name: String, money: f64) -> (String, f64, u32) {
    (name, money, 0)
}

fn retirar(cuenta: &mut (String, f64, u32)) -> (bool, f64) {
    println!("Introduce la cantidad a retirar:");

    let mut cantidad_str: String = String::new();

    io::stdin()
        .read_line(&mut cantidad_str)
        .expect("Error al leer cantidad");

    let cantidad: f64 = cantidad_str.trim().parse().expect("Se esperaba número!");

    if cantidad <= cuenta.1 {
        cuenta.1 -= cantidad;
        cuenta.2 += 1;
        (true, cantidad)
    } else {
        (false, 0.0)
    }
}

fn depositar(cuenta: &mut (String, f64, u32), cantidad: f64) {
    cuenta.1 += cantidad;
    cuenta.2 += 1;
}

fn mostrar_cuenta(cuenta: &(String, f64, u32)) {
    println!("Resumen de la cuenta:");
    println!("Titular    : {}", cuenta.0);
    println!("Saldo      : {:.2}", cuenta.1);
    println!("Movimientos: {}", cuenta.2);
    println!("- - - - - - - - - - - -");
}

fn clasificar_cuenta(cuenta: &(String, f64, u32)) -> &str {
    match cuenta.1 {
        saldo if saldo > 10_000.0 => "VIP",
        saldo if saldo > 5_000.0 => "Cliente Premium",
        saldo if saldo > 1_000.0 => "Cliente Estándar",
        _ => "Básica",
    }
}

fn mostrar_cuentas(cuentas: &[&mut (String, f64, u32); 3]) {
    let mut indice: u32 = 1;
    for i in cuentas {
        print!("{indice} ");
        mostrar_cuenta(i);
        indice += 1;
    }
}

fn transferir(cuentas: &mut [&mut (String, f64, u32); 3], actual_account: usize) {
    println!("Selecciona la cuenta de destino:");
    mostrar_cuentas(cuentas);

    let mut cuenta_seleccionada: String = String::new();
    io::stdin()
        .read_line(&mut cuenta_seleccionada)
        .expect("Algo salió mal.");

    let cuenta_seleccionada: usize = cuenta_seleccionada.trim().parse().expect("Algo salió mal");

    if cuenta_seleccionada - 1 == actual_account {
        println!("No puedes transferir a tu propia cuenta...");
    } else if cuenta_seleccionada > cuentas.len() {
        println!("Cuenta incorrecta...");
    } else {
        let response = retirar(&mut cuentas[actual_account]);
        if response.0 {
            depositar(&mut cuentas[cuenta_seleccionada - 1], response.1);
            println!("Transferencia realizada con éxito.");
        } else {
            println!("Saldo insuficiente...");
        }
    }
}

fn menu() {
    println!();
    println!("1. Depositar");
    println!("2. Retirar");
    println!("3. Transferir");
    println!("4. Mostrar cuenta");
    println!("5. Clasificar cuenta");
    println!("6. Change Account");
    println!("7. Exit");
}
