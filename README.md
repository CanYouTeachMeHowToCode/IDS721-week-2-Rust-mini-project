# IDS721 Weekly Rust Mini Project

## Week 2

A Basic Calculater that supports plus('+'), minus('-'), multiply('*') and division('/') (will round down to nearest integer). Unfortunately, it currently does not support computations with parentheses ('('). 

### Usage
> Run `cargo run -- basic-calculator -i [expr]` to get the value of the final result of expression `expr` after evaluation; `expr` is `String` type; e.g. the output of running command `cargo run -- basic-calculator -i "2*2+3*4"` is 
`16`, since $ 2 \times 2 + 3 \times 4 = 4 + 12 = 16$ by doing some quick math.

## Week 3

A Integer-to-Roman-Integer converter that takes in an Integer and return a Roman Integer (e.g. it will convert 19 to 'XIX')

### Usage
> Run `cargo run -- integer-to-roman -i [num]` to get the Roman Integer representation of the input number [num]; `num` is `i32` type; e.g. the output of running command `cargo run -- integer-to-roman -i 19` is `XIX`, and the output of running command `cargo run -- integer-to-roman -i 514` is `DXIV`. 

## Week 4

A longest palidromic substring finder that takes in an string and returns its longest substring such that the substring is also a palindrome. 

### Usage
> Run `cargo run -- longest-parlindromic-substring -i [s]` to get the longest palidromic substring of the input string [s]; `s` is `String` type; e.g. the output of running command `cargo run -- longest-parlindromic-substring -i "babadba"` is `bab`, and the output of running command `cargo run -- longest-parlindromic-substring -i "asdwfesddsfsdsdsdsdsdsgvqwerwqsdfsadas"` is `sdsdsdsdsds`. 

## Week 5

An IP address restorer that takes in a string and return all possible valid IP addresses (in standard format, e.g. 245.255.255.255)

### Usage
> Run `cargo run -- restore-ip-addresses -i [s]` to get the all possible restored IP addresses of the input string [s]; `s` is `String` type; e.g. the output of running command `cargo run -- restore-ip-addresses -i "25525511135"` is 
```
[
    "255.255.11.135",
    "255.255.111.35",
]
```
> , and the output of running command `cargo run -- restore-ip-addresses -i "1010238"` is 
```
[
    "1.0.10.238",
    "1.0.102.38",
    "10.102.3.8",
    "10.10.2.38",
    "10.10.23.8",
    "10.1.0.238",
    "101.0.2.38",
    "101.0.23.8",
]
```
> ; if the input string is unable to form any valid IP addresses, it will return an empty list `[]`.

## Week 6

An Excel sheet column title converter that takes in a column title in string format and return its corresponding column number (e.g. "A" -> 1, "C" -> 3, "Z" -> 26, "AA" -> 27, "ASD" -> 1174)

### Usage
> Run `cargo run -- convert-title-to-num -i [title]` to get the  corresponding column number for input title string `title`; `title` is `String` type; e.g. the output of running command `cargo run -- convert-title-to-num -i "AA"` is `27` and the output of running command `cargo run -- convert-title-to-num -i "ASD"` is `1174`

> Note that the input string `title` will be automatically converted to upper case in order to align with the Excel column title format. For instance, the outputs of running command `cargo run -- convert-title-to-num -i "ASD"` and `cargo run -- convert-title-to-num -i "asd"` are both `1174`.

## Week 7

An anagram generator that takes in a string return all of its possible anagrams in lexicographical order. An **Anagram** is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

### Usage
> Run `cargo run -- find-anagrams -i [s]` to get the anagrams of `s`. `s` is `String` type; e.g. the output of running command `cargo run -- find-anagrams -i "race"` is 
```
["rsut", "rtsu", "rust", "ruts", "srut", "stru", "trsu", "tsru", "turs", "urst", "urts", "utrs"]
```
> and the output of running command `cargo run -- find-anagrams -i "python"` is 
```
["hnoytp", "hnptyo", "hntoyp", "hnypto", "hnytop", "hontyp", "honytp", "hotpyn", "hoytpn", "hpotyn", "hptyon", "hpynto", "hpytno", "htnoyp", "htnpyo", "htnyop", "htopyn", "htoypn", "htpoyn", "htpyon", "htynpo", "htyonp", "htyopn", "htypno", "htypon", "hynotp", "hyntpo", "hyotnp", "hypnto", "hypton", "hytopn", "hytpno", "nhoytp", "nhptyo", "nhtoyp", "nhypto", "nhytop", "nohtyp", "noptyh", "nothyp", "notyph", "noyhtp", "noypth", "nphtyo", "npoyth", "npthyo", "npytoh", "nthpyo", "nthyop", "ntohyp", "ntopyh", "ntoyph", "ntpyoh", "ntyhpo", "nyhotp", "nyhpto", "nyhtop", "nyhtpo", "nyohtp", "nyopth", "nyotph", "nyphto", "nythop", "nythpo", "ohntyp", "ohnytp", "ohtpyn", "ohytpn", "onhtyp", "onptyh", "onthyp", "ontyph", "onyhtp", "onypth", "ophtyn", "opntyh", "opnyth", "opthyn", "optnyh", "optyhn", "opynth", "opythn", "othypn", "otnpyh", "otphyn", "otpyhn", "otyhnp", "otynph", "otyphn", "oyhtnp", "oyhtpn", "oyntph", "oypnth", "oypthn", "oythnp", "oythpn", "oytnph", "oytphn", "photyn", "phtyon", "phynto", "phytno", "pnhtyo", "pnoyth", "pnthyo", "pnytoh", "pohtyn", "pontyh", "ponyth", "pothyn", "potnyh", "potyhn", "poynth", "poythn", "pthoyn", "ptnhyo", "ptnyoh", "ptonyh", "ptoyhn", "ptyhon", "ptyohn", "pyhtno", "pyhton", "pynhto", "pyntoh", "pyothn", "pythno", "python", "pytnoh", "pytohn", "thnoyp", "thnpyo", "thnyop", "thopyn", "thoypn", "thpoyn", "thpyon", "thynpo", "thyonp", "thyopn", "thypno", "thypon", "tnhpyo", "tnhyop", "tnohyp", "tnopyh", "tnoyph", "tnpyoh", "tnyhpo", "tohypn", "tonpyh", "tophyn", "topyhn", "toyhnp", "toynph", "toyphn", "tphoyn", "tpnhyo", "tpnyoh", "tponyh", "tpoyhn", "tpyhon", "tpyohn", "tyhnpo", "tyhonp", "tyhpon", "tynhop", "tyohpn", "tyophn", "typhno", "typnoh", "typohn", "yhnotp", "yhntpo", "yhotnp", "yhpnto", "yhpton", "yhtopn", "yhtpno", "ynhotp", "ynhpto", "ynhtop", "ynhtpo", "ynohtp", "ynopth", "ynotph", "ynphto", "ynthop", "ynthpo", "yohtnp", "yohtpn", "yontph", "yopnth", "yopthn", "yothnp", "yothpn", "yotnph", "yotphn", "yphtno", "yphton", "ypnhto", "ypntoh", "ypothn", "ypthno", "ypthon", "yptnoh", "yptohn", "ythnpo", "ythonp", "ythpon", "ytnhop", "ytohpn", "ytophn", "ytphno", "ytpnoh", "ytpohn"]
```

## Week 8

A Strobogrammatic Number determiner that checks if the input number is a **Strobogrammatic Number** or not. A **Strobogrammatic Number** is s a number that looks the same when rotated 180 degrees (looked at upside down), such as `69`, `18081`, `88`, etc. (After rotating 180 degrees, `0`, `1`, `8` looks the same, `6` looks like `9` and vice versa.)

### Usage
> Run `cargo run -- check-strobogrammatic -i [n]` to get whether or not the input number [n] is strobogrammatic; `n` is `String` type; e.g. the output of running command `cargo run -- check-strobogrammatic -i 18081` is `True`, and the output of `cargo run -- check-strobogrammatic -i 696` is `False`.

## Week 9

An Gray Code generator that generates the **n-bit gray code sequence** of any input `n` such that 
- Every integer is in the inclusive range `[0, 2^n-1]`
- The first integer is `0`
- An integer appears no more than once in the sequence
- The *binary representation* of every pair of adjacent integers differs by exactly one bit
- The *binary representation* of the first and last integers differs by exactly one bit.

For example, one valid **n-bit gray code sequence** with `n=3` is `[0, 1, 3, 2, 6, 7, 5, 4]` 

### Usage
> Run `cargo run -- generate-gray-code -i [n]` to get one of the valid `n`-bit gray code sequence; `n` is `i32` type; e.g. the output of running command `cargo run -- generate-gray-code -i 3` is `[0, 1, 3, 2, 6, 7, 5, 4]`, and the output of running command `cargo run -- generate-gray-code -i 10` is 

```
[0, 1, 3, 2, 6, 7, 5, 4, 12, 13, 15, 14, 10, 11, 9, 8, 24, 25, 27, 26, 30, 31, 29, 28, 20, 21, 23, 22, 18, 19, 17, 16, 48, 49, 51, 50, 54, 55, 53, 52, 60, 61, 63, 62, 58, 59, 57, 56, 40, 41, 43, 42, 46, 47, 45, 44, 36, 37, 39, 38, 34, 35, 33, 32, 96, 97, 99, 98, 102, 103, 101, 100, 108, 109, 111, 110, 106, 107, 105, 104, 120, 121, 123, 122, 126, 127, 125, 124, 116, 117, 119, 118, 114, 115, 113, 112, 80, 81, 83, 82, 86, 87, 85, 84, 92, 93, 95, 94, 90, 91, 89, 88, 72, 73, 75, 74, 78, 79, 77, 76, 68, 69, 71, 70, 66, 67, 65, 64, 192, 193, 195, 194, 198, 199, 197, 196, 204, 205, 207, 206, 202, 203, 201, 200, 216, 217, 219, 218, 222, 223, 221, 220, 212, 213, 215, 214, 210, 211, 209, 208, 240, 241, 243, 242, 246, 247, 245, 244, 252, 253, 255, 254, 250, 251, 249, 248, 232, 233, 235, 234, 238, 239, 237, 236, 228, 229, 231, 230, 226, 227, 225, 224, 160, 161, 163, 162, 166, 167, 165, 164, 172, 173, 175, 174, 170, 171, 169, 168, 184, 185, 187, 186, 190, 191, 189, 188, 180, 181, 183, 182, 178, 179, 177, 176, 144, 145, 147, 146, 150, 151, 149, 148, 156, 157, 159, 158, 154, 155, 153, 152, 136, 137, 139, 138, 142, 143, 141, 140, 132, 133, 135, 134, 130, 131, 129, 128, 384, 385, 387, 386, 390, 391, 389, 388, 396, 397, 399, 398, 394, 395, 393, 392, 408, 409, 411, 410, 414, 415, 413, 412, 404, 405, 407, 406, 402, 403, 401, 400, 432, 433, 435, 434, 438, 439, 437, 436, 444, 445, 447, 446, 442, 443, 441, 440, 424, 425, 427, 426, 430, 431, 429, 428, 420, 421, 423, 422, 418, 419, 417, 416, 480, 481, 483, 482, 486, 487, 485, 484, 492, 493, 495, 494, 490, 491, 489, 488, 504, 505, 507, 506, 510, 511, 509, 508, 500, 501, 503, 502, 498, 499, 497, 496, 464, 465, 467, 466, 470, 471, 469, 468, 476, 477, 479, 478, 474, 475, 473, 472, 456, 457, 459, 458, 462, 463, 461, 460, 452, 453, 455, 454, 450, 451, 449, 448, 320, 321, 323, 322, 326, 327, 325, 324, 332, 333, 335, 334, 330, 331, 329, 328, 344, 345, 347, 346, 350, 351, 349, 348, 340, 341, 343, 342, 338, 339, 337, 336, 368, 369, 371, 370, 374, 375, 373, 372, 380, 381, 383, 382, 378, 379, 377, 376, 360, 361, 363, 362, 366, 367, 365, 364, 356, 357, 359, 358, 354, 355, 353, 352, 288, 289, 291, 290, 294, 295, 293, 292, 300, 301, 303, 302, 298, 299, 297, 296, 312, 313, 315, 314, 318, 319, 317, 316, 308, 309, 311, 310, 306, 307, 305, 304, 272, 273, 275, 274, 278, 279, 277, 276, 284, 285, 287, 286, 282, 283, 281, 280, 264, 265, 267, 266, 270, 271, 269, 268, 260, 261, 263, 262, 258, 259, 257, 256, 768, 769, 771, 770, 774, 775, 773, 772, 780, 781, 783, 782, 778, 779, 777, 776, 792, 793, 795, 794, 798, 799, 797, 796, 788, 789, 791, 790, 786, 787, 785, 784, 816, 817, 819, 818, 822, 823, 821, 820, 828, 829, 831, 830, 826, 827, 825, 824, 808, 809, 811, 810, 814, 815, 813, 812, 804, 805, 807, 806, 802, 803, 801, 800, 864, 865, 867, 866, 870, 871, 869, 868, 876, 877, 879, 878, 874, 875, 873, 872, 888, 889, 891, 890, 894, 895, 893, 892, 884, 885, 887, 886, 882, 883, 881, 880, 848, 849, 851, 850, 854, 855, 853, 852, 860, 861, 863, 862, 858, 859, 857, 856, 840, 841, 843, 842, 846, 847, 845, 844, 836, 837, 839, 838, 834, 835, 833, 832, 960, 961, 963, 962, 966, 967, 965, 964, 972, 973, 975, 974, 970, 971, 969, 968, 984, 985, 987, 986, 990, 991, 989, 988, 980, 981, 983, 982, 978, 979, 977, 976, 1008, 1009, 1011, 1010, 1014, 1015, 1013, 1012, 1020, 1021, 1023, 1022, 1018, 1019, 1017, 1016, 1000, 1001, 1003, 1002, 1006, 1007, 1005, 1004, 996, 997, 999, 998, 994, 995, 993, 992, 928, 929, 931, 930, 934, 935, 933, 932, 940, 941, 943, 942, 938, 939, 937, 936, 952, 953, 955, 954, 958, 959, 957, 956, 948, 949, 951, 950, 946, 947, 945, 944, 912, 913, 915, 914, 918, 919, 917, 916, 924, 925, 927, 926, 922, 923, 921, 920, 904, 905, 907, 906, 910, 911, 909, 908, 900, 901, 903, 902, 898, 899, 897, 896, 640, 641, 643, 642, 646, 647, 645, 644, 652, 653, 655, 654, 650, 651, 649, 648, 664, 665, 667, 666, 670, 671, 669, 668, 660, 661, 663, 662, 658, 659, 657, 656, 688, 689, 691, 690, 694, 695, 693, 692, 700, 701, 703, 702, 698, 699, 697, 696, 680, 681, 683, 682, 686, 687, 685, 684, 676, 677, 679, 678, 674, 675, 673, 672, 736, 737, 739, 738, 742, 743, 741, 740, 748, 749, 751, 750, 746, 747, 745, 744, 760, 761, 763, 762, 766, 767, 765, 764, 756, 757, 759, 758, 754, 755, 753, 752, 720, 721, 723, 722, 726, 727, 725, 724, 732, 733, 735, 734, 730, 731, 729, 728, 712, 713, 715, 714, 718, 719, 717, 716, 708, 709, 711, 710, 706, 707, 705, 704, 576, 577, 579, 578, 582, 583, 581, 580, 588, 589, 591, 590, 586, 587, 585, 584, 600, 601, 603, 602, 606, 607, 605, 604, 596, 597, 599, 598, 594, 595, 593, 592, 624, 625, 627, 626, 630, 631, 629, 628, 636, 637, 639, 638, 634, 635, 633, 632, 616, 617, 619, 618, 622, 623, 621, 620, 612, 613, 615, 614, 610, 611, 609, 608, 544, 545, 547, 546, 550, 551, 549, 548, 556, 557, 559, 558, 554, 555, 553, 552, 568, 569, 571, 570, 574, 575, 573, 572, 564, 565, 567, 566, 562, 563, 561, 560, 528, 529, 531, 530, 534, 535, 533, 532, 540, 541, 543, 542, 538, 539, 537, 536, 520, 521, 523, 522, 526, 527, 525, 524, 516, 517, 519, 518, 514, 515, 513, 512]
```

## Week 10

An invalid parentheses remover that removes the minimum number of invalid parentheses to make the input string `s` valid; i.e. every open parenthesis `(` is closed by an close parethesis `)` and return all possible unique valid strings after removal.

### Usage
> Run `cargo run -- remove-invalid-parentheses -i "()(Python))((Rust))))"` to get all the valid results after removal of minimum number of invalid parentheses; `s` is `String` type; e.g. the output of running command `cargo run -- remove-invalid-parentheses -i "()(Python))((Rust))))"` is 
```
[
    "((Python((Rust))))",
    "((Python)((Rust)))",
    "((Python))((Rust))",
    "()(Python((Rust)))",
    "()(Python)((Rust))",
]
```
> and the output of running command `cargo run -- remove-invalid-parentheses -i "("` is `[""]`

## Week 11

An Unix path simplifier that converts an input **absolute path** `s` to a file/directory in a Unix-style file system to the simplified **canonical path**.

In a Unix-style file system, a period '.' refers to the current directory, a double period '..' refers to the directory up a level, and any multiple consecutive slashes (i.e. '//') are treated as a single slash '/'. 

The **canonical path** should have the following format:

- The path starts with a single slash '/'.
- Any two directories are separated by a single slash '/'.
- The path does not end with a trailing '/'.
- The path only contains the directories on the path from the root directory to the target file or directory (i.e., no period '.' or double period '..')

### Usage
> Run `cargo run -- remove-invalid-parentheses -i "()(Python))((Rust))))"` to get all the valid results after removal of minimum number of invalid parentheses; `s` is `String` type; e.g. the output of running command `cargo run -- simplify-path -i "home///user/yilunwu//"` is `"/home/user/yilunwu"` and the output of running command `cargo run -- simplify-path -i "/home/rust/../python/././PyTorch/../HuggingFace"` is `"/home/python/HuggingFace"`