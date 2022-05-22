# ODisk
This program is used to overwrite disks with zeros or with random data.  
This process should make it harder to recover data from the disk.  
This program is so far only available and tested on GNU/Linux.  

## Note  
The *Chunksize* * *Unit* will be the allocated memory.  
The result must not be larger than the physical RAM you have available.  
For example:  
odisk -m zero -u mib -c 4 /dev/sdc - This will allocate 4 MiB of memory.  

# Building
```
git clone https://github.com/SpizzyCoder/odisk.git
cd odisk
cargo install --path .
```
The executable is now located in your ./cargo/bin folder.
