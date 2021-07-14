# Image-Shifter
Manipulate images from the terminal

- [Image-Shifter](#image-shifter)
  - [Notice](#notice)
  - [How to use](#how-to-use)
  - [Commands](#commands)
    - [Grayscale](#grayscale)
    - [Brighten](#brighten)
    - [Huerotate](#huerotate)
    - [Contrast](#contrast)
    - [Crop](#crop)
## Notice
This is still very much in development and many things will not work properly😊

## How to use
Clone the repository and build the project. The run the excutable generated in the terminal

```bash
target\debug\rust-image-processor.exe <input> <action> 
```
## Commands
### Grayscale
Convert image to black and white.
```bash
target\debug\rust-image-processor.exe <input> grayscale 
```
### Brighten
Brighten the supplied image. `value` is the amount to brighten each pixel by. Negative values decrease the brightness and positive values increase it.
```bash
target\debug\rust-image-processor.exe <input> brighten --value <value> 
``` 

### Huerotate
Hue rotate the supplied image. `value` is the degrees to rotate each pixel by. 0 and 360 do nothing, the rest rotates by the given degree value. just like the css webkit filter hue-rotate(180).
```bash
target\debug\rust-image-processor.exe <input> huerotate --value <value> 
```
### Contrast
Adjust the contrast of the supplied image. `contrast` is the amount to adjust the contrast by. Negative values decrease the contrast and positive values increase the contrast.

```bash
target\debug\rust-image-processor.exe <input> contrast --contrast <value> 
```
### Crop
Crop image. `height` is the height of the output image. `width` is the width of the output image. `x` is position of the top left corner on the x-axis. `y` is position of the top left corner on the y-axis.

```bash
target\debug\rust-image-processor.exe <input> crop --height <value> --width <value> --x <value> --y <value>  
```