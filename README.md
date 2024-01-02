# HashMap en Rust

Esta app es para práctica del uso del HashMap en Rust.
El HashMap es otro tipo de coleccion, al igual que un Array o un Vector pero con la diferencia de
que éste, nos permite asociar una clave al elemento almacenado en lugar de asignarle una posicion,
esto lo hace mediante una funcion de hashing la cual determina como se almacena las claves y
datos en la memoria, por lo general la forma de almacenarlo sera:

HashMap<K, V>

Donde 'K' es el key (clave) y 'V' es el value (valor).

#

En cuanto a los archivos app.rs, exercise.rs, exercise02.rs:

- app.rs: Explicación del HashMap, inserción y eliminación de valores.

- exercise.rs: Aplicación de HashMap con un ejercicio que simula una fábrica de automóviles.
En el mismo se realizan los pedidos (órdenes) una por una, así como se procede al mapeo de
los mismas y posterior printing por pantalla (consola).

- exercise02.rs: Continuación del ejercicio anterior, pero aplicando bucles (while), con la finalidad
de optimizar los pedidos (y por supuesto el código también). Esto se puede apreciar al hacer una
comparación entre ambos códigos.
