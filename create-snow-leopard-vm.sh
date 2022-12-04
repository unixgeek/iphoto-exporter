#!/bin/sh -ex

VM_NAME="Snow Leopard"
TIME_MACHINE="${1}"
WORK=$(mktemp -d)

VBoxManage createvm --ostype MacOS106 --default --name "${VM_NAME}" --register

VBoxManage modifyvm "${VM_NAME}" \
    --memory 4096 \
    --vram 128 \
    --cpus 2 \
    --firmware bios \
    --cpu-profile "Intel Core2 T7600 2.33GHz"

VBoxManage createmedium  --size 204800 --filename "${HOME}/VirtualBox VMs/${VM_NAME}/os.vdi" --format VDI
VBoxManage storageattach "${VM_NAME}" --storagectl "SATA" --port 0 --type hdd --medium "${HOME}/VirtualBox VMs/${VM_NAME}/os.vdi"
VBoxManage storageattach "${VM_NAME}" --storagectl "SATA" --port 1 --type dvddrive --medium "${HOME}/Downloads/iBoot.iso"
VBoxManage storageattach "${VM_NAME}" --storagectl "SATA" --port 2 --type dvddrive --medium "${HOME}/Downloads/Snow Leopard.iso"

VBoxManage modifyvm "${VM_NAME}" --nic2 intnet --intnet2 macnet

VBoxManage internalcommands createrawvmdk -filename "${WORK}/time_machine.vmdk" -rawdisk "${TIME_MACHINE}"
VBoxManage storageattach "${VM_NAME}" --storagectl "SATA" --port 3 --type hdd --medium "${WORK}/time_machine.vmdk"

truncate --size=1M "${WORK}/script.img"
SCRIPT_DISK=$(sudo losetup --show --find "${WORK}/script.img")
sudo mkfs.hfsplus "${SCRIPT_DISK}"
mkdir "${WORK}/script"
sudo mount "${SCRIPT_DISK}" "${WORK}/script"
sudo chmod 777 "${WORK}/script"
cat >> "${WORK}/script/fix.sh" <<EOF
sudo rm -fr /Volumes/OS/System/Library/Extensions/IOUSBFamily.kext /Volumes/OS/System/Library/Extensions/IOUSBMassStorageClass.kext
sudo cp -r /Volumes/Mac\ OS\ X\ Install\ DVD/System/Library/Extensions/IOUSBFamily.kext /Volumes/Mac\ OS\ X\ Install\ DVD/System/Library/Extensions/IOUSBMassStorageClass.kext/ /Volumes/OS/System/Library/Extensions
EOF
sudo umount "${WORK}/script"
sudo losetup -d "${SCRIPT_DISK}"
VBoxManage internalcommands createrawvmdk -filename "${WORK}/script.vmdk" -rawdisk "${WORK}/script.img"
VBoxManage storageattach "${VM_NAME}" --storagectl "SATA" --port 4 --type hdd --medium "${WORK}/script.vmdk"