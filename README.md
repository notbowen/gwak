# Gwak

Read: gwak gwak (the sound made when sucking someone off)

This project aims to help improve the lives of CTF players
by allowing them to clone all of the CTFd challenge files
onto their local machine.

The files will be downloaded into different folders based
on their categories. The folder structure should be customizable,
with templates in place for users to put their own solve script
or folder structure for easier organization.

## To-Do

- [ ] Implement an adapter for CTFd and get the basic structure out (In Progress)
- [ ] Implement custom errors
- [ ] Create a default config directory programatically
- [ ] Figure out how to parse a default config directory
- [ ] Implement multithreading to speed up downloading

## Rough Idea

1. Users run the binary with their CTFd API key
2. Script cycles through the challs and downloads each file (idfk how)
3. Profit??!!

Users should be able to have a configuration directory where they
can customise their cloned structure, i.e.:

```txt
folder
|- web
 |- solve.py
 |- notes.md
|- pwn
 |- solve.py
 |- notes.md
|- rev
|- osint
```

Each file, such as `solve.py` can be configured with "templating"
style stuff? I'm thinking something like:

```py
# solve.py (pwn)

from pwn import *

elf = context.binary = ELF("%filename%")

# ...
```

Web challs can have special parameters such as `%url%` which gives
them the conn string so that they can populate their files with
neccessary info?

```py
# solve.py (web)

import requests

r = requests.get("%url%")

# ...
```

More category specific features could/would be added along the way.

