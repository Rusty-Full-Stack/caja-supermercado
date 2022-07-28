use std::io::stdin;
use caja_supermercado::pago::{MetodoDePago, ResultadoPago};
use caja_supermercado::compra::{Item, agregar_item, quitar_item, mostrar_items, total_compra, pagar_compra};

fn mostrar_menu() {
    println!("OPCIONES:");
    println!("1. Agregar Item");
    println!("2. Quitar Item");
    println!("3. Mostrar Items");
    println!("4. Total a Pagar");
    println!("5. Realizar Pago");
    println!("6. Cancelar Compra y Salir");
    println!("Selecciona una opcion");
}

fn manejar_agregar_item(items_compra: &mut Vec<Item>) {
    // Solicitando que el usuario registre los datos del item
    // por facilidad no hemos colocado validaciones, en un ambiente
    // laboral SI debes colocarlas.
    // Quiza no es el input mas optimo del planeta pero sirve a manera
    // de ejemplo de uso de modulos
    let mut input: String = String::new();
    println!("Escribe los detalles del Item");

    println!("NOMBRE:");
    stdin().read_line(&mut input).unwrap();
    let nombre = input.replace("\n", "").replace("\r", "");

    println!("CANTIDAD:");
    input = String::new();
    stdin().read_line(&mut input).unwrap();
    let cantidad = input.replace("\n", "").replace("\r", "").parse::<f32>().unwrap();

    println!("PRECIO UNITARIO: ");
    input = String::new();
    stdin().read_line(&mut input).unwrap();
    let precio_unitario = input.replace("\n", "").replace("\r", "").parse::<f32>().unwrap();

    // Creando el item con la estructura que importamos de nuestro modulo
    let item: Item = Item {
        nombre,
        precio_unitario,
        cantidad
    };

    // Agregando el item a la compra
    agregar_item(items_compra, item);

    println!("Item Agregado!");
}

fn manejar_quitar_item(items_compra: &mut Vec<Item>) {

    // mostrando los items par que el usuario pueda saber cual quitar
    // REUTILIZANDO NUESTRO MODULO !!!
    println!("Selecciona el indice que quieres quitar");
    mostrar_items(items_compra);

    // Obteniendo el itema a eliminar
    let mut input: String = String::new();
    stdin().read_line(&mut input).unwrap();
    // limpiando el input de la terminal
    let indice = input.replace("\n", "").replace("\r", "").parse::<usize>().unwrap();

    // Eliminando el item utilizando la funcion dentro del modulo de compra
    quitar_item(items_compra, indice);
    println!("Item eliminado");
}

fn manejar_realizar_pago(items_compra: &mut Vec<Item>) {

    let monto_a_pagar = total_compra(items_compra);

    println!("Monto a Pagar: ${}", monto_a_pagar);
    println!("Selecciona el metodo de pago.");
    println!("1. En Efectivo");
    println!("2. Tarjeta");
    println!("3. Transferencia Bancaria");

    let mut recibido_del_cliente = 0.0;
    let mut tarjeta = String::from("N/A");

    // Obtenemos la opcion que selecciona el usuario
    let mut opcion: String = String::new();
    stdin().read_line(&mut opcion).unwrap();
    // limpiando el input de la terminal
    let opcion_seleccionada = opcion.replace("\n", "").replace("\r", "").parse::<usize>().unwrap();

    let metodo_de_pago = match opcion_seleccionada {
        1 => MetodoDePago::Efectivo,
        2 => MetodoDePago::Tarjeta,
        3 => MetodoDePago::TransferenciaBancaria,
        _ => MetodoDePago::Efectivo // por facilidad
    };

    if opcion_seleccionada == 1 {
        // El metodo de pago es efectivo, necesitamos saber cuanto recibimos del cliente
        println!("Monto Recibido del Cliente:");
        let mut recibido: String = String::new();
        stdin().read_line(&mut recibido).unwrap();
        recibido_del_cliente = recibido.replace("\n", "").replace("\r", "").parse::<f32>().unwrap();
    }

    if opcion_seleccionada == 2 {
        println!("Num. De Tarjeta:");
        // El metodo de pago es con tarjeta, necesitamos saber el numero
        let mut input: String = String::new();
        stdin().read_line(&mut input).unwrap();
        tarjeta = input.replace("\n", "").replace("\r", "");
    }

    let resultado_del_pago: ResultadoPago = pagar_compra(metodo_de_pago, monto_a_pagar, recibido_del_cliente, &tarjeta);

    if resultado_del_pago.fue_exitoso {
        println!("El pago fue exitoso");
        println!("Metodo de Pago: {}", resultado_del_pago.metodo_pago);
        println!("Cambio: ${}", resultado_del_pago.cambio);
    } else {
        println!("Hubo un problema procesando el pago, intentalo de nuevo");
    }

}

fn main() {
    // Creamos un vector para llevar el registro de los items de la compra
    let mut items_compra: Vec<Item> = Vec::new();

    // Iniciamos un loop en el cual vamos a preguntar al usuario la accion a realizar
    // Dependiendo de sus seeleccion vamos a realizar una tarea, todas ellas dependeran
    // de funciones dentro del modulo "compra"
    loop {
        mostrar_menu();

        // Obtenemos la opcion que selecciona el usuario
        let mut opcion: String = String::new();
        stdin().read_line(&mut opcion).unwrap();
        // limpiando el input de la terminal
        let opcion_seleccionada = opcion.replace("\n", "").replace("\r", "").parse::<usize>().unwrap();

        match opcion_seleccionada {
            1 => manejar_agregar_item(&mut items_compra), // Agregar un item
            2 => manejar_quitar_item(&mut items_compra), // quitar un item
            3 => mostrar_items(&items_compra), // mostrar todos los items y sus indices
            4 => println!("Total a pagar: ${}", total_compra(&items_compra)), // mostrando el total a pagar
            5 => manejar_realizar_pago(&mut items_compra), // realizar el pago
            6 => break, // terminando el programa
            _ => println!("Opcion Invalida") // la opcion no es valida, el programa continua
        };
    }

    println!("Programa Finalizado");
}
