
# Challengue PREX: Mini Procesador de Pagos en Rust


Este proyecto implementa un mini procesador de pagos desarrollado en Rust utilizando el framework Actix-web.

## Instalación y Ejecución

1. Clonar el Repositorio: Para comenzar, clona este repositorio utilizando el siguiente comando:
```bash
  -git clone https://github.com/flor-salva/prexCoreDevChallenge
```
2. Navegar al Directorio del Proyecto: Accede al directorio del proyecto

3. Compilar y Ejecutar el Proyecto: Compila y ejecuta el proyecto utilizando Cargo, el administrador de paquetes de Rust:
```bash
  cargo run
```
Esto iniciará el servidor Actix-web en http://localhost:8080.

## Uso
Una vez que el servidor esté en funcionamiento, puedes interactuar con la API REST utilizando herramientas como cURL, Postman o escribiendo scripts en Rust.

## Endpoints disponibles:

```bash
  POST /new_client
```
Crea un nuevo cliente con la información proporcionada a través del body del request.

```bash
  POST /new_credit_transaction
```
Realiza una transacción de crédito para un cliente existente la cual se envía a través del body del request.

```bash
  POST /new_debit_transaction
```
Realiza una transacción de débito para un cliente existente la cual se envía a través del body del request.

```bash
  POST /store_balances
```
Persiste los saldos de los clientes en un archivo.

```bash
  GET /client_balance/{client_id}
```
Obtiene el saldo de un cliente por su ID.

## Importante
Dentro del proyecto, encontrarás un archivo JSON en Postman que puedes importar para probar las solicitudes. Consulta la documentación de Postman para obtener instrucciones sobre cómo importar y utilizar este archivo.

### ¡Gracias por utilizar el mini procesador de pagos en Rust!


## Autora

- [flor-salva](https://github.com/flor-salva)

