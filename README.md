O Antartica OS é um Sistema Operacional ainda em Desenvolvimento, criado pelo Grupo GNU/Linux Terminators no Telegram

Grupo: https://t.me/GNULinux_Terminators

Para compilar é necessário ter os seguintes programas:

nasm, qemu, xorriso build-essential rust xargo

Para instalar os mesmos execute os seguintes comandos (Debian e Derivados):

    $ sudo apt-get install nasm xorriso qemu build-essential

(Arch Linux e Derivados):

    $ sudo pacman -S --needed binutils grub mtools libisoburn nasm qemu

(Fedora):

    $ sudo dnf install nasm xorriso qemu

Para instalar a Linguagem rust execute o seguinte comando:

    $ curl https://sh.rustup.rs -sSf | sh

Para instalar o xargo execute o seguinte comando:

    $ cargo install xargo

(o cargo já vem com a linguagem rust)

Para instalar o toolchain usado pelo projeto (especificado no arquivo
`rust-toolchain`), execute:

    $ rustup install nightly-2017-09-22
	$ rustup component add rust-src

Para compilar o projeto execute o seguinte comando:

    $ make iso

Se você tiver o qemu, você pode compilar e rodar:

    $ make run
