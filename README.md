# Arbitrum Node Setup Guide

Este documento contiene los pasos y comandos utilizados para configurar un nodo de Arbitrum, incluyendo la instalación de Rust, Docker, y `cargo-stylus`, así como la solución de problemas encontrados durante el proceso.

## Requisitos Previos

1. **Sistema operativo:** Ubuntu en un entorno virtual dentro de Windows.
2. **Virtualización habilitada:** Asegúrate de que la virtualización esté activada en tu BIOS.
3. **Docker instalado y funcionando:** Docker será necesario para ejecutar el nodo de Arbitrum.

## Pasos para Configurar el Nodo

### **1. Configurar Docker**

Primero, instala Docker y asegúrate de que está funcionando correctamente:

```bash
sudo apt update
sudo apt install docker.io
sudo systemctl start docker
sudo systemctl enable docker
```

Verifica la instalación de Docker:

```bash
docker --version
```

### **2. Instalar Rust**

Rust es necesario para compilar y utilizar herramientas relacionadas con el nodo de Arbitrum. Sigue estos pasos para instalarlo:

#### **2.1 Instalar Rust usando Rustup**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Sigue las instrucciones en pantalla y luego reinicia tu terminal o ejecuta:

```bash

```

Verifica la instalación:

```bash
rustc --version
```

#### **2.2 Actualizar Rust**

En caso de necesitar una versión específica (como 1.81.0):

```bash
rustup install 1.81.0
rustup default 1.81.0
```

Confirma que la versión es correcta:

```bash
rustc --version
```

### **3. Instalar Dependencias de Sistema**

Antes de instalar `cargo-stylus`, asegúrate de tener instaladas las herramientas necesarias:

```bash
sudo apt update
sudo apt install pkg-config libssl-dev
```

### **4. Instalar `cargo-stylus`**

Instala `cargo-stylus` con el siguiente comando:

```bash
cargo install cargo-stylus
```

Verifica la instalación:

```bash
cargo stylus --version
```

### **5. Configurar el Nodo de Arbitrum**

Usa Docker para descargar y ejecutar la imagen del nodo de Arbitrum:

#### **5.1 Descargar la imagen del nodo**

Ya se puede continuar con la referencia del [GitHub Original](https://github.com/OffchainLabs/cargo-stylus)

```bash
docker pull offchainlabs/arbitrum
```

#### **5.2 Ejecutar el contenedor del nodo**

```bash
docker run -d -p 8547:8547 offchainlabs/arbitrum
```

Este comando ejecutará el nodo en el puerto 8547.

### **6. Verificar el Nodo**

Para confirmar que el nodo está funcionando, realiza una solicitud al endpoint:

```bash
curl http://localhost:8547
```

Si recibes una respuesta válida, tu nodo está funcionando correctamente.

## Solución de Problemas

### **Error: pkg-config not found**

Si encuentras un error relacionado con `pkg-config`, instala la herramienta:

```bash
sudo apt install pkg-config
```

### **Error: OpenSSL not found**

Si falta OpenSSL o sus dependencias de desarrollo:

```bash
sudo apt install libssl-dev
```

### **Rutas incorrectas en Rust**

Si hay problemas con versiones de Rust o rutas incorrectas, asegúrate de que `$HOME/.cargo/bin` esté en tu variable `PATH`:

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```
