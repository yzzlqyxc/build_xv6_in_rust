#[allow(dead_code)]

pub const NPROC : u64 = 64;  // maximum number of processes
pub const NCPU : u64 = 8;  // maximum number of CPUs
pub const NOFILE : u64 = 16;  // open files per process
pub const NFILE : u64 = 100;  // open files per system
pub const NINODE : u64 = 50;  // maximum number of active i-nodes
pub const NDEV : u64 = 10;  // maximum major device number
pub const ROOTDEV : u64 = 1;  // device number of file system root disk
pub const MAXARG : u64 = 32;  // max exec arguments
pub const MAXOPBLOCKS : u64 = 10;  // max # of blocks any FS op writes
pub const LOGSIZE : u64 = MAXOPBLOCKS*3;  // max data blocks in on-disk log
pub const NBUF : u64 = MAXOPBLOCKS*3;  // size of disk block cache
pub const FSSIZE : u64 = 2000;  // size of file system in blocks
pub const MAXPATH : u64 = 128;   // maximum file path name