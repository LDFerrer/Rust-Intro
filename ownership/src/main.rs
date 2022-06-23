fn main() {
    let x = 1;
    //  Si declaramos una variable, su ámbito será en su bloque y descendente.
    let a = 5;

    //  Al entrar en un bloque, se crea un nuevo ámbito.
    if x == 1 {
        /*  Podemos decidir la forma de asignar valores a la nueva variable.
            let a = a;      //  Copiar en la nueva declaración, el valor superior.
            let a = 10;     //  Asignar en la nueva declaración, el valor deseado.
            a = 10;         //  Solo podemos modificar variables mutables, esto fallará al compilar.
        */
        let a = 10;
        println!( "A = {}", a );    //  Imprimimos el valor establecido.
    }

    println!( "A = {}", a );        //  Imprimimos el valor del ambito superior.
}
