# Deletedisk
This program is used to overwrite disks with zeros.  
This process should make it harder to recover data from the disk.  
This program is so far only available and tested on GNU/Linux.  

# Using deletedisk
The "syntax" looks like the following:  
deletedisk *Disk* *Unit* *Chunksize*  
*Disk* -> Path to the disk (example: /dev/sdc)  
*Unit* -> Chunksize unit (b,B,k,K,m,M,g,G,t,T)  
*Chunksize* -> Size of the chunk (> 0)

## Units
b = Bytes  
B = Bytes  
k = KB  
K = KiB  
m = MB  
M = MiB  
g = GB  
G = GiB  
t = TB  
T = TiB

## Note  
The *Chunksize* * *Unit* will be the allocated memory.  
The result must not be larger than the physical RAM you have available.  
For example:  
deletedisk /dev/sdc M 4 - This will allocate 4 MiB of memory.  

# Building
```
git clone https://gitlab.com/SpizzyCoder/deletedisk.git
cd deletedisk
cargo build --release
```
The executable is now located in ./target/release/deletedisk
