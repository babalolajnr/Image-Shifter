# Image-Shifter
Manipulate images from the terminal

- [Image-Shifter](#image-shifter)
  - [Notice](#notice)
  - [How to use](#how-to-use)
  - [Actions](#actions)
  - [Commands](#commands)
    - [Grayscale](#grayscale)
    - [Brighten](#brighten)
## Notice
This is still very much in development and many things will not work properly😊

## How to use
Clone the repository and build the project. The run the excutable generated in the terminal

```bash
target\debug\rust-image-processor.exe <input> <action> 
```
## Actions
1. Grayscale
2. Brighten
## Commands
### Grayscale
```bash
target\debug\rust-image-processor.exe <input> grayscale 
```
### Brighten

```bash
target\debug\rust-image-processor.exe <input> brighten <value> 
```
the `value` parameter is the amount to brighten each pixel by. Negative values decrease the brightness and positive values increase it. 