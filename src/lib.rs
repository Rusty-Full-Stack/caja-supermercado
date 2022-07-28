pub mod pago {

    pub enum MetodoDePago {
        Efectivo,
        Tarjeta,
        TransferenciaBancaria,
    }

    pub struct ResultadoPago {
        pub metodo_pago: String, // La descripcion del metodo de pago
        pub fue_exitoso: bool, // true si el pago se pudo hacer o false si no se pudo hacer
        pub cambio: f32, // Cambio a devolver al cliente.
    }

    pub fn pagar(metodo_de_pago: MetodoDePago, monto_a_pagar: f32, recibido_del_cliente: f32, tarjeta: &str) -> ResultadoPago{
        // El parametro metodo_de_pago es la forma de pago elegida por el cliente.
        // El parametro monto_a_pagar es el total a pagar de la compra.
        // recibido_del_cliente es la cantidad de dinero recibida del cliente, si no es efectivo, es igual al monto a pagar
        // tarjeta, es el numero de tarjeta del cliente, si el pago es en efectivo o por transferencia, no es necesario, puede ser cualquiera

        // Ahora, dependiendo del metodo de pago elegido por el cliente, invocamos las funciones privadas, esto puede hacerse
        // porque estan dentro del mismo alcance de este metodo.
        let resultado = match metodo_de_pago {
            MetodoDePago::Efectivo => pago_en_efectivo(monto_a_pagar, recibido_del_cliente),
            MetodoDePago::Tarjeta => pago_con_tarjeta(monto_a_pagar, tarjeta),
            MetodoDePago::TransferenciaBancaria => pago_por_transferencia_bancaria(monto_a_pagar)
        };

        resultado
    }

    fn pago_en_efectivo(monto_a_pagar: f32, recibido_del_cliente: f32) -> ResultadoPago {
        // Si el pago es en efectivo, se calculara el cambio a devolver al cliente
        let mut cambio = recibido_del_cliente - monto_a_pagar;
        // redondeando a dos decimales
        let y = 10i32.pow(2) as f32;
        cambio = (cambio * y).round() / y;

        ResultadoPago {
            metodo_pago: String::from("En Efectivo"),
            fue_exitoso: true,
            cambio
        }
    }

    fn pago_con_tarjeta(monto_a_pagar: f32, numero_tarjeta: &str) -> ResultadoPago {
        // Si el pago es con tarjeta, simularemos el resultado

        // Aca se estaria procesando todo aquello critico a nivel de seguridad.
        println!("Realizando el pago con el servicio de tarjeta credito/debito");
        println!("Monto a pagar: {}", monto_a_pagar);
        println!("Tarjeta: {}", numero_tarjeta);

        //Ahora simulamos el resultado
        ResultadoPago {
            metodo_pago: String::from("Tarjeta"),
            fue_exitoso: true,
            cambio: 0.0
        }

    }

    fn pago_por_transferencia_bancaria(monto_a_pagar: f32) -> ResultadoPago {
        // Si el pago es via transferencia, simularemos que solamente necesitamos la cuenta del supermercado
        // la cual seria la cuenta a recibir el dinero y tambien simularemos el resultado de la transferencia
        // Esta cuenta supondriamos que es secreta

        // Aca se estaria procesando todo aquello critico a nivel de seguridad.
        println!("Realizando las conexiones y transferencias con el banco");
        println!("Monto a pagar: {}", monto_a_pagar);
        println!("Cuenta Bancaria Secreta (o no tanto jejeej): 123456789-0");

        //Ahora simulamos el resultado
        ResultadoPago {
            metodo_pago: String::from("Transferencia Bancaria"),
            fue_exitoso: true,
            cambio: 0.0
        }
    }
}

pub mod compra {
    use crate::pago;

    #[derive(Debug)]
    pub struct Item {
        pub nombre: String, // Nombre del item
        pub precio_unitario: f32, // Precio Unitario del item
        pub cantidad: f32, // Cantidad a comprar del item, es float porque pueden ser fracciones de unidades, como kilos
    }

    pub fn agregar_item(items_compra: &mut Vec<Item>, item: Item) {
        // Agrega un item a un vector con todos los items de la compra
        items_compra.push(item);
    }

    pub fn quitar_item(items_compra: &mut Vec<Item>, indice: usize) {
        // Quitara un item del array a partir de un indice
        items_compra.remove(indice);
    }

    pub fn mostrar_items(items_compra: &Vec<Item>) {
        // Mostrando los items y el indice
        for (index, item) in items_compra.iter().enumerate() {
            let subtotal = item.cantidad * item.precio_unitario;
            println!("[{}]. {} - Cantidad: {} - Precio U: ${} - Subtotal: ${}", index, item.nombre, item.cantidad, item.precio_unitario, subtotal);
        }
    }

    pub fn total_compra(items_compra: &Vec<Item>) -> f32 {
        // Devolvera el totla a pagar de todos los items de la compra
        let mut total_compra: f32 = 0.0;
        for item in items_compra {
            total_compra = total_compra + (item.cantidad * item.precio_unitario);
        }

        // redondeando a dos decimales
        let y = 10i32.pow(2) as f32;
        total_compra = (total_compra * y).round() / y;

        total_compra
    }

    pub fn pagar_compra(metodo_de_pago: pago::MetodoDePago, monto_a_pagar: f32, recibido_del_cliente: f32, tarjeta: &str) -> pago::ResultadoPago{
        // El parametro metodo_de_pago es la forma de pago elegida por el cliente.
        // El parametro monto_a_pagar es el total a pagar de la compra.
        // recibido_del_cliente es la cantidad de dinero recibida del cliente, si no es efectivo, es igual al monto a pagar
        // tarjeta, es el numero de tarjeta del cliente, si el pago es en efectivo o por transferencia, no es necesario, puede ser cualquiera

        // Ahora, dependiendo del metodo de pago elegido por el cliente, invocamos las funciones privadas, esto puede hacerse
        // porque estan dentro del mismo alcance de este metodo.

        let resultado = pago::pagar(metodo_de_pago, monto_a_pagar, recibido_del_cliente, tarjeta);

        resultado
    }


}
