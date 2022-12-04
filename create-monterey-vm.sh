#!/bin/sh -ex

VM_NAME="Monterey"

VBoxManage createvm --ostype MacOS1013_64 --default --name "${VM_NAME}" --register

VBoxManage modifyvm "${VM_NAME}" \
    --memory 8192 \
    --vram 128 \
    --rtcuseutc off \
    --usbxhci on \
    --cpuidset 00000001 000106e5 00100800 0098e3fd bfebfbff \
    --cpu-profile "Intel Core i7-6700K"

VBoxManage modifyvm "${VM_NAME}" --nic2 intnet --intnet2 macnet

VBoxManage createmedium  --size 204800 --filename "${HOME}/VirtualBox VMs/${VM_NAME}/os.vdi" --format VDI
VBoxManage storageattach "${VM_NAME}" --storagectl "SATA" --port 0 --type hdd --medium "${HOME}/VirtualBox VMs/${VM_NAME}/os.vdi"
VBoxManage storageattach "${VM_NAME}" --storagectl "SATA" --port 1 --type dvddrive --medium emptydrive
VBoxManage storageattach "${VM_NAME}" --storagectl "SATA" --port 1 --type dvddrive --medium "${HOME}/Downloads/Monterey.iso"

VBoxManage setextradata "${VM_NAME}" "VBoxInternal/Devices/efi/0/Config/DmiSystemProduct" "iMac11,3"
VBoxManage setextradata "${VM_NAME}" "VBoxInternal/Devices/efi/0/Config/DmiSystemVersion" "1.0"
VBoxManage setextradata "${VM_NAME}" "VBoxInternal/Devices/efi/0/Config/DmiBoardProduct" "Iloveapple"
VBoxManage setextradata "${VM_NAME}" "VBoxInternal/Devices/smc/0/Config/DeviceKey" "ourhardworkbythesewordsguardedpleasedontsteal(c)AppleComputerInc"
VBoxManage setextradata "${VM_NAME}" "VBoxInternal/Devices/smc/0/Config/GetKeyFromRealSMC" 1
VBoxManage setextradata "${VM_NAME}" "VBoxInternal2/EfiGraphicsResolution" "1280x720"