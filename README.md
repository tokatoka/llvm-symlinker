## LLVM symlinker

When you install llvm the "Automatic installation script" from https://apt.llvm.org/. 
It will create files named "-<VERSION>" for example "clang-19".

This tool will remove the last suffix and create a symlink to all the executables that you installed.
Works on linux only.
