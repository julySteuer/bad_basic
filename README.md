# BASIC CLONE
## Introduction
This is a clone of the 1980's Apple basic programming langauge. It is not complete and propably never will be complete to the specs of the original. Its just a side project of mine so it won't live up to industry standart of wirting programming langauges.
## Install
1. *git clone* this repo from master if not stated otherwise
2. start is with *cargo run*
3. Enjoy!
## Usage and Getting Started
After you ran the rust programm you will be greeted with the __Bad Basic__ shell. Like in the original closing bracets (__]__) shows the lines you are working in and the commands you used in this session. Like in the original you have to specify a line number in the beginning otherwise nothing will work.
### General Structure
#### Variable
___
```
]1 LET x=1
```
1. __]__ stands for the line. It is automatically pasted by the Shell.
2. __LET__ stands for a variable declaration
3. __x__ is the name(*identifier*) of the variable
4. __=__ is the assign operator
5. __1__ is the value of the variable
#### Functions
___
````
]2 PRINT(x)
````
1. __PRINT__ is the name of the function. Functions in basic are written in caps
2. __(__ and __)__ wrap around the arguments
3. __x__ is the argument of the function
#### Term
___
```
]3 1+2
```
1. Add __1__ and __2__ with the __+__ operator
