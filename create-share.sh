#!/bin/sh -ex

truncate --size=100G "${HOME}/share.img"
SHARE=$(sudo losetup --show --find "${HOME}/share.img")
sudo mkfs.hfsplus -s -v share "${SHARE}"
VBoxManage internalcommands createrawvmdk -filename "${HOME}/share.vmdk" -rawdisk "${HOME}/share.img"
VBoxManage storageattach "Snow Leopard" --storagectl "SATA" --port 3 --type hdd --medium "${HOME}/share.vmdk"
