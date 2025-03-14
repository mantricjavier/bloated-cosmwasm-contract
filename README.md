# Bloated cosmwasm contract

- To increase the size of the optimized wasm, just run  

`dd if=/dev/urandom bs=1000k count=1 status=progress | base64 > filler.txt`

where `1000k` is the size of the file in kilobytes.

- Compile the code
- Boom! Bloated wasm