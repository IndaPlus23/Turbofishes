# DD1337 Week 6

## Getting started with C

### Install GCC toolchain
> Note: This is already installed in most linux-distros, so if you are using something like Ubuntu (hint for windows users) you can skip this
1) Install [MingW](https://www.mingw-w64.org/). *Windows-users are recommended to just use WSL*. 
2) Test your installation by entering `gcc --help` in your terminal or command prompt(*host pleb *host).

Congrats dear programmer. You can now compile your C files by entering the following command.
```
gcc ./program.c -o name_of_executable
```
To run: 
```
./name_of_executable
```

### Prepare for your assigment

1) Create a repository named `<KTH_ID>-clang`.
2) Clone your newly created repository.
3) Create one `.c` file per subassignment. Name then descriptivly.  

For help with code setup, begin by copying the contents of `./kattis_template/main.c` into your `.c` file.

## Assignment 1 - Kattis

Solve [A Different Problem](https://open.kattis.com/problems/different), which is now on Open Kattis and not KTH's Kattis (i.e. not locked behind courses😄).

See `./minimal_scalar_product` for a Kattis solution example.

**Don't forget** to include screenshots of Kattis to prove that your solutions work.

## Assignment 2.1 - Encryption

Create a program called `encrypt.c` which takes the name of a text file as a command line argument and encrypts the content into a new file prefixed with "encrypted": `encrypted_<previous name>.txt`.

It is optional what encryption algorithm to use but it needs to use a key of some sort (for rotation ciphers, a number to rotate with, vigenere has a word of text as the key, RSA has two keys) some recommended ones are:
- [Caesar cipher](https://en.wikipedia.org/wiki/Caesar_cipher) 
- [Vigenere cipher](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher)
- [XOR cipher](https://en.wikipedia.org/wiki/XOR_cipher)

Very hard:
- [RSA](https://sv.wikipedia.org/wiki/RSA)
- [DES](https://en.wikipedia.org/wiki/Data_Encryption_Standard) 

> Note: the algorithm only has to support upper and lowercase A-Z.

The key should be specified as command line arguments along with the name of the text file:
```
./encrypt text.txt 12
```
Which outputs: `encrypted_text.txt`
  
 
Write in your `README.md` what algorithm you've implemented and how to use the program.
  
  
Command line arguments are retrieved from `argv` when using the following function header:
```c
int main(int argc, char **argv) {}
```
- `int argc` is the argument count, i.e. the number of arguments supplied. This is useful for either error checking that the correct number of arguments where supplied or looping through the arguments when there can be a varied amount of them.
- `char **argv` are the argument values and is a C-style string array since `char*` is a string and arrays are just pointers, hence the double pointer.


## Assignment 2.2 - Decryption

Create a new program called `decrypt.c` which takes the same command line arguments as `encrypt.c` but instead reverses the encryption of the supplied text file. The result shall be in a new file with either the prefix "encrypted" removed or with a new prefix "decrypted" for fun growing names🤪.

## Questions

You don't have to write the answers to these but think about them and know the answers during our next lesson.

### Data control

Observe the following code:

```c
int length = 0;
scanf("%d", &length); 

int* vector = malloc(length * sizeof(int));
for (int i = 0; i < length; i++) 
    scanf("%d", &vector[i]);

free(vector);
```

Know the answer of the following question:
- What is happening line for line?

### Gammal hederlig läsförståelse

Observe the following function:

```c
#include <stdio.h>
#include <time.h>

// Assume that foo is a function which takes longer time to execute
// for a larger value n.

void someFunction(void (*f)(int), int milliseconds, int n) {
    int milliseconds_since = 1;
    int end = 1;

    do {
        n = (int)(n * (double)end / milliseconds_since);

        milliseconds_since = clock() * 1000 / CLOCKS_PER_SEC;
        end = milliseconds_since + milliseconds;

        foo(n);

        milliseconds_since = clock() * 1000 / CLOCKS_PER_SEC;
    } while (end - milliseconds_since > 100);

    printf("\nLargest n: %d", n);
}
```

Know the answer of the following question:
- What does `someFunction` do?

### Data types are like whaaat?

Observe the following function:

```c
int x;

printf("Nothing: %d\nGive x a value: ", x);

scanf("%d", x);

printf("\nYour value is: %d\n", x);
```

Know the answer of the following question:
- What is printed?
