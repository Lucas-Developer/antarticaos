O Antartica OS é um Sistema Operacional ainda em Desenvolvimento, criado pelo Grupo GNU/Linux Terminators no Telegram

Para compilar é necessário ter os seguintes programas:

-Nasm
-qemu
-grub-mkrescue
-xorriso

Para compilar o projeto rode o seguinte comando:

make iso

Se quiser só compilar o kernel:

make kernel

Se você tiver o qemu, você pode compilar e rodar:

make run