# SNA (sna)

An external tool to add "Extract Here (Smart)" on Windows.

SNA stands for "SNA's not an archiver". Since it calls 7-Zip's command line for extraction.

## What is "Extract Here (Smart)"?

"Smart Extract" works as follows:

- If there is only **one** file directly under the archive's root, the file will be extracted into the current working
  directory.

- If there is only **one** directory directly under the archive's root, the directory will be extracted into the current
  working directory.

- Otherwise, all contents of the archive are extracted into a new directory named by the archive's stem (stem means the
  file name without the extension.)
