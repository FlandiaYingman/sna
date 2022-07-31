# SNA (sna)

An external tool to add "smart extract" feature to any archiver software you'd like. 

SNA stands for "SNA's not archiver".

This is a project for me to study and learn and practice Rust. I expect it can, and will work stable. Since it doesn't do the actual extract job, instead it calls up an installed archiver software and delegates the job, I wish that I will complete it sucessfully as a Rust beginner.

## What is "Smart Extract"? 

"Smart Extract" works as follows: 

1. If there is only 1 file inside the archive file, that file will be extracted into the currect working directory.

2. If there is only 1 directory under the root of the archive file, that directory will be extracted into the working directory.

3. If there are files and directories under the root of the archive file, these files and directories will be extracted into a new directory named by the archive file name (without extension) under the currect working directory.

For `~/a.zip`, and its content
```
~/a.zip/foo.txt
```
will be smart extracted to 
```
~/foo.txt
```

For `~/b.zip`, and its content
```
~/b.zip/foo
~/b.zip/foo/bar1.txt
~/b.zip/foo/bar2.txt
~/b.zip/foo/bar3.txt
```
will be smart extracted to
```
~/foo
~/foo/bar1.txt
~/foo/bar2.txt
~/foo/bar3.txt
```

For `~/c.zip`, and its content
```
~/b.zip/foo1.txt
~/b.zip/foo2.txt
~/b.zip/foo3.txt
```
will be smart extracted to
```
~/b/foo1.txt
~/b/foo2.txt
~/b/foo3.txt
```