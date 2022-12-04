# Description
Very simple utility to export albums out of iPhoto, so they can be imported into the latest Photos app or other tool like
Google Photos. For my use case, I exported from iPhoto '08 (7.1.5) on Snow Leopard and imported to Photos 7.0 on Monterey.
Given the throw-away nature of the project, error handling is almost always going to be a panic. Leaving this here in
case it is useful to someone else.

# Virtual Machine Method
For test runs, I chose to use virtual machines using Virtual Box. The following is a guide if you would like to do
something similar.
## Monterey Setup
* Need ISO generated from an Apple device.
* https://techsviewer.com/how-to-create-macos-monterey-iso-image/
* https://techsviewer.com/install-macos-monterey-on-virtualbox-windows-pc/
* https://github.com/hkdb/VBoxMacSetup

### Virtual Machine Steps
1. `./create-monterey-vm`
2. Format disk as APFS (Case-sensitive) with OS as the name
3. Install OS
4. Check software update
5. System Preferences -> Mouse, uncheck Scroll direction: Natural
6. Remove Monterey ISO
7. Create Virtualbox Snapshot

## Snow Leopard Setup
* Need snow leopard.iso from internet archive
* Need iBoot.iso from https://www.tonymacx86.com/resources/iboot-3-3-0.38 (registration required)
* https://www.youtube.com/watch?v=b2fgOPvkmH8

### Virtual Machine Steps
1. `./create-snow-leopard-vm.sh /dev/sda # Where sda is the Time Machine disk`
2. Boot to iBoot
3. Attach Snow Leopard ISO
4. F5 in iBoot
5. Boot to Snow Leopard ISO
6. Format disk as Mac OS Extended case-sensitive with OS as the name
7. Install OS
8. Restore from Time Machine Backup
9. Boot to OS in Single User mode
10. `mount -u -o rw /`
11. `passwd ${ADMIN_USER}`
12. Exit Single User mode
13. Log in as ${ADMIN_USER}
14. Create test account with Admin
15. Log in to test account
16. Enable network sharing for Photos
17. Check software update (2x)
18. Run `/bin/sh /Volumes/untitled/fix.sh`
19. Power off
20. Remove Snow Leopard ISO
21. Remove Script disk
22. Remove Time Machine disk
23. Create Virtualbox Snapshot

## Shared Disk Setup

    ./create-share.sh

# Other Links
* https://www.fatcatsoftware.com/iplm/Help/iphoto%20library%20internals.html
* https://github.com/codez/ImportPhotoFolders
* https://theforensicscooter.com/2021/11/23/photos-sqlite-queries/
* https://simonwillison.net/2020/May/21/dogsheep-photos/
* https://developer.apple.com/documentation/photokit/browsing_and_modifying_photo_albums
* https://applehelpwriter.com/2015/04/15/how-to-easily-import-images-into-new-photos-app/
* https://discussions.apple.com/docs/DOC-8931
* https://github.com/patrikhson/photo-export/blob/master/Import-photos.applescript
* https://photosautomation.com/scripting/script-library-01.html
* https://exiftool.org/forum/index.php?PHPSESSID=2a5dd09160336fa79cb8c1ac67168dd5&topic=6591.0
* https://trac.macports.org/ticket/64146

# Usage
The export ignores Rolls (a.k.a Events) and the following album types: "Book", "Shelf", "Slideshow", "Special Roll". 
1. Run export in a "dry run" which will indicate what it will do.

       RUST_LOG=debug ./iphoto-exporter --dry-run --iphoto-library-plist iPhoto\ Library/AlbumData.xml --album-export From_iPhoto 2> iphoto-converter.log
2. Review the log and cleanup in iPhoto as desired.
3. Run export.

       RUST_LOG=info ./iphoto-exporter --iphoto-library-plist iPhoto\ Library/AlbumData.xml --album-export From_iPhoto 2> iphoto-converter.log
4. Import
In the Photos App, Choose Import and select "keep folder structure". Choose top-level directory From_iPhoto. 
