# SNA (sna)

An external tool to add "Extract Here (Smart)" on Windows.

SNA stands for "SNA's not an archiver". Since it calls 7-Zip's command line for extraction.

An installer is provided to help install SNA. It also adds "Extract Here (Smart)" to the context menu of archive files.

Note that 7-Zip must be installed on the machine and its binaries must be included in PATH.
If you do not know how to do this,
just install [NanaZip](https://apps.microsoft.com/store/detail/nanazip/9N8G7TSCL18R) from Windows Store.

## What is "Extract Here (Smart)"?

"Smart Extract" works as follows:

- If there is only **one** file directly under the archive's root, the file will be extracted into the current working
  directory.

- If there is only **one** directory directly under the archive's root, the directory will be extracted into the current
  working directory.

- Otherwise, all contents of the archive are extracted into a new directory named by the archive's stem (stem means the
  file name without the extension.)
